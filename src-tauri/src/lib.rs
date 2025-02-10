///Container for backend scripts
mod note;

use note::{create_note, Note};
use std::{fs, sync::Mutex};
use tauri::State;
// use tauri::Manager;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct NotesState(pub Mutex<Notes>);

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Notes {
    note_list: Vec<Note>,
}

impl Notes {
    pub fn new() -> Notes {
        return Notes {
            note_list: Vec::new(),
        };
    }
    pub fn serial(&self) -> String {
        return serde_json::to_string(self).expect("can't serialize");
    }
}

//For now save as a json, figure out the other type later
const SAVEFILE: &str = r"..\saves.json";

#[tauri::command]
fn save_data(state: State<NotesState>) {
    let e = state.0.lock().unwrap().serial();
    fs::write(SAVEFILE, e).expect("Unable to write file");
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let notes = Notes::new();
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(NotesState(notes.into()))
        .invoke_handler(tauri::generate_handler![save_data, create_note])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
