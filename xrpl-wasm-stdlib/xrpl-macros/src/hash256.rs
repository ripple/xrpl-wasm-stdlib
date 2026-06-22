//! `hash256!` — compile-time 64-hex-char string → 32-byte `Hash256` (`UInt<32>`).

use proc_macro::TokenStream;
use quote::quote;
use syn::{LitStr, parse_macro_input};

use crate::hex_util::decode_hex;

pub fn expand(input: TokenStream) -> TokenStream {
    let hash_lit = parse_macro_input!(input as LitStr);
    let hash = hash_lit.value();
    match decode_hash256(&hash) {
        Ok(bytes) => {
            let bytes_tokens = bytes.iter().map(|b| quote! {#b});
            // `Hash256` is a type alias for `UInt<32>`, which cannot be used as a tuple-struct
            // constructor — so emit the underlying generic struct directly.
            let expanded = quote! {
                ::xrpl_wasm_stdlib::core::types::uint::UInt::<32>([#(#bytes_tokens),*])
            };
            TokenStream::from(expanded)
        }
        Err(reason) => syn::Error::new(hash_lit.span(), format!("Invalid hash: {reason}"))
            .to_compile_error()
            .into(),
    }
}

fn decode_hash256(input: &str) -> Result<Vec<u8>, &'static str> {
    if input.len() != 64 {
        return Err("expected 64 hex characters");
    }
    if !input.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err("non-hex character");
    }
    Ok(decode_hex(input))
}

#[cfg(test)]
mod tests {
    use super::decode_hash256;

    const HASH_BYTES: [u8; 32] = [
        0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD,
        0xEF, 0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x01, 0x23, 0x45, 0x67, 0x89, 0xAB,
        0xCD, 0xEF,
    ];

    #[test]
    fn decodes_uppercase() {
        let bytes =
            decode_hash256("0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF")
                .unwrap();
        assert_eq!(bytes, HASH_BYTES);
    }

    #[test]
    fn decodes_lowercase() {
        let bytes =
            decode_hash256("0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef")
                .unwrap();
        assert_eq!(bytes, HASH_BYTES);
    }

    #[test]
    fn decodes_mixed_case() {
        let bytes =
            decode_hash256("0123456789AbCdEf0123456789aBcDeF0123456789AbCdEf0123456789aBcDeF")
                .unwrap();
        assert_eq!(bytes, HASH_BYTES);
    }

    #[test]
    fn rejects_wrong_length() {
        assert!(decode_hash256("abc").is_err());
    }

    #[test]
    fn rejects_non_hex() {
        assert!(
            decode_hash256("GGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGG")
                .is_err()
        );
    }
}
