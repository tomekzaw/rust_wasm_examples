use serde_json::Value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse(s: &str) -> JsValue {
    let mut value: Value = serde_json::from_str(s).unwrap();
    JsValue::from_serde(&value).unwrap()
}

#[wasm_bindgen]
pub fn stringify(obj: &JsValue) -> String {
    let value: Value = serde_wasm_bindgen::from_value(obj).unwrap();
    value.to_string()
}
