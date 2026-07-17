//! Re-exports of the mock host-bindings machinery that lives inline in `xrpl-wasm-stdlib`.
//!
//! `mockall::automock` generates `MockHostBindings` right next to the `HostBindings` trait
//! definition, so the type itself can't live in this crate. What lives here instead is the
//! author-facing entry point: import from `xrpl_stdlib_test_utils` instead of reaching into
//! `xrpl_wasm_stdlib::host::*` directly.

pub use xrpl_wasm_stdlib::host::host_bindings_trait::{HostBindings, MockHostBindings};
pub use xrpl_wasm_stdlib::host::{
    MockGuard, apply_default_expectations, create_default_mock, setup_mock,
};
