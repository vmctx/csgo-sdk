use winapi::ctypes::c_void;

interface!(
    IKeyValuesSystem,
    pub register_size_of_key_values[0](i_size: i32) -> (),
    pub alloc_key_values_memory[1](i_size: i32) -> *const c_void
);
