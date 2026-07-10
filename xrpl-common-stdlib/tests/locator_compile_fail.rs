//! Compile-fail tests for the fluent `ctx.tx().path()` builder ([`TxPathBuilder`]).
//!
//! The terminal `.get::<T>()` is bounded on `T: FromCurrentTx`, so it must reject types that
//! aren't readable from a transaction — aggregate placeholders (`Array`/`Object`) and types that
//! only opt into the ledger-object context. Unit tests can exercise the runtime behavior but only
//! a compile-fail snapshot can prove these misuses don't type-check.
//!
//! Regenerate snapshots with:
//!   TRYBUILD=overwrite cargo test -p xrpl-common-stdlib --test locator_compile_fail

#[test]
fn compile_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/locator/fail_*.rs");
}
