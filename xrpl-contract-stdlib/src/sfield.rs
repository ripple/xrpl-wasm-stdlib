#![allow(non_upper_case_globals)]

//! Re-exports the shared SFields from `xrpl_wasm_stdlib::sfield` so contract
//! code can access them via `use crate::sfield;`.

pub use xrpl_wasm_stdlib::sfield::*;
