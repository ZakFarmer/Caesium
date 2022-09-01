use super::{
    bodies::Particle3D,
    vector::{Vector, Vector3D},
};

pub fn generate_particle(x: f32, y: f32, mass: f32, charge: f32) -> Particle3D {
    Particle3D {
        position: Vector3D::from_xy(x, y),
        velocity: Vector3D::zero(),
        mass,
        charge,
    }
}
