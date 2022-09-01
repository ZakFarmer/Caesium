use rand::Rng;

use super::{
    bodies::Particle3D,
    vector::{Vector, Vector3D},
};

pub fn generate_particle(x: f32, y: f32, mass: f32, charge: f32) -> Particle3D {
    let mut rng = rand::thread_rng();
    Particle3D {
        position: Vector3D::from_xy(x, y),
        velocity: Vector3D::from_xy(rng.gen_range(0.0..2.0), rng.gen_range(0.0..2.0)),
        mass,
        charge,
    }
}
