/**
 * This bounding box implementation is heavily based on barnes-hut-rs by Katsutoshii
 * Source: https://github.com/Katsutoshii/barnes-hut-rs/blob/master/src/quadtree/bb.rs
 */

#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
    pub max_x: f32,
    pub min_x: f32,
    pub max_y: f32,
    pub min_y: f32,
}

impl BoundingBox {
    // Returns center X position of the bounding box
    pub fn center_x(&self) -> f32 {
        (self.min_x + self.max_x) / 2.0
    }

    // Returns center Y position of the bounding box
    pub fn center_y(&self) -> f32 {
        (self.max_y + self.min_y) / 2.0
    }

    /// Gets the subquadtrant of this bounding box.
    /// The quadtrant number must be between 0 and 3.
    /// The LSB represents left (0) or right (1) in the x direction.
    /// The MSB represents left (0) or right (1) in the y direction.
    pub fn child(&self, quadrant: usize) -> Self {
        match quadrant {
            0b00 => Self {
                min_x: self.min_x,
                max_x: self.center_x(),
                min_y: self.min_y,
                max_y: self.center_y(),
            },
            0b01 => Self {
                min_x: self.center_x(),
                max_x: self.max_x,
                min_y: self.min_y,
                max_y: self.center_y(),
            },
            0b10 => Self {
                min_x: self.min_x,
                max_x: self.center_x(),
                min_y: self.center_y(),
                max_y: self.max_y,
            },
            0b11 => Self {
                min_x: self.center_x(),
                max_x: self.max_x,
                min_y: self.center_y(),
                max_y: self.max_y,
            },
            _ => Self {
                min_x: self.min_x,
                max_x: self.max_x,
                min_y: self.min_y,
                max_y: self.max_y,
            },
        }
    }

    // Returns the quadrant that a point is in
    pub fn quadrant(&self, x: f32, y: f32) -> usize {
        let x_bit = (x >= self.center_x()) as usize;
        let y_bit = (y >= self.center_y()) as usize;

        x_bit + (y_bit << 1)
    }

    // Returns the width of the bounding box
    pub fn width(&self) -> f32 {
        self.max_x - self.min_x
    }
}
