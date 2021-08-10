use wasm_bindgen::prelude::*;
use web_sys::console;

fn inspect(value: &dyn std::fmt::Debug) {
    console::log_1(&JsValue::from_str(&format!("{:?}", value)[..]));
}

macro_rules! inspect {
    ($l:tt) => {
        let value = JsValue::from_str(&format!("{:?}", $l)[..]);
        unsafe {
            console::log_1(&value);
        }
    };
}
