use serde_json::Value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn serialize(s: &str) -> JsValue {
    let value: Value = serde_json::from_str(s).unwrap();
    JsValue::from_serde(&value).unwrap()
}

#[wasm_bindgen]
pub fn parse(obj: &JsValue) -> String {
    let value: Value = obj.into_serde().unwrap();
    value.to_string()
}
