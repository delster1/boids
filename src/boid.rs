use crate::pixel::{Pixel};
use std::fmt;
use wasm_bindgen::prelude::*;
use crate::log;
use crate::utils;
#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[wasm_bindgen]
pub struct Boid {
    pub x: u32,
    pub y: u32,
    pub vx: f64,
    pub vy: f64,
    max_speed: f64,
    separation: f64,
    alignment: f64,
    cohesion: f64


}

impl Boid{
    pub fn update_position(&mut self) {
        // Update the boid's position based on its velocity
        // Since x and y are u32, we need to round the result of the addition
        self.x = (self.x as f64 + self.vx).round() as u32;
        self.y = (self.y as f64 + self.vy).round() as u32;

        // Here you might want to include logic to handle boundaries, such as wrapping around or bouncing back
        // For example, wrapping around:
        let max_width = 800; // Assuming a universe width of 800
        let max_height = 800; // Assuming a universe height of 800

        // Wrap around the horizontal axis
        if self.x >= max_width {
            self.x = 0;
        } else if self.x == 0 && self.vx < 0.0 {
            self.x = max_width - 1;
        }

        // Wrap around the vertical axis
        if self.y >= max_height {
            self.y = 0;
        } else if self.y == 0 && self.vy < 0.0 {
            self.y = max_height - 1;
        }
    }
}