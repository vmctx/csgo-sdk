use libc::{c_char, c_void};

pub type RecvVarProxyFn =
    fn(data: *const CRecvProxy, struct_ptr: *mut c_void, out_ptr: *mut c_void);
type ArrayLengthRecvProxyFn =
    fn(struct_ptr: *mut c_void, object_id: i32, current_array_length: i32);
type DataTableRecvVarProxyFn =
    fn(prop: *const CRecvProp, out_ptr: *mut *mut c_void, data_ptr: *mut c_void, object_id: i32);

#[repr(i32)]
#[derive(Copy, Clone, PartialEq)]
pub enum EPropType {
    Int = 0,
    Float,
    Vec,
    VecXY,
    String,
    Array,
    DataTable,
    Int64,
}

#[repr(C)]
pub union CVariantData {
    pub float: f32,
    pub int: i32,
    pub string: *const c_char,
    pub data: *mut c_void,
    pub vector: [f32; 0x3],
    pub int64: i64,
}

#[repr(C)]
pub struct CVariant {
    data: CVariantData,
    prop_type: EPropType,
}

#[repr(C)]
pub struct CRecvTable {
    pub p_props: *mut CRecvProp,
    pub n_props: i32,
    decoder: *const c_void,
    pub table_name: *const c_char,
    is_initialized: bool,
    is_in_main_list: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CRecvProp {
    pub prop_name: *const c_char,
    pub prop_type: EPropType,
    prop_flags: i32,
    buffer_size: i32,
    is_inside_array: i32,
    extra_data_ptr: *const c_void,
    array_prop: *const CRecvProp,
    array_length_proxy: ArrayLengthRecvProxyFn,
    pub proxy_fn: RecvVarProxyFn,
    data_table_proxy_fn: DataTableRecvVarProxyFn,
    pub data_table: *mut CRecvTable,
    pub offset: i32,
    element_stride: i32,
    elements_count: i32,
    parent_array_prop_name: *const c_char,
}

#[repr(C)]
pub struct CRecvProxy {
    recv_prop: *const CRecvProp,
    value: CVariant,
    element_index: i32,
    object_id: i32,
}
