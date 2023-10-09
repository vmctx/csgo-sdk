use crate::classes::entity::CEntity;
use crate::classes::Entity;
use crate::interface::Interface;
use crate::utils::memory::NotNull;

interface!(
    IEntityList,
    get_entity_by_id_virtual[3](id: i32) -> NotNull<CEntity>,
    get_entity_from_handle_virtual[4](handle: i32) -> NotNull<CEntity>,
    pub get_highest_entity_index[6]() -> i32
);

impl IEntityList {
    pub fn get_entity_by_id<T: Entity + Interface>(&self, id: i32) -> Option<T> {
        self.get_entity_by_id_virtual(id)
            .get()
            .map(|entity| unsafe { T::from_raw_unchecked(entity.as_ptr()) })
    }

    pub fn get_entity_from_handle<T: Entity + Interface>(&self, handle: i32) -> Option<T> {
        self.get_entity_from_handle_virtual(handle)
            .get()
            .map(|entity| unsafe { T::from_raw_unchecked(entity.as_ptr()) })
    }
}
