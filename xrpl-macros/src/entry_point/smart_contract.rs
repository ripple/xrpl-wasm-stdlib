//! `#[smart_contract]` attribute macro.
//!
//! Unlike `#[smart_escrow]`, this macro is module-level: it scans the inner
//! items of the annotated `mod` for functions tagged `#[init]`, `#[call]`,
//! `#[user_delete]`, or `#[clawback]` and emits one named `extern "C"` export
//! per tagged function. Lifecycle attributes (`init`, `user_delete`,
//! `clawback`) always export under their fixed name; `#[call]` exports under
//! the Rust function identifier by default, or under an explicit
//! `#[call(name = "...")]` override. At most one `#[init]` function is
//! allowed, and every export symbol in a module must be unique.

use std::collections::HashMap;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Ident, Item, ItemFn, ItemMod, Lit, Meta, MetaNameValue, parse_quote};

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
    /// which exports under the Rust function identifier (or its `name`
    /// override) instead.
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
    let mut warnings: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut init_seen = false;
    let mut seen_symbols: HashMap<String, proc_macro2::Span> = HashMap::new();

    for item in items {
        let mut func = match item {
            Item::Fn(f) => f,
            other => {
                new_items.push(other);
                continue;
            }
        };

        let (kind, call_name) = match take_entry_attr(&mut func) {
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

        let fn_ident = func.sig.ident.clone();

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

        let is_defaulted_call_name = matches!(kind, EntryKind::Call) && call_name.is_none();
        let export_symbol = kind
            .fixed_export_symbol()
            .map(str::to_string)
            .or(call_name)
            .unwrap_or_else(|| fn_ident.to_string());

        if let Some(prev_span) = seen_symbols.insert(export_symbol.clone(), fn_ident.span()) {
            let mut err = syn::Error::new(
                fn_ident.span(),
                format!(
                    "export symbol \"{export_symbol}\" is already used by another entry-point function in this module"
                ),
            );
            err.combine(syn::Error::new(
                prev_span,
                "previous use of this export symbol",
            ));
            return err.to_compile_error().into();
        }

        if is_defaulted_call_name {
            warnings.push(emit_default_name_warning(&mod_ident, &fn_ident));
        }

        let call_path = {
            let fn_name = &fn_ident;
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
        #(#warnings)*
    }
    .into()
}

/// Finds and removes the single `#[init]`/`#[call]`/`#[user_delete]`/`#[clawback]`
/// attribute on `func`, if any. Returns the matched kind together with the
/// `#[call(name = "...")]` override, if present (always `None` for lifecycle
/// attributes, which take no arguments).
///
/// Errors if more than one entry-point attribute is present, if a lifecycle
/// attribute carries arguments, or if `#[call]`'s arguments aren't a single
/// well-formed `name = "..."`.
fn take_entry_attr(func: &mut ItemFn) -> syn::Result<Option<(EntryKind, Option<String>)>> {
    let mut found: Option<usize> = None;

    for (i, attr) in func.attrs.iter().enumerate() {
        let Some(name) = attr.path().get_ident().map(Ident::to_string) else {
            continue;
        };
        if EntryKind::from_ident(&name).is_none() {
            continue;
        }

        if found.is_some() {
            return Err(syn::Error::new_spanned(
                attr,
                "function can have at most one entry-point attribute",
            ));
        }
        found = Some(i);
    }

    let Some(idx) = found else {
        return Ok(None);
    };

    let attr = func.attrs.remove(idx);
    let ident_name = attr.path().get_ident().unwrap().to_string();
    let kind = EntryKind::from_ident(&ident_name).unwrap();

    let call_name = match kind {
        EntryKind::Call => parse_call_name(&attr)?,
        _ => {
            if !matches!(attr.meta, Meta::Path(_)) {
                return Err(syn::Error::new_spanned(
                    &attr,
                    format!("#[{ident_name}] takes no arguments"),
                ));
            }
            None
        }
    };

    Ok(Some((kind, call_name)))
}

/// Parses `#[call]`'s optional argument. Accepts a bare `#[call]` (`None`) or
/// `#[call(name = "...")]` (`Some(name)`), where `name` must be a non-empty
/// valid Rust identifier — it's spliced directly into the generated
/// `extern "C" fn` export.
fn parse_call_name(attr: &syn::Attribute) -> syn::Result<Option<String>> {
    match &attr.meta {
        Meta::Path(_) => Ok(None),
        Meta::List(list) => {
            let nv: MetaNameValue = syn::parse2(list.tokens.clone()).map_err(|_| {
                syn::Error::new_spanned(list, "#[call] only accepts `name = \"...\"`")
            })?;
            if !nv.path.is_ident("name") {
                return Err(syn::Error::new_spanned(
                    &nv.path,
                    "#[call] only accepts `name = \"...\"`",
                ));
            }
            let syn::Expr::Lit(syn::ExprLit {
                lit: Lit::Str(s), ..
            }) = &nv.value
            else {
                return Err(syn::Error::new_spanned(
                    &nv.value,
                    "#[call(name = \"...\")] value must be a string literal",
                ));
            };
            let name = s.value();
            if name.is_empty() {
                return Err(syn::Error::new_spanned(
                    s,
                    "#[call(name = \"...\")] name must not be empty",
                ));
            }
            if syn::parse_str::<Ident>(&name).is_err() {
                return Err(syn::Error::new_spanned(
                    s,
                    "#[call(name = \"...\")] name must be a valid identifier",
                ));
            }
            Ok(Some(name))
        }
        Meta::NameValue(_) => Err(syn::Error::new_spanned(
            attr,
            "#[call] takes no arguments, or `#[call(name = \"...\")]`",
        )),
    }
}

/// Emits a non-fatal, `#[deprecated]`-based diagnostic recommending an
/// explicit `#[call(name = "...")]` in place of a defaulted export symbol.
/// Real `Diagnostic::warning` requires nightly, so this drives the same
/// `deprecated` lint indirectly: a private const marked `#[deprecated]` is
/// referenced once, which rustc reports as a warning (or, under this repo's
/// `RUSTFLAGS=-Dwarnings`, an error) at the reference site.
fn emit_default_name_warning(mod_ident: &Ident, fn_ident: &Ident) -> proc_macro2::TokenStream {
    let marker = format_ident!("__xrpl_call_default_name_warn_{mod_ident}_{fn_ident}");
    let note = format!(
        "`#[call]` on `{fn_ident}` has no explicit name; exporting as \"{fn_ident}\". \
         Add #[call(name = \"...\")] to pin the on-chain export symbol independently \
         of the Rust function name."
    );
    quote! {
        #[deprecated(note = #note)]
        #[allow(non_upper_case_globals)]
        const #marker: () = ();
        #[doc(hidden)]
        const _: () = #marker;
    }
}
