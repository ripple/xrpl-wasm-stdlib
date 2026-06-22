//! Token generation for entry-point macros.

use proc_macro2::TokenStream;

use super::parse::EntryFn;
use super::validate::ReturnKind;

pub(crate) struct CodegenConfig {
    pub export_symbol: &'static str,
    pub ctx_path: TokenStream,
}

pub(crate) fn emit(_entry: &EntryFn, _kind: &ReturnKind, _cfg: &CodegenConfig) -> TokenStream {
    todo!("entry-point code generation")
}
