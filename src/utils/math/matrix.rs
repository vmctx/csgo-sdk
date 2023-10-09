//! A collection of custom matrixes.

/// A 4x3 matrix.
/// This type is marked as `#[repr(C)]`.
#[repr(C)]
pub struct Matrix4x3T {
    pub body: [[f32; 4]; 3],
}

/// A 3x4 matrix.
/// This type is marked as `#[repr(C)]`.
#[repr(C)]
pub struct Matrix3x4T {
    pub body: [[f32; 3]; 4],
}
