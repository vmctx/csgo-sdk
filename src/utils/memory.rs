//! Utility functions to interact with the game's memory.

use crate::interface::Interface;

use crate::utils::error::Error;
#[doc(hidden)]
#[cfg(target_os = "linux")]
pub use linux::*;
#[doc(hidden)]
#[cfg(target_os = "windows")]
pub use windows::*;

/// Will guarantee that the return value is not `null()`.
///
/// It is instead representated with a `None` value.
#[repr(transparent)]
pub struct NotNull<T: Interface> {
    ptr: T,
}

impl<T: Interface> NotNull<T> {
    /// Returns `None` if the contained value is `null()`,
    /// if not it returns `Some(T)`.
    pub fn get(self) -> Option<T> {
        if !self.ptr.is_null() {
            return Some(self.ptr);
        }

        None
    }

    /// Even if the contained value is `null()` a new `T`
    /// with the value will be created and returned.
    pub fn unwrap(self) -> T {
        self.ptr
    }
}

#[cfg(target_os = "windows")]
mod windows {
    use winapi::um::winnt::IMAGE_DOS_SIGNATURE;
    use winapi::{
        ctypes::{c_char, c_void},
        shared::minwindef::HMODULE,
        um::libloaderapi::{GetModuleHandleA, GetProcAddress},
        um::winnt::{PIMAGE_DOS_HEADER, PIMAGE_NT_HEADERS},
    };

    /// Safe wrapper around `GetModuleHandle`.
    ///
    /// # Examples
    /// ```
    /// use sdk::utils::memory::get_module_handle;
    ///
    ///     let handle = get_module_handle("ntdll.dll\0".as_ptr() as _);
    ///
    ///     assert!(!handle.is_null())
    /// ```
    pub fn get_module_handle(name: *const i8) -> HMODULE {
        unsafe { GetModuleHandleA(name) }
    }

    /// Safe wrapper around `GetProcAddress`.
    ///
    /// Returns `None` if `GetProcAddress` returns `null()`.
    /// # Examples
    /// ```
    /// use sdk::utils::memory::{get_proc_address, get_module_handle};
    ///
    ///     let address = get_proc_address(
    ///     get_module_handle("ntdll.dll\0".as_ptr() as _), "DbgPrintEx\0".as_ptr() as _
    ///     );
    ///
    ///     assert!(address.is_some())
    /// ```
    pub fn get_proc_address(module: HMODULE, name: *const i8) -> Option<*const c_void> {
        let result = unsafe { GetProcAddress(module, name) };

        if !result.is_null() {
            return Some(result as _);
        }

        None
    }

    /// Scans for a byte string in the given module.
    ///
    /// Will return `None` when [`get_module_handle()`] returns `null`, or
    /// if it does not find the pattern.
    /// # Examples
    /// ```
    /// // A '?' is represented by \x00
    /// use sdk::utils::memory::pattern_scan;
    ///
    /// let result = pattern_scan("ntdll.dll\0".as_ptr() as _, b"\xFF\x00\xCD\xA9");
    ///
    /// assert_eq!(result, None)
    /// ```
    pub fn pattern_scan(module_name: *const c_char, sig: &[u8]) -> Option<*mut usize> {
        let module = get_module_handle(module_name);

        if module.is_null() {
            return None;
        }

        let dos_headers = unsafe { (module as PIMAGE_DOS_HEADER).read() };

        if dos_headers.e_magic != IMAGE_DOS_SIGNATURE {
            return None;
        }

        let nt_headers = (module as usize + dos_headers.e_lfanew as usize) as PIMAGE_NT_HEADERS;

        let size_of_image = (unsafe { *nt_headers }).OptionalHeader.SizeOfImage as usize;

        let bytes = module as *mut u8;

        let size = sig.len();

        for i in 0..(size_of_image - size as usize) {
            let mut found = true;

            for (j, byte) in sig.iter().enumerate().take(size) {
                if unsafe { *bytes.offset(i as isize + j as isize) } != *byte && *byte != 0 {
                    found = false;
                    break;
                }
            }

            if found {
                return Some(unsafe { bytes.add(i) } as _);
            }
        }

        None
    }
}

#[cfg(target_os = "linux")]
mod linux {
    use libc::c_char;

    /// Scans for a byte string in the given module.
    ///
    /// Will return `None` when [`get_module_handle()`] returns `null`, or
    /// if it does not find the pattern.
    /// # Examples
    /// ```
    /// // A '?' is represented by \x00
    /// use sdk::utils::memory::pattern_scan;
    ///
    /// let result = pattern_scan("ntdll.dll\0".as_ptr() as _, b"\xFF\x00\xCD\xA9");
    ///
    /// assert_eq!(result, None)
    /// ```
    pub fn pattern_scan(_module_name: *const c_char, _sig: &[u8]) -> Option<*mut usize> {
        None
    }
}

/// Get the address of a virtual function by its vtable-ptr and the index.
/// # Safety
/// The VTable pointer is checked for `null()`, although if the index is invalid this will
/// very likely lead to a crash when using transmute on the return value.
/// # Examples
/// ```
/// use sdk::utils::memory::{get_virtual_function, get_module_handle};
///
///     let module_address = get_module_handle("test.dll\0".as_ptr() as _);
///
///     if !module_address.is_null() {
///     let address = unsafe { get_virtual_function(module_address as _, 0) };
///     }
/// ```
pub unsafe fn get_virtual_function(
    vtable_ptr: *const usize,
    idx: isize,
) -> Result<*const usize, Error> {
    if !vtable_ptr.is_null() {
        let vt = *vtable_ptr as *const usize;
        let vfn = vt.offset(idx).read() as *const usize;

        if vfn.is_null() {
            return Err(Error::NotFound {
                item: obfstr!("VTable index not found.").into(),
            });
        }

        Ok(vfn)
    } else {
        Err(Error::Null {
            item: obfstr!("VTable ptr is null-ptr.").into(),
        })
    }
}

/// Wrapper for `core::ptr::read`.
/// # Safety
/// Behavior is undefined if any of the following conditions are violated:
/// - address must be valid for reads.
/// - address must be properly aligned. Use read_unaligned if this is not the case.
///-  address must point to a properly initialized value of type T.
pub unsafe fn read<T>(address: usize) -> T {
    core::ptr::read::<T>(address as *const T)
}

/// Casts the address as raw pointer, dereferences it and returns a mutable reference.
/// # Safety
/// Behavior is undefined if any of the following conditions are violated:
/// - address must be valid for reads.
/// - address must be properly aligned. Use read_unaligned if this is not the case.
///-  address must point to a properly initialized value of type T.
pub unsafe fn read_mut<T>(address: usize) -> &'static mut T {
    &mut *(address as *mut T)
}
