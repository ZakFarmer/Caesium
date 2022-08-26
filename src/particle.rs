use graphics::math::{add, mul_scalar};
use graphics::types::Vec2d;

use crate::constants::DAMPING_FACTOR;

pub struct Particle {
    pub position: Vec2d,
    pub velocity: Vec2d,
}

impl Particle {
    pub fn new(position: Vec2d, velocity: Vec2d) -> Particle {
        Particle {
            position: position,
            velocity: velocity,
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.position = add(self.position, mul_scalar(self.velocity, dt));

        if self.position[1] < 0.0 {
            self.position[1] = 0.0;
            self.velocity[1] = -self.velocity[1] * DAMPING_FACTOR;
        }

        if self.position[0] < 0.0 {
            self.position[0] = 0.0;
            self.velocity[0] = -self.velocity[0] * DAMPING_FACTOR;
        }

        if self.position[0] > 800.0 {
            self.position[0] = 800.0;
            self.velocity[0] = -self.velocity[0] * DAMPING_FACTOR;
        }

        if self.position[1] > 800.0 {
            self.position[1] = 800.0;
            self.velocity[1] = -self.velocity[1] * DAMPING_FACTOR;
        }
    }
}
