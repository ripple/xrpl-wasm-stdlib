#![allow(non_upper_case_globals)]

//! Re-exports the shared transaction flags from `xrpl_wasm_stdlib::sflags` so
//! contract code can access them via `use crate::sflags;`.

pub use xrpl_wasm_stdlib::sflags::*;
