//! NetVar Manager.

use alloc::collections::BTreeMap;

use static_init::dynamic;

use alloc::string::{String, ToString};

#[dynamic]
static mut NETVARS: BTreeMap<String, usize> = BTreeMap::new();
#[dynamic]
static mut PROPS: BTreeMap<String, usize> = BTreeMap::new();

/// Returns the NetVar value of the given NetVar.
/// # Examples
/// ```
/// use sdk::netvar;
///
/// let offset = netvar::get_offset("DT_BasePlayer", "m_iHealth");
///
/// assert_eq!(offset, 0x100);
/// ```
pub fn get_offset(table: &str, netvar: &str) -> usize {
    *NETVARS
        .read()
        .get(&(table.to_string() + "->" + netvar))
        .unwrap_or(&0)
}

pub mod hook {
    //! Easily hook NetVars.
    use crate::definitions::recv_props::{CRecvProp, RecvVarProxyFn};
    use crate::netvar::PROPS;

    pub struct RecvProxyHook {
        property: usize,
        original: RecvVarProxyFn,
    }

    impl RecvProxyHook {
        pub(crate) fn hook(property: usize, proxy_fn: RecvVarProxyFn) -> Self {
            let prop = unsafe { transmute!(property, *mut CRecvProp).as_mut().unwrap() };

            let original = prop.proxy_fn;

            prop.proxy_fn = proxy_fn;

            Self { property, original }
        }

        pub fn reset(&mut self) {
            unsafe {
                let prop = transmute!(self.property, *mut CRecvProp).as_mut().unwrap();
                prop.proxy_fn = self.original;
            }
        }

        pub fn get_original(&self) -> RecvVarProxyFn {
            unsafe { transmute!(self.original, RecvVarProxyFn) }
        }
    }

    /// Hook NetVars
    ///
    /// # Examples
    /// ```
    /// use sdk::definitions::recv_props::CRecvProxy;
    /// use sdk::netvar::hook::hook_netvar;
    /// use std::ffi::c_void;
    ///
    /// // Thats how the hooked fn should look like
    /// fn hook(proxy_data_const: *const CRecvProxy, entity: *mut c_void, output: *mut c_void) {
    /// // ...
    /// }
    ///
    /// // You would store this globally to be able to call the original in the hooked fn.
    /// let hook = hook_netvar("CBaseViewModel->m_nSequence", hook);
    /// ```
    pub fn hook_netvar(name: &str, hook: RecvVarProxyFn) -> Option<RecvProxyHook> {
        PROPS
            .read()
            .get(name)
            .map(|prop| RecvProxyHook::hook(*prop, hook))
    }
}

pub(crate) mod manager {
    use crate::definitions::recv_props::{CRecvTable, EPropType};
    use crate::interfaces::client::ClientClass;
    use crate::netvar::{NETVARS, PROPS};
    use crate::utils::error::Error;
    use crate::utils::error::Error::{Null, Unknown};
    use alloc::string::{String, ToString};
    use cstr_core::CStr;

    unsafe fn store_props(group_name: String, recv_table: *mut CRecvTable, child_offset: usize) {
        for i in 0..(*recv_table).n_props as isize {
            let prop = (*recv_table).p_props.offset(i).read();
            let child = prop.data_table;

            let var_name = CStr::from_ptr(prop.prop_name).to_str().unwrap().to_string();

            if var_name.chars().next().unwrap().is_numeric() {
                continue;
            }

            if var_name.eq(obfstr!("baseclass")) {
                continue;
            }

            if !child.is_null() {
                let table_name = CStr::from_ptr(child.read().table_name)
                    .to_str()
                    .unwrap()
                    .to_string();

                if prop.prop_type == EPropType::DataTable && table_name.starts_with('D') {
                    store_props(
                        group_name.to_string(),
                        child,
                        prop.offset as usize + child_offset,
                    );
                }
            }

            let formatted = format!("{}->{}", group_name, var_name);

            PROPS
                .write()
                .insert(formatted.clone(), (*recv_table).p_props.offset(i) as usize);

            NETVARS.write().insert(
                formatted.replacen("C", "DT_", 1),
                prop.offset as usize + child_offset as usize,
            );
        }
    }

    /// Loads all NetVar's, this is used in sdk::initialize only.
    pub(crate) fn scan() -> Result<(), Error> {
        let mut client_class_ptr = crate::get_interfaces().client.get_all_classes();

        if client_class_ptr.is_null() {
            return Err(Null {
                item: obfstr!("ClientClass").into(),
            });
        }

        while !client_class_ptr.is_null() {
            unsafe {
                let recv_table = client_class_ptr.read().recv_table;

                let table_name = CStr::from_ptr(client_class_ptr.read().network_name)
                    .to_str()
                    .unwrap()
                    .to_string();

                store_props(table_name, recv_table, 0);

                client_class_ptr = client_class_ptr.read().next as *const ClientClass;
            }
        }

        if NETVARS.read().len() == 0 {
            return Err(Unknown {
                message: obfstr!("Failed to scan NetVars.").into(),
            });
        }

        Ok(())
    }
}
