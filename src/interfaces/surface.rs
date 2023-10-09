use libc::{c_char, c_void, wchar_t};

use crate::utils;
use crate::utils::math::vector::{Vec2, VertexT};
use crate::utils::string::StrExt;

pub type HFONT = *mut c_void;

pub enum EFontFlags {
    FontflagNone,
    FontflagItalic = 0x001,
    FontflagUnderline = 0x002,
    FontflagStrikeout = 0x004,
    FontflagSymbol = 0x008,
    FontflagAntialias = 0x010,
    FontflagGaussianblur = 0x020,
    FontflagRotary = 0x040,
    FontflagDropshadow = 0x080,
    FontflagAdditive = 0x100,
    FontflagOutline = 0x200,
    FontflagCustom = 0x400,
    FontflagBitmap = 0x800,
}

#[derive(Clone, Copy)]
pub enum GradientType {
    GradientHorizontal = 0,
    GradientVertical,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Color {
    pub r: i32,
    pub g: i32,
    pub b: i32,
    pub a: i32,
}

impl Color {
    pub fn new_rgb(r: i32, g: i32, b: i32) -> Self {
        Self::new_rgba(r, g, b, 255)
    }

    pub fn new_rgba(r: i32, g: i32, b: i32, a: i32) -> Self {
        Self { r, g, b, a }
    }

    pub fn blend(&self, other: Color, t: f32) -> Self {
        Self {
            r: (self.r as f32 + t * (other.r - self.r) as f32) as i32,
            g: (self.g as f32 + t * (other.g - self.g) as f32) as i32,
            b: (self.b as f32 + t * (other.b - self.b) as f32) as i32,
            a: (self.a as f32 + t * (other.a - self.a) as f32) as i32,
        }
    }
}

interface!(
    ISurface,
    pub set_draw_color[15](color: Color) -> (),
    pub draw_filled_rect[16](x: i32, y: i32, width: i32, height: i32) -> (),
    pub draw_outlined_rect[18](x: i32, y: i32, width: i32, height: i32) -> (),
    pub draw_line[19](x: i32, y: i32, width: i32, height: i32) -> (),
    set_text_font[23](font: HFONT) -> (),
    set_text_color[25](color: Color) -> (),
    set_text_pos[26](x: i32, y: i32) -> (),
    draw_print_text[28](text: *const u16, len: i32, font_draw_type: i32) -> (),
    pub set_draw_texture_rgba[37](id: i32, rgba: *const u8, width: i32, height: i32) -> (),
    pub set_draw_texture[38](id: i32) -> (),
    pub create_new_texture_id[43](procedural: bool) -> i32,
    pub unlock_cursor[66]() -> bool,
    pub create_font[71]() -> HFONT,
    set_font_glyph_set[72](font: HFONT, font_name: *const c_char, tall: i32, weight: i32, blur: i32, scan_lines: i32, flags: i32, range_min: i32, range_max: i32) -> bool,
    get_text_size_virtual[79](font: HFONT, text: *const wchar_t, wide: &mut i32, tall: &mut i32) -> (),
    pub draw_outlined_circle[103](x: i32, y: i32, radius: i32, segments: i32) -> (),
    pub draw_textured_polygon[106](count: i32, vertex: *const VertexT, unk: bool) -> (),
    pub draw_rect_fade[123](x: i32, y: i32, width: i32, height: i32, color1: u32, color2: u32, gradient_type: bool) -> bool,
    pub reset_font_cache[141]() -> (),
    pub draw_colored_circle[162](x: i32, y: i32, radius: f32, color: Color) -> ()
);

impl ISurface {
    pub fn custom_font(&self, name: *const i8, size: i32, weight: i32, fontflags: i32) -> HFONT {
        let font = self.create_font();
        self.set_font_glyph_set(font, name, size, weight, 0, 0, fontflags, 0, 0);
        font
    }

    pub fn get_text_size(&self, font: HFONT, text: *const wchar_t) -> Vec2 {
        let mut width = 0;
        let mut height = 0;
        self.get_text_size_virtual(font, text, &mut width, &mut height);

        Vec2::new(width as f32, height as f32)
    }

    pub fn render_text_centered(&self, font: HFONT, text: &str, x: i32, y: i32, color: Color) {
        let text = text.to_lpcwstr();
        let width = self.get_text_size(font, text as _).x;
        self.set_text_font(font);
        self.set_text_pos(x - (width as i32) / 2, y);
        self.set_text_color(color);
        unsafe {
            self.draw_print_text(text, libc::wcslen(text as _) as i32, 0);
        }
    }

    pub fn render_text(&self, font: HFONT, text: &str, x: i32, y: i32, color: Color) {
        let text = text.to_lpcwstr();
        self.set_text_font(font);
        self.set_text_pos(x, y);
        self.set_text_color(color);
        unsafe {
            self.draw_print_text(text, libc::wcslen(text as _) as i32, 0);
        }
    }

    pub fn draw_rectangle_fade(
        &self,
        start: [i32; 2],
        end: [i32; 2],
        first: Color,
        second: Color,
        g_type: GradientType,
    ) {
        self.set_draw_color(first.blend(second, 0.5));
        self.draw_filled_rect(start[0], start[1], start[0] + end[1], start[1] + end[1]);

        self.set_draw_color(first);
        self.draw_rect_fade(
            start[0],
            start[1],
            start[0] + end[0],
            start[1] + end[1],
            255,
            0,
            utils::variant_eq(g_type, GradientType::GradientHorizontal),
        );

        self.set_draw_color(second);
        self.draw_rect_fade(
            start[0],
            start[1],
            start[0] + end[0],
            start[1] + end[1],
            0,
            255,
            utils::variant_eq(g_type, GradientType::GradientHorizontal),
        );
    }
}
