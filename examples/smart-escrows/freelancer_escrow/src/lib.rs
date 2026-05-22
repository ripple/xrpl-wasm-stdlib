#![cfg_attr(target_arch = "wasm32", no_std)]

use xrpl_wasm_stdlib::core::current_tx::escrow_finish::get_current_escrow_finish;
use xrpl_wasm_stdlib::core::current_tx::traits::TransactionCommonFields;
use xrpl_wasm_stdlib::core::ledger_objects::current_escrow::{CurrentEscrow, get_current_escrow};
use xrpl_wasm_stdlib::core::ledger_objects::traits::CurrentEscrowFields;
use xrpl_wasm_stdlib::core::locator::Locator;
use xrpl_wasm_stdlib::host::get_tx_nested_field;
use xrpl_wasm_stdlib::sfield;
use xrpl_wasm_stdlib::host::{Error, Result, Result::Err, Result::Ok};


#[cfg(not(target_arch = "wasm32"))]
extern crate std;


pub fn get_first_memo() -> Result<Option<u8>> {
    let mut data = [0u8; 1];
    let mut locator = Locator::new();
    locator.pack(sfield::Memos);
    locator.pack(0);
    locator.pack(sfield::MemoData);
    let result_code = unsafe {
        get_tx_nested_field(
            locator.as_ptr(),
            locator.num_packed_bytes(),
            data.as_mut_ptr(),
            data.len(),
        )
    };

    match result_code {
        result_code if result_code > 0 => Ok(Some(data[0])),
        0 => Err(Error::InternalError),
        result_code => Err(Error::from_code(result_code)),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn finish() -> i32 {
    let tx_account = match get_current_escrow_finish().get_account() {
        Ok(v) => v,
        Err(e) => return e.code(),
    };
    let client = match get_current_escrow().get_account() {
        Ok(v) => v,
        Err(e) => return e.code()
    };
    let freelancer = match get_current_escrow().get_destination() {
        Ok(v) => v,
        Err(e) => return e.code()
    };
    let mut data = match get_current_escrow().get_data() {
        Ok(v) => v,
        Err(e) => return e.code()
    };
    let mut arbitrator = [0u8; 20];
    arbitrator.copy_from_slice(&data.data[0..20]);
    let intent = match get_first_memo() {
        Ok(Some(v)) => v,
        Ok(None) => 0,
        Err(e) => return e.code()
    };
    match (tx_account.0, intent) {
        a if (a.0 == client.0 || a.0 == freelancer.0) && a.1 < 2 && data.data[26] != 1 => {
            // Participant, not a dispute request, no current dispute. Confirm / deconfirm.
            data.data[if tx_account.0 == client.0 {24} else {25}] = intent ^ 1;
            let should_release = data.data[24] == 1 && data.data[25] == 1;
            match CurrentEscrow::update_current_escrow_data(data) {
                Ok(()) => {},
                Err(e) => return e.code()
            }
            return should_release as i32;
        },
        a if (a.0 == client.0 || a.0 == freelancer.0) && a.1 == 2 && data.data[26] != 1 => {
            // Participant, no current dispute. Calling dispute.
            data.data[26] = 1;
            data.data[24..26].fill(0);
            data.data[27] = if tx_account.0 == client.0 {1} else {2};
            match CurrentEscrow::update_current_escrow_data(data) {
                Ok(()) => {},
                Err(e) => return e.code()
            }
        },
        a if (a.0 == client.0 || a.0 == freelancer.0) && a.1 == 2 => {
            // Participant, current dispute. Resolve dispute if you're the one who disputed.
            if data.data[27] == 1 && tx_account.0 == client.0 {
                data.data[26] = 0;
            } else if data.data[27] == 2 && tx_account.0 == freelancer.0 {
                data.data[26] = 0;
            } else {
                return 0;
            }
            data.data[27] = 0;
            match CurrentEscrow::update_current_escrow_data(data) {
                Ok(()) => {},
                Err(e) => return e.code()
            }
        },
        a if a.0 == arbitrator && data.data[26] == 1 && a.1 == 2 => {
            // Arbitrator, current dispute. You rule in favor of the freelancer.
            return 1;
        }
        _ => return 0
    };
    0
}
