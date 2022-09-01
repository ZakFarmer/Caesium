use crate::quadtree::{BoundingBox, ParticleQuadtree, ParticleQuadtreeIterator};

use super::{
    simulation::Simulation3D,
    vector::{Vector, Vector3D},
};

pub fn step_simulation(sim: &mut Simulation3D, dt: f32, theta: f32) {
    let (min_x, min_y) = sim.config.min_r.to_xy();
    let (max_x, max_y) = sim.config.max_r.to_xy();

    let bounding_box: BoundingBox = BoundingBox {
        min_x,
        max_x,
        min_y,
        max_y,
    };

    let particle_tree = ParticleQuadtree::new(&sim.position, &sim.charge, &sim.mass, bounding_box);

    for i in 0..sim.number {
        sim.acceleration[i] = Vector3D::zero();

        let particle_tree_iterator = ParticleQuadtreeIterator::new(
            sim.position[i].x,
            sim.position[i].y,
            theta,
            &particle_tree,
            bounding_box,
        );

        for node in particle_tree_iterator {
            let distance = Vector3D {
                x: node.x - sim.position[i].x,
                y: node.y - sim.position[i].y,
                z: 0.0,
            };

            let distance_sqrd: f32 = distance.l2_sqrd();

            if distance_sqrd < sim.config.min_dist_sqrd {
                continue;
            }

            let inverse_distance_cubed: f32 = 1.0 / distance_sqrd.powf(3.0);
            sim.acceleration[i] += distance * node.mass * inverse_distance_cubed;
        }
    }

    sim.integrate(dt);
}
