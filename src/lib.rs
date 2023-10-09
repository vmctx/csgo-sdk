#![feature(abi_thiscall)]
#![no_std]

//! # Initialization
//! ```no_run
//! sdk::initialize()?;
//! ```
//! To actually use the SDK you need to initialize it somewhere in, or after,
//! [DllMain](https://docs.microsoft.com/en-us/windows/win32/dlls/dllmain). `entry_point` is a function being called by
//! [CreateThread](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createthread)
//! in DllMain. This example also uses [winapi-rs](https://docs.rs/winapi/0.3.9/winapi/).
//! # Example
//! ```no_run
//! use winapi::um::consoleapi::AllocConsole;
//!
//! unsafe extern "system" fn entry_point(_lib: *mut std::ffi::c_void) -> u32 {
//!     AllocConsole();
//!
//!     if sdk::initialize().is_err() {
//!         panic!("failed to init sdk!");
//!     }
//!
//!     let (mut width, mut height) = (0, 0);
//!
//!     sdk::get_interfaces().engine.get_screen_size(&mut width, &mut height);
//!
//!     println!("{},{}", width, height);
//!
//!     0
//! }
//! ```

#[macro_use]
extern crate alloc;
#[macro_use]
extern crate obfstr;

use alloc::vec::Vec;

use fehler::throws;
use static_init::dynamic;
use static_init::lazy::lesser_locked_lazy::{ReadGuard, WriteGuard};

use crate::classes::entity::CEntity;
use crate::interface::Interfaces;
use crate::utils::error::Error;
use crate::utils::math::get_player_distance;

#[macro_use]
pub mod utils;
#[allow(dead_code)]
pub mod classes;
pub mod definitions;
pub mod interface;
#[allow(dead_code)]
pub mod interfaces;
pub mod netvar;

/* TODO
  document all public functions and types, atleast the ones that could need some.
*/

#[dynamic]
static mut INTERFACES: Interfaces = Interfaces::default();

/// Initializes the static `Interfaces` struct and loads all NetVars.
#[throws(Error)]
pub fn initialize() {
    *INTERFACES.write() = Interfaces::load()?;
    netvar::manager::scan()?;
}

/// Returns a static reference to the `Interfaces` struct.
pub fn get_interfaces() -> ReadGuard<'static, Interfaces> {
    INTERFACES.read()
}

/// Returns a static mutable reference to the `Interfaces` struct.
pub fn get_interfaces_mut() -> WriteGuard<'static, Interfaces> {
    INTERFACES.write()
}

/// Returns the closest entity from the localplayers position, if any.
pub fn search_closest_entity() -> Option<CEntity> {
    let mut closest_entity: Option<CEntity> = None;

    let local_pos = get_local_player().get_origin();

    get_all_players().into_iter().for_each(|entity| {
        if let Some(closest) = &closest_entity {
            if get_player_distance(local_pos, entity.get_origin())
                < get_player_distance(local_pos, closest.get_origin())
            {
                closest_entity = Some(entity);
            }
        } else {
            closest_entity = Some(entity)
        }
    });

    closest_entity
}

/// Returns a Vec<CEntity> containing all players in the current game.
///
/// The size of this is the same as `sdk::get_interfaces().global_vars.max_clients`.
pub fn get_all_players() -> Vec<CEntity> {
    let max_players = get_interfaces().global_vars.max_clients;
    let mut players = Vec::with_capacity(max_players as usize);

    for id in 0..max_players {
        if let Some(entity) = get_entity_by_id(id) {
            if entity.is_valid_player() {
                players.push(entity);
            }
        }
    }

    players
}

/// Returns a Vec<CEntity> containing all entities in the current game.
///
/// The size of this is the same as
/// `sdk::get_interfaces().entity_list.get_highest_entity_index()`.
pub fn get_all_entities() -> Vec<CEntity> {
    let max_entities = get_interfaces().entity_list.get_highest_entity_index();
    let mut entities = Vec::with_capacity(max_entities as usize);

    for id in 0..max_entities {
        if let Some(entity) = get_entity_by_id(id) {
            entities.push(entity)
        }
    }

    entities
}

/// Returns a local player CEntity.
///
/// # Example
/// ```no_run
/// if sdk::get_interfaces().engine.is_ingame() {
///     println!("{}", sdk::get_local_player().get_name());
/// }
/// ```
/// # Panics
/// This will panic if `get_entity_by_id` returns `null()`.
pub fn get_local_player() -> CEntity {
    let id = get_interfaces().engine.get_local_player();
    get_interfaces().entity_list.get_entity_by_id(id).unwrap()
}

/// Wrapper for IEntityList's `get_entity_by_id`.
///
/// Use IEntityList's method if you for example want it to return CWeapon.
pub fn get_entity_by_id(id: i32) -> Option<CEntity> {
    get_interfaces().entity_list.get_entity_by_id::<CEntity>(id)
}
