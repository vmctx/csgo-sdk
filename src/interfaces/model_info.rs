use libc::c_char;

use crate::classes::entity::Model;
use crate::utils::math::vector::Vec3;

#[repr(C)]
pub struct StudioBone {
    name_index: i32,
    pub parent: i32,
    pad0: [c_char; 0x98],
    pub flags: i32,
    pad: [c_char; 0x34],
}

#[repr(C)]
pub struct StudioHdr {
    pub id: i32,
    version: i32,
    checksum: i32,
    name: [c_char; 0x40],
    length: i32,
    eye_position: Vec3,
    illum_position: Vec3,
    hull_min: Vec3,
    hull_max: Vec3,
    bb_min: Vec3,
    bb_max: Vec3,
    flags: i32,
    pub num_bones: i32,
    bone_index: i32,
    num_bone_controllers: i32,
    bone_controller_index: i32,
    num_hitbox_sets: i32,
    hitbox_set_index: i32,
}

interface!(
    IModelInfo,
    pub get_model_index[2](model_name: *const c_char) -> i32,
    pub get_studio_model[32](model: *const Model) -> StudioHdr
);
