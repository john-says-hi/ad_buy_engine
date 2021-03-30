// This file is included in `bindings.rs`

export function getPayload() {
  return new Date().toString();
}

export function getPayloadLater(callback) {
  setTimeout(() => {
    callback(getPayload());
  }, 1000);
}

export function uikitNotify(msg, status) {
    UIkit.notification({
        message: msg,
        status: status,
        pos: 'bottom-right',
        timeout: 7000
    });
}

export function replaceLocationLogin() {
    window.location.replace("/tertiary#login");
}

export function createDB() {
    var db = new PouchDB('visit_data');
}

export function toggle_uk_dropdown(element) {
    UIkit.dropdown(element).hide(false);
}

export function show_uk_modal(id) {
var modal = UIkit.modal(id);
  modal.show();
}

export function hide_uk_modal(id) {
var modal = UIkit.modal(id);
  modal.hide();
}

export function hide_uk_drop(element) {
    UIkit.drop(element).hide(false);
}

export function show_uk_drop(element) {
    UIkit.drop(element).show();
}

export function remove_element_by_id(id) {
    try {
         document.getElementById(id).remove();
    } catch (e) {
        console.log("Got exception ${e}" );
    }
}

export function select_option(id, idx) {
    try {
         document.getElementById(id).options[idx].selected = true;
    } catch (e) {
        console.log("Got exception ${e}" );
    }
}

export function copy_to_clipboard(id) {
    try {
         let elem = document.getElementById(id);
         elem.select();
         elem.setSelectionRange(0, 99999);
         elem.execCommand("copy");
         alert("Copied: " + elem.value);
    } catch (e) {
        console.log("Got exception ${e}" );
    }
}