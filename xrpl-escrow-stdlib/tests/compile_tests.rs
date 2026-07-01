#[test]
fn compile_fail_tests() {
    let t = trybuild::TestCases::new();
    // Pass test is required: trybuild uses `cargo check` when there are only
    // compile_fail cases, which skips generic monomorphization and misses
    // const { assert!() } failures. A single pass test switches it to `cargo build`.
    t.pass("tests/pass/*.rs");
    t.compile_fail("tests/finish_result/compile_fail/*.rs");
}
