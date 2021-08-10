use js_sys::JSON;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::Blob;

#[wasm_bindgen]
pub fn json_stringify_with_blobs(obj: JsValue) -> JsValue {
    // Result<JsString, JsValue>
    let lambda = &|_key: String, value: JsValue| -> JsValue {
        match value.is_instance_of::<Blob>() {
            true => JsValue::from_str("this is blob"),
            false => value,
        }
    };

    // approach #1: manually written JS wrapper
    // json_stringify_wrapper(obj, lambda)

    // approach #2: import function from JS
    // https://rustwasm.github.io/docs/wasm-bindgen/examples/import-js.html
    // original_json_stringify(obj, lambda)

    // approach #3: use js_sys::JSON::stringify_with_replacer
    // https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen/closure/struct.Closure.html
    let replacer = Closure::wrap(Box::new(lambda) as Box<dyn Fn(String, JsValue) -> JsValue>);
    JSON::stringify_with_replacer(&obj, replacer.as_ref().unchecked_ref())
        .unwrap()
        .into()
}

// approach #1
#[wasm_bindgen(module = "/js/imported.js")]
extern "C" {
    fn json_stringify_wrapper(obj: JsValue, lambda: &dyn Fn(String, JsValue) -> JsValue)
        -> JsValue;
}

// approach #2
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = JSON, js_name = stringify)]
    fn original_json_stringify(
        obj: JsValue,
        lambda: &dyn Fn(String, JsValue) -> JsValue,
    ) -> JsValue;
}
