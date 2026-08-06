//! Compile-fail tests for the `r_address!` macro.
//!
//! trybuild only covers `fail_non_literal` — the parser-level error that has
//! no decode-function equivalent. All other rejection paths (bad checksum,
//! missing `r` prefix, wrong length, wrong version byte, …) are unit-tested
//! directly against `decode_classic_address_to_20bytes` next to the decoder,
//! which is faster and avoids fragile `.stderr` snapshots.
//!
//! Happy-path coverage lives in `xrpl-common-stdlib/tests/macros.rs`.
//!
//! Regenerate snapshots with:
//!   TRYBUILD=overwrite cargo test -p xrpl-macros --test r_address

#[test]
fn r_address_compile_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/r_address/fail_non_literal.rs");
}
