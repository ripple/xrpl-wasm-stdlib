//! `hash256!` — compile-time 64-hex-char string → 32-byte `Hash256` (`UInt<32>`).

use proc_macro2::TokenStream;
use quote::quote;
use syn::LitStr;

use crate::hex_util::decode_hex;

pub fn expand(input: TokenStream) -> syn::Result<TokenStream> {
    let hash_lit = syn::parse2::<LitStr>(input)?;
    let hash = hash_lit.value();

    let bytes = decode_hash256(&hash)
        .map_err(|reason| syn::Error::new(hash_lit.span(), format!("Invalid hash: {reason}")))?;

    let bytes_tokens = bytes.iter().map(|b| quote! {#b});
    // `Hash256` is a type alias for `UInt<32>`, which cannot be used as a tuple-struct
    // constructor — so emit the underlying generic struct directly.
    let expanded = quote! {
<<<<<<<< HEAD:xrpl-macros/src/hash256.rs
        ::xrpl_common_stdlib::types::uint::UInt::<32>([#(#bytes_tokens),*])
========
        ::xrpl_common_stdlib::core::types::uint::UInt::<32>([#(#bytes_tokens),*])
>>>>>>>> 38f2382 (renames, import fixes):xrpl-common-stdlib/xrpl-macros/src/hash256.rs
    };
    Ok(expanded)
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
    use super::{decode_hash256, expand};
    use quote::quote;

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
        let err = decode_hash256("abc").unwrap_err();
        assert_eq!(err, "expected 64 hex characters");
    }

    #[test]
    fn rejects_non_hex() {
        let err =
            decode_hash256("GGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGG")
                .unwrap_err();
        assert_eq!(err, "non-hex character");
    }

    #[test]
    fn expand_emits_tokens_for_valid_hash() {
        let input = quote! { "0000000000000000000000000000000000000000000000000000000000000001" };
        assert!(expand(input).is_ok());
    }

    #[test]
    fn expand_errors_on_wrong_length() {
        let input = quote! { "abc" };
        let err = expand(input).unwrap_err();
        assert!(err.to_string().contains("expected 64 hex characters"));
    }

    #[test]
    fn expand_errors_on_non_hex() {
        let input = quote! { "GGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGG" };
        let err = expand(input).unwrap_err();
        assert!(err.to_string().contains("non-hex character"));
    }
}
