use crate::pixel::{Pixel};
use crate::boid::Boid;
use std::fmt::{self, Alignment};
use wasm_bindgen::prelude::*;
use crate::log;
use crate::utils;

static MAX_SPEED : f32 = 6.0;
static MIN_SPEED : f32 = 3.0;
static SEPARATION : f32 = 0.05;
static ALIGNMENT : f32 = 0.05;
static COHESION : f32 = 0.0005;
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

 
    fn draw_boid(&mut self, boid: &Boid, pixels: &mut Vec<Pixel>, pixel_color: Pixel) {
        if boid.x >= self.width || boid.y >= self.height {
            return; // Skip drawing if boid position is out of screen bounds
        }
            
        let idx = self.get_index(boid.y, boid.x);
        pixels[idx] = pixel_color; // Central pixel

        // Optional: draw a simple cross for visibility
        // Draw to the left if there is space
        if boid.x > 1 {
            let left_idx = self.get_index(boid.y, boid.x - 1);
            pixels[left_idx] = pixel_color;
        }

        // Draw to the right if there is space
        if boid.x + 1 < self.width {
            let right_idx = self.get_index(boid.y, boid.x + 1);
            pixels[right_idx] = pixel_color;
        }

        // Draw above if there is space
        if boid.y > 1 {
            let up_idx = self.get_index(boid.y - 1, boid.x);
            pixels[up_idx] = pixel_color;
        }

        // Draw below if there is space
        if boid.y + 1 < self.height {
            let down_idx = self.get_index(boid.y + 1, boid.x);
            pixels[down_idx] = pixel_color;
        }
    }
    pub fn tick(&mut self) {
        let mut next: Vec<Pixel> = (0..self.width * self.height).map(
            |_i| Pixel {Red: 255, Green: 255, Blue: 255, Alpha: 255 }
        ).collect();
        
        let empty_pixel = Pixel {Red: 255, Green: 255, Blue: 255, Alpha: 255};
        let boid_pixel = Pixel {Red: 0, Green: 0, Blue: 0, Alpha: 255};

        let mut boids_copy = self.boids.clone();

        for boid in &mut boids_copy {
            self.draw_boid(boid, &mut next, boid_pixel);
            boid.update_position(MAX_SPEED, MIN_SPEED, SEPARATION, ALIGNMENT, COHESION, &self.boids);
        }

        self.boids = boids_copy;
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
        
        let mut boids = (0..300)
            .map(|_i| Boid::new()
        ).collect();

        Universe {
            width, 
            height, 
            pixels,
            boids,
        }
    }
}