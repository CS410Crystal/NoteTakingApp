use note_taking_app_lib::{import_notes, import_notes_from_pdf, Notes, NotesState};
use pdf_writer::{Content, Pdf};
use std::fs::{self, File};
use std::sync::Mutex;
use tauri::State;

#[test]
fn test_import_notes() {
    let formats = vec!["json", "txt", "md"];
    let base_path = "test_input";
    let note_content = "Test Imported Note\nThis is a test import.";

    let state = State::new(NotesState(Mutex::new(Notes { note_list: vec![] })));
    let mut test_files = vec![];

    for format in &formats {
        let file_path = format!("{}_{}", base_path, format);
        test_files.push(file_path.clone());

        // Create sample files
        match *format {
            "json" => {
                let test_data = serde_json::to_string_pretty(&vec![Note {
                    name: "Imported JSON Note".to_string(),
                    content: note_content.to_string(),
                    last_updated: 1234567890,
                }])
                .unwrap();
                fs::write(&file_path, test_data).unwrap();
            }
            "txt" | "md" => {
                let test_data = format!("# Imported Text Note\n\n{}", note_content);
                fs::write(&file_path, test_data).unwrap();
            }
            _ => continue,
        }

        // Test Import
        let result = import_notes(state.clone(), format.to_string(), file_path.clone());
        assert!(result, "Failed to import notes from {}", format);
    }

    // Verify that imported notes exist
    let notes = state.0.lock().unwrap();
    assert!(!notes.note_list.is_empty(), "No notes were imported.");

    // Cleanup all test files
    for file in test_files {
        fs::remove_file(&file).unwrap();
    }
}

#[test]
fn test_import_notes_from_pdf() {
    let file_path = "test_import.pdf";
    let mut pdf = Pdf::new();
    let page_ref = pdf.add_page();
    let font_ref = pdf.add_font(include_bytes!("../assets/fonts/Arial.ttf"));
    let mut content = Content::new();
    content.set_font(font_ref, 14.0)
           .move_to(50.0, 700.0)
           .show("Imported PDF Note")
           .newline(50.0, 680.0)
           .show("This is a test import from a PDF.");
    pdf.add_page_content(page_ref, &content);

    let mut file = File::create(&file_path).expect("Failed to create test PDF file");
    file.write_all(&pdf.finish()).unwrap();

    let state = State::new(NotesState(Mutex::new(Notes { note_list: vec![] })));
    let result = import_notes_from_pdf(state.clone(), file_path.to_string());

    assert!(result, "Failed to import notes from PDF.");
    assert!(!state.0.lock().unwrap().note_list.is_empty(), "Imported notes are empty.");

    fs::remove_file(file_path).unwrap();
}

