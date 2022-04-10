#![no_std]
#![recursion_limit = "1024"]
#![cfg_attr(feature = "doc", feature(doc_cfg))]

#[macro_use]
mod macros;

pub mod cmt;
pub mod icua;
pub mod ports;
pub mod scia;
mod utils;

/// I/O register memory mappings for RX microcontrollers
pub mod devices {
    pub mod rx62n;
    pub mod rx63n;
}
