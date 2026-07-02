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
    let export = Ident::new(cfg.export_symbol, Span::call_site());
    let ctx = &cfg.ctx_path;

    let call = match kind {
        ReturnKind::FinishResult => quote! { i32::from(#fn_name) },
        ReturnKind::I32 => quote! { #fn_name(ctx) },
    };

    quote! {
        #user_fn

        #[no_mangle]
        pub extern "C" fn #export() -> i32 {
            let ctx = #ctx::default();
            #call
        }
    }
}
