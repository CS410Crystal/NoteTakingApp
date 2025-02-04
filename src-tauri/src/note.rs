use tauri::State;

use crate::NotesState;

/**
 * Add a note to the Notes state
 * return true if success(, false if existing note title)
 */
#[tauri::command]
pub fn create_note(state: State<NotesState>, object: String) -> bool {
    let note: Note = serde_json::from_str(&object).unwrap();
    let mut notes = state.0.lock().unwrap();
    let note_content = &note.content;
    notes.note_list.push(note.clone());
    //
    println!("Saved data: {}",note_content);
    //
    return true;
}

#[derive(Clone)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Note {
    content: String,
}

// mod tests {
//     use super::create_note;

//     #[test]
//     fn test_add_note() {
//         let note: String = "{\"content\":\"Hello World\"}"
//         create_note(state, note);
//     }
// }