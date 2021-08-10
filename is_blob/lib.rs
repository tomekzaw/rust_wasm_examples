use wasm_bindgen::{prelude::*, JsCast};
use web_sys::Blob;

// #[wasm_bindgen]
// pub fn isblob(obj: JsValue) -> JsValue {
//     let is = obj.is_instance_of::<Blob>();
//     JsValue::from_bool(is)
// }

#[wasm_bindgen]
pub fn isblob(obj: JsValue) -> bool {
    obj.is_instance_of::<Blob>()
}
