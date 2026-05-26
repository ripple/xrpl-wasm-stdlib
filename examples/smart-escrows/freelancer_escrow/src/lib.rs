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

const ARBITRATOR_START: usize = 0;
const DEADLINE_START: usize = 20;
const DEADLINE_END: usize = 24;
const CLIENT_CONFIRMED: usize = 24;
const FREELANCER_CONFIRMED: usize = 25;
const DISPUTE_RAISED: usize = 26;
const DISPUTING_PARTY: usize = 27;


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
    let escrow = get_current_escrow();
    let client = match escrow.get_account() {
        Ok(v) => v,
        Err(e) => return e.code()
    };
    let freelancer = match escrow.get_destination() {
        Ok(v) => v,
        Err(e) => return e.code()
    };
    let mut data = match escrow.get_data() {
        Ok(v) => v,
        Err(e) => return e.code()
    };
    let mut arbitrator = [0u8; 20];
    arbitrator.copy_from_slice(&data.data[ARBITRATOR_START..DEADLINE_START]);
    let intent = match get_first_memo() {
        Ok(Some(v)) => v,
        Ok(None) => 0,
        Err(e) => return e.code()
    };
    let mut deadline_bytes = [0u8; 4];
    deadline_bytes.copy_from_slice(&data.data[DEADLINE_START..DEADLINE_END]);
    let deadline = u32::from_le_bytes(deadline_bytes);
    let freelancer_confirmed = data.data[FREELANCER_CONFIRMED] == 1;
    match (tx_account.0, intent) {
        a if (a.0 == client.0 || a.0 == freelancer.0) && a.1 < 2 && data.data[DISPUTE_RAISED] != 1 => {
            // Participant, not a dispute request, no current dispute. Confirm / deconfirm.
            data.data[if tx_account.0 == client.0 {CLIENT_CONFIRMED} else {FREELANCER_CONFIRMED}] = intent ^ 1;
            let should_release = data.data[CLIENT_CONFIRMED] == 1 && data.data[FREELANCER_CONFIRMED] == 1;
            match CurrentEscrow::update_current_escrow_data(data) {
                Ok(()) => {},
                Err(e) => return e.code()
            }
            return should_release as i32;
        },
        a if (a.0 == client.0 || a.0 == freelancer.0) && a.1 == 2 && data.data[DISPUTE_RAISED] != 1 => {
            // Participant, no current dispute. Calling dispute.
            data.data[DISPUTE_RAISED] = 1;
            data.data[CLIENT_CONFIRMED..DISPUTE_RAISED].fill(0);
            data.data[DISPUTING_PARTY] = if tx_account.0 == client.0 {1} else {2};
            match CurrentEscrow::update_current_escrow_data(data) {
                Ok(()) => {},
                Err(e) => return e.code()
            }
        },
        a if (a.0 == client.0 || a.0 == freelancer.0) && a.1 == 2 => {
            // Participant, current dispute. Resolve dispute if you're the one who disputed.
            if data.data[DISPUTING_PARTY] == 1 && tx_account.0 == client.0 {
                data.data[DISPUTE_RAISED] = 0;
            } else if data.data[DISPUTING_PARTY] == 2 && tx_account.0 == freelancer.0 {
                data.data[DISPUTE_RAISED] = 0;
            } else {
                return 0;
            }
            data.data[DISPUTING_PARTY] = 0;
            match CurrentEscrow::update_current_escrow_data(data) {
                Ok(()) => {},
                Err(e) => return e.code()
            }
        },
        a if a.0 == arbitrator && data.data[DISPUTE_RAISED] == 1 && a.1 == 2 => {
            // Arbitrator, current dispute. You rule in favor of the freelancer.
            return 1;
        }
        _ => return 0
    };
    if freelancer_confirmed {
        let mut time_buf = [0u8; 4];
        let time = unsafe {
            xrpl_wasm_stdlib::host::get_parent_ledger_time(time_buf.as_mut_ptr(), time_buf.len())
        };
        if time < 0 {
            return time;
        }
        let curr_time = u32::from_le_bytes(time_buf);
        if curr_time > deadline {
            return 1;
        }
    }
    0
}
