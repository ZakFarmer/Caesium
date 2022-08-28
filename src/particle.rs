use graphics::math::{add, mul_scalar};
use graphics::types::Vec2d;

use crate::constants::{DAMPING_FACTOR, PHYSICS_SCALE, SCREEN_HEIGHT, SCREEN_WIDTH, TIMESCALE};
use crate::physics::scale_to_screen;

//use crate::constants::DAMPING_FACTOR;

#[derive(Debug)]
pub struct Particle {
    pub acceleration: Vec2d, // 2D vector for acceleration
    pub position: Vec2d,     // 2D vector for position
    pub radius: f64,
    pub velocity: Vec2d,  // 2D vector for velocity
    pub mass: f64,        // Mass of particle
    pub colour: [f32; 4], // Colour of the particle
}

impl Particle {
    pub fn new(
        acceleration: Vec2d,
        position: Vec2d,
        radius: f64,
        velocity: Vec2d,
        mass: f64,
        colour: [f32; 4],
    ) -> Particle {
        Particle {
            acceleration: acceleration,
            position: position,
            radius: radius,
            velocity: velocity,
            mass: mass,
            colour: colour,
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.velocity = add(self.velocity, self.acceleration);
        self.position = add(self.position, mul_scalar(self.velocity, TIMESCALE));

        if self.position[0] < 0.0 {
            self.acceleration[0] = 0.0;
            self.position[0] = SCREEN_WIDTH - self.radius;
        }

        if self.position[1] < 0.0 {
            self.acceleration[1] = 800.0;
            self.position[1] = SCREEN_HEIGHT - self.radius;
        }

        if self.position[0] > SCREEN_WIDTH / PHYSICS_SCALE {
            self.acceleration[0] = 0.0;
            self.position[0] = self.radius * 2.0;
        }

        if self.position[1] > SCREEN_HEIGHT / PHYSICS_SCALE {
            self.acceleration[1] = 0.0;
            self.position[1] = self.radius * 2.0;
        }
    }
}
