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
  invoke("create_new_folder", { folderName })
    .then((response) => {
      if (response) {
        invoke("save_data_to_database"); //  Save the folder
        closeNewFolderDialog();
        alert("Folder created successfully!");
        //color the file menu button to show where to find folder
        //check if file menu is open first
        const filemenu = document.getElementById("filemenu")
        filemenu.style.backgroundColor = "rgb(0,255,0)"
        console.log("test")
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
  console.log("createNewNote");
  const noteName = document.getElementById("newNoteName").value;
  if (noteName == null || noteName == "") {
    return
  }
  //here we need to createe the note in database (appears we're doing)
  invoke("create_note_in_db", { name: noteName }).then((response) => {
    if (response == true) {
      invoke("save_new_note_in_db")//save the data in db (appears we're doing)
      //should return a string
      closeNewNoteDialog();

      // location.reload();
      let note = [
        0,noteName,"",Date.now()
      ]
      let note_element = create_note_element(note);
      notes_list.appendChild(note_element);
    }
  });
}

const notes_list = document.getElementById("notes_list");//where is notes_list defined?
//Load Notes
// JJ: NEW CODE START
function loadNotes() {
  scaleHeight();
  //here we should be loading notes from database
  console.log("invoke load data from db")
  invoke("load_data_from_db").then(() => {
    //here we should be getting notes from database
    console.log("invoke get_notes_from_db")
    invoke("get_notes_from_dbManager").then((response) => {
      console.log(response)
      for (const note of response) {
        let note_element = create_note_element(note);
        notes_list.appendChild(note_element);
      }
      // let notes = JSON.parse(response); //not json
      // notes.sort(compare_last_updated);
      // for (const note of notes) {
      //   let note_element = create_note_element(note);
      //   //print the note name
      //   console.log("js: "+note.name)
      //   notes_list.appendChild(note_element);
      // }
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

function editNote() {
  openEditNoteDialog();
  console.log("editNote");
  const noteName = document.getElementById("editNoteName").value;
  if (noteName == null || noteName == "") {
    return
  }
  //here we need to createe the note in database (appears we're doing)
  invoke("edit_note_in_db", { name: noteName }).then((response) => {
    if (response == true) {
      invoke("save_data_to_db")//save the data in db (appears we're doing)
      //should return a string
      closeEditNoteDialog();
      location.reload();
    }
  });
}

// JJ: NEW CODE END
//load and display notes on main screen



/**
 * Compare notes by last updated
 * Prefers to show newer notes first
 * @param {*} a Note Object
 * @param {*} b Note Object
 * @returns priority number (greater means newer)
 */
function compare_last_updated(a,b) {
  if (a.last_updated < b.last_updated) {
    return 1;
  }
  if (a.last_updated > b.last_updated) {
    return -1;
  }
  return 0;
}

let edit_note = document.getElementById("edit_note")
let edit_container = document.getElementById("edit")
const edit_name = document.getElementById("edit-name")

let currently_editing_note;
let currently_editing_note_element;

/**
 * 
 * @param {
 * 0: id
 * 1: name
 * 2: content
 * 3: last_updated
 * } note 
 * @returns object
 */
function create_note_element(note) {
  let note_element = document.createElement("div")
  note_element.setAttribute("name",note[1])
  note_element.style.width = "200px"
  note_element.style.height = "200px"
  note_element.style.padding = "10px"
  let button = document.createElement("button")
  button.style.width = "200px"
  button.style.height = "200px"
  button.innerText = note[1];
  note_element.appendChild(button)
  button.classList.add("note")
  let lastdate = document.createElement("div")
  lastdate.innerText = timeAgo(note[3]*1000)
  note_element.appendChild(lastdate)

  button.addEventListener("click", function() {                                 //edit note button
    console.log(edit_container.style.display)
    if (edit_container.style.display == "") {
      // invoke("db_get_note_by_name",{name: note[1]}).then((response) => {
      invoke("db_get_note_by_id",{id: note[0]}).then((response) => {
        if (response != "note not found") {
          let note_response = response;
          //get elements of note from database
          
          console.log(note_response)
          edit_container.style.display = "block"
          edit_name.innerText = "Editing Note Name: " + note_response[1];
          currently_editing_note = note_response;
          currently_editing_note_element = lastdate; //temporary
          edit_note.innerText = note_response[2];
          edit_note.value = note_response[2];
          lastdate.innerText = timeAgo(Number(note_response[3])*1000)
        } else {
          console.error("note not found with name: " + note.name);
        }
      })
    }
  })

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
    // In the future: Load folder contents or navigate to folder view.
  });

  return folder_element;
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

const edit_save_note = document.getElementById("edit-tab-save")//
edit_save_note.addEventListener("click", function() {
  if (currently_editing_note != null) {
    currently_editing_note.content = edit_note.value;
    currently_editing_note.last_updated = Date.now();
  }
  const object = JSON.stringify(currently_editing_note)
  invoke("edit_note_in_db", {object}).then((response) => {                      //change this to edit_note_in_db
    // invoke("save_data_to_db").then((save_data_response) => {                    //may need to add response here
    //   console.log("success")
    //   currently_editing_note_element.innerText = timeAgo(Number(currently_editing_note.last_updated))
    // })
  })
})

const delete_note = document.getElementById("delete_note");
delete_note.addEventListener("click",function() {
if (currently_editing_note == "" || currently_editing_note == null || currently_editing_note == undefined) {
  return
}
  invoke("delete_note", {name: currently_editing_note.name}).then((response) => {
    if (response == true) {
      invoke("save_data_to_db").then((save_data_response) => {                  //may need to add response here
        edit_container.style.removeProperty("display")
        edit_name.innerText = "Editing Note Name: {}";
        currently_editing_note_element.parentElement.parentElement.remove();
        currently_editing_note_element = null;
      })
    }
  });
});

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

/*AI: Still have to fully implement the alphabetical sort: It will display the notes by alphabetical order
      -all it does for now is hide or show the notes depending if the checkbox is checked or not */
function alphabetSort() {
  let checkBox = document.getElementById("alphabetSort");
  let notes = document.querySelectorAll(".note");
  
  if (checkBox.checked == true){
    notes.forEach(note => {
      note.parentElement.style.display = "none";
    });
  } else {
     notes.forEach(note => {
      note.parentElement.style.display = "block";
     });
  }
}

/*AI: Still have to fully implement the date sort: It will display the notes by newest -> oldest
      -all it does for now is hide or show the notes depending if the checkbox is checked or not */
function dateSort() {
  let checkBox = document.getElementById("dateSort");
  let notes = document.querySelectorAll(".note");
  
  if (checkBox.checked == true){
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
  notes_list.innerHTML = ""; // Clear current notes view
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
  invoke("get_notes_from_db_main_display").then((response) => {
    //let notes = JSON.parse(response);
    //let notes equal list of notes with their names and last updated from the database


    console.log(notes);
    notes.sort(compare_last_updated);
    for (const note of notes) {
      let note_element = create_note_element(note);
      notes_list.appendChild(note_element);
    }
  });

}
