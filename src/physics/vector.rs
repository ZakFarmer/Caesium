//! Custom 3D vector struct.
use std::fmt::Debug;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

/// Vector type that supports linear combinations, cloning, and l2 norm.
pub trait Vector:
    Sized
    + Copy
    + Clone
    + PartialEq
    + Debug
    + Add<Output = Self>
    + AddAssign
    + Mul<f32, Output = Self>
    + MulAssign<f32>
    + Sub<Output = Self>
    + SubAssign
{
    fn zero() -> Self;
    fn from_xy(x: f32, y: f32) -> Self;
    fn to_xy(self) -> (f32, f32);
    fn l2_sqrd(self) -> f32;
    fn in_bounds(self, min: &Self, max: &Self) -> bool;
}

// Generic 2D vector
pub struct Vector2D {
    pub x: f32,
    pub y: f32,
}

/// Generic 3D vector.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Mul<f32> for Vector3D {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl MulAssign<f32> for Vector3D {
    fn mul_assign(&mut self, rhs: f32) {
        *self = Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        };
    }
}

impl Add for Vector3D {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vector3D {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        };
    }
}

impl Sub for Vector3D {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for Vector3D {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        };
    }
}

impl Vector for Vector3D {
    fn zero() -> Self {
        Self {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }

    fn from_xy(x: f32, y: f32) -> Self {
        Self { x, y, z: 0. }
    }

    fn to_xy(self) -> (f32, f32) {
        (self.x, self.y)
    }

    fn l2_sqrd(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn in_bounds(self, min: &Self, max: &Self) -> bool {
        self.x >= min.x
            && self.x <= max.x
            && self.y >= min.y
            && self.y <= max.y
            && self.z >= min.z
            && self.z <= max.z
    }
}
