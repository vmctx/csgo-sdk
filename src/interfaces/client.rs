use libc::c_char;

use crate::definitions::recv_props::CRecvTable;

type CreateClientClassFn = extern "system" fn(ent: i32, serial: i32);
type CreateEventFn = extern "system" fn();

#[repr(C)]
pub struct ClientClass {
    create_client_class: CreateClientClassFn,
    create_event: CreateEventFn,
    pub network_name: *mut c_char,
    pub recv_table: *mut CRecvTable,
    pub next: *mut usize,
    pub class_id: i32,
}

interface!(
    IClient,
    pub get_all_classes[8]() -> *const ClientClass
);
