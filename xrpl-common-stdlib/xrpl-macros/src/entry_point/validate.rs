//! Signature validation for entry-point macros.
#![allow(dead_code)]

use super::parse::EntryFn;

pub(crate) struct ValidationRules<'a> {
    pub expected_ctx_type: &'a str,
}

pub(crate) enum ReturnKind {
    FinishResult,
    I32,
}

pub(crate) fn validate(_entry: &EntryFn, _rules: &ValidationRules<'_>) -> syn::Result<ReturnKind> {
    todo!("entry-point signature validation")
}
