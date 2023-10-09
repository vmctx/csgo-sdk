use libc::{c_char, c_void};

use crate::interfaces::material::IMaterial;
use crate::utils::math::matrix::Matrix3x4T;
use crate::utils::math::vector::Vec3;

#[repr(C)]
pub enum OverrideType {
    Normal = 0,
    BuildShadows,
    DepthWrite,
    CustomMaterial,
    // weapon skins
    SsaoDepthWrite,
}

#[repr(C)]
pub struct Model {
    handle: *const c_void,
    pub name: [c_char; 0x104],
    load_flags: i32,
    server_count: i32,
    typ: i32,
    flags: i32,
    mins: Vec3,
    maxs: Vec3,
}

#[repr(C)]
pub struct ModelRenderInfo {
    origin: Vec3,
    angles: Vec3,
    pad: [c_char; 0x4],
    renderable: *mut c_void,
    pub model: *const Model,
    model_to_world: *const Matrix3x4T,
    lighting_offset: *const Matrix3x4T,
    lighting_origin: *const Vec3,
    flags: i32,
    pub entity_index: i32,
}

interface!(
    IModelRender,
    pub set_forced_material_override[1](material: IMaterial, override_type: OverrideType, overrides: i32) -> (),
    pub is_forced_material_override[2]() -> bool
);
