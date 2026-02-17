//! The `r_address!` macro for compile-time address conversion converts XRPL classic addresses (r-addresses)
//! to 20-byte arrays at compile time.
//!
//! **Important**: The macro only accepts string literals, not runtime values.
//! It runs during compilation and outputs only the final byte array - no base58
//! decoding code is included in the WASM binary.
//!
//! # Example
//! ```shell
//! use xrpl_wasm_stdlib::r_address;
//!
//! // ✅ Works - compile-time literal
//! const ACCOUNT: [u8; 20] = r_address!("rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh");
//!
//! // ❌ Does NOT work - runtime value
//! // fn convert(addr: &str) -> [u8; 20] {
//! //     r_address!(addr)  // ERROR: expected string literal
//! // }
//! ```

use proc_macro::TokenStream;
use quote::quote;
use syn::{LitStr, parse_macro_input};

#[proc_macro]
pub fn r_address(input: TokenStream) -> TokenStream {
    let addr_lit = parse_macro_input!(input as LitStr);
    let addr = addr_lit.value();

    match decode_classic_address_to_20bytes(&addr) {
        Some(bytes) => {
            if bytes.len() != 20 {
                return syn::Error::new(
                    addr_lit.span(),
                    format!("Address decoded to {} bytes, expected 20", bytes.len()),
                )
                .to_compile_error()
                .into();
            }

            let bytes_tokens = bytes.iter().map(|b| quote! { #b });
            let expanded = quote! {
                [#(#bytes_tokens),*]
            };

            TokenStream::from(expanded)
        }
        None => syn::Error::new(addr_lit.span(), format!("Invalid r-address: {addr}"))
            .to_compile_error()
            .into(),
    }
}

fn decode_classic_address_to_20bytes(addr: &str) -> Option<Vec<u8>> {
    if !addr.starts_with('r') {
        return None;
    }
    let alphabet =
        bs58::Alphabet::new(b"rpshnaf39wBUDNEGHJKLM4PQRST7VWXYZ2bcdeCg65jkm8oFqi1tuvAxyz").ok()?;
    let full = bs58::decode(addr)
        .with_alphabet(&alphabet)
        .into_vec()
        .ok()?;
    if full.len() < 1 + 20 + 4 {
        return None;
    }
    // Version byte should be 0x00 for classic AccountID
    if full[0] != 0x00 {
        return None;
    }
    // Split payload and checksum
    let (payload, checksum) = full.split_at(full.len() - 4);
    // Verify checksum: double SHA-256 of payload, take first 4 bytes
    use sha2::{Digest, Sha256};
    let first = Sha256::digest(payload);
    let second = Sha256::digest(first);
    if &second[0..4] != checksum {
        return None;
    }
    // Payload is version (1) + 20 bytes account id
    if payload.len() != 1 + 20 {
        return None;
    }
    Some(payload[1..].to_vec())
}
