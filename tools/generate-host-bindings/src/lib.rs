//! Generates `xrpl-common-stdlib/src/host/host_bindings_trait.rs` and
//! `host_bindings_list.rs` from the `host_functions! { ... }` block in rippled's
//! `crates/xrpl-host-functions/src/lib.rs`. See
//! `docs/superpowers/specs/2026-09-29-host-bindings-generator-design.md`.

pub mod parse;

#[cfg(test)]
pub(crate) mod test_fixtures;
