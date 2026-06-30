//! # xrpl-escrow-stdlib
//!
//! Smart Escrow-specific types and field accessors for XRPL WebAssembly contracts.
//!
//! Generic XRPL primitives (AccountID, Locator, host bindings, trace, etc.) live in
//! [`xrpl_wasm_stdlib`]. This crate hosts only the types tied specifically to escrows:
//! the `Escrow`/`CurrentEscrow` ledger objects, the `EscrowFinish` transaction wrapper,
//! and the escrow-specific field accessor traits.
//!
//! Authors typically import named items from both crates:
//!
//! ```ignore
//! use xrpl_wasm_stdlib::core::types::account_id::AccountID;
//! use xrpl_wasm_stdlib::host::trace::trace_num;
//! use xrpl_escrow_stdlib::current_tx::escrow_finish::{self, EscrowFinish};
//! use xrpl_escrow_stdlib::ledger_objects::traits::CurrentEscrowFields;
//! ```

#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

pub mod ctx;
pub mod current_tx;
pub mod ledger_objects;

pub use ctx::escrow_finish::EscrowFinishContext;
