#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_wasm_stdlib::core::keylets::credential_keylet;
use xrpl_wasm_stdlib::core::ledger_objects::current_escrow;
use xrpl_wasm_stdlib::core::ledger_objects::current_escrow::CurrentEscrow;
use xrpl_wasm_stdlib::core::ledger_objects::traits::CurrentEscrowFields;
use xrpl_wasm_stdlib::host::trace::{DataRepr, trace_data, trace_num};
use xrpl_wasm_stdlib::host::{Result::Err, Result::Ok};

#[unsafe(no_mangle)]
pub extern "C" fn finish() -> i32 {
    let current_escrow: CurrentEscrow = current_escrow::get_current_escrow();

    let account_id = match current_escrow.get_destination() {
        Ok(account_id) => account_id,
        Err(e) => {
            let _ = trace_num("Error getting destination", e.code() as i64);
            return e.code(); // <-- Do not execute the escrow.
        }
    };

    let cred_type: &[u8] = b"termsandconditions";
    match credential_keylet(&account_id, &account_id, cred_type) {
        Ok(keylet) => {
            let _ = trace_data("cred_keylet", &keylet, DataRepr::AsHex);

            let slot = unsafe {
                xrpl_wasm_stdlib::host::cache_ledger_obj(keylet.as_ptr(), keylet.len(), 0)
            };
            if slot < 0 {
                let _ = trace_num("CACHE ERROR", i64::from(slot));
                return 0;
            };
            1 // <-- Finish the escrow to indicate a successful outcome
        }
        Err(e) => {
            let _ = trace_num("Error getting credential keylet", e.code() as i64);
            e.code() // <-- Do not execute the escrow.
        }
    }
}
