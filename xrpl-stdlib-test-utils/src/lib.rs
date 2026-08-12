//! Test harness for XRPL WebAssembly smart contracts.
//!
//! `MockHostBindings` (re-exported via [`mock_common`]) is defined inline in
//! `xrpl-common-stdlib` because `mockall::automock` generates it next to the `HostBindings`
//! trait. This crate is the author-facing entry point on top of it: a plain re-export for the
//! raw mock, plus domain-specific scenario builders (see [`mock_escrow`]) that translate
//! escrow facts into mock expectations. Always a dev-dependency; never compiled to WASM.
//!
//! Depend on it gated to non-WASM targets — it pulls in `mockall`, which is not `no_std`:
//!
//! ```toml
//! [target.'cfg(not(target_arch = "wasm32"))'.dev-dependencies]
//! xrpl-stdlib-test-utils = "0.9"
//! ```

pub mod mock_common;
pub mod mock_escrow;

pub use mock_common::*;
pub use mock_escrow::*;
