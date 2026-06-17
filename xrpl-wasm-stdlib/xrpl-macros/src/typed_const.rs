use proc_macro::TokenStream;
use quote::quote;
use syn::{LitStr, parse_macro_input};

pub fn hash256(input: TokenStream) -> TokenStream {
    let hash_lit = parse_macro_input!(input as LitStr);
    let hash = hash_lit.value();
    match decode_hash256(&hash) {
        Some(bytes) => {
            let bytes_tokens = bytes.iter().map(|b| quote! {#b});
            let expanded = quote! {
                ::xrpl_wasm_stdlib::core::types::uint::Hash256([#(#bytes_tokens),*])
            };
            TokenStream::from(expanded)
        }
        None => syn::Error::new(hash_lit.span(), format!("Invalid hash: {hash}"))
            .to_compile_error()
            .into(),
    }
}

fn decode_hex(s: &str) -> Vec<u8> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
        .collect()
}

fn decode_hash256(input: &str) -> Option<Vec<u8>> {
    if input.len() != 64 || !input.chars().all(|c| c.is_ascii_hexdigit()) {
        return None;
    }
    Some(decode_hex(input))
}
