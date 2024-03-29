#![recursion_limit = "1024"]

use wasm_bindgen::prelude::*;
use web_sys::window;

/// JavaScript functions called from Rust.
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    fn javascript_function(argument: JsValue);
}

/// A Rust function called from JavaScript.
#[wasm_bindgen]
pub fn rust_function(name: &str) {
    alert(&format!(
        "This is Rust's `rust_function()`: {}!`)",
        name
    ));
}

fn main() {
    console_error_panic_hook::set_once();

    // Modify the DOM.
    {
        let document = window()
            .and_then(|win| win.document())
            .expect("Could not get document");

        let body = document.body().expect("Could not get body");
        let text_node = document.create_text_node("Hello world!");

        body.append_child(text_node.as_ref())
            .expect("Could not append text");
    }

    // Call JavaScript functions.
    {
        alert("This is Rust's `main()` calling `alert()`");
        javascript_function(JsValue::from_str(
            "called from Rust's `main()`",
        ));
    }
}
