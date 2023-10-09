use libc::c_char;
use num_traits::FromPrimitive;

use crate::classes::weapon::CWeapon;

enum_num_str! {
    pub enum WeaponId {
        Unknown = 0,
        Deagle = 1,
        Elite = 2,
        Fiveseven = 3,
        Glock = 4,
        Ak47 = 7,
        Aug = 8,
        Awp = 9,
        Famas = 10,
        G3SG1 = 11,
        GalilAr = 13,
        M249 = 14,
        M4A1 = 16,
        Mac10 = 17,
        P90 = 19,
        ZoneRepulsor = 20,
        Mp5sd = 23,
        Ump45 = 24,
        Xm1014 = 25,
        Bizon = 26,
        Mag7 = 27,
        Negev = 28,
        Sawedoff = 29,
        Tec9 = 30,
        Taser = 31,
        Hkp2000 = 32,
        Mp7 = 33,
        Mp9 = 34,
        Nova = 35,
        P250 = 36,
        Shield = 37,
        Scar20 = 38,
        Sg553 = 39,
        Ssg08 = 40,
        GoldenKnife = 41,
        Knife = 42,
        Flashbang = 43,
        HeGrenade = 44,
        SmokeGrenade = 45,
        Molotov = 46,
        Decoy = 47,
        IncGrenade = 48,
        C4 = 49,
        Healthshot = 57,
        KnifeT = 59,
        M4a1S = 60,
        UspS = 61,
        Cz75a = 63,
        Revolver = 64,
        TaGrenade = 68,
        Axe = 75,
        Hammer = 76,
        Spanner = 78,
        GhostKnife = 80,
        Firebomb = 81,
        Diversion = 82,
        FragGrenade = 83,
        Snowball = 84,
        BumpMine = 85,
        Bayonet = 500,
        ClassicKnife = 503,
        Flip = 505,
        Gut = 506,
        Karambit = 507,
        M9Bayonet = 508,
        Huntsman = 509,
        Falchion = 512,
        Bowie = 514,
        Butterfly = 515,
        Daggers = 516,
        Paracord = 517,
        SurvivalKnife = 518,
        Ursus = 519,
        Navaja = 520,
        NomadKnife = 521,
        Stiletto = 522,
        Talon = 523,
        SkeletonKnife = 525,
        NameTag = 1200,
        Sticker = 1209,
        MusicKit = 1314,
        SealedGraffiti = 1348,
        Graffiti = 1349,
        OperationHydraPass = 1352,
        BronzeOperationHydraCoin = 4353,
        Patch = 4609,
        GloveStuddedBrokenfang = 4725,
        GloveStuddedBloodhound = 5027,
        GloveT = 5028,
        GloveCT  = 5029,
        GloveSporty = 5030,
        GloveSlick = 5031,
        GloveLeatherWrap = 5032,
        GloveMotorcycle = 5033,
        GloveSpecialist = 5034,
        GloveHydra = 5035,
    }
}

// todo use virtual func

#[derive(PartialEq)]
pub enum WeaponType {
    AutoSniper,
    Sniper,
    Pistol,
    AssaultRifle,
    MachineGun,
    MachinePistol,
    Shotgun,
    Melee,
    Grenade,
    Unknown,
}

pub(crate) fn get_type(weapon: &CWeapon) -> WeaponType {
    use WeaponId::*;

    let weapon_type = some_or_ret!(
        <WeaponId as FromPrimitive>::from_i32(weapon.get_id() as i32),
        WeaponType::Unknown
    );

    match weapon_type {
        Deagle | Elite | Fiveseven | Glock | ZoneRepulsor | Hkp2000 | P250 | UspS | Revolver => {
            WeaponType::Pistol
        }
        Ak47 | Famas | GalilAr | M4A1 | M4a1S => WeaponType::AssaultRifle,
        M249 | Negev => WeaponType::MachineGun,
        Mac10 | P90 | Ump45 | Bizon | Mp7 => WeaponType::MachinePistol,
        Xm1014 | Mag7 | Sawedoff => WeaponType::Shotgun,
        Awp | Ssg08 => WeaponType::Sniper,
        G3SG1 | Scar20 => WeaponType::AutoSniper,
        GoldenKnife | Knife | KnifeT | Axe | GhostKnife | Bayonet | ClassicKnife | Flip | Gut
        | Karambit | M9Bayonet | Huntsman | Falchion | Bowie | Butterfly | Daggers | Paracord
        | SurvivalKnife | Ursus | Navaja | NomadKnife | Stiletto | Talon | SkeletonKnife => {
            WeaponType::Melee
        }
        Flashbang | HeGrenade | SmokeGrenade | Molotov | Decoy | IncGrenade => WeaponType::Grenade,
        C4 => WeaponType::Unknown,
        _ => WeaponType::Unknown,
    }
}

#[repr(C)]
pub struct CWeaponInfo {
    _pad0: [c_char; 0x14],
    pub max_clip: i32,
    _pad1: [c_char; 0x70],
    name: *const c_char,
    _pad2: [c_char; 0x3C],
    weapon_type: WeaponType,
    _pad3: [c_char; 0x4],
    price: i32,
    _pad4: [c_char; 0x8],
    cycle_time: f32,
    _pad5: [c_char; 0xC],
    full_auto: bool,
    _pad6: [c_char; 0x3],
    pub damage: i32,
    headshot_mult: f32,
    pub armor_ratio: f32,
    bullets: i32,
    pub penetration: f32,
    _pad7: [c_char; 0x8],
    pub range: f32,
    pub range_modifier: f32,
    _pad8: [c_char; 0x10],
    silencer: bool,
    _pad9: [c_char; 0xF],
    max_speed: f32,
    max_speed_alt: f32,
    _pad10: [c_char; 0x64],
    recoil_magnitude: f32,
    recoil_magnitude_alt: f32,
    _pad11: [c_char; 0x10],
    recovery_time_stand: f32,
}
