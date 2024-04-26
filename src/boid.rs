use crate::pixel::{Pixel};
use std::fmt;
use wasm_bindgen::prelude::*;
use crate::log;
use crate::utils;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
#[wasm_bindgen]

pub enum BoidGroup {
    None, 
    One,
    Two
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
#[wasm_bindgen]
pub struct Boid {
    pub x: u32,
    pub y: u32,
    pub vx: f32,
    pub vy: f32,
    pub group: BoidGroup,
    pub bias: f32,
}

impl Boid{
    
    fn calculate_distance(&mut self, other : Boid) -> f64{
        let x_dist = ((self.x - other.x) as f32).powf(2.0);
        let y_dist = ((self.y - other.y) as f32).powf(2.0);

        let diff = (x_dist + y_dist) as f64;
        js_sys::Math::sqrt(diff)
    }

    fn find_close_boids(&mut self, boids : &Vec<Boid>) -> Vec<Boid>{
        let mut next_boids: Vec<Boid> = vec!();
        for (i, boid) in boids.iter().enumerate() {
            let distance : f64  = self.calculate_distance(*boid);
            if distance < 40.0 && self != boid && distance >= 8.0 {
                next_boids.push(boid.clone());
            }
        }
        next_boids
    }

    fn separate(&mut self, boids : &Vec<Boid>, separation: f32){
        let mut close_dx : f32 = 0.0 ;
        let mut close_dy : f32 = 0.0;
        for boid in boids.iter() {
            let distance : f64  = self.calculate_distance(*boid);
            if distance < 8.0 && self != boid  {
                close_dx += (self.x as f32) - (boid.x as f32);
                close_dy += (self.y as f32) - (boid.y as f32);
            }
        }
        self.vx += close_dx  * separation ;
        self.vy += close_dy * separation ;
    }

    fn align_and_cohere(&mut self, boids: &Vec<Boid>, alignment: f32, cohesion: f32) {
        let mut xvel_avg = 0.0;
        let mut yvel_avg = 0.0;

        let neighboring_boids = boids.len() as f32;

        let mut xpos_avg = 0.0;
        let mut ypos_avg = 0.0;
        for boid in boids.iter() {
            xvel_avg += boid.vx;
            yvel_avg += boid.vy;

            xpos_avg += boid.x as f32;
            ypos_avg += boid.y as f32;
        }
        xvel_avg = xvel_avg / neighboring_boids;
        yvel_avg = yvel_avg / neighboring_boids;
    
        xpos_avg = xpos_avg/neighboring_boids;
        ypos_avg = ypos_avg/neighboring_boids;

        self.vx += ((xvel_avg - self.vx) * alignment + (xpos_avg - (self.x as f32)) * cohesion);
        self.vy += ((yvel_avg - self.vy) * alignment + (ypos_avg - (self.y as f32)) * cohesion);

    }



    pub fn update_position(&mut self, max_speed: f32, min_speed: f32, separation: f32, alignment: f32, cohesion: f32, boids: &Vec<Boid>) {
        self.separate(&boids, separation);
        let close_boids = self.find_close_boids(&boids);
        if close_boids.len() > 0 {
            self.align_and_cohere(&close_boids, alignment, cohesion);
        }

        match self.group {
            BoidGroup::None => {},
            BoidGroup::One => {
                self.vx = (1.0 - self.bias)*self.vx + (self.bias * 1.0)
            },
            BoidGroup::Two => {
                self.vx = (1.0 - self.bias)*self.vx + (self.bias * -1.0)}
        }
    
        let speed = (self.vx.powi(2) + self.vy.powi(2)).sqrt();
        if speed > max_speed {
            self.vx = (self.vx / speed) * max_speed;
            self.vy = (self.vy / speed) * max_speed;
        } else if speed < min_speed {
            self.vx = (self.vx / speed) * min_speed;
            self.vy = (self.vy / speed) * min_speed;
        }
    
        // Randomize direction slightly to prevent sticking
        self.vx += (js_sys::Math::random() as f32 - 0.5) * 0.1;
        self.vy += (js_sys::Math::random() as f32 - 0.5) * 0.1;
    
        // Update position based on velocity
        self.x = ((self.x as f32 + self.vx).round() as u32).clamp(1, 799);
        self.y = ((self.y as f32 + self.vy).round() as u32).clamp(1, 799);
        let max_width = 800; // Assuming a universe width of 800
        let max_height = 800; // Assuming a universe height of 800
        let leftmargin = 100;
        let rightmargin = max_width - leftmargin;
        let turnfactor = 0.2;
        let topmargin = 100;
        let bottommargin = max_height - topmargin;
        // Handle wrapping around the boundaries
        if self.x <= 1 {
            self.x = 799; // Move to the right side if too far left
        } else if self.x >= 799 {
            self.x = 1; // Move to the left side if too far right
        }
        if self.x < leftmargin{
            self.vx = self.vx + turnfactor;
        }
        if self.x > rightmargin {
            self.vx = self.vx - turnfactor;

        }
        
        if self.y <= 1 {
            self.y = 799; // Move to the bottom if too high
        } else if self.y >= 799 {
            self.y = 1; // Move to the top if too low
        }
        if self.y > bottommargin {
            
            self.vy = self.vy - turnfactor;
        }
        if self.y < topmargin{

            self.vy = self.vy + turnfactor;
        }
    }
    
    pub fn new() -> Boid{
        let mut boid_group = BoidGroup::None;
        let mut bias = 0.0;
        let boid_group_chance : f32 = 20.0;
        let random_val = {js_sys::Math::random() as f32 } * 100.0;
        if random_val < boid_group_chance {
            let coin_flip = {js_sys::Math::random() as f32} > 0.5;
            bias = random_val / 100.0;
            boid_group = if coin_flip { BoidGroup::One } else { BoidGroup::Two };
        }
        Boid { x: {js_sys::Math::random() as f32} as u32, y: {js_sys::Math::random() as f32} as u32, vx: {js_sys::Math::random() as f32} / 10.0, vy: {js_sys::Math::random() as f32} / 10.0, group: boid_group, bias: bias}
    }
}