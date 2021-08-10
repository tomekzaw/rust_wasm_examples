use js_sys::Function;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn call_lambda(f: Function) -> f64 {
    let this = JsValue::null();
    let x: JsValue = f.call0(&this).unwrap();
    x.as_f64().unwrap() + 3.0
}
