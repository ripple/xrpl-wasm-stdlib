//! Entry-point function parsing.
//!
//! Converts a raw [`proc_macro::TokenStream`] into an [`EntryFn`] IR by
//! parsing it as a `syn::ItemFn`. Keeping the parsed form in one place lets
//! [`validate`] and [`codegen`] inspect any part of the function without
//! independently re-parsing.
//!
//! [`validate`]: super::validate
//! [`codegen`]: super::codegen

use proc_macro::TokenStream;
use syn::ItemFn;

/// Typed IR for an entry-point function after parsing.
///
/// Holds the full `ItemFn` so both `validate` and `codegen` can inspect any
/// part of the function without re-parsing.
pub(crate) struct EntryFn {
    pub func: ItemFn,
}

/// Parse `item` as a bare `fn` and wrap it in [`EntryFn`].
///
/// Returns a span-attached error if `item` is not a function.
pub(crate) fn parse_entry_fn(item: TokenStream) -> syn::Result<EntryFn> {
    let func = syn::parse::<ItemFn>(item)?;
    Ok(EntryFn { func })
}
