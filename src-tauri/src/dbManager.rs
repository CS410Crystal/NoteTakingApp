use rusqlite::{params, Connection, Result};
use tauri::State;

//use crate::note::get_notes;
use crate::NotesState;

//create database connection
pub fn create_connection() -> Result<Connection> {
    let conn = Connection::open("notes.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS notes (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            content TEXT NOT NULL,
            created_at INTEGER NOT NULL
        )",
        [],
    )?;
    Ok(conn)
}
//create note                                                                   //this is working
#[tauri::command]
pub fn create_note_in_db(name: &str) -> Result<bool, String> {
    println!("name: {}", name);
    println!("Tried to run create_note_in_db");
    let conn = create_connection().map_err(|e| e.to_string())?;
    println!("Created connection");
    //print the connection
    println!("{:?}", conn);
    //get the highest id in the database
    conn.execute(
        "INSERT INTO notes (name, content, created_at) VALUES (?1, ?2, ?3)",
        params![name, "content", chrono::Utc::now().timestamp()],
    ).map_err(|e| e.to_string())?;
    println!("Ran execute");
    println!("Created note in database: {}", name);
    // save_new_note_in_db(name);
    Ok(true)
}

//don't think we need this -jj
//save a new note to the database
#[tauri::command]
pub fn save_new_note_in_db(name: &str) -> Result<(), String> {
    println!("Tried to run save_new_note_in_db");
    let conn = create_connection().map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO notes (name, created_at) VALUES (?1, ?3)",
        params![name, chrono::Utc::now().timestamp()],
    ).map_err(|e| e.to_string())?;
    println!("Saved new note in database: {}", name);
    Ok(())
}


//edit note
pub fn edit_note_in_db(id: i32, name: &str, content: &str) -> Result<()> {
    // conn.execute(
    //     "UPDATE notes SET name = ?1, content = ?2 WHERE id = ?3",
    //     params![name, content, id],
    // )?;
    // Ok(())
    println!("Tried to run edit_note_in_db");
    let conn = create_connection().expect("Failed to create database connection");
    conn.execute(
        "UPDATE notes SET name = ?1, content = ?2 WHERE id = ?3",
        params![name, content, id],
    )?;

    //new
    get_notes_from_db_main_display().expect("Failed to get notes from db_main_display");

    Ok(())
}

//old edit function
// #[tauri::command]
// pub fn edit_note(state: State<NotesState>, object: String) -> bool {
//     let note: Note = serde_json::from_str(&object).unwrap();
//     //same but for dbNote
//     let con = dbManager::create_connection().expect("Failed to create database connection");
//     let dbNote = dbManager::db_get_note_by_name(&con, &note.name).expect("Failed to get note");
//     //edit note in db
//     match dbManager::db_edit_note(&con, dbNote.0, &note.name, &note.content) {
//         Ok(_) => {
//             println!("Edited note in database file: {}", note.name);
//         }
//         Err(e) => {
//             eprintln!("Failed to edit note: {}", e);
//             return false
//         }
//     }
//     //continue with original func
//     let mut notes = state.0.lock().unwrap();

//     let mut note_index: usize = 0;
//     let mut can_edit: bool = false;
//     for lock_note in &notes.note_list {
//         if lock_note.name == note.name {
//             can_edit = true;
//             break;
//         }
//         note_index += 1;
//     }

//     if can_edit == true {
//         notes.note_list[note_index] = note;
//         return true;
//     }
//     return false;
// }

//delete note
pub fn delete_note_from_db(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("DELETE FROM notes WHERE id = ?1", params![id])?;
    Ok(())
}

//get all notes (returns a vector of tuples)
#[tauri::command]
pub fn get_notes_from_dbManager() -> Result<Vec<(i32, String, String, i64)>, String> {
    println!("Tried to run get_notes_from_dbManager");
    let conn = create_connection().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare("SELECT id, name, content, created_at FROM notes").map_err(|e| e.to_string())?;
    let note_iter = stmt.query_map([], |row| {
        Ok((
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            row.get(3)?,
        ))
    }).map_err(|e| e.to_string())?;
    let mut notes = Vec::new();
    for note in note_iter {
        notes.push(note.map_err(|e| e.to_string())?);
    }
    //print the notes
    for note in &notes {
        println!("Got From Manager:\n Note ID: {}, name: {}, content: {}, created_at: {}", note.0, note.1, note.2, note.3);
    }
    Ok(notes)
}

//get the notes from the database and return the name and timestamp in serde_json format
#[tauri::command]
pub fn get_notes_from_db_main_display() -> Result<Vec<(String, i64)>, String> {
    println!("Tried to run get_notes_from_db_main_display");
    let conn = create_connection().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare("SELECT name, created_at FROM notes").map_err(|e| e.to_string())?;
    //print the notes
    
    let note_iter = stmt.query_map([], |row| {
        Ok((
            row.get(0)?,
            row.get(1)?,
        ))
    }).map_err(|e| e.to_string())?;
    let mut notes = Vec::new();
    for note in note_iter {
        notes.push(note.map_err(|e| e.to_string())?);
    }
    //print the notes
    for note in &notes {
        println!("Got From Manager:\n name: {}, created_at: {}", note.0, note.1);
    }
    Ok(notes)
}

//get note by name (returns a tuple)
#[tauri::command]
pub fn db_get_note_by_name(name: &str) -> Result<(i32, String, String, i64), String> {
    println!("tried to get note by name");
    let conn = create_connection().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare("SELECT id, name, content, created_at FROM notes WHERE name = ?1").map_err(|e| e.to_string())?;
    let note_iter = stmt.query_map(params![name], |row| {
        Ok((
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            row.get(3)?,
        ))
    }).map_err(|e| e.to_string())?;
    let mut notes = Vec::new();
    for note in note_iter {
        notes.push(note.map_err(|e| e.to_string())?);
    }
    //print the note
    println!("Got From Manager by Name:\n Note ID: {}, name: {}, content: {}, created_at: {}", notes[0].0, notes[0].1, notes[0].2, notes[0].3);
    //return the note
    //print what we're returning:
    println!("Returning note: {:?}", notes[0]);

    Ok(notes[0].clone())

}

//get note by last_updated (returns a tuple)
pub fn db_get_note_by_last_updated(conn: &Connection, last_updated: i64) -> Result<(i32, String, String, i64)> {
    let mut stmt = conn.prepare("SELECT id, name, content, created_at FROM notes WHERE created_at = ?1")?;
    let note_iter = stmt.query_map(params![last_updated], |row| {
        Ok((
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            row.get(3)?,
        ))
    })?;
    let mut notes = Vec::new();
    for note in note_iter {
        notes.push(note?);
    }
    Ok(notes[0].clone())
}

pub fn to_string(conn: &Connection) -> String {
    let notes = get_notes_from_dbManager().expect("Failed to get notes");
    let mut notes_string = String::new();
    for note in notes {
        notes_string.push_str(&format!("{:?}\n", note));
    }
    notes_string
}