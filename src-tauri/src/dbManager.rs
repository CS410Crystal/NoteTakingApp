use rusqlite::{params, Connection, Result};

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

//create note
pub fn db_create_note(conn: &Connection, name: &str, content: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO notes (name, content, created_at) VALUES (?1, ?2, ?3)",
        params![name, content, chrono::Utc::now().timestamp()],
    )?;
    Ok(())
}


//edit note
pub fn db_edit_note(conn: &Connection, id: i32, name: &str, content: &str) -> Result<()> {
    conn.execute(
        "UPDATE notes SET name = ?1, content = ?2 WHERE id = ?3",
        params![name, content, id],
    )?;
    Ok(())
}

//delete note
pub fn db_delete_note(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("DELETE FROM notes WHERE id = ?1", params![id])?;
    Ok(())
}

//get all notes (returns a vector of tuples)
pub fn db_get_notes(conn: &Connection) -> Result<Vec<(i32, String, String, i64)>> {
    let mut stmt = conn.prepare("SELECT id, name, content, created_at FROM notes")?;
    let note_iter = stmt.query_map([], |row| {
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
    Ok(notes)
}

//get note by name (returns a tuple)
pub fn db_get_note_by_name(conn: &Connection, name: &str) -> Result<(i32, String, String, i64)> {
    let mut stmt = conn.prepare("SELECT id, name, content, created_at FROM notes WHERE name = ?1")?;
    let note_iter = stmt.query_map(params![name], |row| {
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
    let notes = db_get_notes(conn).expect("Failed to get notes");
    let mut notes_string = String::new();
    for note in notes {
        notes_string.push_str(&format!("{:?}\n", note));
    }
    notes_string
}