#[repr(C)]
pub struct CUtlVec<T> {
    pub memory: *mut T,
    pub allocation_count: i32,
    pub grow_size: i32,
    pub size: i32,
    pub elements: *mut T,
}

impl<T> CUtlVec<T> {
    pub fn get_mut(&mut self, index: i32) -> Option<&mut T> {
        unsafe {
            core::slice::from_raw_parts_mut(self.memory, self.size as usize).get_mut(index as usize)
        }
    }

    pub fn get(&self, index: i32) -> Option<&T> {
        unsafe { core::slice::from_raw_parts(self.memory, self.size as usize).get(index as usize) }
    }

    pub fn size(&self) -> i32 {
        self.size
    }
}
