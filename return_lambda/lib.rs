use js_sys::Function;
use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// pub fn make_f() -> JsValue {
//     // result lambda can be called multiple number of times
//     let cb = Closure::wrap(Box::new(|x: i32| -> i32 { x * 2 }) as Box<dyn FnMut(i32) -> i32>);
//     cb.into_js_value()
// }

// #[wasm_bindgen]
// pub fn make_f() -> JsValue {
//     // result lambda can be called only once
//     let c = Closure::once(move |x: i32| -> i32 { x * 2 });
//     c.into_js_value()
// }

#[wasm_bindgen]
pub fn make_f() -> JsValue {
    // result lambda can be called only once
    Closure::once_into_js(move |x: i32| -> i32 { x * 2 })
}
