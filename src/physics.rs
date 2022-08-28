use graphics::types::Vec2d;

use crate::constants::{GRAVITATIONAL_CONSTANT, PHYSICS_SCALE};

pub fn scale_to_screen(pos: Vec2d) -> Vec2d {
    [pos[0] * PHYSICS_SCALE, pos[1] * PHYSICS_SCALE]
}

/**
 * Calculates the gravitational force between two bodies with respective masses m1 and m2
 * and distance between them d.
 *
 * F = G * m1 * m2 / d^2
 */
pub fn calculate_gravitational_force(m1: f64, m2: f64, d: f64) -> f64 {
    GRAVITATIONAL_CONSTANT * (m1 * m2) / (d * d)
}
