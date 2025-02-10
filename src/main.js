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
// THOUGHTS?
function createNewNote() {
  const noteName = document.getElementById("newNoteName").value;
  if (noteName == null || noteName == "") {
    return
  }
  invoke("create_note", { name: noteName }).then((response) => {
    if (response == true) {
      invoke("save_data")
      closeNewNoteDialog();
    }
  });
}

// JJ: NEW CODE END