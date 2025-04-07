use std::string;

use rusqlite::{params, Connection};
use tauri_plugin_opener::reveal_item_in_dir;

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
            reference_list TEXT,
            last_updated INTEGER NOT NULL
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
    println!("Tried to run create_folder function in folder.rs");
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
        "INSERT INTO folders (name, reference_list, last_updated) VALUES (?1, ?2, ?3)",
        params![name, "", chrono::Utc::now().timestamp()],
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
pub fn get_folders() -> Result<Vec<(i32, String, String, i64)>, String> {
    let conn = create_connection().map_err(|e| e.to_string())?;
    //loop folders
    //export as JSON
    println!("Tried to run get_folders");
    let mut stmt = conn
        .prepare("SELECT id, name, reference_list, last_updated FROM folders")
        .map_err(|e| e.to_string())?;
    let folder_iter = stmt
        .query_map([], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
        })
        .map_err(|e| e.to_string())?;
    let mut folders = Vec::new();
    for folder in folder_iter {
        folders.push(folder.map_err(|e| e.to_string())?);
    }
    //print the folders (debug)
    for folder in &folders {
        println!(
            "Got From Manager:\n Folder ID: {}, name: {}, references: {}, last_updated: {}",
            folder.0, folder.1, folder.2, folder.3
        );
    }

    Ok(folders)
}

/**
 * Delete a folder by id
 */
#[tauri::command]
pub fn delete_folder(id: i32) -> Result<(), String> {
    let conn = create_connection().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM folders WHERE id = ?1", params![id])
        .unwrap();
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
pub fn search_folders_by_name(name: String) -> Result<String, String> {
    //return JSON
    Ok("".to_string())
}

/**
 * All delete calls on notes will call this function, shifting all indexes based on deleted index by -1
 */
pub fn folder_delete_shift(id: i32) -> Result<(), String> {
    //loop all folders
    let conn = create_connection().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT id, name, reference_list, last_updated FROM folders")
        .map_err(|e| e.to_string())?;
    let folder_iter = stmt
        .query_map(params![], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
        })
        .map_err(|e| e.to_string())?;
    let mut folders: Vec<(i32, String, String, i64)> = Vec::new();
    for folder in folder_iter {
        folders.push(folder.map_err(|e| e.to_string())?);
    }

    let mut new_folders: Vec<(i32, String, String, i64)> = Vec::new();
    for folder in folders {
        //get the note reference list
        println!("{}",&folder.2);
        if &folder.2 == "" {
            continue;
        }
        let mut reference_list: Vec<i32> = serde_json::from_str(&folder.2).unwrap();

        //loop list
        let mut references_to_delete: Vec<usize> = Vec::new();
        // let mut references_to_shift: Vec<usize> = Vec::new();
        let mut index = 0;
        for reference in &reference_list {
            //reduce id by 1 if greater than id
            if reference > &id {
                // references_to_shift.push(index)
            }
            //delete id if equal to id
            if reference == &id {
                references_to_delete.push(index)
            }
            index+=1;
        }
        // for reference in references_to_shift {
        //     reference_list[reference] -=1;
        // }
        references_to_delete = references_to_delete.into_iter().rev().collect();
        for reference in references_to_delete {
            reference_list.remove(reference);
        }
        println!("new list: {:?}",reference_list);
        // let mut folder_to_clone = folder.clone();
        let reference_list_string = serde_json::to_string(&reference_list).unwrap();
        println!("replace folder list with edited list by deleting id {}, current_index = {}",id, folder.0);
        //replace folder list with edited list
        conn.execute(
            "UPDATE folders SET reference_list = ?1 WHERE id = ?2",
            params![reference_list_string, folder.0],
        ).map_err(|e| e.to_string())?;
        // new_folders.push(folder_to_clone);
    }
    //replace folder list with edited list
    Ok(())
}

#[tauri::command]
pub fn edit_folder_in_db(object: String) -> i64 {
    println!("{}", object);
    let test: (i32, String, String, i64) = serde_json::from_str(&object).unwrap();
    // let mut note: Note = Note::new();
    let _con = create_connection().expect("Failed to create database connection");
    let db_folder = db_get_folder_by_id(test.0).expect("Failed to get note");
    println!("folder id to edit: {}", test.0);
    match edit_folder_in_db_internal(test.0, &test.1, &test.2) {
        Ok(value) => {
            println!("Edited folder in database file: {}", test.1);
            return value;
        }
        Err(e) => {
            eprintln!("Failed to edit folder: {}", e);
            return -1;
        }
    }
}

#[tauri::command]
pub fn db_get_folder_by_id(id: i32) -> Result<(i32, String, String, i64), String> {
    println!("tried to get folder by id");
    let conn = create_connection().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT id, name, reference_list, last_updated FROM folders WHERE id = ?1")
        .map_err(|e| e.to_string())?;
    let folder_iter = stmt
        .query_map(params![id], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
        })
        .map_err(|e| e.to_string())?;
    let mut folders = Vec::new();
    for folder in folder_iter {
        folders.push(folder.map_err(|e| e.to_string())?);
    }
    //print the note
    println!(
        "Got From Manager by ID:\n Folder ID: {}, name: {}, reference_list: {}, last_updated: {}",
        folders[0].0, folders[0].1, folders[0].2, folders[0].3
    );
    //return the note
    //print what we're returning:
    println!("Returning note: {:?}", folders[0]);

    Ok(folders[0].clone())
}

pub fn edit_folder_in_db_internal(
    id: i32,
    name: &str,
    reference_list: &str,
) -> Result<i64, rusqlite::Error> {
    let conn = create_connection().expect("Failed to create database connection");
    let timestamp = chrono::Utc::now().timestamp();
    conn.execute(
        "UPDATE folders SET name = ?1, reference_list = ?2, last_updated = ?3 WHERE id = ?4",
        params![name, reference_list, timestamp, id],
    )?;

    //new
    // get_notes_from_db_main_display().expect("Failed to get notes from db_main_display");

    Ok(timestamp)
}
