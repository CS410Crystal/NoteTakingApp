/**
 * When typing in search bar, hide notes that don't match
 */
function searchNotes() {
    let input = document.getElementById("searchInput").value.toLowerCase();
    let notes = document.querySelectorAll(".note");
    
    notes.forEach(note => {
      let noteName = note.textContent.toLowerCase();
      if (noteName.includes(input)) {
        note.parentElement.style.display = "block";
      } else {
        note.parentElement.style.display = "none";
      }
    });
  }

  function search_by_content() {
    //get notes contents (will be a query instead of instant search)
    //delete all shown entries
    clear_note_entries();
    //get notes
    invoke("")
    //
  }

  /**
   * clears note entries allowing queries to add their own results based on other than name
   */
  function clear_note_entries() {
    const element = document.getElementById('notes_list');
    element.innerHTML = '';
  }

  function handleContentSearch(event) {
    if (event.key === 'Enter') {
        const searchTerm = document.getElementById('contentSearchInput').value;
        if (searchTerm.trim() === '') {
            showNotes(); // Show all notes if empty search
            return;
        }
        searchByContent(searchTerm);
    }
}

function searchByContent(searchTerm) {
    invoke('search_notes_by_content', { searchTerm })
        .then(response => {
            notes_list.innerHTML = "";
            for (const note of response) {
                // Create proper note structure from full note data
                let note_element = create_note_element([
                    note[0],    // id
                    note[1],    // name
                    note[2],    // content
                    note[3]     // created_at
                ]);
                notes_list.appendChild(note_element);
            }
        })
        .catch(error => {
            console.error('Search failed:', error);
            alert('Search failed: ' + error);
        });
}