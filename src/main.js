const { invoke } = window.__TAURI__.core;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/


function openNav() {
  document.getElementById("theSidenav").style.width = "250px";
}

function closeNav() {
  document.getElementById("theSidenav").style.removeProperty("width");
}

// JJ: NEW CODE START:

function openNewFolderDialog() {                                             //data
  //close the previous window
  document.getElementById("newNoteToFolderDialog").style.display = "none";
  document.getElementById("newFolderDialog").style.display = "block";
  // document.getElementById("newFolderDialog").setAttribute("data", data);
}

function closeNewFolderDialog() {
  document.getElementById("newFolderDialog").style.display = "none";
}

function displayExistingFolders() {
  document.getElementById("existingFoldersDisplay").style.display;
}

function closeExistingFoldersDisplay() {
  document.getElementById("existingFoldersDisplay").style.display = "none";
}
// THOUGHTS?
function createNewFolder() {                                                    //data
  //create a new folder without FOREIGNING the note
  console.log("createNewFolder");
  const folderName = document.getElementById("newFolderName").value;
  if (folderName == null || folderName == "") {
    return
  }
  invoke("create_folder_in_db", { name: folderName }).then((response) => {
    console.log(response);
    closeNewFolderDialog();
    location.reload();
  });
  //open the new folder dialog
  document.getElementById("newFolderDialog").style.display = "block";
}

function newNoteAndNewFolderDialog() {
  //get current id of the note
  const noteId = document.getElementById("newNoteDialog").getAttribute("data");
  console.log(noteId);
  //open the new folder dialog
  document.getElementById("newFolderDialog").style.display = "block";
  //close the new note to folder dialog
  document.getElementById("newNoteToFolderDialog").style.display = "none";
  //add the note to the newly created folder
  invoke("add_note_to_folder_in_db", { folder_id: noteId, note_id: data }).then((response) => {
    console.log(response);
    closeNewFolderDialog();
    location.reload();
  }
  );
  //get the folder id
  const folderId = document.getElementById("newFolderDialog").getAttribute("data");
  console.log(folderId);
  //add the note to the folder
  invoke("add_note_to_folder_in_db", { folder_id: folderId, note_id: data }).then((response) => {
    console.log(response);
    closeNewFolderDialog();
    location.reload();
  }
  );


}
function createFolderAndAddNote() {
  console.log("createFolderAndAddNote");
  const folderName = document.getElementById("newFolderName").value;
  if (folderName == null || folderName == "") {
    return
  }
  invoke("create_folder_in_db", { name: folderName }).then((response) => {
    console.log(response);
    closeNewFolderDialog();
    location.reload();
  }); 
  //add the note to the folder
  const folderId = document.getElementById("newFolderDialog").getAttribute("data");
  console.log(folderId);
  invoke("add_note_to_folder_in_db", { folder_id: folderId, note_id: data }).then((response) => {
    console.log(response);
    closeNewFolderDialog();
    location.reload();
  });
}

function addToFolder() {                                                  //data
  console.log("addToFolder");
  const folderId = document.getElementById("newNoteToFolderDialog").getAttribute("data");
  console.log(folderId);
  invoke("add_note_to_folder_in_db", { folder_id: folderId, note_id: data }).then((response) => {
    console.log(response);
    closeNewNoteToFolderDialog();
    location.reload();
  });
}


function openNewNoteDialog() {
  document.getElementById("newNoteDialog").style.display = "block";
}

function closeNewNoteDialog() {
  document.getElementById("newNoteDialog").style.display = "none";
}

function openNewNoteToFolderDialog() {//data
  document.getElementById("newNoteToFolderDialog").style.display = "block";
  // document.getElementById("newNoteToFolderDialog").setAttribute("data", data);
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
    invoke("save_new_note_in_db")//save the data in db (appears we're doing)
    //should return a string

    // addToFolderDialog(noteName, response);
    closeNewNoteDialog();
    openNewNoteToFolderDialog();

    // Append new note element
    let note = [Number(response), noteName, "", Date.now()];
    let note_element = create_note_element(note);
    notes_list.appendChild(note_element);

    closeNewNoteDialog();
    openNewNoteToFolderDialog(); //data
  });
}



const notes_list = document.getElementById("notes_list");
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
      let folders = JSON.parse(response);
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

function create_folder_element(folder) {
  let folder_element = document.createElement("div");
  folder_element.classList.add("folder-item");
  folder_element.innerText = `📁 ${folder.name}`;
  folder_element.style.width = "200px";
  folder_element.style.height = "50px";
  folder_element.style.padding = "10px";
  folder_element.style.border = "1px solid gray";
  folder_element.style.marginBottom = "5px";
  folder_element.style.cursor = "pointer";
  folder_element.style.backgroundColor = "#e6e6e6";
  folder_element.style.display = "flex";
  folder_element.style.alignItems = "center";
  folder_element.style.justifyContent = "center";

  folder_element.addEventListener("click", function () {
    alert(`Opening folder: ${folder.name}`);
  });

  return folder_element;
}

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

  if (checkBox.checked === true) {
    notes.forEach(note => {
      note.parentElement.style.display = "none";
    });
  } else {
    notes.forEach(note => {
      note.parentElement.style.display = "block";
    });
  }
}

function showFolders() {
  notes_list.innerHTML = "";
  invoke("get_folders").then((response) => {
    let folders = JSON.parse(response);
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
