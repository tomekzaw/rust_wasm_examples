use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn ping() -> String {
    "pong".into()
}
