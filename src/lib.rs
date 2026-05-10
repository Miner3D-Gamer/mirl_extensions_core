//! The core lib of `mirl_extensions`
//!
//! This lib defines traits that cannot be automatically implemented
//!
//! Is it recommended to use `mirl_extensions` over this
// Const
#![feature(const_trait_impl)]
#![feature(rustc_attrs)]

mod macro_fun;

mod maps;
pub use maps::*;
