use core::ptr::null;

use libc::{c_char, c_void};

use crate::interfaces::material::IMaterial;
use crate::interfaces::material::ITexture;

type MaterialHandleT = u16;

interface!(
    IMaterialSystem,
    pub create_material[83](name: *const c_char) -> IMaterial,
    find_material_virtual[84](name: *const c_char, texture_group_name: *const c_char, complain: bool, complain_prefix: *const c_char) -> IMaterial,
    pub first_material[86]() -> MaterialHandleT,
    pub next_material[87](material: MaterialHandleT) -> MaterialHandleT,
    pub invalid_material[88]() -> MaterialHandleT,
    pub get_material[89](material: MaterialHandleT) -> IMaterial,
    pub get_num_materials[90]() -> i32,
    pub find_texture[91](name: *const c_char, texture_group_name: *const c_char, complain: bool, additional_creation_flags: i32) -> ITexture,
    pub begin_render_target_alloc[94]() -> (),
    pub end_render_target_alloc[95]() -> (),
    pub get_render_context_virtual[115]() -> IMatRenderContext
);

impl IMaterialSystem {
    pub fn find_material(&self, name: *const c_char, group: *const c_char) -> IMaterial {
        self.find_material_virtual(name, group, true, null())
    }
}

interface!(
    IMatRenderContext,
    pub draw_screen_space_rect[114](material: IMaterial, dest_x: i32, dest_y: i32, width: i32, height: i32, src_texture_x0: f32, src_texture_y0: f32, src_texture_x1: f32, src_texture_y1: f32, src_texture_width: i32, src_texture_height: i32, client_renderable: *const c_void, x_dice: i32, y_dice: i32) -> ()
);
