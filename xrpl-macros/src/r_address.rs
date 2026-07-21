//! The `r_address!` macro for compile-time address conversion converts XRPL classic addresses (r-addresses)
//! to an [`AccountID`] at compile time.
//!
//! **Important**: The macro only accepts string literals, not runtime values.
//! It runs during compilation and outputs only the final `AccountID` value - no
//! base58 decoding code is included in the WASM binary.
//!
//! # Example
//! ```shell
//! use xrpl_common_stdlib::r_address;
//! use xrpl_common_stdlib::core::types::account_id::AccountID;
//!
//! // ✅ Works - compile-time literal
//! const ACCOUNT: AccountID = r_address!("rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh");
//!
//! // ❌ Does NOT work - runtime value
//! // fn convert(addr: &str) -> AccountID {
//! //     r_address!(addr)  // ERROR: expected string literal
//! // }
//! ```
use proc_macro2::TokenStream;
use quote::quote;
use syn::LitStr;

pub fn expand(input: TokenStream) -> syn::Result<TokenStream> {
    let addr_lit = syn::parse2::<LitStr>(input)?;
    let addr = addr_lit.value();

    let bytes = decode_classic_address_to_20bytes(&addr).map_err(|reason| {
        syn::Error::new(addr_lit.span(), format!("Invalid r-address: {reason}"))
    })?;

    let bytes_tokens = bytes.iter().map(|b| quote! { #b });
    let expanded = quote! {
        ::xrpl_common_stdlib::core::types::account_id::AccountID([#(#bytes_tokens),*])
    };

    Ok(expanded)
}

fn decode_classic_address_to_20bytes(addr: &str) -> Result<Vec<u8>, &'static str> {
    if !addr.starts_with('r') {
        return Err("missing 'r' prefix");
    }
    let alphabet =
        bs58::Alphabet::new(b"rpshnaf39wBUDNEGHJKLM4PQRST7VWXYZ2bcdeCg65jkm8oFqi1tuvAxyz")
            .map_err(|_| "invalid base58 alphabet")?;
    let full = bs58::decode(addr)
        .with_alphabet(&alphabet)
        .into_vec()
        .map_err(|_| "invalid base58 character")?;
    if full.len() < 1 + 20 + 4 {
        return Err("decoded length too short");
    }
    // Version byte should be 0x00 for classic AccountID
    if full[0] != 0x00 {
        return Err("invalid version byte");
    }
    // Split payload and checksum
    let (payload, checksum) = full.split_at(full.len() - 4);
    // Verify checksum: double SHA-256 of payload, take first 4 bytes
    use sha2::{Digest, Sha256};
    let first = Sha256::digest(payload);
    let second = Sha256::digest(first);
    if &second[0..4] != checksum {
        return Err("checksum mismatch");
    }
    // Payload is version (1) + 20 bytes account id
    if payload.len() != 1 + 20 {
        return Err("decoded length too short");
    }
    Ok(payload[1..].to_vec())
}

#[cfg(test)]
mod tests {
    use super::{decode_classic_address_to_20bytes, expand};
    use quote::quote;

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
        let err =
            decode_classic_address_to_20bytes("xHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh").unwrap_err();
        assert_eq!(err, "missing 'r' prefix");
    }

    #[test]
    fn rejects_bad_checksum() {
        // Last character flipped — payload is fine, checksum no longer matches.
        let err =
            decode_classic_address_to_20bytes("rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTa").unwrap_err();
        assert_eq!(err, "checksum mismatch");
    }

    #[test]
    fn rejects_too_short() {
        let err = decode_classic_address_to_20bytes("rTooShort").unwrap_err();
        assert_eq!(err, "decoded length too short");
    }

    #[test]
    fn rejects_chars_outside_xrpl_alphabet() {
        // '0', 'O', 'I', 'l' are excluded from the XRPL base58 alphabet — bs58
        // decoding must fail before we even reach length / checksum checks.
        let err =
            decode_classic_address_to_20bytes("r0b9CJAWyB4rj91VRWn96DkukG4bwdtyTh").unwrap_err();
        assert_eq!(err, "invalid base58 character");
    }

    #[test]
    fn rejects_empty() {
        let err = decode_classic_address_to_20bytes("").unwrap_err();
        assert_eq!(err, "missing 'r' prefix");
    }

    #[test]
    fn rejects_bare_r() {
        let err = decode_classic_address_to_20bytes("r").unwrap_err();
        assert_eq!(err, "decoded length too short");
    }

    #[test]
    fn expand_emits_account_id_for_valid_address() {
        let input = quote! { "rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh" };
        assert!(expand(input).is_ok());
    }

    #[test]
    fn expand_errors_on_invalid_address() {
        let input = quote! { "xHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh" };
        let err = expand(input).unwrap_err();
        assert!(err.to_string().contains("Invalid r-address"));
    }

    #[test]
    fn expand_errors_on_bad_checksum() {
        let input = quote! { "rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTa" };
        let err = expand(input).unwrap_err();
        assert!(err.to_string().contains("checksum mismatch"));
    }
}
