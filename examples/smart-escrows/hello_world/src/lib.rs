#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_escrow_stdlib::{EscrowFinishContext, FinishResult, smart_escrow};
use xrpl_wasm_stdlib::host::trace::trace;

#[smart_escrow]
fn run(_ctx: EscrowFinishContext) -> FinishResult {
    let _ = trace("Hello World");
    FinishResult::succeed()
}
