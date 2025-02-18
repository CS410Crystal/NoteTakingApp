/**
 * Folders allow grouping notes together, and hides notes that aren't relevant to your study
 */
struct Folder {
    name: String,
    last_updated: u64,
    note_references: Vec<String> //list of references to notes, updates every time a note is edited or deleted
}

/**
 * Creates a new folder based on name, starts out empty
 */
#[tauri::command]
pub fn create_folder() {

}

/**
 * Gets the list of folders
 */
#[tauri::command]
pub fn get_folders() {

}

/**
 * Delete a folder by name
 */
#[tauri::command]
pub fn delete_folder() {

}

/**
 * Edit a folder by name
 */
#[tauri::command]
pub fn edit_folder() {

}