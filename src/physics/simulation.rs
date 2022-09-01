use super::{
    bodies::Particle3D,
    vector::{Vector, Vector3D},
};

#[derive(Debug)]
pub struct SimulationConfig3D {
    pub min_dist: f32,
    pub min_dist_sqrd: f32,
    pub min_r: Vector3D,
    pub max_r: Vector3D,
}

impl SimulationConfig3D {
    pub fn new(min_dist: f32, min_r: Vector3D, max_r: Vector3D) -> Self {
        Self {
            min_dist,
            min_dist_sqrd: min_dist * min_dist,
            min_r,
            max_r,
        }
    }
}

#[derive(Debug)]
pub struct Simulation3D {
    pub timescale: f32,              // The timescale of the simulation
    pub number: usize,               // The number of particles in the simulation
    pub charge: Vec<f32>,            // Vector of particle charges (in Coulombs)
    pub mass: Vec<f32>,              //  Vector of particle masses (in kg)
    pub position: Vec<Vector3D>,     // Vector of particle positions (vector of m)
    pub velocity: Vec<Vector3D>,     // Vector of particle velocities (vector of m/s)
    pub acceleration: Vec<Vector3D>, // Vector of particle accelerations (vector of m/s^2)
    pub config: SimulationConfig3D,
}

impl Simulation3D {
    pub fn empty(number: usize, config: SimulationConfig3D) -> Self {
        Self {
            timescale: 1.0,
            number,
            charge: vec![0.0; number],
            mass: vec![0.0; number],
            position: vec![Vector3D::zero(); number],
            velocity: vec![Vector3D::zero(); number],
            acceleration: vec![Vector3D::zero(); number],
            config,
        }
    }

    pub fn get(&self, i: usize) -> Particle3D {
        Particle3D {
            charge: self.charge[i],
            mass: self.mass[i],
            position: self.position[i],
            velocity: self.velocity[i],
        }
    }

    pub fn integrate(&mut self, dt: f32) {
        for i in 0..self.number {
            self.velocity[i] += self.acceleration[i] * dt * self.timescale;
            self.position[i] += self.velocity[i] * dt * self.timescale;

            if self.position[i].x <= (self.mass[i] / (0.5 * 1e-26)) {
                self.velocity[i].x *= -1.0;
            } else if self.position[i].y <= (self.mass[i] / (0.5 * 1e-26)) {
                self.velocity[i].y *= -1.0;
            }

            if self.position[i].x >= self.config.max_r.x - (self.mass[i] / (0.5 * 1e-26)) {
                self.velocity[i].x *= -1.0;
            } else if self.position[i].y >= self.config.max_r.y - (self.mass[i] / (0.5 * 1e-26)) {
                self.velocity[i].y *= -1.0;
            }
        }
    }

    pub fn set(&mut self, i: usize, particle: &Particle3D) {
        self.charge[i] = particle.charge;
        self.mass[i] = particle.mass;
        self.position[i] = particle.position;
        self.velocity[i] = particle.velocity;
        self.acceleration[i] = Vector3D::zero();
    }
}
