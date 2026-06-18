//! Happy-path integration tests for every compile-time literal macro re-exported
//! from this crate (`r_address!`, `hash256!`, `pubkey!`, `currency!`, `blob!`).
//!
//! Each test invokes the macro in a `const` (or `let`) binding and asserts the
//! resulting struct contents. The matching rejection cases live in
//! `xrpl-macros/tests/compile_fail.rs` (trybuild).

use xrpl_wasm_stdlib::core::types::account_id::AccountID;
use xrpl_wasm_stdlib::core::types::blob::Blob;
use xrpl_wasm_stdlib::core::types::currency::Currency;
use xrpl_wasm_stdlib::core::types::public_key::PublicKey;
use xrpl_wasm_stdlib::core::types::uint::Hash256;
use xrpl_wasm_stdlib::{blob, currency, hash256, pubkey, r_address};

// ----- r_address! -----

#[test]
fn r_address_expands_to_const_account_id() {
    const ACCOUNT: AccountID = r_address!("rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh");
    assert_eq!(
        ACCOUNT.0,
        [
            0xb5, 0xf7, 0x62, 0x79, 0x8a, 0x53, 0xd5, 0x43, 0xa0, 0x14, 0xca, 0xf8, 0xb2, 0x97,
            0xcf, 0xf8, 0xf2, 0xf9, 0x37, 0xe8,
        ]
    );
}

// ----- hash256! -----

#[test]
fn hash256_decodes_uppercase() {
    const H: Hash256 = hash256!("0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF");
    assert_eq!(H.0[0], 0x01);
    assert_eq!(H.0[31], 0xEF);
}

#[test]
fn hash256_decodes_lowercase() {
    const H: Hash256 = hash256!("0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef");
    assert_eq!(H.0[1], 0x23);
}

#[test]
fn hash256_decodes_mixed_case() {
    const H: Hash256 = hash256!("0123456789AbCdEf0123456789aBcDeF0123456789AbCdEf0123456789aBcDeF");
    assert_eq!(H.0.len(), 32);
}

// ----- pubkey! -----

#[test]
fn pubkey_decodes_secp256k1_02_prefix() {
    const K: PublicKey =
        pubkey!("02C7387FFC25C156CA7F8A6D760C8D01EF642CEE9CE4680C33FFB3FF39AFECFE70");
    assert_eq!(K.0[0], 0x02);
    assert_eq!(
        K.0,
        [
            0x02, 0xC7, 0x38, 0x7F, 0xFC, 0x25, 0xC1, 0x56, 0xCA, 0x7F, 0x8A, 0x6D, 0x76, 0x0C,
            0x8D, 0x01, 0xEF, 0x64, 0x2C, 0xEE, 0x9C, 0xE4, 0x68, 0x0C, 0x33, 0xFF, 0xB3, 0xFF,
            0x39, 0xAF, 0xEC, 0xFE, 0x70,
        ]
    );
}

#[test]
fn pubkey_decodes_secp256k1_03_prefix() {
    const K: PublicKey =
        pubkey!("03C7387FFC25C156CA7F8A6D760C8D01EF642CEE9CE4680C33FFB3FF39AFECFE70");
    assert_eq!(K.0[0], 0x03);
}

#[test]
fn pubkey_decodes_ed25519_uppercase_prefix() {
    const K: PublicKey =
        pubkey!("EDD9B3599802B214A99D757712D6ABDF72F83C63BBD53861411790B13D04B2C5C9");
    assert_eq!(K.0[0], 0xED);
}

#[test]
fn pubkey_decodes_ed25519_lowercase_prefix() {
    const K: PublicKey =
        pubkey!("edD9B3599802B214A99D757712D6ABDF72F83C63BBD53861411790B13D04B2C5C9");
    assert_eq!(K.0[0], 0xED);
}

// ----- currency! -----

#[test]
fn currency_decodes_standard_three_char() {
    const C: Currency = currency!("USD");
    assert_eq!(&C.0[..12], &[0u8; 12]);
    assert_eq!(&C.0[12..15], b"USD");
    assert_eq!(&C.0[15..], &[0u8; 5]);
}

#[test]
fn currency_decodes_standard_numeric() {
    const C: Currency = currency!("US1");
    assert_eq!(&C.0[12..15], b"US1");
}

#[test]
fn currency_decodes_nonstandard_hex() {
    const C: Currency = currency!("0158415500000000C1F76FF6ECB0BAC600000000");
    assert_eq!(C.0[0], 0x01);
    assert_eq!(C.0[19], 0x00);
}

// ----- blob! -----

#[test]
fn blob_exact_fit_decodes_bytes() {
    const B: Blob<4> = blob!("DEADBEEF");
    assert_eq!(B.len(), 4);
    assert_eq!(B.as_slice(), &[0xDE, 0xAD, 0xBE, 0xEF]);
}

#[test]
fn blob_lowercase_hex_is_accepted() {
    const B: Blob<4> = blob!("deadbeef");
    assert_eq!(B.as_slice(), &[0xDE, 0xAD, 0xBE, 0xEF]);
}

#[test]
fn blob_with_capacity_pads_with_zeros() {
    const B: Blob<8> = blob!("DEADBEEF", 8);
    assert_eq!(B.len(), 4);
    assert_eq!(B.capacity(), 8);
    assert_eq!(B.as_slice(), &[0xDE, 0xAD, 0xBE, 0xEF]);
    assert_eq!(&B.data[4..], &[0u8; 4]);
}

#[test]
fn blob_empty_is_accepted() {
    const B: Blob<0> = blob!("");
    assert!(B.is_empty());
}

#[test]
fn blob_with_capacity_only_padding() {
    const B: Blob<16> = blob!("", 16);
    assert_eq!(B.len(), 0);
    assert_eq!(B.capacity(), 16);
    assert_eq!(B.data, [0u8; 16]);
}
