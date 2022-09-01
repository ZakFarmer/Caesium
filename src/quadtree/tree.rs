use crate::physics::vector::Vector3D;

use super::boundingbox::BoundingBox;

const EPSILON: f32 = 1e-4;

fn l2(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    let dx: f32 = x2 - x1;
    let dy: f32 = y2 - y1;

    (dx * dx + dy * dy).sqrt()
}

#[derive(Debug)]
pub struct ParticleQuadtree {
    pub x: f32,      // X coordinate of the center of mass
    pub y: f32,      // Y coordinate of the center of mass
    pub charge: f32, // Total charge of the node
    pub mass: f32,   // Total mass of the node
    pub children: Vec<Option<Self>>,
}

impl ParticleQuadtree {
    // Creates a ParticleQuadtree with no children.
    pub fn empty() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            charge: 0.0,
            mass: 0.0,
            children: vec![None, None, None, None],
        }
    }

    // Inserts a body into the tree.
    pub fn insert(&mut self, x: f32, y: f32, charge: f32, mass: f32, bounding_box: BoundingBox) {
        if charge == 0. || mass == 0. {
            return;
        }

        if self.charge == 0. || self.mass == 0. {
            self.x = x;
            self.y = y;
            self.charge = charge;
            self.mass = mass;
            return;
        }

        let mut parent: &mut Self = self;
        let mut parent_bounding_box: BoundingBox = bounding_box;
        let mut quadrant: usize = parent_bounding_box.quadrant(x, y);

        while parent.children[quadrant].is_some() {
            parent.update_center_of_mass(x, y, mass);

            parent_bounding_box = parent_bounding_box.child(quadrant);
            parent = parent.children[quadrant].as_mut().unwrap();

            quadrant = parent_bounding_box.quadrant(x, y);
        }

        if parent.is_leaf() {
            let (parent_x, parent_y, parent_charge, parent_mass) =
                (parent.x, parent.y, parent.charge, parent.mass);

            // Edge case: if the parent is too close to the child, don't insert as child
            if (parent_x - x).abs() < EPSILON && (parent_y - y).abs() < EPSILON {
                return;
            }

            // Find the center of mass between the two
            parent.update_center_of_mass(x, y, mass);

            let (delta_x, delta_y, delta_charge, delta_mass) =
                (parent.x, parent.y, parent.charge, parent.mass);

            // Then split until the parent and child are in separate cells
            let mut parent_quadrant: usize = parent_bounding_box.quadrant(parent_x, parent_y);

            while quadrant == parent_quadrant {
                if parent_bounding_box.max_x == 0.0
                    || parent_bounding_box.max_y == 0.0
                    || parent_bounding_box.min_x == 0.0
                    || parent_bounding_box.min_y == 0.0
                {
                    break;
                }

                // Create the cell containing both
                parent.new_child(quadrant, delta_x, delta_y, delta_charge, delta_mass);
                parent = parent.children[quadrant].as_mut().unwrap();

                // Split the center and continue down
                parent_bounding_box = parent_bounding_box.child(quadrant);
                quadrant = parent_bounding_box.quadrant(x, y);
                parent_quadrant = parent_bounding_box.quadrant(parent_x, parent_y);
            }

            // Once the quadrants are different, insert the parent into its quadrant
            parent.new_child(
                parent_quadrant,
                parent_x,
                parent_y,
                parent_charge,
                parent_mass,
            );
        }

        parent.new_child(quadrant, x, y, charge, mass);
    }

    // Returns a bool indicating whether the node is a leaf.
    pub fn is_leaf(&self) -> bool {
        for child in &self.children {
            if child.is_some() {
                return false;
            }
        }

        true
    }

    pub fn new(
        position: &[Vector3D],
        charge: &[f32],
        mass: &[f32],
        bounding_box: BoundingBox,
    ) -> Self {
        let mut root: ParticleQuadtree = Self::empty();

        for i in 0..position.len() {
            root.insert(
                position[i].x,
                position[i].y,
                charge[i],
                mass[i],
                bounding_box,
            );
        }

        root
    }

    pub fn new_child(&mut self, quadrant: usize, x: f32, y: f32, charge: f32, mass: f32) {
        self.children[quadrant] = Some(Self {
            x,
            y,
            charge,
            mass,
            children: vec![None, None, None, None],
        })
    }

    pub fn update_center_of_mass(&mut self, x: f32, y: f32, mass: f32) {
        let total_mass: f32 = self.mass + mass;

        self.x = (self.mass * self.x + mass * x) / total_mass;
        self.y = (self.mass * self.y + mass * y) / total_mass;
        self.mass = total_mass;
    }
}

pub struct ParticleQuadtreeIterator<'a> {
    x: f32,
    y: f32,
    theta: f32,
    stack: Vec<(&'a ParticleQuadtree, BoundingBox)>,
}

impl<'a> ParticleQuadtreeIterator<'a> {
    pub fn new(
        x: f32,
        y: f32,
        theta: f32,
        tree: &'a ParticleQuadtree,
        bounding_box: BoundingBox,
    ) -> Self {
        Self {
            x,
            y,
            theta,
            stack: vec![(tree, bounding_box)],
        }
    }
}

impl<'a> Iterator for ParticleQuadtreeIterator<'a> {
    type Item = &'a ParticleQuadtree;

    /// Gets the next node that should count towards the force calculation for the current particle.
    ///
    /// Whether a node is or isn't sufficiently far away from a body,
    /// depends on the quotient s/d,
    /// where s is the width of the region represented by the internal node,
    /// and d is the distance between the body and the node's center of mass.
    /// The node is sufficiently far away when this ratio is smaller than a threshold value θ.
    /// The parameter θ determines the accuracy of the simulation;
    /// larger values of θ increase the speed of the simulation but decreases its accuracy.
    /// If θ = 0, no internal node is treated as a single body and the algorithm degenerates to a direct-sum algorithm.
    fn next(&mut self) -> Option<&'a ParticleQuadtree> {
        while !self.stack.is_empty() {
            let (node, bounding_box) = self.stack.pop()?;

            let d: f32 = l2(node.x, node.y, self.x, self.y);
            let s: f32 = bounding_box.width();

            if s / d < self.theta || node.is_leaf() {
                return Some(node);
            }

            // If not far enough away, add children to the stack.
            for (quadrant, child) in node.children.iter().enumerate() {
                match child {
                    Some(child) => {
                        self.stack.push((child, bounding_box.child(quadrant)));
                    }
                    None => {}
                }
            }
        }
        None
    }
}
