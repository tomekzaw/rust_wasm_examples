use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen(module = "/js/imported.js")]
extern "C" {
    fn bar() -> Promise;
}

#[wasm_bindgen]
pub async fn foo() -> Result<JsValue, JsValue> {
    let promise: Promise = bar();
    let jsfuture: JsFuture = JsFuture::from(promise);
    let jsvalue: JsValue = jsfuture.await?;
    let bar: String = jsvalue.as_string().unwrap(); // TODO: error handling
    let foobar: String = String::from("foo") + &bar;
    let result: JsValue = JsValue::from(foobar);
    Ok(result)
}
