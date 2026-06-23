//! The core lib of `mirl_extensions`
//!
//! This lib defines traits that cannot be automatically implemented
//!
//! Is it recommended to use `mirl_extensions` over this
// Const
#![feature(const_trait_impl)]
#![feature(rustc_attrs)]
#![allow(incomplete_features)]
#![feature(specialization)] // This has not yet been implemented by the rust team.
#![feature(slice_swap_unchecked)]
#![feature(f128)]
#![feature(f16)]
#![feature(const_ops)]
#![feature(const_cmp)]
#![feature(const_destruct)]

mod macros;

mod maps;
pub use maps::*;

mod lists;
pub use lists::*;

mod numeric;

pub use numeric::*;

mod supported_crates;