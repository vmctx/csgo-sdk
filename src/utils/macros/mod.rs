/// Creates a function (getter and or setter) for a netvar.
/// Safe to use, in case that the entity is null-ptr it will return
/// the default value of the given return type.
///
/// # Examples
///
/// ```
/// impl MyEntity {
///     // returns health value, or, if self.base is null, 0.
///     macros::netvar!("DT_BasePlayer", "m_iHealth", get_health, i32);
///     // instead lets also create a setter, its as simple as that!
///     macros::netvar!("DT_BasePlayer", "m_iHealth", get_health, set_health, i32);
/// }
/// ```
macro_rules! netvar {
    ($table:literal, $name:literal, $func_name:ident, $return_type:ident) => {
        netvar!($table, $name, 0, $func_name, $return_type);
    };
    ($table:literal, $name:literal, $func_name:ident, $setter_name:ident, $return_type:ident) => {
        netvar!($table, $name, 0, $func_name, $setter_name, $return_type);
    };
    ($table:literal, $name:literal, $extra:expr, $func_name:ident, $return_type:ident) => {
        pub fn $func_name(&self) -> $return_type {
            use crate::netvar::get_offset;

            if self.is_null() {
                return $return_type::default();
            }

            let value = self
                .get_value::<$return_type>(get_offset(obfstr!($table), obfstr!($name)) + $extra);
            return value;
        }
    };
    ($table:literal, $name:literal, $extra:expr, $func_name:ident, $setter_name:ident, $return_type:ident) => {
        pub fn $func_name(&self) -> $return_type {
            use crate::netvar::get_offset;

            if self.is_null() {
                return $return_type::default();
            }

            let value = self
                .get_value::<$return_type>(get_offset(obfstr!($table), obfstr!($name)) + $extra);
            return value;
        }

        pub fn $setter_name(&self, state: $return_type) {
            use crate::netvar::get_offset;

            if self.is_null() {
                return;
            }

            self.set_value(get_offset(obfstr!($table), obfstr!($name)) + $extra, state);
        }
    };
}

/// Creates an interface without any virtual functions, you can define
/// them manually using:
/// ```
/// impl InterfaceName { /* ... */ }
/// ```
/// or using the interface! macro, which allows you to define all virtual
/// funcs in the macro call.
///
/// # Examples
/// ```
/// macros::create_interface!(IExampleInterface);
/// ```
macro_rules! create_interface {
    ($name:ident) => {
        #[repr(C)]
        pub struct $name {
            base: *const usize,
        }

        impl crate::interface::Interface for $name {
            unsafe fn from_raw_unchecked(addr: *const usize) -> Self {
                Self { base: addr }
            }

            fn is_null(&self) -> bool {
                self.base.is_null()
            }

            fn as_ptr(&self) -> *const usize {
                self.base
            }
        }

        impl $name {
            pub(crate) fn from_raw(addr: *const usize) -> Result<Self, ()> {
                if addr.is_null() {
                    return Err(());
                }

                Ok(Self { base: addr })
            }

            pub(crate) unsafe fn from_raw_unchecked(addr: *const usize) -> Self {
                Self { base: addr }
            }

            pub(crate) fn default() -> Self {
                Self {
                    base: core::ptr::null(),
                }
            }

            fn get_value<T>(&self, offset: usize) -> T {
                unsafe { ((self.base as usize + offset) as *mut T).read() }
            }

            fn set_value<T>(&self, offset: usize, value: T) {
                if !self.is_null() {
                    unsafe { core::ptr::write((self.base as usize + offset) as *mut T, value) }
                }
            }

            pub fn as_ptr(&self) -> *const usize {
                self.base
            }

            pub fn is_null(&self) -> bool {
                self.base.is_null()
            }
        }
    };
}

/// Creates an interface and lets you define any virtual functions that you need.
/// # Examples
/// ```
/// macros::interface!(
///     IMyInterface,
///     pub get_something[69]() -> i32
/// );
/// ```
macro_rules! interface {
    ($name:ident, $($vis:vis $func_name:ident[$index:tt]($($arg_name:ident: $arg_type:ty),*) -> $return_type:ty),* ) => {
        /// Interface
        #[repr(C)]
        pub struct $name {
            base: *const usize,
        }

        impl crate::interface::Interface for $name {
            unsafe fn from_raw_unchecked(addr: *const usize) -> Self {
                Self { base: addr }
            }

            fn is_null(&self) -> bool {
                self.base.is_null()
            }

            fn as_ptr(&self) -> *const usize {
                self.base
            }
        }

        impl Default for $name {
            fn default() -> Self {
                unsafe {
                    Self::from_raw_unchecked(core::ptr::null_mut())
                }
            }
        }

        /// # Virtual functions
        impl $name {
            pub(crate) fn from_raw(addr: *const usize) -> Result<Self, ()> {
                if addr.is_null() {
                    return Err(());
                }

                Ok(Self { base: addr})
            }

            pub(crate) unsafe fn from_raw_unchecked(addr: *const usize) -> Self {
                Self { base: addr }
            }

            pub(crate) fn default() -> Self {
                Self {
                    base: core::ptr::null(),
                }
            }

            fn get_value<T>(&self, offset: usize) -> T {
                unsafe { ((self.base as usize + offset) as *mut T).read() }
            }

            fn set_value<T>(&self, offset: usize, value: T) {
                if !self.is_null() {
                     unsafe { core::ptr::write((self.base as usize + offset) as *mut T, value) }
                }
            }

            pub(crate) fn as_ptr(&self) -> *const usize {
                self.base
            }

            pub(crate) fn is_null(&self) -> bool {
                self.base.is_null()
            }

            create_virtual_func!{$(("thiscall" $vis $func_name[$index] -> $return_type;$(($arg_name,$arg_type));*)),*}
        }
    };
}

/// Creates an interface trait and lets you define any virtual functions that you need.
/// # Examples
/// ```
/// macros::interface!(
///     IMyInterface,
///     pub get_something[69]() -> i32
/// );
/// ```
macro_rules! interface_trait {
    ($name:ident, $offset:expr, $($func_name:ident[$index:tt]($($arg_name:ident: $arg_type:ty),*) -> $return_type:ty),* ) => {
        pub trait $name<T: crate::interface::Interface> {
            create_trait_virtual_func!{ $offset $(($func_name[$index] -> $return_type;$(($arg_name,$arg_type));*)),*}
        }
    };
}

/// This is only intended for use inside of InterfaceTrait Impls since
/// it requires self.base to exist. It is called by the interface!
/// macro to create all specified virtual functions.
macro_rules! create_trait_virtual_func {
    ($offset:literal $(($function_name:ident[$index:literal] -> $return_type:ty;$(($param_name:ident,$param_type:ty));*)),*) => {
        $(fn $function_name(&self, $($param_name: $param_type),*) -> $return_type
        where Self: crate::interface::Interface {
            unsafe {
                use core::mem::transmute;
                use crate::utils::memory::get_virtual_function;

                transmute::<_, extern "thiscall" fn(*const usize, $($param_type),*) -> $return_type>(get_virtual_function((self.as_ptr() as usize + $offset) as _, $index).unwrap())((self.as_ptr() as usize + $offset) as _, $($param_name),*)
            }
        })*
    };
    ($offset:literal $(($function_name:ident[$index:literal];$(($param_name:ident,$param_type:ty));*)),*) => {
        $(fn $function_name(&self, $($param_name: $param_type),*)
        where Self: crate::interface::Interface {
            unsafe {
                use core::mem::transmute;
                use crate::utils::memory::get_virtual_function;

                if let Ok(vfunc) = get_virtual_function((self.as_ptr() as usize + $offset) as _, $index) {
                    return transmute::<_, extern "thiscall" fn(*const usize, $($param_type),*)>(vfunc)((self.as_ptr() as usize + $offset) as _, $($param_name),*);
                }
            }
        })*
    };
}

/// This is only intended for use inside of Interface Impls since
/// it requires self.base to exist. It is called by the interface!
/// macro to create all specified virtual functions.
macro_rules! create_virtual_func {
    ($(($calling_convention:tt $visible:vis $function_name:ident[$index:literal] -> $return_type:ty;$(($param_name:ident,$param_type:ty));*)),*) => {
        $($visible fn $function_name(&self, $($param_name: $param_type),*) -> $return_type {
            unsafe {
                use core::mem::transmute;
                use crate::utils::memory::get_virtual_function;

                if $calling_convention == "thiscall" {
                    #[cfg(target_os = "windows")]
                    return transmute::<_, extern $calling_convention fn(*const usize, $($param_type),*) -> $return_type>(get_virtual_function(self.as_ptr(), $index).unwrap())(self.as_ptr(), $($param_name),*);
                    #[cfg(target_os = "linux")]
                    return transmute::<_, extern $calling_convention fn(*const usize, $($param_type),*) -> $return_type>(get_virtual_function(self.as_ptr(), $index + 1).unwrap())(self.as_ptr(), $($param_name),*);
                }

                #[cfg(target_os = "windows")]
                return transmute::<_, extern $calling_convention fn($($param_type),*) -> $return_type>(get_virtual_function(self.as_ptr(), $index).unwrap())($($param_name),*);
                #[cfg(target_os = "linux")]
                return transmute::<_, extern $calling_convention fn($($param_type),*) -> $return_type>(get_virtual_function(self.as_ptr(), $index + 1).unwrap())($($param_name),*);
            }
        })*
    };
    ($(($calling_convention:tt $visible:vis $function_name:ident[$index:literal];$(($param_name:ident,$param_type:ty));*)),*) => {
        $($visible fn $function_name(&self, $($param_name: $param_type),*) {
            unsafe {
                use core::mem::transmute;
                use crate::utils::memory::get_virtual_function;

                #[cfg(target_os = "windows")]
                let index = $index;
                #[cfg(target_os = "linux")]
                let index = $index + 1;

                if let Ok(vfunc) = get_virtual_function(self.as_ptr(), index) {
                    if $calling_convention == "thiscall" {
                        return transmute::<_, extern $calling_convention fn(*const usize, $($param_type),*)>(vfunc)(self.as_ptr(), $($param_name),*);
                    }

                    transmute::<_, extern $calling_convention fn($($param_type),*)>(vfunc)($($param_name),*);
                }
            }
        })*
    };
}

#[doc(hidden)]
macro_rules! enum_num_str {
    (pub enum $name:ident {
        $($variant:ident = $val:expr),*,
    }) => {
        use num_derive::FromPrimitive;

        #[repr(i16)]
        #[derive(FromPrimitive)]
        pub enum $name {
            $($variant = $val),*
        }

        impl $name {
            pub fn name(&self) -> &'static str {
                match self {
                    $($name::$variant => stringify!($variant)),*
                }
            }

            pub fn from_i32(num: i32) -> Option<Self> {
                num_traits::FromPrimitive::from_i16(num as i16)
            }
        }
    };
}

/// Checks if the expression is some, if not it will
/// return void, or the value given as second argument, if any.
/// # Examples
///
/// ```
/// // always returns false.
/// macros::some_or_ret!(None, false)
/// ```
macro_rules! some_or_ret {
    ($option:expr) => {
        match $option {
            Some(x) => x,
            None => return,
        }
    };
    ($result:expr, $default:expr) => {
        match $result {
            Some(x) => x,
            None => return $default,
        }
    };
}

/// Converts a literal into a null-terminated c_char ptr.
///
/// # Examples
///
/// ```
/// // converted into *const i8 (c_char)
/// macros::cstr!("test")
/// ```
#[macro_export]
macro_rules! cstr {
    ( $ literal: literal ) => {
        (concat!($literal, "\0").as_ptr() as *const i8)
    };
}

#[doc(hidden)]
macro_rules! transmute {
    ($address:expr, $type:ty) => {
        core::mem::transmute::<_, $type>($address)
    };
}
