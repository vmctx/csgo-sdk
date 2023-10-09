//! This re-exports either windows or linux depending on your OS.
//! # Example
//! ```
//! use sdk::utils::platform::{linux, windows, modules};
//! use sdk::utils::memory::get_module_handle;
//!
//! #[cfg(target_os = "linux")]
//! assert_eq!(modules::CLIENT, linux::CLIENT);
//! #[cfg(target_os = "windows")]
//! assert_eq!(modules::CLIENT, windows::CLIENT);
//! ```

#[doc(hidden)]
#[cfg(target_os = "linux")]
pub use linux as modules;
#[doc(hidden)]
#[cfg(target_os = "windows")]
pub use windows as modules;

pub mod windows {
    //! Module names for windows
    pub const CLIENT: *const i8 = cstr!("client.dll");
    pub const ENGINE: *const i8 = cstr!("engine.dll");
    pub const TIER0: *const i8 = cstr!("tier0.dll");
    pub const VGUI: *const i8 = cstr!("vguimatsurface.dll");
    pub const VGUI2: *const i8 = cstr!("vgui2.dll");
    pub const INPUT_SYSTEM: *const i8 = cstr!("inputsystem.dll");
    pub const VSTD_LIB: *const i8 = cstr!("vstdlib.dll");
    pub const MATERIAL_SYSTEM: *const i8 = cstr!("materialsystem.dll");
    pub const LOCALIZE: *const i8 = cstr!("localize.dll");
    pub const PHYSICS: *const i8 = cstr!("vphysics.dll");
}

pub mod linux {
    //! Module names for linux
    pub const CLIENT: *const i8 = cstr!("csgo/bin/linux64/client_client.so");
    pub const ENGINE: *const i8 = cstr!("engine_client.so");
    pub const TIER0: *const i8 = cstr!("libtier0_client.so");
    pub const VGUI: *const i8 = cstr!("vguimatsurface_client.so");
    pub const VGUI2: *const i8 = cstr!("vgui2_client.so");
    pub const INPUT_SYSTEM: *const i8 = cstr!("inputsystem_client.so");
    pub const VSTD_LIB: *const i8 = cstr!("libvstdlib_client.so");
    pub const MATERIAL_SYSTEM: *const i8 = cstr!("materialsystem_client.so");
    pub const LOCALIZE: *const i8 = cstr!("localize_client.so");
    pub const PHYSICS: *const i8 = cstr!("vphysics_client.so");
}
