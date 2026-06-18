//! Consolidated compile-fail tests for every macro in `xrpl-macros`.
//!
//! Each `tests/<macro>/fail_*.rs` file is paired with a captured `.stderr`
//! snapshot. Regenerate the snapshots with:
//!
//! ```sh
//! TRYBUILD=overwrite cargo test -p xrpl-macros --test compile_fail
//! ```
//!
//! Happy-path coverage for these macros lives in
//! `xrpl-wasm-stdlib/tests/macros.rs`.

#[test]
fn compile_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/r_address/fail_*.rs");
    t.compile_fail("tests/hash256/fail_*.rs");
    t.compile_fail("tests/pubkey/fail_*.rs");
    t.compile_fail("tests/currency/fail_*.rs");
    t.compile_fail("tests/blob/fail_*.rs");
}
