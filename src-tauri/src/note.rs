use std::{alloc::System, time::{SystemTime, UNIX_EPOCH}};

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