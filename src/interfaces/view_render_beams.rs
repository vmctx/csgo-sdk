use core::ptr::null;

use libc::c_char;

use crate::utils::math::vector::Vec3;

pub const TE_BEAMPOINTS: i32 = 0;
pub const TE_SPRITE: i32 = 1;
pub const TE_BEAMDISK: i32 = 2;
pub const TE_BEAMCYLINDER: i32 = 3;
pub const TE_BEAMFOLLOW: i32 = 4;
pub const TE_BEAMRING: i32 = 5;
pub const TE_BEAMSPLINE: i32 = 6;
pub const TE_BEAMRINGPOINT: i32 = 7;
pub const TE_BEAMLASER: i32 = 8;
pub const TE_BEAMTESLA: i32 = 9;
pub const MAX_BEAM_ENTS: i32 = 10;
pub const NOISE_DIVISIONS: i32 = 128;

#[repr(C)]
pub enum EBeamType {
    FbeamStartEntity = 0x00000001,
    FbeamEndEntity = 0x00000002,
    FbeamFadeIn = 0x00000004,
    FbeamFadeOut = 0x00000008,
    FbeamSineNoise = 0x00000010,
    FbeamSolid = 0x00000020,
    FbeamShadein = 0x00000040,
    FbeamShadeout = 0x00000080,
    FbeamOnlyNoiseOnce = 0x00000100,
    FbeamNotile = 0x00000200,
    FbeamUseHitboxes = 0x00000400,
    FbeamStartVisible = 0x00000800,
    FbeamEndVisible = 0x00001000,
    FbeamIsActive = 0x00002000,
    FbeamForever = 0x00004000,
    FbeamHaloBeam = 0x00008000,
    FbeamReversed = 0x00010000,
}

#[repr(C)]
pub struct BeamT;

#[repr(C)]
pub struct BeamInfoT {
    pub beam_type: i32,

    // Entities
    start_entity: *const usize,
    start_attachment: i32,
    end_entity: *const usize,
    end_attachment: i32,

    // Points
    pub vec_start: Vec3,
    pub vec_end: Vec3,

    pub model_index: i32,
    pub model_name: *const c_char,
    pub halo_index: i32,
    pub halo_name: *const c_char,
    pub halo_scale: f32,
    pub life: f32,
    pub width: f32,
    pub end_width: f32,
    pub fade_length: f32,
    pub amplitude: f32,
    pub brightness: f32,
    pub speed: f32,
    pub start_frame: i32,
    pub frame_rate: f32,
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub render_able: bool,
    pub segments: i32,
    pub flags: i32,

    // Rings
    pub center: Vec3,
    pub start_radius: f32,
    pub end_radius: f32,
}

impl Default for BeamInfoT {
    fn default() -> Self {
        Self {
            beam_type: TE_BEAMPOINTS,
            start_entity: null(),
            start_attachment: 0,
            end_entity: null(),
            end_attachment: 0,
            vec_start: Vec3::default(),
            segments: -1,
            model_name: null(),
            halo_name: null(),
            halo_scale: 0.0,
            life: 0.0,
            width: 0.0,
            end_width: 0.0,
            fade_length: 0.0,
            amplitude: 0.0,
            brightness: 0.0,
            speed: 0.0,
            start_frame: 0,
            frame_rate: 0.0,
            red: 0.0,
            green: 0.0,
            model_index: -1,
            halo_index: -1,
            render_able: true,
            flags: 0,
            center: Vec3::default(),
            start_radius: 0.0,
            vec_end: Vec3::default(),
            blue: 0.0,
            end_radius: 0.0,
        }
    }
}

interface!(
    IViewRenderBeams,
    pub draw_beam[4](beam: *const BeamT) -> (),
    pub create_ring_beam_point[16](beam_info: &mut BeamInfoT) -> *const BeamT
);
