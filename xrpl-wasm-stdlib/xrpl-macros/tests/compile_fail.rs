//! Compile-fail tests for every macro in `xrpl-macros`.
//!
//! For the typed-constant macros (`hash256!`, `pubkey!`, `currency!`, `blob!`)
//! trybuild only covers `fail_non_literal` — the parser-level errors that have
//! no decode-function equivalent. All other rejection paths (wrong length,
//! invalid hex, bad prefix, XRP, capacity overflow, …) are unit-tested
//! directly against the per-macro `decode_*` / `check_*` helpers, which is
//! faster and avoids fragile `.stderr` snapshots.
//!
//! `r_address!` still exercises its full set of fixtures here (no decoder
//! unit tests exist for it yet).
//!
//! Happy-path coverage lives in `xrpl-wasm-stdlib/tests/macros.rs`.
//!
//! Regenerate snapshots with:
//!   TRYBUILD=overwrite cargo test -p xrpl-macros --test compile_fail

#[test]
fn compile_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/r_address/fail_*.rs");
    t.compile_fail("tests/hash256/fail_*.rs");
    t.compile_fail("tests/pubkey/fail_*.rs");
    t.compile_fail("tests/currency/fail_*.rs");
    t.compile_fail("tests/blob/fail_*.rs");
}
