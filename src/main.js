const { invoke } = window.__TAURI__.core;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

function new_note(object) {
  invoke("create_note",{object}).then((response) => {
    if (response == true) {
      // invoke("save_notes"); save to file, likely json first before deciding
    } else {
      //display fail message, maybe print reason
    }
  });
}

function openNav() {
  document.getElementById("theSidenav").style.width = "250px";
}

function closeNav() {
  document.getElementById("theSidenav").style.removeProperty("width")
}