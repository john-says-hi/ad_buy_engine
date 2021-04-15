use wasm_bindgen::prelude::*;
use web_sys::Element;

// wasm-bindgen will automatically take care of including this script
#[wasm_bindgen(module = "/src/utils/javascript/js-scripts.js")]
extern "C" {
    #[wasm_bindgen(js_name = "getPayload")]
    pub fn get_payload() -> String;

    #[wasm_bindgen(js_name = "getPayloadLater")]
    pub fn get_payload_later(payload_callback: JsValue);

    #[wasm_bindgen(js_name = "uikitNotify")]
    pub fn uikit_notify(msg: JsValue, status: JsValue);

    #[wasm_bindgen(js_name = "replaceLocationLogin")]
    pub fn redirect_login();

    #[wasm_bindgen(js_name = "toggle_uk_dropdown")]
    pub fn toggle_uk_dropdown(element: Element);

    #[wasm_bindgen(js_name = "hide_uk_drop")]
    pub fn hide_uk_drop(element: Element);
    #[wasm_bindgen(js_name = "show_uk_drop")]
    pub fn show_uk_drop(element: Element);

    #[wasm_bindgen(js_name = "show_uk_modal")]
    pub fn show_uk_modal(id: &str);

    #[wasm_bindgen(js_name = "hide_uk_modal")]
    pub fn hide_uk_modal(id: &str);

    #[wasm_bindgen(js_name = "remove_element_by_id")]
    pub fn remove_element_by_id(id: &str);

    #[wasm_bindgen(js_name = "select_option")]
    pub fn select_option(id: &str, idx: usize);

    #[wasm_bindgen(js_name = "copy_to_clipboard")]
    pub fn copy_to_clipboard(id: &str);

    #[wasm_bindgen(js_name = "add_class_name")]
    pub fn add_class_name(element: Element, class_name: &str);

    #[wasm_bindgen(js_name = "remove_class_name")]
    pub fn remove_class_name(element: Element, class_name: &str);
}
