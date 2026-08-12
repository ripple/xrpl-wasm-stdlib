#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_common_stdlib::host::chain::ledger_sqn;
use xrpl_common_stdlib::host::trace::trace_num;
use xrpl_escrow_stdlib::EscrowFinishContext;
use xrpl_macros::smart_escrow;

#[smart_escrow]
fn check_ledger_sqn(_ctx: EscrowFinishContext) -> i32 {
    // `ledger_sqn` owns the buffer, the byte count check and the decode, so the contract sees a
    // `u32` or an error. Panicking on that error is this example's documented behavior.
    let ledger_sequence = ledger_sqn().unwrap_or_panic();
    trace_num("Ledger Sequence", ledger_sequence as i64);
    (ledger_sequence >= 5) as i32 // Return 1 if true (successful outcome), 0 if false (failed outcome)
}
