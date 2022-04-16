#![feature(asm_experimental_arch)]
#![feature(const_ptr_offset_from)]
#![feature(generic_const_exprs)]
#![feature(const_refs_to_cell)]
#![feature(const_trait_impl)]
#![feature(naked_functions)]
#![feature(const_mut_refs)]
#![feature(slice_ptr_len)]
#![feature(decl_macro)]
#![feature(asm_const)]
#![feature(fn_align)]
#![feature(asm_sym)]
#![deny(unsafe_op_in_unsafe_fn)]
#![cfg_attr(
    feature = "doc",
    doc(html_logo_url = "https://r3-os.github.io/r3/logo-small.svg")
)]
#![doc = include_str!("./lib.md")]
#![no_std]

#[cfg(doc)]
#[doc = include_str!("../CHANGELOG.md")]
pub mod _changelog_ {}

/// The [`r3_kernel::PortThreading`] implementation.
#[doc(hidden)]
pub mod threading {
    pub mod cfg;
    #[cfg(target_os = "none")]
    pub mod imp;
}

// TODO: Startup
// /// The standard startup code.
// #[doc(hidden)]
// pub mod startup {
//     pub mod cfg;
//     #[cfg(target_os = "none")]
//     pub mod imp;
// }

// TODO: Timer implementation
// /// The tickful [`r3_kernel::PortTimer`] implementation based on SysTick.
// #[doc(hidden)]
// pub mod systick_tickful {
//     pub mod cfg;
//     #[cfg(target_os = "none")]
//     pub mod imp;
// }

pub use self::threading::cfg::*;

/// Used by `use_port!`
#[doc(hidden)]
#[cfg(target_os = "none")]
pub extern crate core;
/// Used by `use_port!`
#[doc(hidden)]
pub extern crate r3_core;
/// Used by `use_port!`
#[doc(hidden)]
pub extern crate r3_kernel;
