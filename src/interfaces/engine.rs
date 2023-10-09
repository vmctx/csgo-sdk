use libc::{c_char, c_float};

use crate::utils::math;
use crate::utils::math::vector::Vec3;

#[repr(C)]
pub struct D3DMATRIX {
    m: [[c_float; 4]; 4],
}

#[repr(C)]
pub struct PlayerInfo {
    version: u32,
    xuid: u32,
    pub xuid_low: u32,
    xuid_high: u32,
    pub name: [c_char; 0x80],
    pub user_id: i32,
    _guid: [c_char; 0x21],
    _friends_id: u32,
    _friends_name: [i32; 0x80],
    pub fake_player: bool,
    hltv: bool,
    _customfiles: [i32; 0x4],
    _files_downloaded: u8,
}

interface!(
    IEngine,
    pub get_screen_size[5](width: &mut i32, height: &mut i32) -> (),
    get_player_info_virtual[8](entity_id: i32, player_info: &mut PlayerInfo) -> (),
    pub get_player_for_user_id[9](user_id: i32) -> i32,
    pub get_local_player[12]() -> i32,
    set_view_angles_virtual[19](view_angles: &Vec3) -> (),
    pub is_ingame[26]() -> bool,
    pub is_connected[27]() -> bool,
    pub world_to_screen_matrix[37]() -> *const D3DMATRIX,
    pub is_taking_screenshot[92]() -> bool,
    pub execute_client_cmd[108](cmd: *const c_char) -> (),
    pub execute_client_cmd_unrestricted[114](cmd: *const c_char, from_console_or_keybind: bool) -> ()
);

impl IEngine {
    pub fn get_player_info(&self, entity_id: i32) -> PlayerInfo {
        let mut player_info: PlayerInfo = unsafe { core::mem::zeroed() };

        self.get_player_info_virtual(entity_id, &mut player_info);

        player_info
    }

    pub fn set_view_angles(&self, mut view_angles: Vec3) {
        view_angles = math::normalize(view_angles);
        view_angles.x = view_angles.x.clamp(-89.0, 89.0);
        view_angles.y = view_angles.y.clamp(-180.0, 180.0);
        view_angles.z = 0.0;

        self.set_view_angles_virtual(&view_angles);
    }
}
