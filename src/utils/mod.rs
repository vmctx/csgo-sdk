//! This contains various utilities that you may reuse in your own project.

#[macro_use]
pub(crate) mod macros;
pub mod error;
pub mod math;
pub mod memory;
pub(crate) mod patterns;
pub mod platform;
pub mod string;

pub(crate) fn variant_eq<T>(a: T, b: T) -> bool {
    core::mem::discriminant(&a) == core::mem::discriminant(&b)
}
