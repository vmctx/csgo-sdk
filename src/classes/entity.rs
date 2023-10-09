use core::mem::MaybeUninit;
use core::ptr::null_mut;

use libc::{c_char, c_void};
use static_init::dynamic;

use crate::classes::utl_vector::CUtlVec;
use crate::classes::weapon::CWeapon;
use crate::classes::Entity;
use crate::definitions::bones;
use crate::interfaces::client::ClientClass;
use crate::interfaces::engine_trace::{Ray, Trace, TraceFilterGeneric, TraceFilterTrait};
use crate::utils::math::matrix::Matrix3x4T;
use crate::utils::math::vector::{Vec2, Vec3};
use crate::utils::memory::{pattern_scan, read};
use crate::utils::platform::modules;
use crate::utils::string::StringExt;
use crate::utils::{memory, patterns};
use crate::{get_entity_by_id, get_interfaces, get_local_player, netvar};
use alloc::string::String;
use num_traits::One;

type CBaseHandle = usize;

create_interface!(IClientRenderable);
create_interface!(IClientEntity);

interface!(
    IHandleEntity,
    pub set_ref_ehandle[1](ref_handle: &CBaseHandle) -> (),
    pub get_ref_ehandle[2]() -> &'static CBaseHandle
);

#[repr(i32)]
pub enum ESolidType {
    SolidNone = 0,
    SolidBsp,
    SolidBbox,
    SolidObb,
    SolidObbYaw,
    SolidCustom,
    SolidVphysics,
    SolidLast,
}

interface_trait!(
    ICollideable, 0x320,
    get_entity_handle[0]() -> *const IHandleEntity,
    obb_mins[1]() -> &'static Vec3,
    obb_maxs[2]() -> &'static Vec3,
    world_space_trigger_bounds[3](world_mins: *mut Vec3, world_maxs: *mut Vec3) -> bool,
    test_collision[4](ray: &Ray, contents_mask: u32, tr: &mut Trace) -> bool,
    get_collision_model_index[5]() -> i32,
    get_collision_model[6]() -> *const Model,
    get_collision_origin[7]() -> &'static Vec3,
    get_collision_angles[8]() -> &'static Vec2,
    collision_to_world_transform[9]() -> &'static Matrix3x4T,
    get_solid[10]() -> ESolidType,
    get_solid_flags[11]() -> i32,
    get_client_unknown[12]() -> *const IClientUnknown,
    get_collision_group[13]() -> i32,
    world_space_surrounding_bounds[14](mins: *mut Vec3, maxs: *mut Vec3) -> (),
    get_required_trigger_flags[15]() -> u32,
    get_root_parent_world_transform[16]() -> *const Matrix3x4T,
    get_physics_object[17]() -> *mut c_void
);

interface!(
    IClientAlphaProperty,
    pub get_client_unknown[0]() -> *const IClientUnknown,
    pub set_alpha_modulation[1](alpha: u8) -> (),
    pub set_render_fix[2](render_fix: i32, render_mode: i32, start_time: f32, duration: f32) -> (),
    pub set_fade[3](global_fade_scale: f32, dist_fade_start: f32, dist_fade_end: f32) -> (),
    pub set_desync_offset[4](offset: i32) -> (),
    pub enable_alpha_modulation_override[5](enable: bool) -> (),
    pub enable_shadow_alpha_modulation_override[6](enable: bool) -> (),
    pub set_distance_fade_mode[7](fade_mode: i32) -> ()
);

#[repr(C)]
pub struct CClientThinkHandle;

pub type ClientThinkHandleT = *const CClientThinkHandle;

interface!(
    IClientThinkable,
    pub get_client_unknown[0]() -> *const IClientUnknown,
    pub client_think[1]() -> (),
    pub get_think_handle[2]() -> ClientThinkHandleT,
    pub set_think_handle[3](think_handle: ClientThinkHandleT) -> (),
    pub release[4]() -> ()
);

interface!(
    IClientUnknown,
    pub get_client_renderable[2]() -> *const IClientRenderable,
    pub get_client_entity[3]() -> *const IClientEntity,
    pub get_base_entity[4]() -> *const CEntity,
    pub get_client_thinkable[5]() -> *const IClientThinkable,
    pub get_client_alpha_property[6]() -> *const IClientAlphaProperty
);

interface_trait!(
    IClientNetworkable, 0x8,
    get_client_unknown[0]() -> *const IClientUnknown,
    release[1]() -> (),
    get_client_class[2]() -> *const ClientClass,
    pre_data_update[6](update_type: i32) -> (),
    post_data_update[7](update_type: i32) -> (),
    is_dormant[9]() -> bool,
    get_index[10]() -> i32
);

interface!(
    CEntity,
    setup_bone_matrix[0](out: &mut Matrix3x4T) -> bool,
    pub get_abs_origin[10]() -> &'static Vec3,
    pub get_team[88]() -> i32,
    pub is_alive[156]() -> bool,
    pub is_player[158]() -> bool,
    pub is_weapon[166]() -> bool,
    get_eye_pos_virtual[285](pos: &mut Vec3) -> ()
);

impl ICollideable<CEntity> for CEntity {}

impl IClientNetworkable<CEntity> for CEntity {}

impl Entity for CEntity {}

impl CEntity {
    netvar!("DT_BasePlayer", "m_iHealth", get_health, i32);
    netvar!("DT_CSPlayer", "m_ArmorValue", get_armor, i32);
    netvar!("DT_CSPlayer", "m_bIsScoped", is_scoped, bool);
    netvar!("DT_CSPlayer", "m_bIsDefusing", is_defusing, bool);
    netvar!("DT_CSPlayer", "m_bHasDefuser", 0x5c, get_crosshair_id, i32);
    netvar!("DT_BasePlayer", "m_fFlags", get_flags, i32);
    netvar!(
        "DT_CSPlayer",
        "m_flFlashDuration",
        get_flash_duration,
        set_flash_duration,
        i32
    );
    netvar!("DT_BaseEntity", "m_bSpotted", is_spotted, set_spotted, bool);
    netvar!("DT_BaseEntity", "m_vecOrigin", get_origin, Vec3);
    netvar!("DT_BasePlayer", "m_vecViewOffset", get_view_offset, Vec3);
    netvar!("DT_BasePlayer", "m_vecVelocity", get_velocity, Vec3);
    netvar!(
        "DT_BasePlayer",
        "m_hViewModel[0]",
        get_view_model,
        set_view_model,
        i32
    );
    netvar!(
        "DT_BaseEntity",
        "m_Collision",
        0x14,
        get_collideable_max,
        Vec3
    );
    netvar!(
        "DT_CSPlayer",
        "m_flFlashDuration",
        0x18,
        get_glow_index,
        usize
    );
    netvar!("DT_BasePlayer", "m_iObserverMode", get_observer_mode, i32);
    netvar!(
        "DT_BaseCombatWeapon",
        "m_flNextPrimaryAttack",
        get_weapon_cooldown,
        i32
    );
    netvar!(
        "DT_BaseCombatCharacter",
        "m_hActiveWeapon",
        get_active_weapon,
        i32
    );
    netvar!("DT_BasePlayer", "m_nTickBase", get_tickbase, i32);
    netvar!("DT_BaseCombatCharacter", "m_flNextAttack", next_attack, f32);
    netvar!("DT_CSPlayer", "m_iShotsFired", shots_fired, i32);
    netvar!("DT_BasePlayer", "m_aimPunchAngle", get_aim_punch, Vec2);
    netvar!("DT_BasePlayer", "m_lifeState", get_life_state, i32);
    netvar!("DT_CSPlayer", "m_bGunGameImmunity", is_immune, bool);
    netvar!("DT_CSPlayer", "m_bHasHelmet", has_helmet, bool);
    netvar!("DT_CSPlayer", "m_iAccount", get_money, i32);

    pub fn is_valid_player(&self) -> bool {
        self.is_player()
            && self.is_alive()
            && !self.is_dormant()
            && !get_local_player().same_as(self)
    }

    pub fn get_name(&self) -> String {
        String::from_char_slice(
            &get_interfaces()
                .engine
                .get_player_info(self.get_index())
                .name,
        )
    }

    pub fn is_other_enemy(&self, other: &CEntity) -> bool {
        #[dynamic]
        static FN_PTR: Option<extern "thiscall" fn(*const usize, *const usize) -> bool> = {
            let address = pattern_scan(modules::CLIENT, patterns::IS_OTHER_ENEMY)? as usize + 0x3;

            if (address as *const usize).is_null() {
                return None;
            }

            unsafe {
                Some(transmute!(
                    address + 0x4 + read::<usize>(address),
                    extern "thiscall" fn(*const usize, *const usize) -> bool
                ))
            }
        };

        if let Some(function) = *FN_PTR {
            return function(self.base as _, other.base as _);
        }

        false
    }

    pub fn is_audible(&self) -> bool {
        let mut sounds = CUtlVec {
            memory: null_mut(),
            allocation_count: 0,
            grow_size: 0,
            size: 0,
            elements: null_mut(),
        };

        get_interfaces().engine_sound.get_active_sounds(&mut sounds);

        for sound in 0..sounds.size {
            if let Some(sound) = sounds.get_mut(sound) {
                if let Some(sound_entity) = get_entity_by_id(sound.sound_source) {
                    if sound_entity.same_as(self) {
                        return true;
                    }
                }
            }
        }

        false
    }

    pub fn can_fire(&self) -> bool {
        some_or_ret!(self.get_weapon(), false).can_fire()
    }

    pub fn get_weapons(&self) -> [u32; 48] {
        self.get_value::<[u32; 48]>(netvar::get_offset(
            obfstr!("DT_BaseCombatCharacter"),
            obfstr!("m_hMyWeapons"),
        ))
    }

    pub fn get_weapon(&self) -> Option<CWeapon> {
        get_interfaces()
            .entity_list
            .get_entity_from_handle::<CWeapon>(self.get_active_weapon())
    }

    pub fn can_see(&self, other: &CEntity) -> bool {
        bones::get_bone_list()
            .iter()
            .any(|bone| self.is_visible(other, *bone))
    }

    pub fn get_best_bone(&self, target: &CEntity) -> Option<&'static i32> {
        bones::get_bone_list()
            .iter()
            .find(|bone| self.is_visible(target, **bone))
    }

    #[allow(invalid_value)]
    pub fn is_visible(&self, other: &CEntity, bone: i32) -> bool {
        let mut trace: Trace = unsafe { MaybeUninit::uninit().assume_init() };
        let ray = Ray::new(self.get_eye_pos(), other.get_bone_pos(bone));
        let mut filter = TraceFilterGeneric::new(self.base);

        get_interfaces()
            .engine_trace
            .trace_ray(&ray, 0x4600400B, &mut filter, &mut trace);

        trace.ptr_entity as *const _ as usize == other.base as usize || trace.fraction.is_one()
    }

    pub fn get_bone_pos(&self, bone: i32) -> Vec3 {
        let ptr: usize =
            self.get_value(netvar::get_offset("DT_BaseAnimating", "m_nForceBone") + 0x1C);

        if (ptr as *const usize).is_null() {
            return Vec3::empty();
        }

        let pos = unsafe { memory::read::<Matrix3x4T>(ptr + 0x30 * bone as usize) };

        Vec3::new(pos.body[0][2], pos.body[1][2], pos.body[2][2])
    }

    pub fn get_eye_pos(&self) -> Vec3 {
        let mut vec = Vec3::empty();
        self.get_eye_pos_virtual(&mut vec);
        vec
    }

    pub fn same_as(&self, other: &CEntity) -> bool {
        self.base == other.base
    }
}

#[repr(C)]
pub struct Model {
    handle: *const c_void,
    name: [c_char; 0x104],
    load_flags: i32,
    server_count: i32,
    typ: i32,
    flags: i32,
    mins: Vec3,
    maxs: Vec3,
}
