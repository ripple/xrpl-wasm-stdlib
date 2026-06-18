//! Procedural-macro entry points for `xrpl-wasm-stdlib`.
//!
//! Each `#[proc_macro]` here is a thin shim that delegates to the matching
//! module's `expand`. Logic, helpers, and unit tests live in the per-macro
//! files. `hex_util` holds helpers shared across the typed-constant macros.

use proc_macro::TokenStream;

mod blob;
mod currency;
mod hash256;
mod hex_util;
mod pubkey;
mod r_address;

#[proc_macro]
pub fn r_address(input: TokenStream) -> TokenStream {
    match r_address::expand(input.into()) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

#[proc_macro]
pub fn hash256(input: TokenStream) -> TokenStream {
    hash256::expand(input)
}

#[proc_macro]
pub fn pubkey(input: TokenStream) -> TokenStream {
    pubkey::expand(input)
}

#[proc_macro]
pub fn currency(input: TokenStream) -> TokenStream {
    currency::expand(input)
}

#[proc_macro]
pub fn blob(input: TokenStream) -> TokenStream {
    blob::expand(input)
}
