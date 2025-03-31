use std::{alloc::System, time::{SystemTime, UNIX_EPOCH}};

use serde_json::Value;
use tauri::State;

use crate::{dbManager, load_data_from_db, Notes, NotesState};

// use crate::dbManager::{db_get_note_by_name};

/**
 * Add a note to the Notes state
 * return true if success(, false if existing note title)
 */
#[tauri::command]
pub fn edit_note_in_db(state: State<NotesState>, object: String) -> bool {
    println!("{}",object);
    let test: (i32, String, String, i64) = serde_json::from_str(&object).unwrap();
    // let mut note: Note = Note::new();
    let con = dbManager::create_connection().expect("Failed to create database connection");
    let dbNote = dbManager::db_get_note_by_id(test.0).expect("Failed to get note");
    match dbManager::edit_note_in_db(dbNote.0, &test.1, &test.2) {
        Ok(_) => {
            println!("Edited note in database file: {}", test.1);
            return true;
        }
        Err(e) => {
            eprintln!("Failed to edit note: {}", e);
            return false
        }
    }
}

/**
 * note that state is no longer needed
 */
#[tauri::command]
pub fn delete_note(id: i32) -> bool {
    // let mut notes = state.0.lock().unwrap();
    //same but for dbNote
    let con = dbManager::create_connection().expect("Failed to create database connection");
    let dbNote = dbManager::db_get_note_by_id(id).expect("Failed to get note");
    //delete note in db
    match dbManager::delete_note_from_db(&con, dbNote.0) {
        Ok(_) => {
            println!("Deleted note from database: {}", id);
            return true
        }
        Err(e) => {
            eprintln!("Failed to delete note: {}", e);
            return false
        }
    }
}

#[tauri::command]
pub fn text_import(state: State<NotesState>, name: String, content: String) -> bool {
    // Prevent duplicates in memory
    let mut notes = state.0.lock().unwrap();
    for lock_note in &notes.note_list {
        if lock_note.name == name {
            return false;
        }
    }

    // Create or open the DB connection
    let conn = match dbManager::create_connection() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("DB connection failed: {}", e);
            return false;
        }
    };

    // Insert a new row, get back the new ID
    let new_id = match dbManager::create_note_in_db(&name) {
        Ok(id) => id,
        Err(e) => {
            eprintln!("Failed to insert note: {}", e);
            return false;
        }
    };

    // Now update the note’s content
    if let Err(e) = dbManager::edit_note_in_db(new_id, &name, &content) {
        eprintln!("Failed to update note content: {}", e);
        return false;
    }

    // Also track it in-memory
    let mut note = Note::new();
    note.name = name;
    note.content = content;
    notes.note_list.push(note);

    true
}

#[tauri::command]
pub fn pdf_import(state: State<NotesState>, name: String, content: String) -> bool {
    let mut notes = state.0.lock().unwrap();
    // Prevent duplicates in memory
    for lock_note in &notes.note_list {
        if lock_note.name == name {
            return false;
        }
    }

    // Insert a new row
    let conn = dbManager::create_connection().expect("DB connection failed");
    let new_id = match dbManager::create_note_in_db(&name) {
        Ok(id) => id,
        Err(e) => {
            eprintln!("Failed to insert PDF note: {}", e);
            return false;
        }
    };

    // Update the content column to the PDF data
    if let Err(e) = dbManager::edit_note_in_db(new_id, &name, &content) {
        eprintln!("Failed to store PDF content: {}", e);
        return false;
    }

    // Also keep a copy in memory
    let mut note = Note::new();
    note.name = name;
    note.content = content;
    notes.note_list.push(note);

    //println!("Imported PDF as note: {}", name);
    true
}

#[tauri::command]
pub fn docx_import(name: String, content_base64: String) -> Result<bool, String> {
    use std::io::Read;
    use zip::ZipArchive;
    use quick_xml::Reader;
    use quick_xml::events::Event;

    // 1. Decode base64
    let docx_bytes = base64::decode(content_base64).map_err(|e| e.to_string())?;

    // 2. Unzip
    let reader = std::io::Cursor::new(docx_bytes);
    let mut zip = ZipArchive::new(reader).map_err(|e| e.to_string())?;

    // 3. Grab the main document XML
    let mut document_xml = String::new();
    {
        let mut file_in_zip = zip
            .by_name("word/document.xml")
            .map_err(|_| "word/document.xml not found")?;
        file_in_zip
            .read_to_string(&mut document_xml)
            .map_err(|e| e.to_string())?;
    }

    // 4. Parse the XML to extract text content
    let mut text_content = String::new();
    let mut xml_reader = Reader::from_str(&document_xml);
    xml_reader.trim_text(true);

    //let mut buf = Vec::new();
    loop {
        match xml_reader.read_event() {
            Ok(Event::Text(e)) => {
                let t = e.unescape().map_err(|e| e.to_string())?;
                text_content.push_str(&t);
            }
            Ok(Event::Eof) => break, // End of file
            Err(e) => return Err(format!("XML read error: {}", e)),
            _ => {}
        }
    }
    // 5. Store in the DB. This is exactly like your “text_import” pattern:
    // First, insert a new note row to get an ID:
    let new_id = crate::dbManager::create_note_in_db(&name)
        .map_err(|why| format!("DB insertion error: {}", why))?;

    // Then update the content of that new note:
    crate::dbManager::edit_note_in_db(new_id, &name, &text_content)
        .map_err(|why| format!("DB content update error: {}", why))?;

    // Return success
    Ok(true)
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
