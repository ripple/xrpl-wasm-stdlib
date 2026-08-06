use xrpl_escrow_stdlib::FinishResult;

fn main() {
    // Boundary values that must compile
    let _ = FinishResult::succeed_with::<1>();
    let _ = FinishResult::succeed_with::<{ i32::MAX }>();
    let _ = FinishResult::reject_with::<-1>();
    let _ = FinishResult::reject_with::<{ i32::MIN }>();
}
