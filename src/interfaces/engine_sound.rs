use libc::c_void;

use crate::classes::utl_vector::CUtlVec;
use crate::utils::math::vector::Vec3;

#[repr(C)]
pub struct SoundInfoT {
    pub guid: i32,
    file_name: *const c_void,
    pub sound_source: i32,
    channel: i32,
    pub speaker_entity: i32,
    pub volume: f32,
    last_spatialized_volume: f32,
    pub radius: f32,
    pub pitch: i32,
    pub origin: *const Vec3,
    direction: *const Vec3,
    pub update_positions: bool,
    is_sentence: bool,
    dry_mix: bool,
    speaker: bool,
    special_dsp: bool,
    pub from_server: bool,
}

interface!(
    IEngineSound,
    pub get_active_sounds[19](sound_list: &mut CUtlVec<SoundInfoT>) -> ()
);
