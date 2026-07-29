//! `#[smart_contract]` attribute macro.
//!
//! Unlike `#[smart_escrow]`, this macro is module-level: it scans the inner
//! items of the annotated `mod` for functions tagged `#[init]`, `#[call]`,
//! `#[user_delete]`, or `#[clawback]` and emits one named `extern "C"` export
//! per tagged function. Lifecycle attributes (`init`, `user_delete`,
//! `clawback`) always export under their fixed name; `#[call]` exports under
//! the Rust function identifier. At most one `#[init]` function is allowed.

use proc_macro::TokenStream;
use quote::quote;
use syn::{Ident, Item, ItemFn, ItemMod, parse_quote};

use super::codegen::emit_wrapper;
use super::parse::EntryFn;
use super::validate::{ValidationRules, validate};

const CTX_TYPE: &str = "ContractCallContext";

enum EntryKind {
    Init,
    Call,
    UserDelete,
    Clawback,
}

impl EntryKind {
    fn from_ident(ident: &str) -> Option<Self> {
        match ident {
            "init" => Some(Self::Init),
            "call" => Some(Self::Call),
            "user_delete" => Some(Self::UserDelete),
            "clawback" => Some(Self::Clawback),
            _ => None,
        }
    }

    /// Fixed export symbol for lifecycle attributes; `None` for `#[call]`,
    /// which exports under the Rust function identifier instead.
    fn fixed_export_symbol(&self) -> Option<&'static str> {
        match self {
            Self::Init => Some("init"),
            Self::UserDelete => Some("user_delete"),
            Self::Clawback => Some("clawback"),
            Self::Call => None,
        }
    }
}

pub(crate) fn expand(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr2: proc_macro2::TokenStream = attr.into();
    if !attr2.is_empty() {
        return syn::Error::new_spanned(attr2, "#[smart_contract] takes no arguments")
            .to_compile_error()
            .into();
    }

    let mut module = match syn::parse::<ItemMod>(item) {
        Ok(m) => m,
        Err(e) => return e.to_compile_error().into(),
    };

    let Some((brace, items)) = module.content.take() else {
        return syn::Error::new_spanned(
            &module,
            "#[smart_contract] module must have an inline body (`mod name { .. }`, not `mod name;`)",
        )
        .to_compile_error()
        .into();
    };

    let mod_ident = module.ident.clone();
    let mut new_items = Vec::with_capacity(items.len());
    let mut exports = Vec::new();
    let mut init_seen = false;

    for item in items {
        let mut func = match item {
            Item::Fn(f) => f,
            other => {
                new_items.push(other);
                continue;
            }
        };

        let kind = match take_entry_attr(&mut func) {
            Ok(Some(k)) => k,
            Ok(None) => {
                new_items.push(Item::Fn(func));
                continue;
            }
            Err(e) => return e.to_compile_error().into(),
        };

        if matches!(kind, EntryKind::Init) {
            if init_seen {
                return syn::Error::new(
                    func.sig.ident.span(),
                    "at most one #[init] function is allowed",
                )
                .to_compile_error()
                .into();
            }
            init_seen = true;
        }

        let entry = EntryFn { func };
        let return_kind = match validate(
            &entry,
            &ValidationRules {
                expected_ctx_type: CTX_TYPE,
                wrapped_return_type: None,
            },
        ) {
            Ok(k) => k,
            Err(e) => return e.to_compile_error().into(),
        };

        let export_symbol = kind
            .fixed_export_symbol()
            .map(str::to_string)
            .unwrap_or_else(|| entry.func.sig.ident.to_string());

        let call_path = {
            let fn_name = &entry.func.sig.ident;
            quote!(#mod_ident::#fn_name)
        };
        exports.push(emit_wrapper(
            &export_symbol,
            &quote!(::xrpl_contract_stdlib::ContractCallContext),
            &return_kind,
            call_path,
        ));

        let mut func = entry.func;
        func.vis = parse_quote!(pub(super));
        new_items.push(Item::Fn(func));
    }

    module.content = Some((brace, new_items));

    quote! {
        #module

        #(#exports)*
    }
    .into()
}

/// Finds and removes the single `#[init]`/`#[call]`/`#[user_delete]`/`#[clawback]`
/// attribute on `func`, if any. Errors if more than one is present, or if the
/// matched attribute carries arguments (none of these sub-attributes accept any yet).
fn take_entry_attr(func: &mut ItemFn) -> syn::Result<Option<EntryKind>> {
    let mut found: Option<usize> = None;

    for (i, attr) in func.attrs.iter().enumerate() {
        let Some(name) = attr.path().get_ident().map(Ident::to_string) else {
            continue;
        };
        if EntryKind::from_ident(&name).is_none() {
            continue;
        }

        if !matches!(attr.meta, syn::Meta::Path(_)) {
            return Err(syn::Error::new_spanned(
                attr,
                format!("#[{name}] takes no arguments"),
            ));
        }

        if found.is_some() {
            return Err(syn::Error::new_spanned(
                attr,
                "function can have at most one entry-point attribute",
            ));
        }
        found = Some(i);
    }

    match found {
        Some(idx) => {
            let attr = func.attrs.remove(idx);
            let name = attr.path().get_ident().unwrap().to_string();
            Ok(Some(EntryKind::from_ident(&name).unwrap()))
        }
        None => Ok(None),
    }
}
