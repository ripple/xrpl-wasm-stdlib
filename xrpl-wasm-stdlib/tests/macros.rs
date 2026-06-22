//! Happy-path integration tests for every compile-time literal macro re-exported
//! from this crate (`r_address!`, `hash256!`, `pubkey!`, `currency!`, `blob!`).
//!
//! Each test invokes the macro in a `const` binding and asserts the resulting
//! struct contents. The point of these tests is to cover the boundaries the
//! decoder unit tests in `xrpl-macros` can't reach — that the macro emits
//! const-compatible Rust, that the target type path / re-export chain still
//! works, and that the generated struct literal matches the type's actual
//! shape. Decoder-level coverage (case variants, prefix discrimination,
//! length / character rejection) lives next to the decoders themselves.
//!
//! Compile-fail rejection paths live in `xrpl-macros/tests/compile_fail.rs`.

use xrpl_wasm_stdlib::core::types::account_id::AccountID;
use xrpl_wasm_stdlib::core::types::blob::Blob;
use xrpl_wasm_stdlib::core::types::currency::Currency;
use xrpl_wasm_stdlib::core::types::public_key::PublicKey;
use xrpl_wasm_stdlib::core::types::uint::Hash256;
use xrpl_wasm_stdlib::{blob, currency, hash256, pubkey, r_address};

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

#[test]
fn hash256_expands_to_const_hash() {
    const H: Hash256 = hash256!("0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF");
    assert_eq!(
        H.0,
        [
            0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x01, 0x23, 0x45, 0x67, 0x89, 0xAB,
            0xCD, 0xEF, 0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x01, 0x23, 0x45, 0x67,
            0x89, 0xAB, 0xCD, 0xEF,
        ]
    );
}

#[test]
fn pubkey_expands_to_const_public_key() {
    const K: PublicKey =
        pubkey!("02C7387FFC25C156CA7F8A6D760C8D01EF642CEE9CE4680C33FFB3FF39AFECFE70");
    assert_eq!(
        K.0,
        [
            0x02, 0xC7, 0x38, 0x7F, 0xFC, 0x25, 0xC1, 0x56, 0xCA, 0x7F, 0x8A, 0x6D, 0x76, 0x0C,
            0x8D, 0x01, 0xEF, 0x64, 0x2C, 0xEE, 0x9C, 0xE4, 0x68, 0x0C, 0x33, 0xFF, 0xB3, 0xFF,
            0x39, 0xAF, 0xEC, 0xFE, 0x70,
        ]
    );
}

// Standard 3-char and non-standard 40-char go through different decoder branches
// inside the same macro — keep both so the integration boundary covers each.

#[test]
fn currency_expands_to_const_standard_code() {
    const C: Currency = currency!("USD");
    assert_eq!(&C.0[..12], &[0u8; 12]);
    assert_eq!(&C.0[12..15], b"USD");
    assert_eq!(&C.0[15..], &[0u8; 5]);
}

#[test]
fn currency_expands_to_const_nonstandard_hex() {
    const C: Currency = currency!("0158415500000000C1F76FF6ECB0BAC600000000");
    assert_eq!(
        C.0,
        [
            0x01, 0x58, 0x41, 0x55, 0x00, 0x00, 0x00, 0x00, 0xC1, 0xF7, 0x6F, 0xF6, 0xEC, 0xB0,
            0xBA, 0xC6, 0x00, 0x00, 0x00, 0x00,
        ]
    );
}

// The 1-arg and 2-arg forms hit different `BlobInput` parser branches — both
// are exercised so a regression to either parsing path is caught here.

#[test]
fn blob_expands_to_const_exact_fit() {
    const B: Blob<4> = blob!("DEADBEEF");
    assert_eq!(B.len(), 4);
    assert_eq!(B.as_slice(), &[0xDE, 0xAD, 0xBE, 0xEF]);
}

#[test]
fn blob_expands_to_const_with_padded_capacity() {
    const B: Blob<8> = blob!("DEADBEEF", 8);
    assert_eq!(B.len(), 4);
    assert_eq!(B.capacity(), 8);
    assert_eq!(B.as_slice(), &[0xDE, 0xAD, 0xBE, 0xEF]);
    assert_eq!(&B.data[4..], &[0u8; 4]);
}
