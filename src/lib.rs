mod utils;
extern crate log;
use wasm_bindgen::prelude::*;
use std::fmt;
extern crate js_sys;    
extern crate web_sys;
use wasm_bindgen::prelude::*;

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
#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[wasm_bindgen]
pub struct Pixel {
    Red : u8,
    Green : u8,
    Blue : u8,
    Alpha : u8
}
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
