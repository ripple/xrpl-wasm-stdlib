//! Compile-fail tests proving that `FromCurrentTx` and `FromLedger` are
//! independent capabilities: implementing one does not grant the other.
//!
//! Regenerate snapshots with:
//!   TRYBUILD=overwrite cargo test -p xrpl-wasm-stdlib --test decoder_compile_fail

#[test]
fn compile_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/decoder/fail_*.rs");
}
