//! Token generation for entry-point macros.

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;

use super::parse::EntryFn;
use super::validate::ReturnKind;

pub(crate) struct CodegenConfig {
    pub export_symbol: &'static str,
    pub ctx_path: TokenStream,
}

pub(crate) fn emit(entry: &EntryFn, kind: &ReturnKind, cfg: &CodegenConfig) -> TokenStream {
    let user_fn = &entry.func;
    let fn_name = &entry.func.sig.ident;
    let wrapper = emit_wrapper(cfg.export_symbol, &cfg.ctx_path, kind, quote!(#fn_name));

    quote! {
        #user_fn

        #wrapper
    }
}

/// Emits just the `extern "C"` export — no user function alongside it.
///
/// `call_path` is the (possibly path-qualified, e.g. `my_mod::my_fn`)
/// expression called with the constructed context. Used by
/// [`super::smart_contract`], where the annotated functions stay nested in
/// their module rather than being re-emitted next to each wrapper.
pub(crate) fn emit_wrapper(
    export_symbol: &str,
    ctx_path: &TokenStream,
    kind: &ReturnKind,
    call_path: TokenStream,
) -> TokenStream {
    let export = Ident::new(export_symbol, Span::call_site());

    let call = match kind {
        ReturnKind::Wrapped => quote! { i32::from(#call_path(ctx)) },
        ReturnKind::I32 => quote! { #call_path(ctx) },
    };

    quote! {
        #[unsafe(no_mangle)]
        pub extern "C" fn #export() -> i32 {
            let ctx = #ctx_path::default();
            #call
        }
    }
}
