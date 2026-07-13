//! Shared parse → validate → codegen pipeline for entry-point attribute macros.
//!
//! [`parse`] converts the annotated `fn` into a typed IR. [`validate`] checks
//! the signature against per-macro rules and classifies the return type.
//! [`codegen`] emits the user's function plus the `extern "C"` XRPL host export.
//!
//! [`smart_escrow`] and [`smart_contract`] are thin orchestrators that wire the
//! three stages together with macro-specific rules and export symbols. Adding a
//! third entry-point macro means adding a new orchestrator file and a new
//! `#[proc_macro_attribute]` shim in `lib.rs` — the pipeline itself is unchanged.

pub(crate) mod codegen;
pub(crate) mod parse;
pub(crate) mod smart_contract;
pub(crate) mod smart_escrow;
pub(crate) mod validate;
