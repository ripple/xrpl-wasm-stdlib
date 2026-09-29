//! Generates `xrpl-common-stdlib/src/host/host_bindings_trait.rs` and
//! `host_bindings_list.rs` from the `host_functions! { ... }` block in rippled's
//! `crates/xrpl-host-functions/src/lib.rs`. See
//! `docs/superpowers/specs/2026-09-29-host-bindings-generator-design.md`.

pub mod docs;
pub mod emit;
pub mod lower;
pub mod parse;

#[cfg(test)]
pub(crate) mod test_fixtures;

/// Both generated files as strings, pre-`rustfmt`.
pub struct Generated {
    pub trait_rs: String,
    pub list_rs: String,
}

/// Parses rippled's `lib.rs` source and renders both files. `source_label` is recorded in
/// each file's header (the URL or path the run read from).
pub fn generate(source: &str, source_label: &str) -> Result<Generated, String> {
    let fns = parse::parse_host_functions(source)?;
    Ok(Generated {
        trait_rs: emit::emit_trait(&fns, source_label)?,
        list_rs: emit::emit_list(&fns, source_label)?,
    })
}
