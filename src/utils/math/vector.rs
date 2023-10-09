//! A collection of custom vector types.

use core::ops;
use libc::c_float;

/// A 3-dimensional vector.
/// This type is marked as `#[repr(C)]`.
#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: c_float,
    pub y: c_float,
    pub z: c_float,
}

/// A 2-dimensional vector.
/// This type is marked as `#[repr(C)]`.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vec2 {
    pub x: c_float,
    pub y: c_float,
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
        self
    }
}

impl ops::Div for Vec3 {
    type Output = Self;

    fn div(mut self, rhs: Self) -> Self {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
        self
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(mut self, rhs: f32) -> Self {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(mut self, rhs: f32) -> Self {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
        self
    }
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn empty() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn default() -> Self {
        Self::empty()
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn len_sqr(&self) -> f32 {
        f32::powi(self.x, 2) + f32::powi(self.y, 2) + f32::powi(self.z, 2)
    }

    pub fn dot(&self, other: Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn empty() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn default() -> Self {
        Self::empty()
    }
}

#[repr(C)]
pub struct VertexT {
    pub m_position: Vec2,
    m_tex_coord: Vec2,
}

impl VertexT {
    pub fn new(pos: Vec2) -> Self {
        Self {
            m_position: pos,
            m_tex_coord: Vec2::empty(),
        }
    }
}
