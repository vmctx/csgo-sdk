use crate::utils::memory::NotNull;
use libc::c_char;

interface!(
    IConVar,
    pub get_name[5]() -> *const c_char,
    pub get_float[12]() -> f32,
    pub get_int[13]() -> i32,
    pub set_value_char[14](value: *const c_char) -> (),
    pub set_value_float[15](value: f32) -> (),
    pub set_value_int[16](value: i32) -> ()
);

interface!(
    ICVar,
    pub get_cvar[15](name: *const c_char) -> NotNull<IConVar>
);
