#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_wasm_stdlib::core::ledger_objects::current_escrow::{self, CurrentEscrow};
use xrpl_wasm_stdlib::core::ledger_objects::escrow::Escrow;
use xrpl_wasm_stdlib::core::ledger_objects::traits::{CurrentEscrowFields, EscrowFields};
use xrpl_wasm_stdlib::core::locator::Locator;
use xrpl_wasm_stdlib::core::types::contract_data::XRPL_CONTRACT_DATA_SIZE;
use xrpl_wasm_stdlib::core::types::keylets::XRPL_KEYLET_SIZE;
use xrpl_wasm_stdlib::host;
use xrpl_wasm_stdlib::host::Error::InternalError;
use xrpl_wasm_stdlib::host::error_codes::match_result_code;
use xrpl_wasm_stdlib::host::get_tx_nested_field;
use xrpl_wasm_stdlib::host::trace::{DataRepr, trace_data, trace_num};
use xrpl_wasm_stdlib::host::{Error, Result, Result::Err, Result::Ok};
use xrpl_wasm_stdlib::sfield;
use xrpl_wasm_stdlib::types::{ContractData, XRPL_CONTRACT_DATA_SIZE as TX_CONTRACT_DATA_SIZE};

// Security constants for validation
const VALIDATION_FAILED: i32 = 0;
const KEYLET_PLUS_TIMESTAMP_SIZE: usize = 36;

/*
/// Validates if the provided WASM bytes represent a compatible atomic_swap2 contract.
///
/// In a production system, this should validate the WASM hash against known good versions.
/// For this implementation, we perform basic validation by checking WASM structure and size.
///
/// # Arguments
/// * `wasm_bytes` - The WASM bytecode to validate
///
/// # Returns
/// * `true` if the WASM appears to be a valid atomic_swap2 contract
/// * `false` if validation fails
fn is_valid_atomic_swap2_wasm(wasm_bytes: &[u8]) -> bool {
    // Basic WASM validation - check for WASM magic number
    if wasm_bytes.len() < 8 {
        return false;
    }

    // PRODUCTION CONSIDERATION: In production, implement proper hash-based validation:
    // let expected_hash = sha256(wasm_bytes);
    // expected_hash == KNOWN_ATOMIC_SWAP2_HASH

    true
}
*/

/// Extracts the first memo from the transaction.
///
/// This function uses a Locator to navigate the transaction structure:
/// - Memos[0].MemoData contains the counterpart escrow keylet
/// - Returns the memo data as a byte array and its length
/// - Used to get the 32-byte keylet of the counterpart escrow
#[unsafe(no_mangle)]
pub fn get_first_memo() -> Result<Option<(ContractData, usize)>> {
    let mut data: ContractData = [0; TX_CONTRACT_DATA_SIZE];
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
        result_code if result_code > 0 => Ok(Some((data, result_code as usize))),
        0 => Err(InternalError),
        result_code => Err(Error::from_code(result_code)),
    }
}

/// Phase 1: Initialization - validate counterpart escrow and set timing deadline.
///
/// This function:
/// 1. Extracts counterpart escrow keylet from transaction memo
/// 2. Loads and validates the counterpart escrow from the ledger
/// 3. Verifies account reversal (A→B references B→A)
/// 4. Retrieves CancelAfter as the swap deadline
/// 5. Stores the counterpart keylet + deadline in the data field
/// 6. Returns 0 to wait for Phase 2
fn phase1_initialize(current_escrow: &CurrentEscrow) -> i32 {
    let _ = trace_num("Phase 1: Initialization", 0);

    // Extract the counterpart escrow keylet from transaction memo
    let (memo, memo_len) = match get_first_memo() {
        Ok(v) => match v {
            Some(v) => v,
            None => {
                let _ = trace_num(
                    "No memo provided - atomic swap requires counterpart reference",
                    0,
                );
                return VALIDATION_FAILED;
            }
        },
        Err(e) => {
            let _ = trace_num("Error getting first memo:", e.code() as i64);
            return e.code();
        }
    };

    // Validate memo contains a full 32-byte keylet
    if memo_len != XRPL_KEYLET_SIZE {
        let _ = trace_num("Memo too short, expected 32 bytes, got:", memo_len as i64);
        return VALIDATION_FAILED;
    }

    // Extract the counterpart escrow keylet (first 32 bytes of memo)
    let counterpart_escrow_id: [u8; XRPL_KEYLET_SIZE] = memo[0..32].try_into().unwrap();
    let _ = trace_data(
        "Counterpart escrow ID from memo:",
        &counterpart_escrow_id,
        DataRepr::AsHex,
    );

    // Load the counterpart escrow from the ledger
    let counterpart_slot = unsafe {
        host::cache_ledger_obj(
            counterpart_escrow_id.as_ptr(),
            counterpart_escrow_id.len(),
            0,
        )
    };
    if counterpart_slot < 0 {
        let _ = trace_num(
            "Failed to cache counterpart escrow, error:",
            counterpart_slot as i64,
        );
        return VALIDATION_FAILED;
    }

    let counterpart_escrow = Escrow::new(counterpart_slot);
    let _ = trace_num("Starting counterpart security validation", 0);

    // Data Field Validation: Verify counterpart's data field structure
    let counterpart_data = match counterpart_escrow.get_data() {
        Ok(data) => data,
        Err(e) => {
            let _ = trace_num("Error getting counterpart data:", e.code() as i64);
            return e.code();
        }
    };

    // For atomic_swap2: data must be 32 bytes (Phase 1) or 36 bytes (Phase 2)
    if counterpart_data.len != XRPL_KEYLET_SIZE
        && counterpart_data.len != KEYLET_PLUS_TIMESTAMP_SIZE
    {
        let _ = trace_num(
            "Counterpart data field invalid length, expected 32 or 36 bytes, got:",
            counterpart_data.len as i64,
        );
        return VALIDATION_FAILED;
    }

    let _ = trace_data(
        "Counterpart data field:",
        &counterpart_data.data[0..counterpart_data.len],
        DataRepr::AsHex,
    );

    // Get current escrow's account and destination fields
    let current_account = match current_escrow.get_account() {
        Ok(account) => account,
        Err(e) => {
            let _ = trace_num("Error getting current escrow account:", e.code() as i64);
            return e.code();
        }
    };

    let current_destination = match current_escrow.get_destination() {
        Ok(destination) => destination,
        Err(e) => {
            let _ = trace_num("Error getting current escrow destination:", e.code() as i64);
            return e.code();
        }
    };

    // Get counterpart escrow's account and destination fields
    let counterpart_account = match counterpart_escrow.get_account() {
        Ok(account) => account,
        Err(e) => {
            let _ = trace_num("Error getting counterpart escrow account:", e.code() as i64);
            return e.code();
        }
    };

    let counterpart_destination = match counterpart_escrow.get_destination() {
        Ok(destination) => destination,
        Err(e) => {
            let _ = trace_num(
                "Error getting counterpart escrow destination:",
                e.code() as i64,
            );
            return e.code();
        }
    };

    // ATOMIC SWAP VALIDATION: Verify inverted account correlations
    if current_account.0 != counterpart_destination.0 {
        let _ = trace_data("Current account:", &current_account.0, DataRepr::AsHex);
        let _ = trace_data(
            "Expected counterpart destination:",
            &counterpart_destination.0,
            DataRepr::AsHex,
        );
        return VALIDATION_FAILED;
    }

    if current_destination.0 != counterpart_account.0 {
        let _ = trace_data(
            "Current destination:",
            &current_destination.0,
            DataRepr::AsHex,
        );
        let _ = trace_data(
            "Expected counterpart account:",
            &counterpart_account.0,
            DataRepr::AsHex,
        );
        return VALIDATION_FAILED;
    }

    let _ = trace_num("All counterpart security validations passed", 0);

    // Get current escrow's CancelAfter field - this becomes our swap deadline
    let cancel_after = match current_escrow.get_cancel_after() {
        Ok(Some(cancel_after)) => cancel_after,
        Ok(None) => {
            let _ = trace_num("Current escrow has no CancelAfter field", 0);
            return VALIDATION_FAILED;
        }
        Err(e) => {
            let _ = trace_num("Error getting CancelAfter:", e.code() as i64);
            return e.code();
        }
    };

    let _ = trace_num("Current escrow CancelAfter:", cancel_after as i64);

    // Build new data field: counterpart keylet (32 bytes) + CancelAfter (4 bytes)
    let mut new_data = xrpl_wasm_stdlib::core::types::contract_data::ContractData {
        data: [0u8; XRPL_CONTRACT_DATA_SIZE],
        len: 0,
    };
    new_data.data[0..XRPL_KEYLET_SIZE].copy_from_slice(&counterpart_escrow_id);
    let cancel_after_bytes = cancel_after.to_le_bytes();
    new_data.data[XRPL_KEYLET_SIZE..KEYLET_PLUS_TIMESTAMP_SIZE]
        .copy_from_slice(&cancel_after_bytes);
    new_data.len = KEYLET_PLUS_TIMESTAMP_SIZE;

    let _ = trace_num("Updated data length:", new_data.len as i64);
    let _ = trace_data(
        "Updated data:",
        &new_data.data[0..new_data.len],
        DataRepr::AsHex,
    );

    // Persist the updated data field to the escrow object
    match <CurrentEscrow as CurrentEscrowFields>::update_current_escrow_data(new_data) {
        Ok(()) => {
            let _ = trace_num("Successfully updated escrow data", 0);
        }
        Err(e) => {
            let _ = trace_num("Error updating escrow data:", e.code() as i64);
            return e.code();
        }
    }

    // Return 0 (failure) to indicate phase 1 complete, wait for phase 2
    let _ = trace_num("Phase 1 complete - waiting for Phase 2", 0);
    0
}

/// Phase 2: Timing validation - check if we're within the deadline.
///
/// This function:
/// 1. Extracts the CancelAfter timestamp from the data field
/// 2. Gets the current ledger time
/// 3. Validates that current time < CancelAfter (within deadline)
/// 4. Returns 1 (success) if within deadline, 0 (failure) if expired
fn phase2_complete(
    current_data: &xrpl_wasm_stdlib::core::types::contract_data::ContractData,
) -> i32 {
    let _ = trace_num("Phase 2: Timing validation", 0);

    // Validate data field contains at least 36 bytes (32 bytes keylet + 4 bytes timing)
    if current_data.len < KEYLET_PLUS_TIMESTAMP_SIZE {
        let _ = trace_num(
            "Invalid data length for Phase 2, expected at least 36 bytes, got:",
            current_data.len as i64,
        );
        return VALIDATION_FAILED;
    }

    // Extract the CancelAfter timestamp from the last 4 bytes of data field
    let cancel_after_bytes: [u8; 4] = current_data.data
        [XRPL_KEYLET_SIZE..KEYLET_PLUS_TIMESTAMP_SIZE]
        .try_into()
        .unwrap();
    let cancel_after = u32::from_le_bytes(cancel_after_bytes);
    let _ = trace_num("Extracted CancelAfter:", cancel_after as i64);

    // Get current ledger time for deadline comparison
    let current_time = unsafe {
        let result_code = host::get_parent_ledger_time();
        match_result_code(result_code, || Some(result_code as u32))
    };

    let current_time = match current_time {
        Ok(Some(time)) => time,
        Ok(None) => {
            let _ = trace_num("Failed to get parent ledger time", 0);
            return VALIDATION_FAILED;
        }
        Err(e) => {
            let _ = trace_num("Error getting parent ledger time:", e.code() as i64);
            return e.code();
        }
    };

    let _ = trace_num("Current ledger time:", current_time as i64);

    // ATOMIC SWAP TIMING VALIDATION
    if current_time < cancel_after {
        let _ = trace_num("Atomic swap executed before CancelAfter - success!", 0);
        1 // Success - escrow completes within deadline
    } else {
        let _ = trace_num("Atomic swap attempted after CancelAfter - failed", 0);
        0 // Failure - deadline exceeded, swap expired
    }
}

/// Main finish function for memo-based atomic swap validation with two-phase execution.
///
/// This function implements a stateful atomic swap using the escrow's data field:
///
/// PHASE 1 (data.len == 0):
/// 1. Extracts counterpart escrow keylet from transaction memo
/// 2. Validates the counterpart escrow exists and accounts are reversed
/// 3. Stores counterpart keylet + CancelAfter timestamp in data field
/// 4. Returns 0 (failure) to wait for the second execution
///
/// PHASE 2 (data.len > 0):
/// 1. Extracts the CancelAfter timestamp from the data field
/// 2. Gets the current ledger time
/// 3. Validates that current time < CancelAfter (within deadline)
/// 4. Returns 1 (success) if within deadline, 0 (failure) if expired
#[unsafe(no_mangle)]
pub extern "C" fn finish() -> i32 {
    let current_escrow = current_escrow::get_current_escrow();

    // Get the current data field - this stores the atomic swap state
    // FIELD_NOT_FOUND (-2) means no data field exists yet, which indicates Phase 1
    let current_data = match current_escrow.get_data() {
        Ok(data) => data,
        Err(e) => {
            // If the data field doesn't exist, this is Phase 1
            if e.code() == xrpl_wasm_stdlib::host::error_codes::FIELD_NOT_FOUND {
                let _ = trace_num("No data field found - this is Phase 1", 0);
                return phase1_initialize(&current_escrow);
            }
            let _ = trace_num("Error getting current escrow data:", e.code() as i64);
            return e.code();
        }
    };

    let _ = trace_num("Current data length:", current_data.len as i64);

    // STATE MACHINE: Determine execution phase based on data field length
    // Phase 1: data.len == 0 (no state stored yet)
    // Phase 2: data.len >= 36 (contains counterpart keylet + timing data)
    if current_data.len == 0 {
        phase1_initialize(&current_escrow)
    } else {
        phase2_complete(&current_data)
    }
}
