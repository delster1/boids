use crate::pixel::{Pixel};
use crate::boid::Boid;
use std::fmt;
use wasm_bindgen::prelude::*;
use crate::log;
use crate::utils;
#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    pixels: Vec<Pixel>,
    boids: Vec<Boid>
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
    pub fn update_boid_positions(&mut self) {
        for boid in &mut self.boids {
            boid.update_position();
        }
    }
    fn draw_boid(&mut self, boid: &Boid, pixels: &mut Vec<Pixel>, pixel_color: Pixel) {
        let idx = self.get_index(boid.y, boid.x);
        pixels[idx] = pixel_color; // Central pixel

        // Optional: draw a simple cross for visibility
        // You can remove these if you find it's still inefficient or not necessary
        if boid.x > 0 {
            pixels[self.get_index(boid.y, boid.x - 1)] = pixel_color; // Left
        }
        if boid.x < self.width - 1 {
            pixels[self.get_index(boid.y, boid.x + 1)] = pixel_color; // Right
        }
        if boid.y > 0 {
            pixels[self.get_index(boid.y - 1, boid.x)] = pixel_color; // Up
        }
        if boid.y < self.height - 1 {
            pixels[self.get_index(boid.y + 1, boid.x)] = pixel_color; // Down
        }
    }
    pub fn tick(&mut self) {
        let mut next: Vec<Pixel> = (0..self.width * self.height).map(
            |_i| Pixel {Red: 255, Green: 255, Blue: 255, Alpha: 255 }
        ).collect();
        
        let empty_pixel = Pixel {Red: 255, Green: 255, Blue: 255, Alpha: 255};
        let boid_pixel = Pixel {Red: 0, Green: 0, Blue: 0, Alpha: 255};

        let mut boid_copy = self.boids.clone();
        
        for boid in &mut boid_copy {
            self.draw_boid(&boid, &mut next, boid_pixel);
            boid.update_position();
        }

        self.boids = boid_copy;
        self.pixels = next;
    }
    pub fn new() -> Universe {
        utils::set_panic_hook();

        let width = 800;
        let height = 800;

        let pixels = (0..width * height)
            .map(|_i| Pixel {
                Red: 255, Green: 255, Blue:  255,Alpha:  255
        }).collect();
        
        let mut boids = (0..5)
            .map(|_i| Boid { x: {js_sys::Math::random() * 800.0} as u32, y: {js_sys::Math::random() * 800.0} as u32, vx: 1.0, vy: 1.0}
        ).collect();

        Universe {
            width, 
            height, 
            pixels,
            boids,
        }
    }
}