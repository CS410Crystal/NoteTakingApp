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
  if (!folderName.trim()) {
    alert("Folder name cannot be empty!");
    return;
  }
  invoke("create_new_folder", { folder_name: folderName })
    .then((response) => {
      if (response) {
        invoke("save_data"); //  Save the folder
        closeNewFolderDialog();
        alert("Folder created successfully!");
      } else {
        alert("Folder with this name already exists!");
      }
    })
    .catch((error) => {
      console.error("Error creating folder:", error);
      alert("Failed to create folder.");
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

  //invoke("load_data").then(() => { //load from db file now
    invoke("load_data_from_db").then(() => {
    invoke("get_notes_from_db").then((response) => { //was "get_notes"
      let notes = JSON.parse(response);
      for (const note of notes) {
        let note_element = create_note_element(note);
        notes_list.appendChild(note_element);
      }
    })
  })
})();

let edit_note = document.getElementById("edit_note")
let edit_container = document.getElementById("edit")
const edit_name = document.getElementById("edit-name")

let currently_editing_note;
let currently_editing_note_element;

function create_note_element(note) {
  let note_element = document.createElement("div")
  note_element.setAttribute("name",note.name)
  note_element.style.width = "200px"
  note_element.style.height = "200px"
  note_element.style.padding = "10px"
  let button = document.createElement("button")
  button.style.width = "200px"
  button.style.height = "200px"
  button.innerText = note.name;
  note_element.appendChild(button)
  button.classList.add("note")
  let lastdate = document.createElement("div")
  lastdate.innerText = timeAgo(note.last_updated)
  button.appendChild(lastdate)

  button.addEventListener("click", function() {
    console.log(edit_container.style.display)
    if (edit_container.style.display == "") {
      invoke("get_note_by_name",{name: note.name}).then((response) => {
        if (response != "note not found") {
          let note_response = JSON.parse(response);
          console.log(note_response)
          edit_container.style.display = "block"
          edit_name.innerText = "Editing Note Name: " + note_response.name;
          currently_editing_note = note_response;
          currently_editing_note_element = lastdate; //temporary
          edit_note.innerText = note_response.content;
          edit_note.value = note_response.content;
          lastdate.innerText = timeAgo(Number(note_response.last_updated))
        } else {
          console.error("note not found with name: " + note.name);
        }
      })
    }
  })

  return note_element;
}

function scaleHeight() {
  const windowHeight = window.innerHeight;
  const desiredHeight = windowHeight*.85
  notes_list.style.height = desiredHeight + "px"
  console.log("test")
}
window.addEventListener("resize",scaleHeight)

function timeAgo(date) {
  const now = new Date();
  const then = new Date(date);

  const seconds = Math.round((now - then) / 1000);
  const minutes = Math.round(seconds / 60);
  const hours = Math.round(minutes / 60);
  const days = Math.round(hours / 24);
  const months = Math.round(days / 30);
  const years = Math.round(months / 12);

  if (seconds < 30) {
    return 'just now';
  } else if (minutes < 2) {
    return `${seconds} seconds ago`;
  } else if (minutes < 60) {
    return `${minutes} minutes ago`;
  } else if (hours < 24) {
    return `${hours} hours ago`;
  } else if (days < 30) {
    return `${days} days ago`;
  } else if (months < 12) {
    return `${months} months ago`;
  } else {
    return `${years} years ago`;
  }
}

const edit_tab_close = document.getElementById("edit-tab_close")
edit_tab_close.addEventListener("click", function() {
  edit_container.style.removeProperty("display")
  edit_name.innerText = "Editing Note Name: {}";
})

const edit_save_note = document.getElementById("edit-tab-save")
edit_save_note.addEventListener("click", function() {
  if (currently_editing_note != null) {
    currently_editing_note.content = edit_note.value;
    currently_editing_note.last_updated = Date.now();
  }
  const object = JSON.stringify(currently_editing_note)
  invoke("edit_note", {object}).then((response) => {
    invoke("save_data").then((save_data_response) => {
      console.log("success")
      currently_editing_note_element.innerText = timeAgo(Number(currently_editing_note.last_updated))
    })
  })
})

const delete_note = document.getElementById("delete_note");
delete_note.addEventListener("click",function() {
if (currently_editing_note == "" || currently_editing_note == null || currently_editing_note == undefined) {
  return
}
  invoke("delete_note", {name: currently_editing_note.name}).then((response) => {
    if (response == true) {
      invoke("save_data").then((save_data_response) => {
        edit_container.style.removeProperty("display")
        edit_name.innerText = "Editing Note Name: {}";
        currently_editing_note_element.parentElement.parentElement.remove();
        currently_editing_note_element = null;
      })
    }
  });
});
// Search functionality (commented out)
// function searchNotes() {
//   let input = document.getElementById("searchInput").value.toLowerCase();
//   let notes = document.querySelectorAll(".note");
//   
//   notes.forEach(note => {
//     let noteName = note.innerText.toLowerCase();
//     if (noteName.includes(input)) {
//       note.style.display = "block";
//     } else {
//       note.style.display = "none";
//     }
//   });
// }