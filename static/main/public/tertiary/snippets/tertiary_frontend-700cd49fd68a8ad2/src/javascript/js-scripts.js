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

export function sendToSecure() {
    window.location.replace("/secure/");
}

export function loginRedirect() {
    window.location.replace("/tertiary/#login");
}

export function createDB() {
    var db = new PouchDB('visit_data');
}