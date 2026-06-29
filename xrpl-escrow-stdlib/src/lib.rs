#![doc = include_str!("../README.md")]
#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

pub use xrpl_wasm_stdlib::*;
