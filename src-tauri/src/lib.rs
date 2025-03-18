///Container for backend scripts
mod note;
mod folder;
mod dbManager;

use note::{create_note, delete_note, get_note_by_name, text_import, pdf_import, docx_import,  Note};
use dbManager::{create_note_in_db, db_get_note_by_id, get_notes_from_dbManager, get_notes_from_db_main_display, save_new_note_in_db, search_notes_by_content};
use serde_json::Value;
use std::{fs::{self, File}, sync::Mutex};
use tauri::State;
// use tauri::Manager;
//use note::get_notes_from_db;
use note::{edit_note_in_db};
//use dbManager::db_get_notes;

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
fn save_data(notes_state: State<NotesState>, folders_state: State<FoldersState>) {
    let notes = notes_state.0.lock().unwrap();
    let folders = folders_state.0.lock().unwrap();

    let data = serde_json::json!({
        "note_list": notes.note_list,
        "folder_list": folders.folder_list
    });

    fs::write(SAVEFILE, data.to_string()).expect("Unable to write file");
}

// #[tauri::command]
// fn save_data_to_db(notes_state: State<NotesState>, folders_state: State<FoldersState>) {
//     let notes = notes_state.0.lock().unwrap();
//     let folders = folders_state.0.lock().unwrap();
//     let con = dbManager::create_connection().expect("Failed to create database connection");

//     let data = serde_json::json!({
//         "note_list": notes.note_list,
//         "folder_list": folders.folder_list
//     });

//     fs::write(SAVEFILE, data.to_string()).expect("Unable to write file");
//     //save to db
//     for note in &notes.note_list {
//         dbManager::create_note_in_db(&note.name).expect("Failed to create note");
//     }
// }


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
///load data from notes.db
// #[tauri::command]
// fn load_data_from_db(state: State<NotesState>) {
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


// #[tauri::command]
// fn load_data() {
//     println!("Tried to run load_data");
//     //tried to run load data
//     let notes_state: State<NotesState>;
//     let folders_state: State<FoldersState>;
//     let mut notes = notes_state.0.lock().unwrap();
//     let mut folders = folders_state.0.lock().unwrap();

//     let file_data = match fs::read_to_string(SAVEFILE) {
//         Ok(data) => data,
//         Err(_e) => {
//             println!("creating new file");
//             let _f = File::create(SAVEFILE).expect("error when creating file");
//             return;
//         }
//     };

//     let result: Value = match serde_json::from_str(&file_data) {
//         Ok(json) => json,
//         Err(_) => {
//             println!("invalid json, using default data");
//             return;
//         }
//     };

//     if let Some(note_list) = result.get("note_list") {
//         *notes = Notes {
//             note_list: serde_json::from_value(note_list.clone()).unwrap_or_default(),
//         };
//     }

//     if let Some(folder_list) = result.get("folder_list") {
//         *folders = Folders {
//             folder_list: serde_json::from_value(folder_list.clone()).unwrap_or_default(),
//         };
//     }
// }

#[tauri::command]
fn load_data_from_db(state: State<NotesState>) {
    //print tried to run function               //haven't gotten here
    println!("Tried to run load_data_from_db");
    let mut notes = state.0.lock().expect("could not lock mutex");
    let con = dbManager::create_connection().expect("Failed to create database connection");
    let db_notes = dbManager::get_notes_from_dbManager().expect("Failed to get notes");
    for note in db_notes {
        let new_note = Note {
            name: note.1,
            content: note.2,
            // created_at: note.3 as u64, // Convert i64 to u64
            last_updated: note.3 as u64, // Convert i64 to u64
        };
        println!("Loaded note: {}", new_note.name);
        println!("Loaded content: {}", new_note.content);
        println!("Loaded date: {}", new_note.last_updated);
        //print out the loaded notes
        notes.note_list.push(new_note);
    }
    //print the loaded notes
    for note in &notes.note_list {
        println!("Fully Loaded note: {}", note.name);
    }
}
//>>>>>>> 605783d (testing w jed)

// #[tauri::command]
// fn db_get_notes(state: State<NotesState>) -> String {
//     let notes = state.0.lock().unwrap();
//     let con = dbManager::create_connection().expect("Failed to create database connection");
//     let db_notes = dbManager::get_notes_from_dbManager().expect("Failed to get notes");
//     let mut notes_string = String::new();
//     for note in db_notes {
//         //print each note to console
//         println!("Got From Tauri Command:\n Note ID: {}, name: {}, content: {}, created_at: {}", note.0, note.1, note.2, note.3);
//         notes_string.push_str(&format!("{:?}\n", note));
//     }
//     notes_string
    
// }

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let notes = Notes::new();
    let folders = Folders::new();  //  Initialize folders
    //initialize database
    let _ = dbManager::create_connection().expect("Failed to create database connection");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(NotesState(notes.into()))
        .manage(FoldersState(folders.into()))  // ✅ Manage folder state
        //create database
        .invoke_handler(tauri::generate_handler![
            create_note_in_db,
            // create_note,
            save_new_note_in_db,
            // save_data,
            load_data_from_db,
            edit_note_in_db,
            get_notes_from_dbManager,
            get_notes_from_db_main_display,
            // db_get_note_by_name,
            db_get_note_by_id,
            delete_note,
            text_import,
            pdf_import,
            docx_import,
            search_notes_by_content
            // create_new_folder,  //  Register the function
            //new functions
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}