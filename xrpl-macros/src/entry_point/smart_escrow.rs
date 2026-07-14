//! `#[smart_escrow]` attribute macro — scaffold.

use proc_macro::TokenStream;

pub(crate) fn expand(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
