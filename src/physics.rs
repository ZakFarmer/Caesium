use graphics::types::Vec2d;

use crate::constants::PHYSICS_SCALE;

/**
 * Scales a vector to the bounds of the screen
 */
pub fn scale_to_screen(pos: Vec2d) -> Vec2d {
    [pos[0] * PHYSICS_SCALE, pos[1] * PHYSICS_SCALE]
}

/**
 * Calculates the distance between two bodies with respective positions p1 and p2.
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
 * and distance between them d.
 *
 * F = G * m1 * m2 / d^2
 */
pub fn calculate_gravitational_force(m1: f64, m2: f64, d: f64) -> f64 {
    (GRAVITATIONAL_CONSTANT * (m1 * m2)) / (d * d)
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
}
