#![doc = include_str!("../README.md")]
#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

#[cfg(target_arch = "wasm32")]
const VERSION_STR: &str = env!("CARGO_PKG_VERSION");

#[cfg(target_arch = "wasm32")]
#[used]
#[unsafe(link_section = "xrpl-escrow-stdlib-version")]
static VERSION_METADATA: [u8; VERSION_STR.len()] =
    *unsafe { &*VERSION_STR.as_ptr().cast::<[u8; VERSION_STR.len()]>() };

pub mod ctx;
pub mod current_tx;
pub mod ledger_objects;

pub use ctx::escrow_finish::EscrowFinishContext;
pub use ctx::finish_result::FinishResult;

pub use xrpl_common_stdlib::*;
