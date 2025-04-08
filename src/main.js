const { invoke } = window.__TAURI__.core;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/


function openNav() {
  //change background color of file menu button to no background color
  const filemenu = document.getElementById("filemenu")
  filemenu.style.backgroundColor = "transparent"
  document.getElementById("theSidenav").style.width = "250px";
}

function closeNav() {
  document.getElementById("theSidenav").style.removeProperty("width");
}

// JJ: NEW CODE START:

function openNewFolderDialog() {
  document.getElementById("newNoteToFolderDialog").style.display = "none";
  document.getElementById("newFolderDialog").style.display = "block";
}

function closeNewFolderDialog() {
  document.getElementById("newFolderDialog").style.display = "none";
}

function displayExistingFolders() {
  document.getElementById("existingFoldersDisplay").style.display = "block";
}

function closeExistingFoldersDisplay() {
  document.getElementById("existingFoldersDisplay").style.display = "none";
}
// THOUGHTS?
function createNewFolder() {
  const folderName = document.getElementById("newFolderName").value;
  if (!folderName.trim()) {
    alert("Folder name cannot be empty!");
    return;
  }
  invoke("create_folder_in_db", { name: folderName })
    .then((response) => {
      if (response) {
        // invoke("save_data_to_database"); //  Save the folder
        closeNewFolderDialog();
        alert("Folder created successfully!");
        //color the file menu button to show where to find folder
        //check if file menu is open first
        const filemenu = document.getElementById("filemenu")
        filemenu.style.backgroundColor = "rgb(9, 126, 52)"
        //filemenu border color to green
        console.log("test")
      } else {
        alert("Folder with this name already exists!");
      }
    })
    .catch((error) => {
      console.error("Error creating folder:", error);
      alert("Failed to create folder.");
    });
    // refresh the folder list
  notes_list.innerHTML = "";
  invoke("get_folders").then((response) => {
    console.log(response)
    let folders = response;
    // let folders = JSON.parse(response);
    if (folders.length === 0) {
      const emptyMessage = document.createElement("p");
      emptyMessage.innerText = "No folders available.";
      notes_list.appendChild(emptyMessage);
    }
    for (const folder of folders) {
      let folder_element = create_folder_element(folder);
      notes_list.appendChild(folder_element);
    }
  });
}


function openNewNoteDialog() {
  document.getElementById("newNoteDialog").style.display = "block";
}

function closeNewNoteDialog() {
  document.getElementById("newNoteDialog").style.display = "none";
}

function openNewNoteToFolderDialog() {
  document.getElementById("newNoteToFolderDialog").style.display = "block";
}

function closeNewNoteToFolderDialog() {
  document.getElementById("newNoteToFolderDialog").style.display = "none";
}
/**
 * Creates new note based on name
 * fails if note name is empty or name already exists (case sensitive)
 * @returns null
 */
function createNewNote() {
  console.log("createNewNote");
  const noteName = document.getElementById("newNoteName").value;
  if (noteName == null || noteName == "") {
    return
  }
  //here we need to createe the note in database (appears we're doing)
  invoke("create_note_in_db", { name: noteName }).then((response) => {
    //should return a string
    // working here 3/23
    // addToFolderDialog(noteName, response);
    closeNewNoteDialog();
    openNewNoteToFolderDialog();

    // Append new note element
    let note = [Number(response), noteName, "", Date.now()];
    let note_element = create_note_element(note);
    notes_list.appendChild(note_element);
  });
}



const notes_list = document.getElementById("notes_list");//where is notes_list defined?
//Load Notes
// JJ: NEW CODE START
function loadNotes() {
  scaleHeight();
  console.log("invoke load_data_from_db");
  invoke("load_data_from_db").then(() => {
    console.log("invoke get_notes_from_dbManager");
    invoke("get_notes_from_dbManager").then((response) => {
      console.log(response);
      for (const note of response) {
        let note_element = create_note_element(note);
        notes_list.appendChild(note_element);
      }
    })
    invoke("get_folders").then((response) => {
      console.log(response)
      let folders = response;
      // let folders = JSON.parse(response);
      for (const folder of folders) {
        let folder_element = create_folder_element(folder);
        notes_list.appendChild(folder_element);
      }
    });

  })
}

loadNotes();

//open edit note dialog
function openEditNoteDialog() {
  document.getElementById("editNoteDialog").style.display = "block";
}

function closeEditNoteDialog() {
  document.getElementById("editNoteDialog").style.display = "none";
}

function openEditFolderDialog() {
  document.getElementById("editNoteDialog").style.display = "block";
}

function closeEditFolderDialog() {
  document.getElementById("editNoteDialog").style.display = "none";
}

/**
 * Compare notes by last updated
 * Prefers to show newer notes first
 * @param {*} a Note Object
 * @param {*} b Note Object
 * @returns priority number (greater means newer)
 */
function compare_last_updated(a, b) {
  if (a.last_updated < b.last_updated) {
    return 1;
  }
  if (a.last_updated > b.last_updated) {
    return -1;
  }
  return 0;
}

function scaleHeight() {
  const windowHeight = window.innerHeight;
  const desiredHeight = windowHeight * .85
  notes_list.style.height = desiredHeight + "px"
  console.log("test")
}
window.addEventListener("resize", scaleHeight)

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

let edit_note = document.getElementById("edit_note");
let edit_container = document.getElementById("edit");
const edit_name = document.getElementById("edit-name");

let currently_editing_note;
let currently_editing_note_element;

let currently_editing_folder;
let currently_editing_folder_element;

const edit_tab_close = document.getElementById("edit-tab_close");
edit_tab_close.addEventListener("click", function () {
  edit_container.style.removeProperty("display");
  edit_name.innerText = "Editing Note Name: {}";
  const status = document.getElementById("edit-tab-save-status");
  status.style.setProperty("visibility", "hidden");
  status.style.setProperty("color", "black");
});

const input_edit_name = document.getElementById("input-edit-name")
const edit_save_note = document.getElementById("edit-tab-save");

let note_element_editing = undefined

edit_save_note.addEventListener("click", function () {
  if (currently_editing_note != null) {
    currently_editing_note[1] = input_edit_name.value;
    currently_editing_note[2] = edit_note.value;
    currently_editing_note[3] = Date.now();
  }
  const object = JSON.stringify(currently_editing_note);
  invoke("edit_note_in_db", { object }).then((response) => {
    console.log("success");
    if (note_element_editing != null) {
      note_element_editing.textContent = input_edit_name.value;
      console.log(response)
      note_element_editing.parentElement.getElementsByClassName('lastdate')[0].textContent = timeAgo(response * 1000);
    }
    const status = document.getElementById("edit-tab-save-status");
    if (status.style.getPropertyValue("visibility") === "hidden") {
      status.style.setProperty("visibility", "visible");
    }
    if (status.style.getPropertyValue("color") === "black") {
      status.style.setProperty("color", "lime");
    } else {
      status.style.setProperty("color", "black");
    }
  });
});

const delete_note_btn = document.getElementById("delete_note");
delete_note_btn.addEventListener("click", function () {
  if (!currently_editing_note) {
    return;
  }
  invoke("delete_note", { id: currently_editing_note[0] }).then((response) => {
    if (response === true) {
      edit_container.style.removeProperty("display");
      edit_name.innerText = "Editing Note Name: {}";
      currently_editing_note_element.parentElement.remove();
      currently_editing_note_element = null;
    }
  });
});

// Creating elements for notes/folders in the main list
function create_note_element(note) {
  // note: [id, name, content, last_updated]
  let note_element = document.createElement("div");
  note_element.setAttribute("name", note[1]);
  note_element.style.width = "200px";
  note_element.style.height = "200px";
  note_element.style.padding = "10px";
  note_element.classList.add("note_element")
  note_element.value = note[0];
  
  let button = document.createElement("button");
  button.style.width = "200px";
  button.style.height = "200px";
  button.innerText = note[1];
  note_element.appendChild(button);
  button.classList.add("note");

  let lastdate = document.createElement("div");
  lastdate.className = "lastdate";
  // PDF vs text notes might store last_updated differently
  // If it’s stored in seconds, multiply by 1000 if needed
  // For consistency, let's do:
  lastdate.innerText = timeAgo(note[3] * 1000);
  lastdate.dataset.lastUpdated = note[3];
  note_element.appendChild(lastdate);

  // If content is PDF, display PDF; otherwise, open text editor
  if (typeof note[2] === "string" && note[2].startsWith("data:application/pdf")) {
    button.addEventListener("click", function () {
      displayPDFData(note); // pass entire note so we can also delete it
    });
  } else {
    // normal text note
    button.addEventListener("click", function () {
      invoke("db_get_note_by_id", { id: note[0] }).then((response) => {
        if (response !== "note not found") {
          let note_response = response;
          edit_container.style.display = "block";
          edit_name.innerText = "Editing Note Name: " + note_response[1];
          currently_editing_note = note_response;
          currently_editing_note_element = lastdate;
          input_edit_name.value = note_response[1];
          edit_note.innerText = note_response[2];
          edit_note.value = note_response[2];

          note_element_editing = button;

          lastdate.innerText = timeAgo(Number(note_response[3]) * 1000);

        } else {
          console.error("Note not found with name: " + note[1]);
        }
      });
    });
  }

  return note_element;
}

const input_edit_name_folder = document.getElementById("input-edit-name-folder")

const note_list = document.getElementById("folder-note-list");
const folder_banner = document.getElementById("folder_banner");

document.getElementById("folder_banner_close").addEventListener("click",function() {
  let notes_to_check = document.querySelectorAll(".note_element")
  for (let i = 0; i < notes_to_check.length; ++i) {
    notes_to_check[i].style.removeProperty("display")
  }
  folder_banner.style.removeProperty("display")
})

function create_folder_element(folder) {
  let folder_element = document.createElement("div");
  folder_element.classList.add("folder_item");
  folder_element.innerText = `📁 ${folder[1]}`;
  folder_element.style.width = "200px";
  folder_element.style.height = "50px";
  folder_element.style.padding = "10px";
  folder_element.style.border = "1px solid gray";
  folder_element.style.marginBottom = "5px";
  folder_element.style.cursor = "pointer";
  folder_element.style.display = "flex";
  folder_element.style.alignItems = "center";
  folder_element.style.justifyContent = "center";
  folder_element.style.color = "#000"

  let folder_edit_element = document.createElement("button");
  folder_edit_element.textContent = "Edit"
  folder_edit_element.style.visibility = "hidden"
  folder_edit_element.style.position = "static"
  folder_element.appendChild(folder_edit_element)

  folder_element.addEventListener("mouseover", function () {
    folder_edit_element.style.visibility = "visible"
  })
  folder_element.addEventListener("mouseout", function () {
    folder_edit_element.style.visibility = "hidden"
  })
  folder_element.addEventListener("click", function (e) {
    if (e.target != folder_element) {
      return
    }
    //update folder data
    invoke("db_get_folder_by_id", { id: folder[0] }).then((response) => {
      let folder_response = response;
      folder[1] = folder_response[1];
      folder[2] = folder_response[2];
      folder[3] = folder_response[3];
      folder_banner.style.display = "block"
      folder_banner.childNodes[0].nodeValue = "Current Folder: " + folder[1];
      //hide saves based on reference_list
      // console.log(folder[2])
      let notes_to_check = document.querySelectorAll(".note_element")
      let reference_list = JSON.parse(folder[2]);

      for (let i = 0; i < notes_to_check.length; ++i) {
        notes_to_check[i].style.display = "none"
      }
      for (let i = 0; i < notes_to_check.length; ++i) {
        let match_id = notes_to_check[i].value

        let found_index = undefined
        for (let j = 0; j < reference_list.length; ++j) {
          if (reference_list[j] == match_id) {
            notes_to_check[i].checked = true;
            found_index = j;
            notes_to_check[i].style.removeProperty("display")
            break;
          }
        }
        if (found_index != undefined) {
          reference_list.splice(found_index, 1);
        }
        if (reference_list.length == 0) {
          break;
        }
      }

      //loop all saves
      
    }
    )
    
  })
  folder_edit_element.addEventListener("click", function () {
    // alert(`Opening folder: ${folder[1]}`);
    let edit_folder_container = document.getElementById("edit-folder-container");
    edit_folder_container.style.display = "block";
    // let edit_folder = document.getElementById("folder-edit-tab");
    document.getElementById("folder-edit-name").innerText = "Editing Folder Name: " + folder[1];
    document.getElementById("input-edit-name-folder").value = folder[1]

    while (note_list.firstChild) {
      note_list.removeChild(note_list.lastChild)
    }

    invoke("get_notes_from_dbManager").then((response) => {
      // This returns an array of [name, last_updated], so handle accordingly:
      // e.g. let notes = response.map( ... ) or adapt as needed
      console.log(response);
      response.sort((a, b) => b[3] - a[3]); // Sort by last_updated desc
      for (const note of response) {
        //create a separate note element for folder
        create_note_element_for_folder(note)
      }
    });




    invoke("db_get_folder_by_id", { id: folder[0] }).then((response) => {
      if (response !== "folder not found") {
        let folder_response = response;
        edit_folder_container.style.display = "block";
        edit_name.innerText = "Editing Note Name: " + folder_response[1];
        currently_editing_folder = folder_response;
        currently_editing_folder_element = folder_element;
        input_edit_name_folder.value = folder_response[1];

        // check every existing note
        folder_element_editing = folder_element;
        let reference_list = JSON.parse(folder[2]);
        console.log(reference_list)
        let notes_to_check = document.querySelectorAll(".note_element_checkbox")
        for (let i = 0; i < notes_to_check.length; ++i) {
          let match_id = notes_to_check[i].value

          let found_index = undefined
          for (let j = 0; j < reference_list.length; ++j) {
            if (reference_list[j] == match_id) {
              notes_to_check[i].checked = true;
              found_index = j;
              break;
            }
          }
          if (found_index != undefined) {
            reference_list.splice(found_index, 1);
          }
          if (reference_list.length == 0) {
            break;
          }
        }

        // lastdate.innerText = timeAgo(Number(note_response[3]) * 1000);

      } else {
        console.error("Note not found with name: " + note[1]);
      }
    });
  });

  return folder_element;
}

document.getElementById("folder-edit-tab_close").addEventListener("click", function () {
  let edit_folder_container = document.getElementById("edit-folder-container");
  edit_folder_container.style.display = "none";

  const status = document.getElementById("edit-tab-folder-save-status");
    if (status.style.getPropertyValue("visibility") === "visible") {
      status.style.setProperty("visibility", "hidden");
    }
})

let folder_element_editing = undefined

document.getElementById("folder-edit-tab-save").addEventListener("click", function () {
  if (currently_editing_folder != null) {
    let input_edit_name_folder = document.getElementById("input-edit-name-folder");
    currently_editing_folder[1] = input_edit_name_folder.value;
    // currently_editing_folder[2] = edit_note.value;

    //loop through

    let folder_references = [];

    let notes_to_check = document.querySelectorAll(".note_element_checkbox")
    for (let i = 0; i < notes_to_check.length; ++i) {
      if (notes_to_check[i].checked == true) {
        // console.log(notes_to_check[i].value)
        folder_references.push(Number(notes_to_check[i].value))
      }
    }
    currently_editing_folder[2] = JSON.stringify(folder_references);

    currently_editing_folder[3] = Date.now();
  }
  const folder_name_edit = document.getElementById("input-edit-name-folder")
  const object = JSON.stringify(currently_editing_folder);
  console.log(object)
  invoke("edit_folder_in_db", { object: object }).then((response) => {
    console.log("success");
    if (folder_element_editing != null) {
      folder_element_editing.childNodes[0].nodeValue = `📁 ${folder_name_edit.value}`;
      console.log(response)
      // folder_element_editing.parentElement.getElementsByClassName('lastdate')[0].textContent = timeAgo(response * 1000);
    }
    const status = document.getElementById("edit-tab-folder-save-status");
    if (status.style.getPropertyValue("visibility") === "hidden") {
      status.style.setProperty("visibility", "visible");
    }
    if (status.style.getPropertyValue("color") === "black") {
      status.style.setProperty("color", "lime");
    } else {
      status.style.setProperty("color", "black");
    }
  });
})

function create_note_element_for_folder(note) {

  let note_element = document.createElement("div")
  note_element.innerText = note[1];
  note_list.appendChild(note_element)
  let note_element_checkbox = document.createElement("input");
  note_element_checkbox.type = "checkbox"
  note_element_checkbox.value = note[0];
  note_element_checkbox.classList.add("note_element_checkbox")
  note_element.appendChild(note_element_checkbox)
}

//delete folder from the database
document.getElementById("delete_folder").addEventListener("click", function() {
  print("delete folder");
  if (!currently_editing_folder) {
    return;
  }
  invoke("delete_folder_from_db", { id: currently_editing_folder[0] }).then((response) => {
    if (response === true) {
      edit_container.style.removeProperty("display");
      edit_name.innerText = "Editing Note Name: {}";
      currently_editing_folder_element.parentElement.remove();
      currently_editing_folder_element = null;
    }
  });
  //refresh the folder list
  notes_list.innerHTML = "";
  invoke("get_folders").then((response) => {
    console.log(response)
    let folders = response;
    // let folders = JSON.parse(response);
    if (folders.length === 0) {
      const emptyMessage = document.createElement("p");
      emptyMessage.innerText = "No folders available.";
      notes_list.appendChild(emptyMessage);
    }
    for (const folder of folders) {
      let folder_element = create_folder_element(folder);
      notes_list.appendChild(folder_element);
    }
  });
  //close the folder edit dialog
  let edit_folder_container = document.getElementById("edit-folder-container");
  edit_folder_container.style.display = "none";
  //close the edit note dialog
  edit_container.style.removeProperty("display");
  edit_name.innerText = "Editing Note Name: {}";
  currently_editing_folder_element.parentElement.remove();
  currently_editing_folder_element = null;
}
)
// Sorting / Searching
let lastArray = undefined;
function alphabetSort() {
  let checkBox = document.getElementById("alphabetSort");
  let notes = document.querySelectorAll(".note");
  console.log(checkBox.checked)
  if (checkBox.checked === true) {
    const elements = Array.from(notes)
    lastArray = elements.slice()
    elements.sort((a, b) => a.textContent.localeCompare(b.textContent))
    const parent = notes_list;
    elements.forEach(element => parent.appendChild(element.parentNode))

  } else {
    const elements = lastArray
    console.log(elements)
    if (elements == undefined) {
      return
    }
    const parent = notes_list;
    elements.forEach(element => parent.appendChild(element.parentNode))
  }
}

function dateSort() {
  let checkBox = document.getElementById("dateSort");
  let notes = document.querySelectorAll(".note");

  const elements = Array.from(notes).map(note => {
    const parent = note.parentElement;
    const lastUpdatedDiv = parent.querySelector(".lastdate");
    const time = Number(lastUpdatedDiv.dataset.lastUpdated) || 0;
    return { element: parent, time: time };
  });

  // Sort descending if checked (most recent first), ascending if unchecked
  elements.sort((a, b) => checkBox.checked ? b.time - a.time : a.time - b.time);

  const parent = notes_list;
  elements.forEach(item => parent.appendChild(item.element));
}


function showFolders() {
  notes_list.innerHTML = "";
  invoke("get_folders").then((response) => {
    console.log(response)
    let folders = response;
    // let folders = JSON.parse(response);
    if (folders.length === 0) {
      const emptyMessage = document.createElement("p");
      emptyMessage.innerText = "No folders available.";
      notes_list.appendChild(emptyMessage);
    }
    for (const folder of folders) {
      let folder_element = create_folder_element(folder);
      notes_list.appendChild(folder_element);
    }
  });
}

function showNotes() {
  console.log("showNotes");
  notes_list.innerHTML = "";
  invoke("get_notes_from_dbManager").then((response) => {
    // This returns an array of [name, last_updated], so handle accordingly:
    // e.g. let notes = response.map( ... ) or adapt as needed
    console.log(response);
    response.sort((a, b) => b[3] - a[3]); // Sort by last_updated desc
    for (const note of response) {
      // If you need [id, name, content, last_updated], you'll need a different call
      // For demonstration, just create a placeholder with name
      // and note[1] as "last_updated"
      let note_element = create_note_element(note);
      notes_list.appendChild(note_element);
    }
  });
}

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

// File Import function – handles .txt, .pdf, .docx
function importFile() {
  const input = document.createElement("input");
  input.accept = ".txt,.md,.doc,.docx,.pdf";
  input.type = "file";

  input.onchange = function (event) {
    const file = event.target.files[0];
    if (!file) return;

    const fileName = file.name.split('.').slice(0, -1).join('.');

    if (file.name.toLowerCase().endsWith(".pdf")) {
      // PDF Path:
      const reader = new FileReader();
      reader.onload = function (e) {
        const dataUrl = e.target.result;
        invoke("pdf_import", { name: fileName, content: dataUrl })
          .then((response) => {
            if (response) {
              let importedNote = [Date.now(), fileName, dataUrl, Date.now()];
              let note_element = create_note_element(importedNote);
              notes_list.appendChild(note_element);
              alert("PDF imported successfully as a note!");
            } else {
              alert("A note with this name already exists!");
            }
          })
          .catch((error) => {
            console.error("Error importing PDF:", error);
            alert("Failed to import PDF file.");
          });
      };
      reader.readAsDataURL(file);
    } else if (file.name.toLowerCase().endsWith(".docx")) {
      // DOCX Path:
      const reader = new FileReader();
      reader.onload = function (e) {
        const arrayBuffer = e.target.result;
        const bytes = new Uint8Array(arrayBuffer);
        let binary = "";
        for (let i = 0; i < bytes.byteLength; i++) {
          binary += String.fromCharCode(bytes[i]);
        }
        // Convert to base64
        const base64String = btoa(binary);
        // Call Tauri command docx_import
        invoke("docx_import", {
          name: fileName,
          contentBase64: base64String
        }).then((ok) => {
          if (!ok) {
            alert("A note with this name may already exist, or an error occurred.");
            return;
          }
          alert("DOCX imported successfully!");
          let docxNote = [Date.now(), fileName, "", Date.now()];
          let note_element = create_note_element(docxNote);
          notes_list.appendChild(note_element);
        }).catch((error) => {
          console.error("Error importing DOCX:", error);
          alert("Failed to import DOCX file.");
        });
      };
      reader.readAsArrayBuffer(file);
    } else {
      // text-based path (.txt, .md, .doc)
      const reader = new FileReader();
      reader.onload = function (e) {
        const fileContent = e.target.result;
        let importedNote = {
          name: fileName,
          content: fileContent,
          last_updated: Date.now()
        };
        invoke("text_import", { name: importedNote.name, content: importedNote.content })
          .then((response) => {
            if (response) {
              invoke("save_data"); // optional, if you use a separate Tauri command
              let note_element = create_note_element([
                Date.now(),
                importedNote.name,
                importedNote.content,
                importedNote.last_updated
              ]);
              notes_list.appendChild(note_element);
              alert("File imported successfully as a note!");
            } else {
              alert("A note with this name already exists!");
            }
          })
          .catch((error) => {
            console.error("Error importing file:", error);
            alert("Failed to import file.");
          });
      };
      reader.readAsText(file);
    }
  };
  input.click();
}

// PDF Viewing & Deletion
let currently_viewing_pdf_note = null;

/**
 * displayPDFData(note):
 *  note is [id, name, content (dataUrl), last_updated]
 */
function displayPDFData(note) {
  // Save the note array globally in case we want to delete it
  currently_viewing_pdf_note = note;
  // Show the modal
  document.getElementById("pdfViewerModal").style.display = "block";

  // Clear out any canvases from a previous PDF
  const pagesContainer = document.getElementById("pdfPagesContainer");
  pagesContainer.innerHTML = "";

  // Retrieve the data URL we stored in note[2]
  const dataUrl = note[2];
  const loadingTask = pdfjsLib.getDocument(dataUrl);

  loadingTask.promise.then(function (pdf) {
    // pdf.numPages tells us how many pages are in the PDF
    console.log("PDF loaded, total pages = " + pdf.numPages);

    // Loop through all pages
    for (let pageNum = 1; pageNum <= pdf.numPages; pageNum++) {
      pdf.getPage(pageNum).then(function (page) {
        // Create a new canvas for *each* page
        const canvas = document.createElement("canvas");
        pagesContainer.appendChild(canvas);

        // Set up the 2D context
        const context = canvas.getContext("2d");

        // You can adjust 'scale' if you want bigger or smaller pages
        const scale = 1.2;
        const viewport = page.getViewport({ scale });

        canvas.width = viewport.width;
        canvas.height = viewport.height;

        // Render the page into this canvas
        const renderContext = {
          canvasContext: context,
          viewport: viewport,
        };
        page.render(renderContext);
      });
    }
  }).catch(function (error) {
    console.error("Error loading PDF:", error);
  });
}

// Closes the PDF modal
function closePDFViewer() {
  document.getElementById("pdfViewerModal").style.display = "none";
  currently_viewing_pdf_note = null;
}

// Hook up the PDF delete button
document.getElementById("deletePdfButton").addEventListener("click", function () {
  if (!currently_viewing_pdf_note) return;
  const noteId = currently_viewing_pdf_note[0];

  invoke("delete_note", { id: noteId }).then((response) => {
    if (response === true) {
      // remove the note tile from the DOM
      const noteTile = document.querySelector(`div[name='${currently_viewing_pdf_note[1]}']`);
      if (noteTile && noteTile.parentElement) {
        noteTile.parentElement.remove();
      }
      closePDFViewer();
    }
  }).catch((err) => {
    console.error("Failed to delete PDF note:", err);
  });
});
