//! Compile-time literal macros for XRPL typed values.
//!
//! Each section pairs a `pub fn` macro entry point with the pure-Rust decoder
//! it calls. The decoders take `&str` and return `Option<_>` so they can be
//! unit-tested directly, without going through `proc_macro::TokenStream`.

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{LitInt, LitStr, Token, parse_macro_input};

// ---------------------------------------------------------------------------
// Shared helpers
// ---------------------------------------------------------------------------

/// Decode an even-length hex string into bytes. Caller must verify length and
/// that every char is `is_ascii_hexdigit` before calling.
fn decode_hex(s: &str) -> Vec<u8> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
        .collect()
}

// ---------------------------------------------------------------------------
// hash256!
// ---------------------------------------------------------------------------

pub fn hash256(input: TokenStream) -> TokenStream {
    let hash_lit = parse_macro_input!(input as LitStr);
    let hash = hash_lit.value();
    match decode_hash256(&hash) {
        Some(bytes) => {
            let bytes_tokens = bytes.iter().map(|b| quote! {#b});
            let expanded = quote! {
                ::xrpl_wasm_stdlib::core::types::uint::UInt::<32>([#(#bytes_tokens),*])
            };
            TokenStream::from(expanded)
        }
        None => syn::Error::new(hash_lit.span(), format!("Invalid hash: {hash}"))
            .to_compile_error()
            .into(),
    }
}

fn decode_hash256(input: &str) -> Option<Vec<u8>> {
    if input.len() != 64 || !input.chars().all(|c| c.is_ascii_hexdigit()) {
        return None;
    }
    Some(decode_hex(input))
}

// ---------------------------------------------------------------------------
// pubkey!
// ---------------------------------------------------------------------------

pub fn pubkey(input: TokenStream) -> TokenStream {
    let key_lit = parse_macro_input!(input as LitStr);
    let key = key_lit.value();
    match decode_pubkey(&key) {
        Some(bytes) => {
            let bytes_tokens = bytes.iter().map(|b| quote! {#b});
            let expanded = quote! {
                ::xrpl_wasm_stdlib::core::types::public_key::PublicKey([#(#bytes_tokens),*])
            };
            TokenStream::from(expanded)
        }
        None => syn::Error::new(key_lit.span(), format!("Invalid key: {key}"))
            .to_compile_error()
            .into(),
    }
}

fn decode_pubkey(input: &str) -> Option<Vec<u8>> {
    if input.len() != 66 || !input.chars().all(|c| c.is_ascii_hexdigit()) {
        return None;
    }
    let prefix = input[..2].to_ascii_uppercase();
    if prefix != "02" && prefix != "03" && prefix != "ED" {
        return None;
    }
    Some(decode_hex(input))
}

// ---------------------------------------------------------------------------
// currency!
// ---------------------------------------------------------------------------

pub fn currency(input: TokenStream) -> TokenStream {
    let curr_lit = parse_macro_input!(input as LitStr);
    let curr = curr_lit.value();
    match decode_currency(&curr) {
        Some(bytes) => {
            let bytes_tokens = bytes.iter().map(|b| quote! {#b});
            let expanded = quote! {
                ::xrpl_wasm_stdlib::core::types::currency::Currency([#(#bytes_tokens),*])
            };
            TokenStream::from(expanded)
        }
        None => syn::Error::new(curr_lit.span(), format!("Invalid currency: {curr}"))
            .to_compile_error()
            .into(),
    }
}

fn decode_currency(input: &str) -> Option<[u8; 20]> {
    match input.len() {
        3 => decode_standard_currency(input),
        40 => decode_nonstandard_currency(input),
        _ => None,
    }
}

fn decode_standard_currency(input: &str) -> Option<[u8; 20]> {
    if !input.chars().all(|c| c.is_ascii_alphanumeric()) {
        return None;
    }
    if input.eq_ignore_ascii_case("XRP") {
        return None;
    }
    let mut bytes = [0u8; 20];
    bytes[12..15].copy_from_slice(input.as_bytes());
    Some(bytes)
}

fn decode_nonstandard_currency(input: &str) -> Option<[u8; 20]> {
    if !input.chars().all(|c| c.is_ascii_hexdigit()) {
        return None;
    }
    if input[..2].eq_ignore_ascii_case("00") {
        return None;
    }
    let vec = decode_hex(input);
    let mut bytes = [0u8; 20];
    bytes.copy_from_slice(&vec);
    Some(bytes)
}

// ---------------------------------------------------------------------------
// blob!
// ---------------------------------------------------------------------------

/// `blob!("DEADBEEF")`: exact-fit `Blob<N>` where `N` is the byte count.
/// `blob!("DEADBEEF", 128)`: `Blob<128>` with the hex bytes copied in and the
/// remainder zero-padded. `len` is set to the hex byte count in both forms.
struct BlobInput {
    hex: LitStr,
    capacity: Option<LitInt>,
}

impl Parse for BlobInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let hex: LitStr = input.parse()?;
        let capacity = if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            Some(input.parse::<LitInt>()?)
        } else {
            None
        };
        Ok(Self { hex, capacity })
    }
}

pub fn blob(input: TokenStream) -> TokenStream {
    let BlobInput { hex, capacity } = parse_macro_input!(input as BlobInput);
    let hex_str = hex.value();

    let bytes = match decode_blob_hex(&hex_str) {
        Some(b) => b,
        None => {
            return syn::Error::new(hex.span(), format!("Invalid blob hex: {hex_str}"))
                .to_compile_error()
                .into();
        }
    };

    let n = match &capacity {
        Some(lit) => match lit.base10_parse::<usize>() {
            Ok(v) => v,
            Err(e) => return e.to_compile_error().into(),
        },
        None => bytes.len(),
    };

    if bytes.len() > n {
        return syn::Error::new(
            hex.span(),
            format!(
                "Blob hex ({} bytes) exceeds declared capacity ({} bytes)",
                bytes.len(),
                n
            ),
        )
        .to_compile_error()
        .into();
    }

    let len = bytes.len();
    let mut data = bytes;
    data.resize(n, 0u8);
    let data_tokens = data.iter().map(|b| quote! { #b });

    let expanded = quote! {
        ::xrpl_wasm_stdlib::core::types::blob::Blob::<#n> {
            data: [#(#data_tokens),*],
            len: #len,
        }
    };
    TokenStream::from(expanded)
}

fn decode_blob_hex(input: &str) -> Option<Vec<u8>> {
    if !input.len().is_multiple_of(2) {
        return None;
    }
    if !input.chars().all(|c| c.is_ascii_hexdigit()) {
        return None;
    }
    Some(decode_hex(input))
}
