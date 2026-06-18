//! The `r_address!` macro for compile-time address conversion converts XRPL classic addresses (r-addresses)
//! to an [`AccountID`] at compile time.
//!
//! **Important**: The macro only accepts string literals, not runtime values.
//! It runs during compilation and outputs only the final `AccountID` value - no
//! base58 decoding code is included in the WASM binary.
//!
//! # Example
//! ```shell
//! use xrpl_wasm_stdlib::r_address;
//! use xrpl_wasm_stdlib::core::types::account_id::AccountID;
//!
//! // ✅ Works - compile-time literal
//! const ACCOUNT: AccountID = r_address!("rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh");
//!
//! // ❌ Does NOT work - runtime value
//! // fn convert(addr: &str) -> AccountID {
//! //     r_address!(addr)  // ERROR: expected string literal
//! // }
//! ```
use proc_macro::TokenStream;
use quote::quote;
use syn::{LitStr, parse_macro_input};

pub fn expand(input: TokenStream) -> TokenStream {
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
                ::xrpl_wasm_stdlib::core::types::account_id::AccountID([#(#bytes_tokens),*])
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

#[cfg(test)]
mod tests {
    use super::decode_classic_address_to_20bytes;

    const VALID_ADDR: &str = "rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh";
    const VALID_BYTES: [u8; 20] = [
        0xb5, 0xf7, 0x62, 0x79, 0x8a, 0x53, 0xd5, 0x43, 0xa0, 0x14, 0xca, 0xf8, 0xb2, 0x97, 0xcf,
        0xf8, 0xf2, 0xf9, 0x37, 0xe8,
    ];

    #[test]
    fn decodes_known_address() {
        let bytes = decode_classic_address_to_20bytes(VALID_ADDR).unwrap();
        assert_eq!(bytes.as_slice(), VALID_BYTES);
    }

    #[test]
    fn rejects_missing_r_prefix() {
        // Same payload, leading 'r' swapped for 'x'.
        assert!(decode_classic_address_to_20bytes("xHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh").is_none());
    }

    #[test]
    fn rejects_bad_checksum() {
        // Last character flipped — payload is fine, checksum no longer matches.
        assert!(decode_classic_address_to_20bytes("rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTa").is_none());
    }

    #[test]
    fn rejects_too_short() {
        assert!(decode_classic_address_to_20bytes("rTooShort").is_none());
    }

    #[test]
    fn rejects_chars_outside_xrpl_alphabet() {
        // '0', 'O', 'I', 'l' are excluded from the XRPL base58 alphabet — bs58
        // decoding must fail before we even reach length / checksum checks.
        assert!(decode_classic_address_to_20bytes("r0b9CJAWyB4rj91VRWn96DkukG4bwdtyTh").is_none());
    }

    #[test]
    fn rejects_empty() {
        assert!(decode_classic_address_to_20bytes("").is_none());
    }

    #[test]
    fn rejects_bare_r() {
        assert!(decode_classic_address_to_20bytes("r").is_none());
    }
}
