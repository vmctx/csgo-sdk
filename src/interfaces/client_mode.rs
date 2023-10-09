use libc::{c_char, c_float, c_int};

use crate::utils::math;

#[repr(i32)]
pub enum EButtons {
    InAttack = 1 << 0,
    InJump = 1 << 1,
    InDuck = 1 << 2,
    InForward = 1 << 3,
    InBack = 1 << 4,
    InUse = 1 << 5,
    InMoveleft = 1 << 9,
    InMoveright = 1 << 10,
    InAttack2 = 1 << 11,
    InScore = 1 << 16,
    InBullrush = 1 << 22,
}

#[repr(C)]
pub struct CUserCMD {
    _pad: [c_char; 0x1],
    pub command_number: c_int,
    pub tick_count: c_int,
    pub view_angles: math::vector::Vec3,
    aim_angles: math::vector::Vec3,
    pub forward_move: c_float,
    pub side_move: c_float,
    pub up_move: c_float,
    pub i_buttons: c_int,
    _pad2: [c_char; 0xC],
    pub random_seed: c_int,
}

#[repr(C)]
pub struct ViewSetup {
    _pad0: [c_char; 0xB0],
    pub fov: f32,
    _pad1: [c_char; 0x20],
    pub far_z: f32,
}
