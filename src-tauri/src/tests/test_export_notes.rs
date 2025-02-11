use note_taking_app_lib::{export_notes, export_notes_as_pdf, Note, NotesState};
use std::fs;
use std::sync::Mutex;
use tauri::State;

#[test]
fn test_export_notes() {
    let notes = vec![
        Note {
            name: "Test Note".to_string(),
            content: "This is a test note.".to_string(),
            last_updated: 1234567890,
        }
    ];

    let formats = vec!["json", "txt", "md"];
    let base_path = "test_output";
    let state = State::new(Mutex::new(notes.clone()));

    let mut test_files = vec![];

    for format in &formats {
        let file_path = format!("{}_{}", base_path, format);
        test_files.push(file_path.clone());

        let result = export_notes(state.clone(), format.to_string(), file_path.clone());
        assert!(result, "Failed to export notes as {}", format);
        assert!(fs::metadata(&file_path).is_ok(), "Exported file not created for {}", format);

        // Verify content inside exported file
        let file_content = fs::read_to_string(&file_path).expect("Failed to read exported file.");
        assert!(!file_content.is_empty(), "Exported file is empty for {}", format);
    }

    // Cleanup test files
    for file in test_files {
        fs::remove_file(&file).unwrap();
    }
}

#[test]
fn test_export_notes_from_pdf() {
    let notes = vec![
        Note {
            name: "Test PDF Note".to_string(),
            content: "This is a test note for PDF export.".to_string(),
            last_updated: 1234567890,
        }
    ];

    let file_path = "test_export.pdf";
    let result = export_notes_from_pdf(&notes, file_path);

    assert!(result, "Failed to export notes as PDF.");
    assert!(fs::metadata(file_path).is_ok(), "PDF file was not created.");

    fs::remove_file(file_path).unwrap();
}
