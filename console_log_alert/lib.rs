use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn foo() {
    let s = "Hello world!";
    console::log_1(&JsValue::from_str(s));
    alert(s);
}
