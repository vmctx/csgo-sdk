use libc::{c_char, c_short};

#[repr(C)]
pub struct CSurfaceData {
    _pad: [c_char; 0x50],
    pub max_speed_factor: f32,
    pub jump_factor: f32,
    pub penetration_modifier: f32,
    pub damage_modifier: f32,
    pub material: c_short,
    climbable: bool,
}

interface!(
    IPhysicsSurfaceProps,
    pub get_surface_data[5](index: i32) -> &'static CSurfaceData
);
