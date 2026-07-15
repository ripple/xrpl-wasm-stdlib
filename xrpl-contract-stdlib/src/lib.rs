#![doc = include_str!("../README.md")]
#![cfg_attr(target_arch = "wasm32", no_std)]
#![allow(non_upper_case_globals)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

pub mod current_tx;
pub mod params;
pub mod sfield;
pub mod sflags;
pub mod submit;

pub use xrpl_parameter_macro::wasm_export;
pub use xrpl_wasm_stdlib::*;
