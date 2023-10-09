use libc::c_char;

use crate::utils::math::vector::Vec3;
use crate::utils::memory;

#[repr(C)]
pub struct CInput {
    _pad0: [c_char; 0xAD],
    pub cam_in_third_person: bool,
    _pad1: [c_char; 0x2],
    pub cam_offset: Vec3,
    _pad2: [c_char; 0x44],
    cam_third_data: i32,
    cam_command: i32,
}

interface!(
    IInput,
    pub enable_third_person[35]() -> (),
    pub enable_first_person[36]() -> ()
);

impl IInput {
    pub fn is_third_person(&self) -> bool {
        if !self.base.is_null() {
            unsafe {
                memory::read::<bool>(self.base as usize + 0xAD);
            }
        }

        false
    }

    pub fn force_third_person(&self, state: bool) {
        if !self.base.is_null() {
            unsafe {
                core::ptr::write((self.base as usize + 0xAD) as *mut bool, state);
            }
        }
    }
}
