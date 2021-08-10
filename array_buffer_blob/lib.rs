use js_sys::{Array, ArrayBuffer, Uint8Array};
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::Blob;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    Ok(())
}

const LENGTH: usize = 100_000_123; // TODO: 100 MB

#[wasm_bindgen]
pub fn make_box_u8_array_buffer() -> Box<[u8]> {
    let mut v = Vec::<u8>::new();
    v.resize(LENGTH, 42);
    v.into_boxed_slice()
}

#[wasm_bindgen]
pub fn make_array_buffer_web_sys() -> ArrayBuffer {
    let ab = ArrayBuffer::new(LENGTH as u32);
    let ta = Uint8Array::new(&ab);
    ta.fill(42, 0, LENGTH as u32);
    ab
}

#[wasm_bindgen]
pub fn make_blob_with_buffer() -> Result<Blob, JsValue> {
    let ab = ArrayBuffer::new(LENGTH as u32);
    let ta = Uint8Array::new(&ab);
    ta.fill(42, 0, LENGTH as u32);
    let arr = Array::new_with_length(1);
    arr.set(0, JsValue::from(ab));
    Blob::new_with_buffer_source_sequence(&arr)
}

#[wasm_bindgen]
pub fn make_blob_with_u8_array() -> Result<Blob, JsValue> {
    let ab = ArrayBuffer::new(LENGTH as u32);
    let ta = Uint8Array::new(&ab);
    ta.fill(42, 0, LENGTH as u32);
    let arr = Array::new_with_length(1);
    arr.set(0, JsValue::from(ta));
    Blob::new_with_u8_array_sequence(&arr)
}

#[wasm_bindgen]
pub fn take_uint8array(arr: &[u8]) -> u8 {
    *arr.iter().next().unwrap()
}

#[wasm_bindgen]
pub async fn take_blob(blob: Blob) -> u8 {
    let ab = JsFuture::from(blob.array_buffer())
        .await
        .unwrap()
        .unchecked_into();
    let ta = Uint8Array::new(&ab);
    // ta.get_index(1) // this is slow
    let mut arr = vec![0; ta.length() as usize];
    ta.copy_to(&mut arr);
    *arr.iter().last().unwrap()
}
