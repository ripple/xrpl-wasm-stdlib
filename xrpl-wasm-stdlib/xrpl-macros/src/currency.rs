//! `currency!` — compile-time XRPL currency code → `Currency` (20 bytes).
//!
//! Accepts either a 3-char ASCII standard code (e.g. `"USD"`, stored in bytes
//! 12..15) or a 40-char hex non-standard code. `"XRP"` is reserved and rejected.

use proc_macro::TokenStream;
use quote::quote;
use syn::{LitStr, parse_macro_input};

use crate::hex_util::decode_hex;

pub fn expand(input: TokenStream) -> TokenStream {
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

#[cfg(test)]
mod tests {
    use super::decode_currency;

    #[test]
    fn standard_three_char() {
        let bytes = decode_currency("USD").unwrap();
        assert_eq!(bytes.len(), 20);
        assert_eq!(&bytes[12..15], b"USD");
        assert_eq!(&bytes[..12], &[0u8; 12]);
        assert_eq!(&bytes[15..], &[0u8; 5]);
    }

    #[test]
    fn standard_numeric() {
        let bytes = decode_currency("US1").unwrap();
        assert_eq!(&bytes[12..15], b"US1");
    }

    #[test]
    fn rejects_xrp_any_case() {
        assert!(decode_currency("XRP").is_none());
        assert!(decode_currency("xrp").is_none());
        assert!(decode_currency("Xrp").is_none());
    }

    #[test]
    fn rejects_non_alphanumeric() {
        assert!(decode_currency("U$D").is_none());
    }

    #[test]
    fn nonstandard_hex() {
        let key = "0158415500000000C1F76FF6ECB0BAC600000000";
        let bytes = decode_currency(key).unwrap();
        assert_eq!(bytes[0], 0x01);
    }

    #[test]
    fn rejects_nonstandard_zero_prefix() {
        let key = "0000000000000000000000005553440000000000";
        assert!(decode_currency(key).is_none());
    }

    #[test]
    fn rejects_nonstandard_non_hex() {
        let key = "GGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGG";
        assert!(decode_currency(key).is_none());
    }

    #[test]
    fn rejects_wrong_length() {
        assert!(decode_currency("US").is_none());
        assert!(decode_currency("USDT").is_none());
        assert!(decode_currency("01584155").is_none());
    }
}
