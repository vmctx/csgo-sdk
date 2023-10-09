use libc::{c_char, wchar_t};

interface!(
    ILocalize,
    pub find[12](token_name: *const c_char) -> *const wchar_t
);
