//! Test harness for XRPL WebAssembly smart contracts.
//!
//! `MockHostBindings` (re-exported via [`mock_common`]) is defined inline in
//! `xrpl-wasm-stdlib` because `mockall::automock` generates it next to the `HostBindings`
//! trait. This crate is the author-facing entry point on top of it: a plain re-export for the
//! raw mock, plus domain-specific scenario builders (see [`mock_escrow`]) that translate
//! escrow facts into mock expectations. Always a dev-dependency; never compiled to WASM.

pub mod mock_common;
pub mod mock_escrow;

pub use mock_common::*;
pub use mock_escrow::*;
