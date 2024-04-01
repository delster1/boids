mod utils;

use wasm_bindgen::prelude::*;
use std::fmt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, boids!");
}
impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.pixels.as_slice().chunks(self.width as usize) {
            for &pixel in line {
                
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    pixels: Vec<u8>
}

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
    pub fn pixels(&self) -> *const u8 {
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

        let width = 600;
        let height = 400;

        let pixels = (0..width * height)
            .map(|i| {
                0
            }).collect();
        

        Universe {
            width, 
            height, 
            pixels

        }
    }
}
