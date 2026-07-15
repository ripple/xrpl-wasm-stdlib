#![doc = include_str!("../README.md")]
#![cfg_attr(target_arch = "wasm32", no_std)]
#![allow(non_upper_case_globals)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

pub mod ctx;
pub mod current_tx;
pub mod data;
pub mod event;
pub mod params;
pub mod sfield;
pub mod sflags;
pub mod submit;

pub use ctx::{ContractCallContext, ContractStorage};
pub use xrpl_parameter_macro::wasm_export;
pub use xrpl_wasm_stdlib::*;
