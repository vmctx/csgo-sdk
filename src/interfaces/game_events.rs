use alloc::vec::Vec;
use libc::c_char;

interface!(
    IGameEventManager,
    pub add_listener[3](listener: &mut IGameEventListener, name: *const c_char, server_side: bool) -> bool,
    pub remove_listener[5](listener: &mut IGameEventListener) -> ()
);

interface!(
    IGameEvent,
    pub get_name[1]() -> *const c_char,
    pub get_int[6](key_name: *const c_char, default_value: i32) -> i32
);

pub trait GameEventTrait {
    fn fire_game_event(&self, event: &mut IGameEvent);
}

#[repr(C)]
pub struct IGameEventListener {
    vtable: usize,
    vec_vtable: Vec<usize>,
}

impl IGameEventListener {
    pub fn new<T>() -> Self
    where
        T: GameEventTrait,
    {
        extern "thiscall" fn destructor<T: GameEventTrait>(this: T) {
            core::mem::drop(this);
        }

        extern "thiscall" fn fire_game_event<T: GameEventTrait>(this: &T, game_event: *mut usize) {
            T::fire_game_event(this, &mut unsafe {
                IGameEvent::from_raw_unchecked(game_event)
            })
        }

        extern "thiscall" fn get_event_debug_id<T: GameEventTrait>(_: &T) -> i32 {
            42
        }

        let vec = vec![
            destructor::<T> as usize,
            fire_game_event::<T> as usize,
            get_event_debug_id::<T> as usize,
        ];

        Self {
            vtable: vec.as_ptr() as usize,
            vec_vtable: vec,
        }
    }
}
