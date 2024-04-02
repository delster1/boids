use wasm_bindgen::prelude::*;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[wasm_bindgen]
pub struct Pixel {
    pub Red : u8,
    pub Green : u8,
    pub Blue : u8,
    pub Alpha : u8
}