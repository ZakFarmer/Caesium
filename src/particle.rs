use graphics::math::{add, mul_scalar};
use graphics::types::Vec2d;

use crate::constants::{DAMPING_FACTOR, TIMESCALE};

//use crate::constants::DAMPING_FACTOR;

pub struct Particle {
    pub acceleration: Vec2d, // 2D vector for acceleration
    pub position: Vec2d,     // 2D vector for position
    pub velocity: Vec2d,     // 2D vector for velocity
    pub mass: f64,           // Mass of particle
    pub charge: f64,         // Charge of the particle (in coulombs)
    pub colour: [f32; 4],    // Colour of the particle
}

impl Particle {
    pub fn new(
        acceleration: Vec2d,
        position: Vec2d,
        velocity: Vec2d,
        mass: f64,
        charge: f64,
        colour: [f32; 4],
    ) -> Particle {
        Particle {
            acceleration: acceleration,
            position: position,
            velocity: velocity,
            mass: mass,
            charge: charge,
            colour: colour,
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.velocity = add(self.velocity, self.acceleration);
        self.position = add(self.position, self.velocity);
    }
}
