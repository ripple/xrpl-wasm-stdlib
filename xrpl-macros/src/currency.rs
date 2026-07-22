//! `currency!` — compile-time XRPL currency code → 20-byte `Currency`.
//!
//! Two forms:
//! - **Standard (3 ASCII alphanumeric chars)**: stored verbatim in bytes 12–14;
//!   bytes 0–11 and 15–19 are zero. `"XRP"` is reserved and rejected.
//! - **Non-standard (40 hex chars)**: interpreted as a raw 20-byte value; must
//!   not start with `00`.
//!
//! Standard codes are case-sensitive — `"USD"` and `"usd"` are distinct
//! on-ledger identifiers. Use uppercase by convention.

use proc_macro2::TokenStream;
use quote::quote;
use syn::LitStr;

use crate::hex_util::decode_hex;

pub fn expand(input: TokenStream) -> syn::Result<TokenStream> {
    let curr_lit = syn::parse2::<LitStr>(input)?;
    let curr = curr_lit.value();

    let bytes = decode_currency(&curr).map_err(|reason| {
        syn::Error::new(curr_lit.span(), format!("Invalid currency: {reason}"))
    })?;

    let bytes_tokens = bytes.iter().map(|b| quote! {#b});
    let expanded = quote! {
        ::xrpl_common_stdlib::types::currency::Currency([#(#bytes_tokens),*])
    };
    Ok(expanded)
}

fn decode_currency(input: &str) -> Result<[u8; 20], &'static str> {
    match input.len() {
        3 => decode_standard_currency(input),
        40 => decode_nonstandard_currency(input),
        _ => Err("expected a 3-char standard code or 40-char hex non-standard code"),
    }
}

fn decode_standard_currency(input: &str) -> Result<[u8; 20], &'static str> {
    if !input.chars().all(|c| c.is_ascii_alphanumeric()) {
        return Err("standard currency must be ASCII alphanumeric");
    }
    if input.eq_ignore_ascii_case("XRP") {
        return Err("XRP is a reserved currency code");
    }
    let mut bytes = [0u8; 20];
    bytes[12..15].copy_from_slice(input.as_bytes());
    Ok(bytes)
}

fn decode_nonstandard_currency(input: &str) -> Result<[u8; 20], &'static str> {
    if !input.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err("non-standard currency must be valid hex");
    }
    if input[..2].eq_ignore_ascii_case("00") {
        return Err("non-standard currency must not start with 00");
    }
    let vec = decode_hex(input);
    let mut bytes = [0u8; 20];
    bytes.copy_from_slice(&vec);
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::{decode_currency, expand};
    use quote::quote;

    const NONSTANDARD_KEY: &str = "0158415500000000C1F76FF6ECB0BAC600000000";
    const NONSTANDARD_BYTES: [u8; 20] = [
        0x01, 0x58, 0x41, 0x55, 0x00, 0x00, 0x00, 0x00, 0xC1, 0xF7, 0x6F, 0xF6, 0xEC, 0xB0, 0xBA,
        0xC6, 0x00, 0x00, 0x00, 0x00,
    ];

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
        assert_eq!(
            decode_currency("XRP").unwrap_err(),
            "XRP is a reserved currency code"
        );
        assert_eq!(
            decode_currency("xrp").unwrap_err(),
            "XRP is a reserved currency code"
        );
        assert_eq!(
            decode_currency("Xrp").unwrap_err(),
            "XRP is a reserved currency code"
        );
    }

    #[test]
    fn rejects_non_alphanumeric() {
        let err = decode_currency("U$D").unwrap_err();
        assert_eq!(err, "standard currency must be ASCII alphanumeric");
    }

    #[test]
    fn nonstandard_hex() {
        let bytes = decode_currency(NONSTANDARD_KEY).unwrap();
        assert_eq!(bytes, NONSTANDARD_BYTES);
    }

    #[test]
    fn rejects_nonstandard_zero_prefix() {
        let key = "0000000000000000000000005553440000000000";
        let err = decode_currency(key).unwrap_err();
        assert_eq!(err, "non-standard currency must not start with 00");
    }

    #[test]
    fn rejects_nonstandard_non_hex() {
        let key = "GGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGG";
        let err = decode_currency(key).unwrap_err();
        assert_eq!(err, "non-standard currency must be valid hex");
    }

    #[test]
    fn rejects_wrong_length() {
        let expected = "expected a 3-char standard code or 40-char hex non-standard code";
        assert_eq!(decode_currency("US").unwrap_err(), expected);
        assert_eq!(decode_currency("USDT").unwrap_err(), expected);
        assert_eq!(decode_currency("01584155").unwrap_err(), expected);
    }

    #[test]
    fn lowercase_standard_code_stored_as_is() {
        // XRPL 3-char codes are stored as raw ASCII bytes, so "usd" and "USD"
        // are distinct currency identifiers on-ledger. This test documents that
        // the macro does NOT normalise case — callers should use uppercase.
        let lower = decode_currency("usd").unwrap();
        let upper = decode_currency("USD").unwrap();
        assert_ne!(lower, upper);
        assert_eq!(&lower[12..15], b"usd");
        assert_eq!(&upper[12..15], b"USD");
    }

    #[test]
    fn expand_emits_tokens_for_standard_code() {
        let input = quote! { "USD" };
        assert!(expand(input).is_ok());
    }

    #[test]
    fn expand_emits_tokens_for_nonstandard_code() {
        let input = quote! { "0158415500000000C1F76FF6ECB0BAC600000000" };
        assert!(expand(input).is_ok());
    }

    #[test]
    fn expand_errors_on_reserved_xrp() {
        let input = quote! { "XRP" };
        let err = expand(input).unwrap_err();
        assert!(err.to_string().contains("reserved"));
    }

    #[test]
    fn expand_errors_on_wrong_length() {
        let input = quote! { "US" };
        let err = expand(input).unwrap_err();
        assert!(err.to_string().contains("3-char standard code"));
    }
}
