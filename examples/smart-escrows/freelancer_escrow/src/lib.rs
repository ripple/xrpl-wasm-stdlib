#![cfg_attr(target_arch = "wasm32", no_std)]

use xrpl_wasm_stdlib::core::current_tx::escrow_finish::get_current_escrow_finish;
use xrpl_wasm_stdlib::core::current_tx::traits::TransactionCommonFields;
use xrpl_wasm_stdlib::core::ledger_objects::current_escrow::{CurrentEscrow, get_current_escrow};
use xrpl_wasm_stdlib::core::ledger_objects::traits::CurrentEscrowFields;
use xrpl_wasm_stdlib::core::locator::Locator;
use xrpl_wasm_stdlib::host::get_tx_nested_field;
use xrpl_wasm_stdlib::host::trace::trace_num;
use xrpl_wasm_stdlib::host::{Error, Result, Result::Err, Result::Ok};
use xrpl_wasm_stdlib::sfield;

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

const ARBITRATOR_START: usize = 0;
const DEADLINE_START: usize = 20;
const DEADLINE_END: usize = 24;
const CLIENT_CONFIRMED: usize = 24;
const FREELANCER_CONFIRMED: usize = 25;
const DISPUTE_RAISED: usize = 26;
const DISPUTING_PARTY: usize = 27;

const INTENT_CONFIRM: u8 = 0;
const INTENT_DECONFIRM: u8 = 1;
const INTENT_DISPUTE: u8 = 2;

const DISPUTING_CLIENT: u8 = 1;
const DISPUTING_FREELANCER: u8 = 2;
const DISPUTING_ARB_LOCK: u8 = 3; // arbitrator ruled for client, lock contract for CancelAfter

const ARB_RULE_FREELANCER: u8 = INTENT_CONFIRM;
const ARB_RULE_CLIENT: u8 = INTENT_DISPUTE;

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
        Err(e) => {
            let _ = trace_num("Error in getting tx_account", e.code() as i64);
            return e.code();
        }
    };
    let escrow = get_current_escrow();
    let client = match escrow.get_account() {
        Ok(v) => v,
        Err(e) => {
            let _ = trace_num("Error in getting client account", e.code() as i64);
            return e.code();
        }
    };
    let freelancer = match escrow.get_destination() {
        Ok(v) => v,
        Err(e) => {
            let _ = trace_num("Error in getting freelancer account", e.code() as i64);
            return e.code();
        }
    };
    let mut data = match escrow.get_data() {
        Ok(v) => v,
        Err(e) => {
            let _ = trace_num("Error in fetching escrow data", e.code() as i64);
            return e.code();
        }
    };
    if data.len < DISPUTING_PARTY + 1 {
        let _ = trace_num("Error, escrow data malformed / too short", data.len as i64);
        return Error::InvalidParams.code();
    }
    let mut arbitrator = [0u8; 20];
    arbitrator.copy_from_slice(&data.data[ARBITRATOR_START..DEADLINE_START]);
    let intent = match get_first_memo() {
        Ok(Some(v)) => v,
        Ok(None) => 0,
        Err(e) => {
            let _ = trace_num("Error in fetching Memo fields", e.code() as i64);
            return e.code();
        }
    };
    match (tx_account.0, intent) {
        a if (a.0 == client.0 || a.0 == freelancer.0)
            && (a.1 == INTENT_CONFIRM || a.1 == INTENT_DECONFIRM)
            && data.data[DISPUTE_RAISED] != 1 =>
        {
            // Participant, not a dispute request, no current dispute. Confirm / deconfirm.
            data.data[if tx_account.0 == client.0 {
                CLIENT_CONFIRMED
            } else {
                FREELANCER_CONFIRMED
            }] = (intent == INTENT_CONFIRM) as u8;
            let should_release =
                data.data[CLIENT_CONFIRMED] == 1 && data.data[FREELANCER_CONFIRMED] == 1;
            // Capture these before data is moved into update_current_escrow_data.
            let freelancer_confirmed = data.data[FREELANCER_CONFIRMED] == 1;
            let mut deadline_bytes = [0u8; 4];
            deadline_bytes.copy_from_slice(&data.data[DEADLINE_START..DEADLINE_END]);
            let deadline = u32::from_le_bytes(deadline_bytes);
            match CurrentEscrow::update_current_escrow_data(data) {
                Ok(()) => {}
                Err(e) => {
                    let _ = trace_num("Error updating confirm state", e.code() as i64);
                    return e.code();
                }
            }
            if should_release {
                return 1;
            }
            // Auto-release if freelancer has confirmed and deadline has passed (no active dispute).
            if freelancer_confirmed {
                let mut time_buf = [0u8; 4];
                let time = unsafe {
                    xrpl_wasm_stdlib::host::get_parent_ledger_time(
                        time_buf.as_mut_ptr(),
                        time_buf.len(),
                    )
                };
                if time < 0 {
                    let _ = trace_num("Error getting ledger time", time as i64);
                    return time;
                }
                let curr_time = u32::from_le_bytes(time_buf);
                if curr_time > deadline {
                    return 1;
                }
            }
            return 0;
        }
        a if (a.0 == client.0 || a.0 == freelancer.0)
            && a.1 == INTENT_DISPUTE
            && data.data[DISPUTE_RAISED] != 1 =>
        {
            // Participant, no current dispute. Calling dispute.
            data.data[DISPUTE_RAISED] = 1;
            data.data[CLIENT_CONFIRMED..DISPUTE_RAISED].fill(0);
            data.data[DISPUTING_PARTY] = if tx_account.0 == client.0 {
                DISPUTING_CLIENT
            } else {
                DISPUTING_FREELANCER
            };
            match CurrentEscrow::update_current_escrow_data(data) {
                Ok(()) => {}
                Err(e) => {
                    let _ = trace_num("Error updating dispute state", e.code() as i64);
                    return e.code();
                }
            }
        }
        a if (a.0 == client.0 || a.0 == freelancer.0) && a.1 == INTENT_DISPUTE => {
            // Participant, current dispute. Resolve dispute if you're the one who disputed.
            if data.data[DISPUTING_PARTY] == DISPUTING_CLIENT && tx_account.0 == client.0 {
                data.data[DISPUTE_RAISED] = 0;
            } else if data.data[DISPUTING_PARTY] == DISPUTING_FREELANCER
                && tx_account.0 == freelancer.0
            {
                data.data[DISPUTE_RAISED] = 0;
            } else {
                return 0;
            }
            data.data[DISPUTING_PARTY] = 0;
            match CurrentEscrow::update_current_escrow_data(data) {
                Ok(()) => {}
                Err(e) => {
                    let _ = trace_num("Error resolving dispute state", e.code() as i64);
                    return e.code();
                }
            }
        }
        a if a.0 == arbitrator
            && data.data[DISPUTE_RAISED] == 1
            && data.data[DISPUTING_PARTY] != DISPUTING_ARB_LOCK =>
        {
            // Arbitrator, active dispute. You can either rule for freelancer or client, where you lock the escrow until it cancels.
            if a.1 == ARB_RULE_FREELANCER {
                return 1;
            }
            if a.1 == ARB_RULE_CLIENT {
                data.data[DISPUTING_PARTY] = DISPUTING_ARB_LOCK;
                match CurrentEscrow::update_current_escrow_data(data) {
                    Ok(()) => {}
                    Err(e) => {
                        let _ = trace_num("Error writing arb ruling", e.code() as i64);
                        return e.code();
                    }
                }
            }
            return 0;
        }
        _ => return 0,
    };
    0
}
