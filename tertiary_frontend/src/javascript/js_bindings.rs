use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/javascript/js-scripts.js")]
extern "C" {
    #[wasm_bindgen(js_name = "sendToSecure")]
    pub fn send_to_secure();

    #[wasm_bindgen(js_name = "loginRedirect")]
    pub fn login_redirect();
}
