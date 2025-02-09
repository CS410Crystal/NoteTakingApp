const { invoke } = window.__TAURI__.core;

let searchInputEl;
let searchResultsEl;

// Handling of search input
async function handleSearch() {
  const query = searchInputEl.value;
  if (query.length === 0) {
    searchResultsEl.innerHTML = '';
    return;
  }

  // Call of Rust backend
  const results = await invoke("search_notes", { query });
  displayResults(results);
}

// Display results
function displayResults(notes) {
  searchResultsEl.innerHTML = notes.map(note => `
    <div class="search-result-item">
      ${note.content.substring(0, 50)}${note.content.length > 50 ? '...' : ''}
    </div>
  `).join('');
}

// Initializes on load
window.addEventListener("DOMContentLoaded", () => {
  searchInputEl = document.querySelector("#search-input");
  searchResultsEl = document.querySelector("#search-results");
  searchInputEl.addEventListener("input", debounce(handleSearch));
});

// Update note.rs to give access to app's note storage
// and query to enable search term from the user
// Update lib.rs
// Load search functionaly into HTML