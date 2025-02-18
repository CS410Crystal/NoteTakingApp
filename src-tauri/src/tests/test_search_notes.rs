// Current test to see search functionality

#[cfg(test)]
mod tests {
    use super::*;
    use your_crate::note_handling::{Note, get_notes};

    #[test]
    fn test_get_notes_returns_all_notes() {
        // Setup: Adds test notes to the database/state
        let notes = vec![
            Note { name: "Note1".into(), content: "".into(), last_updated: 0 },
            Note { name: "Note2".into(), content: "".into(), last_updated: 0 },
        ];
        
        // Simulates loading notes (implementation-specific)
        load_test_notes(notes.clone());

        // Test: Ensures `get_notes` returns all notes
        let retrieved_notes = get_notes().unwrap();
        assert_eq!(retrieved_notes.len(), 2);
    }
}