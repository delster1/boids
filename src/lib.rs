mod utils;
mod universe;
mod pixel;
use wasm_bindgen::prelude::*;
pub use universe::Universe;
extern crate js_sys;
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}
#[wasm_bindgen]
pub fn greet() {
    alert("Hello, boids!");
}




