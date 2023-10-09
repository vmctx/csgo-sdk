//! Stores all CS:GO interfaces in one struct ([`Interfaces`])

use alloc::string::ToString;
use core::mem::MaybeUninit;

use cstr_core::CStr;
use fehler::throws;
use libc::{c_char, c_void};

use crate::interfaces::*;
use crate::utils::error::Error;
use crate::utils::error::Error::NotFound;
use crate::utils::memory::{get_module_handle, get_proc_address, pattern_scan};
use crate::utils::platform::modules;
use crate::utils::{memory, patterns};

const CLIENT: u32 = hash!("VClient018");
const ENTITY_LIST: u32 = hash!("VClientEntityList003");
const ENGINE: u32 = hash!("VEngineClient014");
const VGUI_PANEL: u32 = hash!("VGUI_Panel009");
const VGUI_SURFACE: u32 = hash!("VGUI_Surface031");
const INPUT_SYSTEM: u32 = hash!("InputSystemVersion001");
const RENDER_VIEW: u32 = hash!("VEngineRenderView014");
const CVAR: u32 = hash!("VEngineCvar007");
const ENGINE_TRACE: u32 = hash!("EngineTraceClient004");
const ENGINE_SOUND: u32 = hash!("IEngineSoundClient003");
const MAT_SYSTEM: u32 = hash!("VMaterialSystem080");
const MODEL_RENDER: u32 = hash!("VEngineModel016");
const MODEL_INFO: u32 = hash!("VModelInfoClient004");
const LOCALIZE: u32 = hash!("Localize_001");
const PHYS_SURFACE_PROPS: u32 = hash!("VPhysicsSurfaceProps001");
const PREDICTION: u32 = hash!("VClientPrediction001");
const GAME_EVENT_MGR: u32 = hash!("GAMEEVENTSMANAGER002");

/// Every interface implements this trait.
pub trait Interface {
    /// # Safety
    /// This does not perform a check for `null()`.
    unsafe fn from_raw_unchecked(ptr: *const usize) -> Self;
    /// Checks if `self.base`, that stores the games interface memory address, is `null()`.
    fn is_null(&self) -> bool;
    /// Returns `self.base`.
    fn as_ptr(&self) -> *const usize;
}

/// Struct that stores every interface.
pub struct Interfaces {
    pub client: client::IClient,
    pub client_mode: *mut usize,
    pub vgui_surface: surface::ISurface,
    pub vgui_panel: panel::IPanel,
    pub entity_list: entity_list::IEntityList,
    pub engine: engine::IEngine,
    pub glow_object_manager: &'static mut glow::IGlowObjectManager,
    pub input_system: input_system::IInputSystem,
    pub input: input::IInput,
    pub global_vars: &'static globals::IGlobalVars,
    pub render_view: render_view::IRenderView,
    pub cvar: convar::ICVar,
    pub engine_trace: engine_trace::IEngineTrace,
    pub engine_sound: engine_sound::IEngineSound,
    pub material_system: material_system::IMaterialSystem,
    pub model_render: model_render::IModelRender,
    pub model_info: model_info::IModelInfo,
    pub localize: localize::ILocalize,
    pub physics_surface_props: physics_surface_props::IPhysicsSurfaceProps,
    pub prediction: prediction::IPrediction,
    pub view_render_beams: view_render_beams::IViewRenderBeams,
    pub game_event: game_events::IGameEventManager,
    pub key_values_system: key_values_system::IKeyValuesSystem,
}

impl Default for Interfaces {
    #[allow(invalid_value)]
    fn default() -> Self {
        unsafe { MaybeUninit::uninit().assume_init() }
    }
}

unsafe impl Send for Interfaces {}

impl Interfaces {
    #[throws(Error)]
    pub(crate) fn load() -> Self {
        unsafe {
            let client_interface = get_interface::<client::IClient>(modules::CLIENT, CLIENT)?;

            Self {
                client_mode: **(((*((*(client_interface.as_ptr() as *mut *mut usize)).offset(10)))
                    + 5) as *mut *mut _),
                global_vars: &*(**(((*((*(client_interface.as_ptr() as *mut *mut usize))
                    .offset(11)))
                    + 10) as *mut *mut usize)
                    as *const globals::IGlobalVars),
                client: client_interface,
                engine: get_interface(modules::ENGINE, ENGINE)?,
                glow_object_manager: core::mem::transmute::<_, &mut glow::IGlowObjectManager>(
                    memory::read_mut::<usize>(
                        some_or_ret!(
                            pattern_scan(modules::CLIENT, patterns::GLOW_MANAGER),
                            Err(NotFound {
                                item: obfstr!("IGlowMgr").into()
                            })
                        ) as usize
                            + 0x3,
                    ),
                ),
                vgui_panel: get_interface(modules::VGUI2, VGUI_PANEL)?,
                entity_list: get_interface(modules::CLIENT, ENTITY_LIST)?,
                vgui_surface: get_interface(modules::VGUI, VGUI_SURFACE)?,
                input_system: get_interface(modules::INPUT_SYSTEM, INPUT_SYSTEM)?,
                input: input::IInput::from_raw_unchecked(memory::read::<*mut usize>(
                    some_or_ret!(
                        pattern_scan(modules::CLIENT, patterns::INPUT_INTERFACE),
                        Err(NotFound {
                            item: obfstr!("IInput").into()
                        })
                    ) as usize
                        + 0x1,
                )),
                render_view: get_interface(modules::ENGINE, RENDER_VIEW)?,
                cvar: get_interface(modules::VSTD_LIB, CVAR)?,
                engine_trace: get_interface(modules::ENGINE, ENGINE_TRACE)?,
                engine_sound: get_interface(modules::ENGINE, ENGINE_SOUND)?,
                material_system: get_interface(modules::MATERIAL_SYSTEM, MAT_SYSTEM)?,
                model_render: get_interface(modules::ENGINE, MODEL_RENDER)?,
                model_info: get_interface(modules::ENGINE, MODEL_INFO)?,
                localize: get_interface(modules::LOCALIZE, LOCALIZE)?,
                physics_surface_props: get_interface(modules::PHYSICS, PHYS_SURFACE_PROPS)?,
                prediction: get_interface(modules::CLIENT, PREDICTION)?,
                view_render_beams: view_render_beams::IViewRenderBeams::from_raw_unchecked(
                    *((some_or_ret!(
                        pattern_scan(modules::CLIENT, patterns::VIEW_RENDER_BEAMS),
                        Err(NotFound {
                            item: obfstr!("IViewRenderBeams").into()
                        })
                    ) as usize
                        + 0x1) as *mut usize) as *mut usize,
                ),
                game_event: get_interface(modules::ENGINE, GAME_EVENT_MGR)?,
                key_values_system: key_values_system::IKeyValuesSystem::from_raw_unchecked(
                    some_or_ret!(
                        get_proc_address(
                            get_module_handle(modules::VSTD_LIB),
                            cstr!("KeyValuesSystem")
                        ),
                        Err(NotFound {
                            item: obfstr!("KeyValuesSystem").into()
                        })
                    ) as *const usize,
                ),
            }
        }
    }
}

#[cfg(target_os = "windows")]
pub(crate) unsafe fn get_interface<T: Interface>(
    module: *const i8,
    interface: u32,
) -> Result<T, Error> {
    use core::mem::size_of;
    use memory::{get_module_handle, get_proc_address};

    let fn_addr = some_or_ret!(
        get_proc_address(get_module_handle(module), cstr!("CreateInterface")),
        Err(NotFound {
            item: obfstr!("CreateInterface address").into()
        })
    );

    let addr = fn_addr as usize + 5;
    let disp = memory::read::<usize>(addr);

    let mut current_interface = transmute!(
        **((addr + size_of::<u32>() + disp + 6) as *mut *mut usize),
        *mut InterfaceLinkedList
    );

    while !current_interface.is_null() {
        let interface_name = CStr::from_ptr((*current_interface).name)
            .to_str()
            .unwrap_or("?");

        if obfstr::hash(interface_name).eq(&interface) {
            let fn_addr = (*current_interface).func;
            return Ok(T::from_raw_unchecked(fn_addr() as _));
        }

        current_interface = (*current_interface).next;
    }

    Err(NotFound {
        item: interface.to_string(),
    })
}

#[cfg(target_os = "linux")]
pub(crate) unsafe fn get_interface<T: Interface>(
    module: *const i8,
    interface: u32,
) -> Result<T, Error> {
    use libc::{RTLD_LOCAL, RTLD_NOLOAD, RTLD_NOW};
    let module = libc::dlopen(module, RTLD_NOLOAD | RTLD_NOW | RTLD_LOCAL);

    let mut current_interface = transmute!(
        libc::dlsym(module, cstr!("s_pInterfaceRegs")),
        *mut InterfaceLinkedList
    );

    libc::dlclose(module);

    while !current_interface.is_null() {
        let interface_name = CStr::from_ptr((*current_interface).name)
            .to_str()
            .unwrap_or("?");

        if obfstr::hash(interface_name).eq(&interface) {
            let fn_addr = (*current_interface).func;
            return Ok(T::from_raw_unchecked(fn_addr() as _));
        }

        current_interface = (*current_interface).next;
    }

    Err(NotFound {
        item: interface.to_string(),
    })
}

#[repr(C)]
struct InterfaceLinkedList {
    func: fn() -> *const c_void,
    name: *const c_char,
    next: *mut InterfaceLinkedList,
}
