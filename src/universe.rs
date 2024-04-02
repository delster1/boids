use crate::pixel::{Pixel};
use std::fmt;
use wasm_bindgen::prelude::*;
use crate::log;
use crate::utils;
#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    pixels: Vec<Pixel>
}


#[wasm_bindgen]
impl Universe {
    fn get_index(&mut self, row : u32, col : u32) -> usize {
        return (row * self.width + col) as usize;
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn pixels(&self) -> *const Pixel {
        self.pixels.as_ptr()
    }
    pub fn tick(&mut self) {
        let mut next = self.pixels.clone();
        for row in 0..self.width {
            for col in 0..self.height {

                
                // behavior every tick
                }

            }
        
        self.pixels = next;
    }
    pub fn new() -> Universe {
        utils::set_panic_hook();

        let width = 800;
        let height = 800;

        let pixels = (0..width * height)
            .map(|_i| Pixel {
                Red: 0, Green: 0, Blue:  0,Alpha:  255
            }).collect();
        

        Universe {
            width, 
            height, 
            pixels

        }
    }
}