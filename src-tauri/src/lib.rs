///Container for backend scripts
mod note;

use note::{create_note, edit_note, get_note_by_name, get_notes, Note};
use serde_json::Value;
use std::{fs::{self, File}, sync::Mutex};
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

#[tauri::command]
fn load_data(state: State<NotesState>) {
    let mut notes = state.0.lock().expect("could not lock mutex");
    let file_data = match fs::read_to_string(SAVEFILE) {
    Ok(data) => data,
    Err(_e) => {
        println!("creating new file");
        let _f = File::create(r"..\saves.json").expect("error when creating file");
        "".to_string()
    },
    };
    let _result = match serde_json::from_str(&file_data){
        Ok(file) => *notes = file,
        Err(_error) => {
            println!("trying to load partial data");
            if &file_data == "" {
                println!("File is empty, using default data");
                return;
            }
            let file_json: Value = serde_json::from_str(&file_data).expect("invalid json");
            if let Some(field) = file_json.get("note_list") {
                let value = serde_json::from_value(field.clone()).expect("could not extract value of note_list");
                notes.note_list = value;
            }
            println!("successfully loaded partial data")
        }
    };
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let notes = Notes::new();
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(NotesState(notes.into()))
        .invoke_handler(tauri::generate_handler![
            save_data,
            load_data,
            create_note,
            edit_note,
            get_notes,
            get_note_by_name
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
