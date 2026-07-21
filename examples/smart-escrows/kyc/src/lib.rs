#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_common_stdlib::core::keylets::credential_keylet;
use xrpl_common_stdlib::host::trace::{DataRepr, trace_data, trace_num};
use xrpl_common_stdlib::host::{Result::Err, Result::Ok};
use xrpl_escrow_stdlib::ledger_objects::traits::CurrentEscrowFields;
use xrpl_escrow_stdlib::{EscrowFinishContext, FinishResult};
use xrpl_macros::smart_escrow;

#[smart_escrow]
fn kyc_finish(ctx: EscrowFinishContext) -> FinishResult {
    let account_id = match ctx.escrow().get_destination() {
        Ok(account_id) => account_id,
        Err(e) => {
            let _ = trace_num("Error getting destination", e.code() as i64);
            return e.code().into(); // <-- Do not execute the escrow.
        }
    };

    let cred_type: &[u8] = b"termsandconditions";
    match credential_keylet(&account_id, &account_id, cred_type) {
        Ok(keylet) => {
            let _ = trace_data("cred_keylet", &keylet, DataRepr::AsHex);

            let slot = unsafe {
                xrpl_common_stdlib::host::cache_ledger_obj(keylet.as_ptr(), keylet.len(), 0)
            };
            if slot < 0 {
                let _ = trace_num("CACHE ERROR", i64::from(slot));
                return FinishResult::reject();
            };
            FinishResult::succeed() // <-- Finish the escrow to indicate a successful outcome
        }
        Err(e) => {
            let _ = trace_num("Error getting credential keylet", e.code() as i64);
            e.code().into() // <-- Do not execute the escrow.
        }
    }
}
