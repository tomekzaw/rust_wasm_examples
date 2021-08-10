use serde_json::Value;
use wasm_bindgen::prelude::*;

type Result<T> = std::result::Result<T, serde_wasm_bindgen::Error>;

pub fn to_value<T: serde::ser::Serialize>(value: &T) -> Result<JsValue> {
    value.serialize(&serde_wasm_bindgen::Serializer::new().serialize_maps_as_objects(true))
}

#[wasm_bindgen]
pub fn parse(s: &str) -> JsValue {
    let value: Value = serde_json::from_str(s).unwrap();
    to_value(&value).unwrap()
}

#[wasm_bindgen]
pub fn stringify(obj: JsValue) -> String {
    let value: Value = serde_wasm_bindgen::from_value(obj).unwrap();
    value.to_string()
}
