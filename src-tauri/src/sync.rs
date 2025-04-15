// sync.rs
use crate::db::{get_unsynced_notes, mark_note_as_synced};
use sqlitecloud::sqlite_cloud_client::SqliteCloudClient;
use sqlitecloud::NoteUploadRequest;

pub async fn sync_to_cloud() -> Result<(), Box<dyn std::error::Error>> {

    let local_notes = get_unsynced_notes()?; 

  
    for note in local_notes {
        push_note_to_cloud(note).await?;
    }

    
    let cloud_notes = fetch_new_cloud_notes().await?;
    for cloud_note in cloud_notes {
        insert_or_update_local(cloud_note)?;
    }

    Ok(())
}

pub async fn sync_to_cloud() -> Result<(), Box<dyn std::error::Error>> {
    let notes = get_unsynced_notes()?; // implement this in db.rs

    let mut client = SqliteCloudClient::connect("sqlitecloud://cf5bbr5ank.g1.sqlite.cloud:8860/chinook.sqlite?apikey=jYx9fAyMaFfQk3tzLshZqKlnrVYaUhbfYwEL6Rost6o").await?;

    for note in notes {
        let request = tonic::Request::new(NoteUploadRequest {
            id: note.id,
            name: note.name,
            content: note.content,
            last_updated: note.last_updated,
        });

        client.upload_note(request).await?;
        mark_note_as_synced(&note.id)?; // reset the sync flag
    }

    Ok(())
}

