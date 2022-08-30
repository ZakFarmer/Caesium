/**
 * This is a module for physics calculations, most of them are quite simple but all of them should have
 * tests (it's good to have a solid foundation when working with physics to avoid debugging headaches later
*/
use graphics::types::Vec2d;

use crate::constants::{GRAVITATIONAL_CONSTANT, PHYSICS_SCALE};

/**
 * Scales a vector to the bounds of the screen
 */
pub fn scale_to_screen(pos: Vec2d) -> Vec2d {
    [pos[0] * PHYSICS_SCALE, pos[1] * PHYSICS_SCALE]
}

/**
 * Calculates the angular velocity of a body at radius r and linear velocity v
 *
 * OMEGA = (v * sin(theta)) / r
 */
pub fn calculate_angular_velocity(r: f64, theta: f64, v: f64) -> f64 {
    (v * f64::sin(theta)) / r
}

/**
 * Calculates the distance between two positions p1 and p2.
 *
 * d = sqrt((p1.x - p2.x)^2 + (p1.y - p2.y)^2)
 */
pub fn calculate_distance(p1: Vec2d, p2: Vec2d) -> f64 {
    let x = p1[0] - p2[0];
    let y = p1[1] - p2[1];

    (x * x + y * y).sqrt()
}

/**
 * Calculates the gravitational force between two bodies with respective masses m1 and m2
 * and distance between them d (notice d is not squared in the denominator as we are working in only two dimensions)
 *
 * F = G * m1 * m2 / d
 */
pub fn calculate_gravitational_force(m1: f64, m2: f64, d: f64) -> f64 {
    (GRAVITATIONAL_CONSTANT * (m1 * m2)) / (d * d)
}

/**
 * Calculates the gravitational force vector between two bodies with respective masses m1 and m2 and distance vector r.
 *
 * F = (-G * m1 * m2) / (d^2 * r) (normalised)
 */
pub fn calculate_gravitational_force_vector(r: Vec2d, m1: f64, m2: f64) -> Vec2d {
    let force_x: f64 =
        (-GRAVITATIONAL_CONSTANT * m1 * m2) / f64::abs(r[0]).powi(2) * normalise(r)[0];

    let force_y: f64 =
        (-GRAVITATIONAL_CONSTANT * m1 * m2) / f64::abs(r[1]).powi(2) * normalise(r)[1];

    [force_x, force_y]
}

/**
 * Calculates kinetic energy (in J) of a body with mass m and velocity v.
 *
 * E = 1/2 * m * v^2
 */
pub fn calculate_kinetic_energy(m: f64, v: f64) -> f64 {
    0.5 * m * v * v
}

/**
 * Calculates the momentum of a body with mass m and velocity v.
 *
 * p = mv
 */
pub fn calculate_momentum(m: f64, v: f64) -> f64 {
    m * v
}

/**
 * Calculates the respective velocities of two bodies
 * (mass m1, m2 & velocity v1, v2) after a collision.
 *
 * https://en.wikipedia.org/wiki/Elastic_collision (See section on Two-dimensional collision with two moving objects)
 */
pub fn calculate_velocities_after_collision(
    v1: Vec2d,
    v2: Vec2d,
    m1: f64,
    m2: f64,
    theta1: f64,
    theta2: f64,
    psi: f64,
) -> [[f64; 2]; 2] {
    let v1_magnitude: f64 = magnitude(v1);
    let v2_magnitude: f64 = magnitude(v2);

    let first_part: f64 = (v1_magnitude * f64::cos(theta1 - psi) * (m1 - m2)
        + 2.0 * m2 * v2_magnitude * f64::cos(theta2 - psi))
        / (m1 + m2);

    // Calculate velocity of body 1 after collision
    let v1_after = [
        first_part * f64::cos(psi)
            + v1_magnitude * f64::sin(theta1 - psi) * f64::cos(psi + std::f64::consts::FRAC_PI_2),
        first_part * f64::sin(psi)
            + v1_magnitude * f64::sin(theta1 - psi) * f64::sin(psi + std::f64::consts::FRAC_PI_2),
    ];

    // Calculate velocity of body 2 after collision
    let v2_after = [
        first_part * f64::cos(psi)
            + v2_magnitude * f64::sin(theta2 - psi) * f64::cos(psi + std::f64::consts::FRAC_PI_2),
        first_part * f64::sin(psi) * -1.0
            + v2_magnitude * f64::sin(theta2 - psi) * f64::sin(psi + std::f64::consts::FRAC_PI_2),
    ];

    [v1_after, v2_after]
}

pub fn magnitude(v: Vec2d) -> f64 {
    (v[0] * v[0] + v[1] * v[1]).sqrt()
}

pub fn normalise(v: Vec2d) -> Vec2d {
    let magnitude = magnitude(v);
    [v[0] / magnitude, v[1] / magnitude]
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_calculate_distance() {
        let p1 = [0.0, 0.0];
        let p2 = [3.0, 4.0];

        assert_eq!(5.0, super::calculate_distance(p1, p2));
    }

    #[test]
    fn test_calculate_gravitational_force() {
        let m1 = 3.0;
        let m2 = 10.0;
        let d = 10.0;

        assert_eq!(
            2.0022240000000002e-10,
            super::calculate_gravitational_force(m1, m2, d)
        );
    }

    #[test]
    fn test_calculate_kinetic_energy() {
        let m = 3.0;
        let v = 10.0;

        assert_eq!(150.0, super::calculate_kinetic_energy(m, v));
    }

    #[test]
    fn test_calculate_momentum() {
        let m = 3.0;
        let v = 10.0;

        assert_eq!(30.0, super::calculate_momentum(m, v));
    }
}
