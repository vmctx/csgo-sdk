use core::ptr::null;

use libc::c_char;

use crate::classes::entity::CEntity;
use crate::classes::utl_vector::CUtlVec;

#[repr(C)]
pub struct Entity {
    pub vtable: usize,
}

#[repr(C)]
pub struct GlowObjectDef {
    next_free_slot: i32,
    pub entity: *const Entity,
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
    fl_unk: bool,
    pad: [c_char; 0x4],
    pub glow_alpha_max: f32,
    pad1: [c_char; 0x4],
    pub render_when_occluded: bool,
    pub render_when_unoccluded: bool,
    full_bloom_render: bool,
    full_bloom_stencil_test: i32,
    pub glow_style: i32,
    split_screen_slot: i32,
}

impl GlowObjectDef {
    pub fn is_unused(&self) -> bool {
        self.next_free_slot == -2
    }
}

#[repr(C)]
pub struct IGlowObjectManager {
    pub glow_objects: &'static mut CUtlVec<GlowObjectDef>,
    first_free_slot: i32,
}

impl IGlowObjectManager {
    pub fn has_glow_effect(&mut self, entity: &CEntity) -> bool {
        let entity = entity.as_ptr() as _;

        for i in 0..self.glow_objects.size {
            if let Some(glow_object) = self.glow_objects.get_mut(i) {
                if !glow_object.is_unused() && core::ptr::eq(entity, glow_object.entity) {
                    return true;
                }
            }
        }

        false
    }

    pub fn register_glow_effect(&mut self, entity: &CEntity) -> i32 {
        let entity = entity.as_ptr() as _;

        let index = self.first_free_slot;
        if let Some(glow_object) = self.glow_objects.get_mut(index) {
            if index != -1 {
                self.first_free_slot = glow_object.next_free_slot;
                glow_object.entity = entity;
                glow_object.full_bloom_render = false;
                glow_object.full_bloom_stencil_test = 0;
                glow_object.split_screen_slot = -1;
                glow_object.next_free_slot = -2;
            }
        }
        index
    }

    pub fn unregister_glow_effect(&mut self, index: i32) {
        if let Some(glow_object) = self.glow_objects.get_mut(index) {
            glow_object.next_free_slot = self.first_free_slot;
            glow_object.entity = null();
            glow_object.render_when_occluded = false;
            glow_object.render_when_unoccluded = false;
            self.first_free_slot = index;
        }
    }
}
