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