#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_escrow_stdlib::host::trace::trace;

#[unsafe(no_mangle)]
pub extern "C" fn finish() -> i32 {
    let _ = trace("Hello World!");

    1 // <-- Finish the escrow to indicate a successful outcome
}
