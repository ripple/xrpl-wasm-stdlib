//! Compile-fail tests for the `r_address!` macro.
//!
//! These verify that invalid inputs produce useful, stable compile errors.
//! The expected error output is captured in the matching `.stderr` files;
//! regenerate them with `TRYBUILD=overwrite cargo test -p xrpl-macros --test r_address`.

#[test]
fn r_address_compile_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/r_address/fail_not_r_prefix.rs");
    t.compile_fail("tests/r_address/fail_bad_checksum.rs");
    t.compile_fail("tests/r_address/fail_too_short.rs");
    t.compile_fail("tests/r_address/fail_non_literal.rs");
}
