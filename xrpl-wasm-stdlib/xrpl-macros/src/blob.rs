//! `blob!` — compile-time hex string → `Blob<N>`.
//!
//! - `blob!("DEADBEEF")` — exact-fit `Blob<N>` where `N` is the byte count.
//! - `blob!("DEADBEEF", 128)` — `Blob<128>` zero-padded after the decoded bytes.
//!   `len` is set to the hex byte count in both forms.

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{LitInt, LitStr, Token, parse_macro_input};

use crate::hex_util::decode_hex;

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

pub fn expand(input: TokenStream) -> TokenStream {
    let BlobInput { hex, capacity } = parse_macro_input!(input as BlobInput);
    let hex_str = hex.value();

    let bytes = match decode_blob_hex(&hex_str) {
        Ok(b) => b,
        Err(reason) => {
            return syn::Error::new(hex.span(), format!("Invalid blob hex: {reason}"))
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

    if let Err(msg) = check_capacity(bytes.len(), n) {
        return syn::Error::new(hex.span(), msg).to_compile_error().into();
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

fn decode_blob_hex(input: &str) -> Result<Vec<u8>, &'static str> {
    if !input.len().is_multiple_of(2) {
        return Err("hex string must have an even number of characters");
    }
    if !input.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err("non-hex character");
    }
    Ok(decode_hex(input))
}

fn check_capacity(bytes_len: usize, capacity: usize) -> Result<(), String> {
    if bytes_len > capacity {
        Err(format!(
            "Blob hex ({bytes_len} bytes) exceeds declared capacity ({capacity} bytes)"
        ))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{check_capacity, decode_blob_hex};

    #[test]
    fn decodes_uppercase() {
        let bytes = decode_blob_hex("DEADBEEF").unwrap();
        assert_eq!(bytes, vec![0xDE, 0xAD, 0xBE, 0xEF]);
    }

    #[test]
    fn decodes_lowercase() {
        let bytes = decode_blob_hex("deadbeef").unwrap();
        assert_eq!(bytes, vec![0xDE, 0xAD, 0xBE, 0xEF]);
    }

    #[test]
    fn decodes_mixed_case() {
        let bytes = decode_blob_hex("DeAdBeEf").unwrap();
        assert_eq!(bytes, vec![0xDE, 0xAD, 0xBE, 0xEF]);
    }

    #[test]
    fn decodes_empty() {
        let bytes = decode_blob_hex("").unwrap();
        assert!(bytes.is_empty());
    }

    #[test]
    fn rejects_odd_length() {
        assert!(decode_blob_hex("ABC").is_err());
    }

    #[test]
    fn rejects_non_hex() {
        assert!(decode_blob_hex("ZZ").is_err());
    }

    #[test]
    fn capacity_check_accepts_exact_fit() {
        assert!(check_capacity(4, 4).is_ok());
    }

    #[test]
    fn capacity_check_accepts_padding() {
        assert!(check_capacity(4, 16).is_ok());
    }

    #[test]
    fn capacity_check_accepts_empty() {
        assert!(check_capacity(0, 0).is_ok());
        assert!(check_capacity(0, 32).is_ok());
    }

    #[test]
    fn capacity_check_rejects_overflow() {
        let err = check_capacity(8, 4).unwrap_err();
        assert!(err.contains("8 bytes"));
        assert!(err.contains("4 bytes"));
    }
}
