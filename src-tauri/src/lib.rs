///Container for backend scripts
mod note;
mod dbManager;

use note::{create_note, delete_note, edit_note, get_note_by_name, get_notes, Note};
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

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Folder {
    name: String,
    created_at: u64,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct FoldersState(pub Mutex<Folders>);

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Folders {
    folder_list: Vec<Folder>,
}

impl Folders {
    pub fn new() -> Folders {
        return Folders {
            folder_list: Vec::new(),
        };
    }
    pub fn serial(&self) -> String {
        return serde_json::to_string(self).expect("can't serialize folders");
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
fn create_new_folder(state: State<FoldersState>, folder_name: String) -> bool {
    let mut folders = state.0.lock().unwrap();

    // Prevent duplicate folder names
    for folder in &folders.folder_list {
        if folder.name == folder_name {
            return false;
        }
    }

    let new_folder = Folder {
        name: folder_name.clone(),
        created_at: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs(),
    };

    folders.folder_list.push(new_folder);
    
    // Save to file
    fs::write(SAVEFILE, folders.serial()).expect("Unable to write file");

    println!("Created new folder: {}", folder_name);
    return true;
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

// NEW STILL TESTING 2/14
// #[tauri::command]
// fn db_load_data(state: State<NotesState>) {
//     let mut notes = state.0.lock().expect("could not lock mutex");
//     let con = dbManager::create_connection().expect("Failed to create database connection");
//     let db_notes = dbManager::db_get_notes(&con).expect("Failed to get notes");
//     for note in db_notes {
//         let new_note = Note {
//             name: note.1,
//             content: note.2,
//             // created_at: note.3 as u64, // Convert i64 to u64
//             last_updated: note.3 as u64, // Convert i64 to u64
//         };
//         notes.note_list.push(new_note);
//     }
// }
// END NEW STILL TESTING 2/14


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let notes = Notes::new();
    let folders = Folders::new();  //  Initialize folders

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(NotesState(notes.into()))
        .manage(FoldersState(folders.into()))  // ✅ Manage folder state
        .invoke_handler(tauri::generate_handler![
            save_data,
            load_data,
            create_note,
            edit_note,
            get_notes,
            get_note_by_name,
            delete_note,
            create_new_folder  //  Register the function
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

