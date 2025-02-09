// Enables real-time search
// Sends user input to Rust backend

const { invoke } = window.__TAURI__.core;

let searchInputEl;
let searchResultsEl;

// Handling of search input
async function handleSearch() {
  const query = searchInputEl.value;
  // Clears results if query is empty
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
  `).join(''); // Convert array to HTML string
}

// Initializes on load
window.addEventListener("DOMContentLoaded", () => {
  searchInputEl = document.querySelector("#search-input");
  searchResultsEl = document.querySelector("#search-results");
  searchInputEl.addEventListener("input", debounce(handleSearch));
});

// TODO:
// Update note.rs to add parameters such as query (search term from user)
// Update lib.rs
// Load search functionaly into HTML