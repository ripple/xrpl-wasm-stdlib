//! `pubkey!` — compile-time 66-hex-char string → 33-byte `PublicKey`.
//!
//! Only `02`/`03` (secp256k1) and `ED` (Ed25519) prefixes are accepted. The
//! prefix check is case-insensitive — `ed` is normalised to `0xED`.

use proc_macro::TokenStream;
use quote::quote;
use syn::{LitStr, parse_macro_input};

use crate::hex_util::decode_hex;

pub fn expand(input: TokenStream) -> TokenStream {
    let key_lit = parse_macro_input!(input as LitStr);
    let key = key_lit.value();
    match decode_pubkey(&key) {
        Ok(bytes) => {
            let bytes_tokens = bytes.iter().map(|b| quote! {#b});
            let expanded = quote! {
                ::xrpl_wasm_stdlib::core::types::public_key::PublicKey([#(#bytes_tokens),*])
            };
            TokenStream::from(expanded)
        }
        Err(reason) => syn::Error::new(key_lit.span(), format!("Invalid key: {reason}"))
            .to_compile_error()
            .into(),
    }
}

fn decode_pubkey(input: &str) -> Result<Vec<u8>, &'static str> {
    if input.len() != 66 {
        return Err("expected 66 hex characters");
    }
    if !input.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err("non-hex character");
    }
    let prefix = input[..2].to_ascii_uppercase();
    if prefix != "02" && prefix != "03" && prefix != "ED" {
        return Err("invalid prefix: expected 02, 03, or ED");
    }
    Ok(decode_hex(input))
}

#[cfg(test)]
mod tests {
    use super::decode_pubkey;

    const SECP256K1_02: &str = "02C7387FFC25C156CA7F8A6D760C8D01EF642CEE9CE4680C33FFB3FF39AFECFE70";
    const SECP256K1_03: &str = "03C7387FFC25C156CA7F8A6D760C8D01EF642CEE9CE4680C33FFB3FF39AFECFE70";
    const ED25519: &str = "EDD9B3599802B214A99D757712D6ABDF72F83C63BBD53861411790B13D04B2C5C9";

    const SECP256K1_02_BYTES: [u8; 33] = [
        0x02, 0xC7, 0x38, 0x7F, 0xFC, 0x25, 0xC1, 0x56, 0xCA, 0x7F, 0x8A, 0x6D, 0x76, 0x0C, 0x8D,
        0x01, 0xEF, 0x64, 0x2C, 0xEE, 0x9C, 0xE4, 0x68, 0x0C, 0x33, 0xFF, 0xB3, 0xFF, 0x39, 0xAF,
        0xEC, 0xFE, 0x70,
    ];
    const SECP256K1_03_BYTES: [u8; 33] = [
        0x03, 0xC7, 0x38, 0x7F, 0xFC, 0x25, 0xC1, 0x56, 0xCA, 0x7F, 0x8A, 0x6D, 0x76, 0x0C, 0x8D,
        0x01, 0xEF, 0x64, 0x2C, 0xEE, 0x9C, 0xE4, 0x68, 0x0C, 0x33, 0xFF, 0xB3, 0xFF, 0x39, 0xAF,
        0xEC, 0xFE, 0x70,
    ];
    const ED25519_BYTES: [u8; 33] = [
        0xED, 0xD9, 0xB3, 0x59, 0x98, 0x02, 0xB2, 0x14, 0xA9, 0x9D, 0x75, 0x77, 0x12, 0xD6, 0xAB,
        0xDF, 0x72, 0xF8, 0x3C, 0x63, 0xBB, 0xD5, 0x38, 0x61, 0x41, 0x17, 0x90, 0xB1, 0x3D, 0x04,
        0xB2, 0xC5, 0xC9,
    ];

    #[test]
    fn decodes_secp256k1_02_prefix() {
        let bytes = decode_pubkey(SECP256K1_02).unwrap();
        assert_eq!(bytes, SECP256K1_02_BYTES);
    }

    #[test]
    fn decodes_secp256k1_03_prefix() {
        let bytes = decode_pubkey(SECP256K1_03).unwrap();
        assert_eq!(bytes, SECP256K1_03_BYTES);
    }

    #[test]
    fn decodes_ed25519_uppercase_prefix() {
        let bytes = decode_pubkey(ED25519).unwrap();
        assert_eq!(bytes, ED25519_BYTES);
    }

    #[test]
    fn decodes_ed25519_lowercase_prefix() {
        // Lowercase 'ed' prefix must normalise to 0xED after decode.
        let key = "edD9B3599802B214A99D757712D6ABDF72F83C63BBD53861411790B13D04B2C5C9";
        let bytes = decode_pubkey(key).unwrap();
        assert_eq!(bytes, ED25519_BYTES);
    }

    #[test]
    fn rejects_bad_prefix() {
        let key = "04C7387FFC25C156CA7F8A6D760C8D01EF642CEE9CE4680C33FFB3FF39AFECFE70";
        assert!(decode_pubkey(key).is_err());
    }

    #[test]
    fn rejects_too_short() {
        assert!(decode_pubkey("02C7387FFC").is_err());
    }

    #[test]
    fn rejects_too_long() {
        let key = format!("{SECP256K1_02}AA");
        assert!(decode_pubkey(&key).is_err());
    }

    #[test]
    fn rejects_non_hex_chars() {
        let key = "02C7387FFC25C156CA7F8A6D760C8D01EF642CEE9CE4680C33FFB3FF39AFECFEZZ";
        assert!(decode_pubkey(key).is_err());
    }
}
