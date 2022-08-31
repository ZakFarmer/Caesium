pub use super::vector::Vector3D;

#[derive(Debug)]
pub struct Particle3D {
    pub charge: f32,        // The charge of the body (in Coulombs)
    pub mass: f32,          // The mass of the body (in kg)
    pub position: Vector3D, // The position of the body (vector of m)
    pub velocity: Vector3D, // The velocity of the body (vector of m/s)
}
