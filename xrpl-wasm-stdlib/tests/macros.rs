//! Happy-path integration test for `r_address!`.
//!
//! Covers the boundaries the decoder unit tests in `xrpl-macros` can't reach —
//! that the macro emits const-compatible Rust, that the `AccountID` re-export
//! chain still works, and that the generated struct literal matches the type's
//! actual shape. Decoder-level coverage (rejection paths, alphabet handling)
//! lives next to `decode_classic_address_to_20bytes`.
//!
//! Compile-fail rejection paths live in `xrpl-macros/tests/r_address.rs`.

use xrpl_wasm_stdlib::core::types::account_id::AccountID;
use xrpl_wasm_stdlib::r_address;

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
