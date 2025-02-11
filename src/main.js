const { invoke } = window.__TAURI__.core;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/


function openNav() {
  document.getElementById("theSidenav").style.width = "250px";
}

function closeNav() {
  document.getElementById("theSidenav").style.removeProperty("width")
}

// JJ: NEW CODE START:

function openNewFolderDialog() {
  document.getElementById("newFolderDialog").style.display = "block";
}

function closeNewFolderDialog() {
  document.getElementById("newFolderDialog").style.display = "none";
}
// THOUGHTS?
function createNewFolder() {
  const folderName = document.getElementById("newFolderName").value;
  invoke("create_new_folder", { folderName }).then(() => {
    closeNewFolderDialog();
  });
}

function openNewNoteDialog() {
  document.getElementById("newNoteDialog").style.display = "block";
}

function closeNewNoteDialog() {
  document.getElementById("newNoteDialog").style.display = "none";
}
/**
 * Creates new note based on name
 * fails if note name is empty or name already exists (case sensitive)
 * @returns null
 */
function createNewNote() {
  const noteName = document.getElementById("newNoteName").value;
  if (noteName == null || noteName == "") {
    return
  }
  invoke("create_note", { name: noteName }).then((response) => {
    if (response == true) {
      invoke("save_data")
      closeNewNoteDialog();
      // location.reload();
      let note = {
        name: noteName,
        content: "",
        last_updated: Date.now()
      }
      let note_element = create_note_element(note);
      notes_list.appendChild(note_element);
    }
  });
}

const notes_list = document.getElementById("notes_list");
//Load Notes
(function() {
  scaleHeight();

  invoke("load_data").then(() => {
    invoke("get_notes").then((response) => {
      let notes = JSON.parse(response);
      for (const note of notes) {
        let note_element = create_note_element(note);
        notes_list.appendChild(note_element);
      }
    })
  })
})();

function create_note_element(note) {
  let note_element = document.createElement("div")
  note_element.style.width = "200px"
  note_element.style.height = "200px"
  note_element.style.padding = "10px"
  let button = document.createElement("button")
  button.style.width = "200px"
  button.style.height = "200px"
  button.innerText = note.name;
  note_element.appendChild(button)
  button.classList.add("note")
  return note_element;
}

function scaleHeight() {
  const windowHeight = window.innerHeight;
  const desiredHeight = windowHeight*.85
  notes_list.style.height = desiredHeight + "px"
  console.log("test")
}
window.addEventListener("resize",scaleHeight)