//! Extensions for `&str` and `String`.

use alloc::string::String;
use alloc::vec::Vec;
use libc::c_char;

pub trait StrExt {
    /// Encodes a `&str` to `LPCWSTR` ([Microsoft Docs](https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp/76f10dd8-699d-45e6-a53c-5aefc586da20))
    fn to_lpcwstr(&self) -> *const u16;
    /// Null terminates a `&str` and returns its pointer for C-FFI.
    /// # Examples
    /// ```
    /// use sdk::utils::string::StrExt;
    /// use std::ffi::CStr;
    ///
    /// let c_str: *const i8 = "test".to_cstr();
    /// // Converting it back to a `&str` can be done like this.
    /// unsafe { assert!(CStr::from_ptr(c_str).to_str()?.eq("test")) }
    /// ```
    fn to_cstr(&self) -> *const c_char;
}

pub trait StringExt {
    /// Converts a slice of c_chars to a String.
    fn from_char_slice(slice: &[c_char]) -> String;
}

impl StrExt for &str {
    #[inline]
    fn to_lpcwstr(&self) -> *const u16 {
        self.encode_utf16()
            .chain(Some(0))
            .collect::<Vec<u16>>()
            .as_ptr()
    }

    fn to_cstr(&self) -> *const c_char {
        [self, "\0"].concat().as_ptr() as _
    }
}

impl StringExt for String {
    fn from_char_slice(slice: &[c_char]) -> Self {
        let mut vec = Vec::with_capacity(slice.len());

        slice.iter().for_each(|x| {
            if *x != 0 {
                vec.push(*x as u8);
            }
        });

        String::from_utf8(vec).unwrap_or_default()
    }
}
