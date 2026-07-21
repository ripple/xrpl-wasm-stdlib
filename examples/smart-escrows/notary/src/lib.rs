#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

// Generic XRPL primitives.
use xrpl_common_stdlib::core::current_tx::traits::TransactionCommonFields;
use xrpl_common_stdlib::core::types::account_id::AccountID;
use xrpl_common_stdlib::ctx::SmartFeatureContext;
use xrpl_common_stdlib::host::trace::trace_num;
use xrpl_common_stdlib::host::{Result::Err, Result::Ok};
use xrpl_common_stdlib::r_address;
use xrpl_escrow_stdlib::EscrowFinishContext;
use xrpl_macros::smart_escrow;

// The notary account that is authorized to complete escrows
// Using example notary account for testing
const NOTARY_ACCOUNT: AccountID = r_address!("rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh");

#[smart_escrow]
fn notary_finish(ctx: EscrowFinishContext) -> i32 {
    let tx_account = match ctx.tx().get_account() {
        Ok(v) => v,
        Err(e) => {
            let _ = trace_num("Error in Notary contract", e.code() as i64);
            return e.code(); // Must return to short circuit.
        }
    };

    // <-- Finish the escrow to indicate a successful outcome
    (tx_account == NOTARY_ACCOUNT) as i32
}
