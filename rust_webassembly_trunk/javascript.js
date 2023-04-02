function javascript_function(argument) {
    alert("This is JavaScript's `javascript_function()`: " + argument);

    window.from_rust.rust_function("called from JavaScript's `javascript_function()`");
}
