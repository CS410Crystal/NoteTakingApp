use rusqlite::{params, Connection};

use crate::dbManager::get_largest_id;

/**
 * Folders allow grouping notes together, and hides notes that aren't relevant to your study
 */
struct Folder {
    name: String,
    last_updated: u64,
    note_references: Vec<String>, //list of references to notes, updates every time a note is edited or deleted
}

pub fn create_connection() -> Result<Connection, rusqlite::Error> {
    let conn = Connection::open("notes.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS folders (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            references TEXT NOT NULL,
            last_updated INTEGER NOT NULL,
        )",
        [],
    )?;
    Ok(conn)
}

/**
 * Creates a new folder based on name, starts out empty
 */
#[tauri::command]
pub fn create_folder(name: &str) -> Result<i32, String> {
    //get the largest id in the database
    println!("name: {}", name);
    println!("Tried to run create_folder_in_db");
    let conn = create_connection().map_err(|e| e.to_string())?;
    //get the largest id in the database
    let id = get_largest_id(&conn).map_err(|e| e.to_string())?;
    // print the id
    println!("Got largest id: {}", id);
    //create the connection
    println!("Created connection");
    //print the connection
    println!("{:?}", conn);
    //get the highest id in the database
    let new_id: i32 = id + 1;
    conn.execute(
        "INSERT INTO notes (name, references, last_updated) VALUES (?1, ?2, ?3)",
        params![name, "content", chrono::Utc::now().timestamp()],
    )
    .map_err(|e| e.to_string())?;
    println!("Ran execute");
    println!("Created folder in database: {}", name);
    Ok(new_id)
}

/**
 * Gets the list of folders
 */
#[tauri::command]
pub fn get_folders() -> Result<String, String> {
    let conn = create_connection().map_err(|e| e.to_string())?;
    //loop folders
    //export as JSON
    Ok("".to_string())
}

/**
 * Delete a folder by id
 */
#[tauri::command]
pub fn delete_folder(id: i32) -> Result<(), String> {
    let conn = create_connection().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM folders WHERE id = ?1", params![id]).unwrap();
    Ok(())
}

/**
 * Edit a folder by id
 * adding new 
 */
#[tauri::command]
pub fn edit_folder(content: String) -> Result<(), Box<dyn std::error::Error>> {
    let conn = create_connection().map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn search_folders_by_name(name: String) -> Result<String, String>{
    //return JSON
    Ok("".to_string())
}

/**
 * All delete calls on notes will call this function, shifting all indexes based on deleted index by -1
 */
pub fn folder_delete_shift(id: i32) {
    //loop all folders
        //get the note reference list
        //loop list
            //reduce id by 1 if greater than id
            //delete id if equal to id
        //replace folder list with edited list
}