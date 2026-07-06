//! Compile-fail tests for every macro in `xrpl-macros`.
//!
//! **Typed-constant macros** (`r_address!`, `hash256!`, `pubkey!`, `currency!`,
//! `blob!`): trybuild covers `fail_non_literal` — the parser-level error that
//! has no decode-function equivalent. All other rejection paths (wrong length,
//! bad prefix, XRP reserved, capacity overflow, …) are unit-tested directly
//! against the per-macro `decode_*` / `check_*` helpers, which is faster and
//! avoids fragile `.stderr` snapshots. `r_address!` still exercises its full
//! fixture set here (no decoder unit tests exist for it yet).
//!
//! Happy-path coverage lives in `xrpl-common-stdlib/tests/macros.rs`.
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
