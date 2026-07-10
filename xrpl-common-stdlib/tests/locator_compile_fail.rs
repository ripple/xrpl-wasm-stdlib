//! Compile-fail tests for the fluent path builders (`ctx.tx().path()` → [`TxPathBuilder`],
//! `ctx.escrow().path()` / `obj.path()` → [`LedgerPathBuilder`]).
//!
//! Each terminal `.get::<T>()` is bounded on its context marker (`FromCurrentTx` for the tx
//! builder, `FromLedger` for the ledger builder), so it must reject types that aren't readable from
//! that context — aggregate placeholders (`Array`/`Object`) and types that only opt into the other
//! context. Unit tests exercise runtime behavior; only a compile-fail snapshot can prove these
//! misuses don't type-check.
//!
//! Regenerate snapshots with:
//!   TRYBUILD=overwrite cargo test -p xrpl-common-stdlib --test locator_compile_fail

#[test]
fn compile_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/locator/fail_*.rs");
}
