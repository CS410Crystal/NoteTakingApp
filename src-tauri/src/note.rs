use std::{alloc::System, time::{SystemTime, UNIX_EPOCH}};

use tauri::State;

use crate::{dbManager, load_data_from_db, Notes, NotesState};

use crate::dbManager::{db_get_note_by_name};

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
    //create connection
    let con = dbManager::create_connection().expect("Failed to create database connection");
    //create note in db
    // match dbManager::db_create_note(&con, &note.name, &note.content) {
    //     Ok(_) => {
    //         notes.note_list.push(note.clone());
    //         println!("Saved new note in database: {}", name);
    //     }
    //     Err(e) => {
    //         eprintln!("Failed to create note: {}", e);
    //         return false
    //     }
    // }
    //continue with original func //commenting out
    //notes.note_list.push(note.clone());
    //
    //println!("Saved new note: {}",name);
    //
    //run load data from db
    return true;
}

#[tauri::command]
pub fn edit_note(state: State<NotesState>, object: String) -> bool {
    let note: Note = serde_json::from_str(&object).unwrap();
    //same but for dbNote
    let con = dbManager::create_connection().expect("Failed to create database connection");
    let dbNote = dbManager::db_get_note_by_name(&note.name).expect("Failed to get note");
    //edit note in db
    match dbManager::edit_note_in_db(dbNote.0, &note.name, &note.content) {
        Ok(_) => {
            println!("Edited note in database file: {}", note.name);
        }
        Err(e) => {
            eprintln!("Failed to edit note: {}", e);
            return false
        }
    }
    //continue with original func
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
pub fn edit_note_in_db(state: State<NotesState>, object: String) -> bool {
    let note: Note = serde_json::from_str(&object).unwrap();
    let con = dbManager::create_connection().expect("Failed to create database connection");
    let dbNote = dbManager::db_get_note_by_name(&note.name).expect("Failed to get note");
    match dbManager::edit_note_in_db(dbNote.0, &note.name, &note.content) {
        Ok(_) => {
            println!("Edited note in database file: {}", note.name);
        }
        Err(e) => {
            eprintln!("Failed to edit note: {}", e);
            return false
        }
    }
    return true;
}

#[tauri::command]
pub fn delete_note(state: State<NotesState>, name: String) -> bool {
    let mut notes = state.0.lock().unwrap();
    //same but for dbNote
    let con = dbManager::create_connection().expect("Failed to create database connection");
    let dbNote = dbManager::db_get_note_by_name(&name).expect("Failed to get note");
    //delete note in db
    match dbManager::delete_note_from_db(&con, dbNote.0) {
        Ok(_) => {
            println!("Deleted note from database: {}", name);
        }
        Err(e) => {
            eprintln!("Failed to delete note: {}", e);
            return false
        }
    }
    //continue with original func
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

// #[tauri::command]
// pub fn get_notes(state: State<NotesState>) -> String {
//     let notes = state.0.lock().unwrap();

//     return serde_json::to_string(&notes.note_list).expect("can't serialize note list");
// }

// #[tauri::command]
// pub fn get_notes_from_db(state: State<NotesState>) -> String {
//     //print getting notes from db
//     println!("Getting notes from db");
//     let con = dbManager::create_connection().expect("Failed to create database connection");
//     let dbNotes = dbManager::get_notes_from_db(&con).expect("Failed to get notes");
//     return serde_json::to_string(&dbNotes).expect("can't serialize note list");
// }

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
pub fn get_note_by_name_from_db(state: State<NotesState>, name: String) -> String {
    let con = dbManager::create_connection().expect("Failed to create database connection");
    let dbNote = dbManager::db_get_note_by_name(&name).expect("Failed to get note");
    println!("Got note by name from db: {}", dbNote.1);
    return serde_json::to_string(&dbNote).expect("can't serialize note struct");
}

#[derive(Clone)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Note {
    pub name: String,
    pub content: String,
    pub last_updated: u64,
    //made public for dbManager
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

//public functions to get the elements of Note for the dbManager
pub  fn get_note_name(note: &Note) -> String {
    return note.name.clone();
}

pub fn get_note_content(note: &Note) -> String {
    return note.content.clone();
}

pub fn get_note_last_updated(note: &Note) -> u64 {
    return note.last_updated;
}