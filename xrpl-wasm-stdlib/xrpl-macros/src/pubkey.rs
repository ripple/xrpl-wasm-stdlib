//! `pubkey!` — compile-time 66-hex-char string → `PublicKey`.
//!
//! Only `02`/`03` (secp256k1) and `ED` (Ed25519) prefixes are accepted.

use proc_macro::TokenStream;
use quote::quote;
use syn::{LitStr, parse_macro_input};

use crate::hex_util::decode_hex;

pub fn expand(input: TokenStream) -> TokenStream {
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

#[cfg(test)]
mod tests {
    use super::decode_pubkey;

    const SECP256K1_02: &str = "02C7387FFC25C156CA7F8A6D760C8D01EF642CEE9CE4680C33FFB3FF39AFECFE70";
    const SECP256K1_03: &str = "03C7387FFC25C156CA7F8A6D760C8D01EF642CEE9CE4680C33FFB3FF39AFECFE70";
    const ED25519: &str = "EDD9B3599802B214A99D757712D6ABDF72F83C63BBD53861411790B13D04B2C5C9";

    #[test]
    fn decodes_secp256k1_02_prefix() {
        let bytes = decode_pubkey(SECP256K1_02).unwrap();
        assert_eq!(bytes.len(), 33);
        assert_eq!(bytes[0], 0x02);
    }

    #[test]
    fn decodes_secp256k1_03_prefix() {
        let bytes = decode_pubkey(SECP256K1_03).unwrap();
        assert_eq!(bytes[0], 0x03);
    }

    #[test]
    fn decodes_ed25519_uppercase_prefix() {
        let bytes = decode_pubkey(ED25519).unwrap();
        assert_eq!(bytes[0], 0xED);
    }

    #[test]
    fn decodes_ed25519_lowercase_prefix() {
        let key = "edD9B3599802B214A99D757712D6ABDF72F83C63BBD53861411790B13D04B2C5C9";
        let bytes = decode_pubkey(key).unwrap();
        assert_eq!(bytes[0], 0xED);
    }

    #[test]
    fn rejects_bad_prefix() {
        let key = "04C7387FFC25C156CA7F8A6D760C8D01EF642CEE9CE4680C33FFB3FF39AFECFE70";
        assert!(decode_pubkey(key).is_none());
    }

    #[test]
    fn rejects_too_short() {
        assert!(decode_pubkey("02C7387FFC").is_none());
    }

    #[test]
    fn rejects_too_long() {
        let key = format!("{SECP256K1_02}AA");
        assert!(decode_pubkey(&key).is_none());
    }

    #[test]
    fn rejects_non_hex_chars() {
        let key = "02C7387FFC25C156CA7F8A6D760C8D01EF642CEE9CE4680C33FFB3FF39AFECFEZZ";
        assert!(decode_pubkey(key).is_none());
    }
}
