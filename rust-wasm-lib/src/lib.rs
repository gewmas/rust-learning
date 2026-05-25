use wasm_bindgen::prelude::*;

// This allows you to call this function from JavaScript/React
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hi from Rust WASM, {}!", name)
}

// You can even pass complex objects back and forth using Serde
#[wasm_bindgen]
pub fn expensive_calculation(a: i32, b: i32) -> i32 {
    // Imagine some heavy math here
    a * b + 42
}
