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
            last_updated INTEGER NOT NULL
        )",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS folders (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            reference_list TEXT,
            last_updated INTEGER NOT NULL,
            num_notes INTEGER NOT NULL
            )",
        [],
        //did have FOREIGN KEY (id) REFERENCES notes(id) at the end
    )?;
    Ok(conn)
}
//create note                                                                   //this is working
#[tauri::command]
pub fn create_note_in_db(name: &str) -> Result<i32, String> {
    //get the largest id in the database
    println!("name: {}", name);
    println!("Tried to run create_note_in_db");
    let conn = create_connection().map_err(|e| e.to_string())?;
    //get the largest id in the database
    let id = get_largest_id(&conn).map_err(|e| e.to_string())?;

    //get the highest id in the database
    let new_id: i32 = id + 1;
    conn.execute(
        "INSERT INTO notes (name, content, last_updated) VALUES (?1, ?2, ?3)",
        params![name, "content", chrono::Utc::now().timestamp()],
    ).map_err(|e| e.to_string())?;
    println!("Ran execute");
    println!("Created note in database: {}", name);
    Ok(new_id)
}
//new
#[tauri::command]
pub fn create_folder_in_db (name: &str) -> Result<i32, String> {

    let conn = create_connection().map_err(|e| e.to_string())?;
    //get the largest id in the database
    let id = get_largest_id(&conn).map_err(|e| e.to_string())?;
    //get the highest id in the database
    let new_id: i32 = id + 1;
    conn.execute(
        "INSERT INTO folders (name, reference_list, last_updated, num_notes) VALUES (?1, ?2, ?3, ?4)",
        params![name, "", chrono::Utc::now().timestamp(), 0],
    ).map_err(|e| e.to_string())?;

    Ok(new_id)
}

#[tauri::command]
pub fn delete_folder_from_db(id: i32) -> Result<(), String> {
    println!("Tried to run delete_folder_from_db");
    let conn = create_connection().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM folders WHERE id = ?1", params![id]).map_err(|e| e.to_string())?;
    Ok(())
}
//

#[tauri::command]
pub fn add_note_to_folder_in_db(folder_id: i32, note_id: i32) -> Result<(), String> {
    let conn = create_connection().map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE folders SET num_notes = num_notes + 1 WHERE id = ?1",
        params![folder_id],
    ).map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE notes SET folder_id = ?1 WHERE id = ?2",
        params![folder_id, note_id],
    ).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn get_largest_id(conn: &Connection) -> Result<i32> {
    let mut stmt = conn.prepare("SELECT id FROM notes ORDER BY id DESC LIMIT 1")?;
    let id_iter = stmt.query_map([], |row| {
        Ok(row.get(0)?)
    })?;
    let mut id = 0;
    for id_result in id_iter {
        id = id_result?;
    }
    Ok(id)
}




//edit note
pub fn edit_note_in_db(id: i32, name: &str, content: &str) -> Result<i64> {

    let conn = create_connection().expect("Failed to create database connection");
    let timestamp = chrono::Utc::now().timestamp();
    conn.execute(
        "UPDATE notes SET name = ?1, content = ?2, last_updated = ?3 WHERE id = ?4",
        params![name, content, timestamp, id],
    )?;

    //new
    get_notes_from_db_main_display().expect("Failed to get notes from db_main_display");

    Ok(timestamp)
}


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
    let mut stmt = conn.prepare("SELECT id, name, content, last_updated FROM notes").map_err(|e| e.to_string())?;
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
        println!("Got From Manager:\n Note ID: {}, name: {}, content: {}, last_updated: {}", note.0, note.1, note.2, note.3);
    }
    Ok(notes)
}

//get the notes from the database and return the name and timestamp in serde_json format
#[tauri::command]
pub fn get_notes_from_db_main_display() -> Result<Vec<(String, i64)>, String> {
    println!("Tried to run get_notes_from_db_main_display");
    let conn = create_connection().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare("SELECT name, last_updated FROM notes").map_err(|e| e.to_string())?;
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
        println!("Got From Manager:\n name: {}, last_updated: {}", note.0, note.1);
    }
    Ok(notes)
}

//get note by last_updated (returns a tuple)
pub fn db_get_note_by_last_updated(conn: &Connection, last_updated: i64) -> Result<(i32, String, String, i64)> {
    let mut stmt = conn.prepare("SELECT id, name, content, last_updated FROM notes WHERE last_updated = ?1")?;
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


#[tauri::command]
pub fn db_get_note_by_id(id: i32) -> Result<(i32, String, String, i64), String> {
    println!("tried to get note by id");
    let conn = create_connection().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare("SELECT id, name, content, last_updated FROM notes WHERE id = ?1").map_err(|e| e.to_string())?;
    let note_iter = stmt.query_map(params![id], |row| {
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
    println!("Got From Manager by ID:\n Note ID: {}, name: {}, content: {}, last_updated: {}", notes[0].0, notes[0].1, notes[0].2, notes[0].3);
    //return the note
    //print what we're returning:
    println!("Returning note: {:?}", notes[0]);

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

#[tauri::command]
pub fn search_notes_by_content(search_term: String) -> Result<Vec<(i32, String, String, i64)>, String> {
    let conn = create_connection().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(
        "SELECT id, name, content, last_updated FROM notes 
        WHERE content LIKE ?1 COLLATE NOCASE"
    ).map_err(|e| e.to_string())?;
    
    let search_pattern = format!("%{}%", search_term);
    let note_iter = stmt.query_map(params![search_pattern], |row| {
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
    Ok(notes)
}