use libc::c_char;

interface!(
    IPanel,
    pub set_keyboard_input_enabled[31](panel_id: u32, state: bool) -> (),
    pub set_mouse_input_enabled[32](panel_id: u32, state: bool) -> (),
    pub get_panel_name[36](panel_id: u32) -> *const c_char
);
