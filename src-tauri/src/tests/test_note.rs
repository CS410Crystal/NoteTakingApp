use note_taking_app_lib::{create_note, edit_note, get_notes, NotesState, Notes};
use std::sync::Mutex;
use tauri::State;

#[test]
fn test_create_and_retrieve_notes() {
    let state = NotesState(Mutex::new(Notes::new()));

    create_note(State::from(&state), "Integration Test Note".to_string());
    let notes_json = get_notes(State::from(&state));
    let notes: Vec<Note> = serde_json::from_str(&notes_json).unwrap();

    assert_eq!(notes.len(), 1, "Expected one note after creation");
    assert_eq!(notes[0].name, "Integration Test Note", "Expected note name to match");
}

#[test]
fn test_edit_note_integration() {
    let state = NotesState(Mutex::new(Notes::new()));

    create_note(State::from(&state), "Editable Integration Note".to_string());

    let edited_note = Note {
        name: "Editable Integration Note".to_string(),
        content: "Updated Content".to_string(),
        last_updated: 123456789,
    };

    let json_note = serde_json::to_string(&edited_note).unwrap();
    let edit_result = edit_note(State::from(&state), json_note);

    assert!(edit_result, "Expected note to be edited successfully");

    let notes_json = get_notes(State::from(&state));
    let notes: Vec<Note> = serde_json::from_str(&notes_json).unwrap();

    assert_eq!(notes[0].content, "Updated Content", "Expected note content to be updated");
}
