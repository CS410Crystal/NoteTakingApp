use note_taking_app_lib::{save_data, load_data, NotesState, Notes};
use std::sync::Mutex;
use std::fs;

#[test]
fn test_save_and_load() {
    let state = NotesState(Mutex::new(Notes::new()));

    save_data(tauri::State::from(&state));
    load_data(tauri::State::from(&state));

    assert!(
        fs::metadata("../saves.json").is_ok(),
        "Save file should exist"
    );
}
