use xrpl_escrow_stdlib::{EscrowFinishContext, FinishResult};
use xrpl_macros::smart_escrow;

#[smart_escrow]
fn run(_ctx: EscrowFinishContext) -> FinishResult {
    FinishResult::succeed()
}
