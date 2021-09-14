# Rust WebAssembly client with `wasm-pack`

This is a minimal example that shows:
* how a browser can call [WebAssembly](https://webassembly.org) generated from [Rust](https://www.rust-lang.org) code
* how Rust code can call a Web API function (`alert()` in this example).

<img width="710" alt="Bildschirmfoto 2019-08-07 um 10 26 55" src="https://user-images.githubusercontent.com/391975/62607213-fe0cd000-b8fd-11e9-91e8-56ba4fe93f43.png">

## Instructions

* make sure that [Rust is installed](https://www.rust-lang.org/tools/install)
* make sure that [`wasm-pack` is installed](https://rustwasm.github.io/wasm-pack/installer/)
* execute `wasm-pack build --target web`
* copy the content of `javascript.js` into `pkg/rust_webassembly.js
* make sure that [local file restrictions are disabled](https://www.thepolyglotdeveloper.com/2014/08/bypass-cors-errors-testing-apis-locally/) in your web browser
* open `index.html` in that web browser
