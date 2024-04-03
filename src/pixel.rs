use crate::boid::Boid;
use std::fmt;
use wasm_bindgen::prelude::*;
use crate::log;
use crate::utils;
#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[wasm_bindgen]
pub struct Pixel {
    
    pub Red : u8,
    pub Green : u8,
    pub Blue : u8,
    pub Alpha : u8
    
}

