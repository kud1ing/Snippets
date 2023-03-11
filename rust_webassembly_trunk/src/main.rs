#![recursion_limit = "1024"]

use web_sys::window;

fn main() {
    console_error_panic_hook::set_once();

    let document = window()
        .and_then(|win| win.document())
        .expect("Could not get document");

    let body = document.body().expect("Could not get body");
    let text_node = document.create_text_node("Hello world!");

    body.append_child(text_node.as_ref())
        .expect("Could not append text");
}