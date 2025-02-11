use std::{alloc::System, time::{SystemTime, UNIX_EPOCH}};

use pdf_writer::{Content, BuiltInFont, Pdf, Ref, Str};
use lopdf::Document;
use tauri::State;

use crate::{Notes, NotesState};

/**
 * Add a note to the Notes state
 * return true if success(, false if existing note title)
 */
#[tauri::command]
pub fn create_note(state: State<NotesState>, name: String) -> bool {
    // let note: Note = serde_json::from_str(&object).unwrap();
    let mut note: Note = Note::new();
    note.name = name.clone();
    let mut notes = state.0.lock().unwrap();

    //don't add if same name
    for lock_note in &notes.note_list {
        if lock_note.name == name {
            return false;
        }
    }

    notes.note_list.push(note.clone());
    //
    println!("Saved new note: {}",name);
    //
    return true;
}

#[tauri::command]
pub fn edit_note(state: State<NotesState>, object: String) -> bool {
    let note: Note = serde_json::from_str(&object).unwrap();
    let mut notes = state.0.lock().unwrap();

    let mut note_index: usize = 0;
    let mut can_edit: bool = false;
    for lock_note in &notes.note_list {
        if lock_note.name == note.name {
            can_edit = true;
            break;
        }
        note_index += 1;
    }

    if can_edit == true {
        notes.note_list[note_index] = note;
        return true;
    }
    return false;
}

#[tauri::command]
pub fn delete_note(state: State<NotesState>, name: String) -> bool {
    let mut notes = state.0.lock().unwrap();

    let mut note_index: usize = 0;
    let mut can_edit: bool = false;
    for lock_note in &notes.note_list {
        if lock_note.name == name {
            can_edit = true;
            break;
        }
        note_index += 1;
    }

    if can_edit == true {
        notes.note_list.remove(note_index);
        return true;
    }
    return false;
}

#[tauri::command]
pub fn get_notes(state: State<NotesState>) -> String {
    let notes = state.0.lock().unwrap();
    return serde_json::to_string(&notes.note_list).expect("can't serialize note list");
}

#[tauri::command]
pub fn get_note_by_name(state: State<NotesState>, name: String) -> String {
    let notes = state.0.lock().unwrap();
    for test_note in &notes.note_list {
        if test_note.name == name {
            return serde_json::to_string(&test_note).expect("can't seralize note struct");
        }
    }
    return "note not found".to_string();
}

#[tauri::command]
pub fn export_notes(state: State<NotesState>, format: String, file_path: String) -> bool {
    let notes = state.0.lock().unwrap();

    match format.as_str() {
        "json" => {
            let data = serde_json::to_string_pretty(&notes.note_list).expect("can't serialize");
            return std::fs::write(&file_path, data).is_ok();
        }
        "txt" => {
            let data = notes.note_list.iter()
                .map(|n| format!("Title: {}\nContent:\n{}\n----\n", n.name, n.content))
                .collect::<Vec<String>>().join("\n");
            return std::fs::write(&file_path, data).is_ok();
        }
        "md" => {
            let data = notes.note_list.iter()
                .map(|n| format!("# {}\n\n{}\n\n---", n.name, n.content))
                .collect::<Vec<String>>().join("\n");
            return std::fs::write(&file_path, data).is_ok();
        }
        "pdf" => {
            return export_notes_as_pdf(&notes.note_list, &file_path);
        }
        _ => false, // Unsupported format
    }
}

#[tauri::command]
pub fn export_notes_as_pdf(notes: &Vec<Note>, file_path: &str) -> bool {
    let mut pdf = pdf_writer::Pdf::new();
    let page_ref = pdf.add_page();
    let font_ref = pdf.add_builtin_font(pdf_writer::BuiltInFont::Helvetica).unwrap(); // Use Helvetica font

    let mut content = pdf_writer::Content::new();
    let mut y_offset = 700.0;

    for note in notes {
        content.set_font(font_ref, 14.0)
               .move_to(50.0, y_offset)
               .show(Str::from(note.name.as_str())) // Convert &str to Str
               .next_line(50.0, y_offset - 20.0)
               .show(Str::from(note.content.as_str()));
        y_offset -= 100.0;
    }

    pdf.add_page_content(page_ref, &content);
    let mut file = std::fs::File::create(std::Path::new(file_path)).expect("Failed to create PDF file");
    file.write_all(&pdf.finish()).is_ok()
}

#[tauri::command]
pub fn import_notes(state: State<NotesState>, format: String, file_path: String) -> bool {
    let mut notes = state.0.lock().unwrap();

    let file_content = match std::fs::read_to_string(&file_path) {
        Ok(content) => content,
        Err(_) => return false,
    };

    match format.as_str() {
        "json" => {
            match serde_json::from_str::<Vec<Note>>(&file_content) {
                Ok(imported_notes) => notes.note_list.extend(imported_notes),
                Err(_) => return false,
            }
        }
        "txt" | "md" => {
            let mut current_title = String::new();
            let mut current_content = String::new();

            for line in file_content.lines() {
                if line.starts_with("# ") {
                    if !current_title.is_empty() {
                        notes.note_list.push(Note {
                            name: current_title.clone(),
                            content: current_content.clone(),
                            last_updated: crate::current_timestamp(),
                        });
                    }
                    current_title = line.trim_start_matches("# ").to_string();
                    current_content.clear();
                } else {
                    current_content.push_str(line);
                    current_content.push('\n');
                }
            }

            if !current_title.is_empty() {
                notes.note_list.push(Note {
                    name: current_title.clone(),
                    content: current_content.clone(),
                    last_updated: crate::current_timestamp(),
                });
            }
        }
        "pdf" => {
            return import_notes_from_pdf(state, file_path);
        }
        _ => return false, // Unsupported format
    };

    true
}

#[tauri::command]
pub fn import_notes_from_pdf(state: State<NotesState>, file_path: String) -> bool {
    let mut notes = state.0.lock().unwrap();
    let doc = match Document::load(&file_path) {
        Ok(d) => d,
        Err(_) => return false,
    };

    let mut extracted_text = String::new();
    for (page_id, _) in doc.get_pages() { // Extract only first element
        if let Ok(text) = doc.extract_text(&[page_id]) {
            extracted_text.push_str(&text);
        }
    }

    if extracted_text.is_empty() {
        return false;
    }

    let note = Note {
        name: format!("Imported from {}", file_path),
        content: extracted_text,
        last_updated: current_timestamp(),
    };

    notes.note_list.push(note);
    true
}

#[derive(Clone)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Note {
    name: String,
    content: String,
    last_updated: u64,
}

impl Note {
    fn new() -> Note {
        let t = SystemTime::now();
        let since = t.duration_since(UNIX_EPOCH).expect("time went backwards");
        let since_millis = since.as_millis() as u64;
        return Note {
            name: "".to_string(),
            content: "".to_string(),
            last_updated: since_millis,
        };
    }
}

// mod tests {
//     use super::create_note;

//     #[test]
//     fn test_add_note() {
//         let note: String = "{\"content\":\"Hello World\"}"
//         create_note(state, note);
//     }
// }