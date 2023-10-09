use num_traits::FromPrimitive;

use crate::classes::entity::IClientNetworkable;
use crate::classes::Entity;
use crate::definitions::weapon_data;
use crate::definitions::weapon_data::CWeaponInfo;
use crate::definitions::weapon_data::{WeaponId, WeaponType};
use crate::utils::math::vector::Vec3;
use crate::{get_interfaces, get_local_player, utils};

interface!(
    CWeapon,
    pub get_abs_origin[10]() -> &'static Vec3,
    pub get_weapon_data[461]() -> &'static CWeaponInfo,
    pub get_inaccuracy[485]() -> f32,
    pub get_spread[553]() -> f32,
    pub update_accuracy_penalty[554]() -> ()
);

impl Entity for CWeapon {}

impl IClientNetworkable<CWeapon> for CWeapon {}

impl CWeapon {
    netvar!(
        "DT_BaseCombatWeapon",
        "m_flNextPrimaryAttack",
        next_attack,
        f32
    );
    netvar!(
        "DT_BaseCombatWeapon",
        "m_flNextPrimaryAttack",
        0x6D,
        is_reloading,
        bool
    );
    netvar!(
        "DT_BaseAttributableItem",
        "m_nFallbackPaintKit",
        get_fallback_paint_kit,
        set_fallback_paint_kit,
        u32
    );
    netvar!(
        "DT_BaseAttributableItem",
        "m_iEntityQuality",
        get_entity_quality,
        set_entity_quality,
        i32
    );
    netvar!(
        "DT_BaseAttributableItem",
        "m_nFallbackSeed",
        get_fallback_seed,
        set_fallback_seed,
        u32
    );
    netvar!(
        "DT_BaseAttributableItem",
        "m_nFallbackStatTrak",
        get_fallback_stat_track,
        set_fallback_stat_track,
        i32
    );
    netvar!(
        "DT_BaseAttributableItem",
        "m_flFallbackWear",
        get_fallback_wear,
        set_fallback_wear,
        f32
    );
    netvar!(
        "DT_BaseCombatWeapon",
        "m_hWeaponWorldModel",
        get_weapon_world_model,
        set_weapon_world_model,
        i32
    );
    netvar!(
        "DT_BaseAttributableItem",
        "m_iItemIDHigh",
        get_id_high,
        set_id_high,
        i32
    );
    netvar!(
        "DT_BaseAttributableItem",
        "m_iItemDefinitionIndex",
        get_id,
        set_id,
        i16
    );
    netvar!(
        "DT_BaseEntity",
        "m_nModelIndex",
        get_model_index,
        set_model_index,
        u32
    );
    netvar!(
        "DT_BaseAttributableItem",
        "m_iAccountID",
        get_account_id,
        set_account_id,
        u32
    );
    netvar!("DT_CSPlayer", "m_hOwnerEntity", get_owner_entity, i32);
    netvar!("DT_BaseEntity", "m_vecOrigin", get_origin, Vec3);
    netvar!("DT_BaseCombatWeapon", "m_iClip1", get_clip, i32);

    pub fn can_fire(&self) -> bool {
        let server_time = get_local_player().get_tickbase() as f32
            * get_interfaces().global_vars.interval_per_tick;

        if self.next_attack() > server_time {
            return false;
        }

        if self.next_attack() > server_time {
            return false;
        }

        if self.get_type() == WeaponType::Melee
            || self.get_type() == WeaponType::Grenade
            || self.get_type() == WeaponType::Unknown
        {
            return false;
        }

        if self.get_clip() == 0 || self.is_reloading() {
            return false;
        }

        true
    }

    pub fn get_weapon_name(&self) -> WeaponId {
        if let Some(weapon) = FromPrimitive::from_i16(self.get_id()) {
            return weapon;
        }

        WeaponId::Unknown
    }

    pub fn apply_skin(
        &self,
        account_id: u32,
        paint_kit: u32,
        wear: f32,
        stat_track: i32,
        seed: u32,
    ) {
        self.set_id_high(-1);
        self.set_account_id(account_id);
        self.set_fallback_paint_kit(paint_kit);
        self.set_fallback_wear(wear);
        self.set_fallback_stat_track(stat_track);
        self.set_fallback_seed(seed);
    }

    pub fn is_melee(&self) -> bool {
        self.is_type(WeaponType::Unknown)
            || self.is_type(WeaponType::Melee)
            || self.is_type(WeaponType::Grenade)
    }

    pub fn get_type(&self) -> WeaponType {
        weapon_data::get_type(self)
    }

    pub fn is_type(&self, weapon_type: WeaponType) -> bool {
        utils::variant_eq(self.get_type(), weapon_type)
    }
}

create_interface!(CBaseViewModel);

impl Entity for CBaseViewModel {}

impl CBaseViewModel {
    netvar!("DT_BaseViewModel", "m_hWeapon", get_weapon, set_weapon, i32);
    netvar!(
        "DT_BaseEntity",
        "m_nModelIndex",
        get_model_index,
        set_model_index,
        u32
    );
}
