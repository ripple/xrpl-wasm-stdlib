//! `#[smart_escrow]` attribute macro.

use proc_macro::TokenStream;
use quote::quote;

use super::codegen::{CodegenConfig, emit};
use super::parse::parse_entry_fn;
use super::validate::{ValidationRules, validate};

pub(crate) fn expand(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr2: proc_macro2::TokenStream = attr.into();
    if !attr2.is_empty() {
        return syn::Error::new_spanned(attr2, "#[smart_escrow] takes no arguments")
            .to_compile_error()
            .into();
    }

    let entry = match parse_entry_fn(item) {
        Ok(e) => e,
        Err(e) => return e.to_compile_error().into(),
    };

    let kind = match validate(
        &entry,
        &ValidationRules {
            expected_ctx_type: "EscrowFinishContext",
        },
    ) {
        Ok(k) => k,
        Err(e) => return e.to_compile_error().into(),
    };

    emit(
        &entry,
        &kind,
        &CodegenConfig {
            export_symbol: "finish",
            ctx_path: quote!(::xrpl_escrow_stdlib::EscrowFinishContext),
        },
    )
    .into()
}
