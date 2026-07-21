#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_escrow_stdlib::current_tx::escrow_finish::get_current_escrow_finish;
use xrpl_wasm_stdlib::core::current_tx::traits::TransactionCommonFields;
use xrpl_wasm_stdlib::core::types::account_id::AccountID;
use xrpl_wasm_stdlib::host::trace::trace_num;
use xrpl_wasm_stdlib::host::{Result::Err, Result::Ok};
use xrpl_wasm_stdlib::r_address;

// The notary account that is authorized to complete escrows
// Using example notary account for testing
const NOTARY_ACCOUNT: AccountID = r_address!("rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh");

#[unsafe(no_mangle)]
pub extern "C" fn finish() -> i32 {
    let escrow_finish = get_current_escrow_finish();
    let tx_account = match escrow_finish.get_account() {
        Ok(v) => v,
        Err(e) => {
            let _ = trace_num("Error in Notary contract", e.code() as i64);
            return e.code(); // Must return to short circuit.
        }
    };

    (tx_account == NOTARY_ACCOUNT) as i32 // <-- Finish the escrow to indicate a successful outcome
}
