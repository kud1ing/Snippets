use wasm_bindgen::prelude::*;

/// JavaScript functions called from Rust.
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    fn javascript_function(argument: JsValue);
}

/// A Rust function called from JavaScript.
#[wasm_bindgen]
pub fn rust_function(name: &str) {
    alert(&format!("`rust_function`: Hello, {}! (via `alert()`)", name));
    javascript_function(JsValue::from_str(&format!(
        "`rust_function`: Hello, {}! (via `javascript_function()`)",
        name
    )));
}
