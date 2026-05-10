#[macro_export]
/// Converts unsigned types to their signed versions
macro_rules! unsigned_to_signed {
    (u8) => {
        i8
    };
    (u16) => {
        i16
    };
    (u32) => {
        i32
    };
    (u64) => {
        i64
    };
    (u128) => {
        i128
    };
    (usize) => {
        isize
    };
}
#[macro_export]
/// Converts signed types to their unsigned versions
macro_rules! signed_to_unsigned {
    (i8) => {
        u8
    };
    (i16) => {
        u16
    };
    (i32) => {
        u32
    };
    (i64) => {
        u64
    };
    (i128) => {
        u128
    };
    (isize) => {
        usize
    };
}
#[macro_export]
/// Switches between unsigned and signed versions
macro_rules! switch_signing {
    (i8) => {
        u8
    };
    (i16) => {
        u16
    };
    (i32) => {
        u32
    };
    (i64) => {
        u64
    };
    (i128) => {
        u128
    };
    (isize) => {
        usize
    };
    (u8) => {
        i8
    };
    (u16) => {
        i16
    };
    (u32) => {
        i32
    };
    (u64) => {
        i64
    };
    (u128) => {
        i128
    };
    (usize) => {
        isize
    };
}
#[macro_export]
/// Gives the next bigger value type excluding usize/isize
macro_rules! upgrade_type {
    (i8) => {
        i16
    };
    (i16) => {
        i32
    };
    (i32) => {
        i64
    };
    (i64) => {
        i128
    };
    (u8) => {
        u16
    };
    (u16) => {
        u32
    };
    (u32) => {
        u64
    };
    (u64) => {
        u128
    };
    (f32) => {
        f64
    };
    (f64) => {
        f128
    };
    (f16) => {
        f32
    };
}
/// Implement const default for the given value like this:
///
/// `impl_default!({type}, {value})`
/// or
/// `impl_default!({type}, {value}, {docstring})`
#[macro_export]
macro_rules! impl_default {
    ($t:ty, $v:expr, $doc:tt) => {
        impl const Default for $t {
            #[inline(always)]
            #[doc = $doc]
            fn default() -> $t {
                $v
            }
        }
    };
    ($t:ty, $v:expr) => {
        impl const Default for $t {
            #[inline(always)]
            fn default() -> $t {
                $v
            }
        }
    };
}
#[macro_export]
/// Mod the given module and import all of its contents.
/// If there is nothing to import, just mod it
macro_rules! mod_and_pub_import {
    ($i:ident) => {
        mod $i;
        #[allow(unused_imports)] // If there is nothing to import
        #[allow(unreachable_pub)]
        // These two must exist or they'll throw warning
        pub use $i::*;
    };
}

#[macro_export]
/// Mod the given module and import all of its contents.
/// If there is nothing to import, just mod it
macro_rules! pub_mod_and_import {
    ($i:ident) => {
        pub mod $i;
        #[allow(unused_imports)] // If there is nothing to import
        #[allow(unreachable_pub)]
        // These two must exist or they'll throw warning
        use $i::*;
    };
}

#[macro_export]
/// Mod the given module and import all of its contents.
/// If there is nothing to import, just mod it
macro_rules! mod_and_import {
    ($i:ident) => {
        mod $i;
        #[allow(unused_imports)] // If there is nothing to import
        #[allow(unreachable_pub)]
        // These two must exist or they'll throw warning
        use $i::*;
    };
}
