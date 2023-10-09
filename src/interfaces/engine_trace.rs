use crate::utils::math::vector::Vec3;

use alloc::vec::Vec;
use libc::c_void;

pub mod hit_group {
    use core::convert::TryFrom;

    #[repr(i32)]
    pub enum HitGroup {
        Invalid = -1,
        Generic = 0,
        Head,
        Chest,
        Stomach,
        LeftArm,
        RightArm,
        LeftLeg,
        RightLeg,
        Gear,
    }

    impl TryFrom<i32> for HitGroup {
        type Error = ();

        fn try_from(v: i32) -> Result<Self, Self::Error> {
            match v {
                x if x == HitGroup::Head as i32 => Ok(HitGroup::Head),
                x if x == HitGroup::Chest as i32 => Ok(HitGroup::Chest),
                x if x == HitGroup::Stomach as i32 => Ok(HitGroup::Stomach),
                x if x == HitGroup::LeftArm as i32 => Ok(HitGroup::LeftArm),
                x if x == HitGroup::RightArm as i32 => Ok(HitGroup::RightArm),
                x if x == HitGroup::RightLeg as i32 => Ok(HitGroup::RightLeg),
                _ => Err(()),
            }
        }
    }

    pub fn get_damage_multiplier(hit_group: i32) -> f32 {
        if let Ok(hit_group) = HitGroup::try_from(hit_group) {
            return match hit_group {
                HitGroup::Head => 4.0,
                HitGroup::Stomach => 1.25,
                HitGroup::RightLeg => 0.75,
                _ => 1.0,
            };
        }
        1.0
    }

    pub fn is_armored(hit_group: i32, helmet: bool) -> bool {
        if let Ok(hit_group) = HitGroup::try_from(hit_group) {
            return match hit_group {
                HitGroup::Head => helmet,
                HitGroup::Chest => true,
                HitGroup::Stomach => true,
                HitGroup::LeftArm => true,
                HitGroup::RightArm => true,
                _ => false,
            };
        }

        false
    }
}

#[repr(C, align(16))]
#[derive(Copy, Clone, Default)]
struct VectorAligned {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl From<Vec3> for VectorAligned {
    fn from(vec: Vec3) -> Self {
        Self {
            x: vec.x as f32,
            y: vec.y as f32,
            z: vec.z as f32,
            w: 0e0,
        }
    }
}

impl From<VectorAligned> for Vec3 {
    fn from(vec_aligned: VectorAligned) -> Self {
        Self::new(vec_aligned.x, vec_aligned.y, vec_aligned.z)
    }
}

#[repr(C)]
pub struct Entity {
    pub vtable: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Ray {
    start: VectorAligned,
    delta: VectorAligned,
    start_offset: VectorAligned,
    extents: VectorAligned,
    pub p_world_axis_transform_matrix: *const c_void,
    pub is_ray: bool,
    pub is_swept: bool,
}

impl Ray {
    pub fn new(start: Vec3, end: Vec3) -> Self {
        let mut instance = unsafe { core::mem::zeroed::<Self>() };
        instance.delta = VectorAligned::from(end - start);
        instance.start = VectorAligned::from(start);
        instance.is_swept = Vec3::from(instance.delta).len_sqr() != 0e0;
        instance.is_ray = true;
        instance
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CSurface {
    pub name: *const i8,
    pub surface_props: i16,
    pub flags: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CPlane {
    pub normal: Vec3,
    pub dist: f32,
    pub r#type: u8,
    pub sign_bit: u8,
    pad: [u8; 0x2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Trace {
    pub start: Vec3,
    pub end: Vec3,
    pub plane: CPlane,
    pub fraction: f32,
    pub contents: i32,
    pub disp_flags: u16,
    pub all_solid: bool,
    pub start_solid: bool,
    pub fraction_solid_left: f32,
    pub surface: CSurface,
    pub hit_group: i32,
    pub physics_bone: i16,
    pub ptr_entity: &'static Entity,
    pub hitbox: i32,
}

#[repr(C)]
pub enum TraceType {
    EVERYTHING = 0,
    WorldOnly = 1,
    EntitiesOnly = 2,
    EverythingFilterProps = 3,
}

pub trait TraceFilterTrait {
    fn should_hit_entity(&self, entity: &Entity, contents_mask: u32) -> bool;
    fn get_trace_type(&self) -> TraceType;
    fn new(skip: *const usize) -> Self;
}

#[repr(C)]
pub struct TraceFilterGeneric {
    vtable: usize,
    skip: *const Entity,
    vec_vtable: Vec<usize>,
}

impl TraceFilterTrait for TraceFilterGeneric {
    fn should_hit_entity(&self, entity: &Entity, _: u32) -> bool {
        entity as *const _ as usize != self.skip as *const _ as usize
    }

    fn get_trace_type(&self) -> TraceType {
        TraceType::EVERYTHING
    }

    fn new(skip: *const usize) -> Self {
        extern "thiscall" fn should_hit_entity_wrapper(
            this: &TraceFilterGeneric,
            entity: &Entity,
            contents_mask: u32,
        ) -> bool {
            this.should_hit_entity(entity, contents_mask)
        }

        extern "thiscall" fn get_trace_type_wrapper(this: &TraceFilterGeneric) -> TraceType {
            this.get_trace_type()
        }

        let mut vec = Vec::<usize>::new();

        vec.push(should_hit_entity_wrapper as usize);
        vec.push(get_trace_type_wrapper as usize);

        Self {
            vtable: vec.as_ptr() as _,
            vec_vtable: vec,
            skip: unsafe { &*(skip as *const Entity) },
        }
    }
}

pub const MASK_SHOT: i32 = 0x1 | 0x4000 | 0x2000000 | 0x2 | 0x4000000 | 0x40000000;
pub const MASK_SHOT_HULL: i32 = 0x1 | 0x4000 | 0x2000000 | 0x2 | 0x4000000 | 0x8;
pub const CONTENTS_HITBOX: i32 = 0x40000000;
pub const SURF_HITBOX: u16 = 0x8000;
pub const SURF_LIGHT: u16 = 0x0001;
pub const SURF_NODRAW: u16 = 0x0080;

interface!(
    IEngineTrace,
    pub get_point_contents[0](abs_pos: &Vec3, contents_mask: i32, entity: *mut usize) -> i32,
    pub clip_ray_to_entity[4](ray: &Ray, mask: u32, ent: &mut Entity, trace: &mut Trace) -> (),
    pub trace_ray_virtual[5](ray: &Ray, mask: u32, filter: *mut usize, trace: &mut Trace) -> ()
);

impl IEngineTrace {
    pub fn trace_ray<T>(&self, ray: &Ray, mask: u32, filter: &mut T, trace: &mut Trace)
    where
        T: TraceFilterTrait,
    {
        self.trace_ray_virtual(ray, mask, unsafe { transmute!(filter, *mut usize) }, trace)
    }
}
