#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_wasm_stdlib::core::ledger_objects::current_escrow;
use xrpl_wasm_stdlib::core::ledger_objects::escrow::Escrow;
use xrpl_wasm_stdlib::core::ledger_objects::traits::{CurrentEscrowFields, EscrowFields};
use xrpl_wasm_stdlib::core::locator::Locator;
use xrpl_wasm_stdlib::core::types::keylets::XRPL_KEYLET_SIZE;
use xrpl_wasm_stdlib::host;
use xrpl_wasm_stdlib::host::Error::InternalError;
use xrpl_wasm_stdlib::host::get_tx_nested_field;
use xrpl_wasm_stdlib::host::trace::{DataRepr, trace_data, trace_num};
use xrpl_wasm_stdlib::host::{Error, Result, Result::Err, Result::Ok};
use xrpl_wasm_stdlib::sfield;
use xrpl_wasm_stdlib::types::{ContractData, XRPL_CONTRACT_DATA_SIZE};

// Security constants for validation
const VALIDATION_FAILED: i32 = 0;

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

    // Check WASM magic number (0x00 0x61 0x73 0x6D)
    if wasm_bytes[0..4] != [0x00, 0x61, 0x73, 0x6D] {
        return false;
    }

    // Check WASM version (0x01 0x00 0x00 0x00)
    if wasm_bytes[4..8] != [0x01, 0x00, 0x00, 0x00] {
        return false;
    }

    // Additional validation: Check reasonable size range for atomic_swap2 contracts
    // Typical atomic_swap2 WASM should be between 1KB and 100KB
    if wasm_bytes.len() < 1024 || wasm_bytes.len() > 102400 {
        return false;
    }

    // TODO: In production, implement proper hash-based validation:
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
    let mut data: ContractData = [0; XRPL_CONTRACT_DATA_SIZE];
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

/// Main finish function for memo-based atomic swap validation.
///
/// This function implements the core atomic swap logic:
/// 1. Extracts counterpart escrow keylet from transaction memo
/// 2. Loads the counterpart escrow from the ledger using cache_ledger_obj
/// 3. Verifies account reversal: current.account == counterpart.destination
/// 4. Returns 1 (success) only if all atomic swap conditions are met
///
/// The atomic swap property is enforced by requiring mutual validation:
/// - Escrow A (Alice→Bob) references Escrow B's keylet in memo
/// - Escrow B (Bob→Alice) references Escrow A's keylet in memo
/// - Both must validate their counterpart before completing
#[unsafe(no_mangle)]
pub extern "C" fn finish() -> i32 {
    // Extract the counterpart escrow keylet from transaction memo
    let (memo, memo_len) = match get_first_memo() {
        Ok(v) => {
            match v {
                Some(v) => v,
                None => return 0, // No memo provided - atomic swap requires counterpart reference
            }
        }
        Err(e) => {
            let _ = trace_num("Error getting first memo:", e.code() as i64);
            return e.code();
        }
    };

    // Validate memo contains a full 32-byte keylet
    if memo_len < XRPL_KEYLET_SIZE {
        let _ = trace_num(
            "Memo too short, expected at least 32 bytes, got:",
            memo_len as i64,
        );
        return VALIDATION_FAILED;
    }

    // Extract the counterpart escrow keylet (first 32 bytes of memo)
    let escrow_id: [u8; XRPL_KEYLET_SIZE] = memo[0..32].try_into().unwrap();
    let _ = trace_data(
        "Counterpart escrow ID from memo:",
        &escrow_id,
        DataRepr::AsHex,
    );

    // Load the counterpart escrow from the ledger
    // This will fail if the escrow doesn't exist or has been consumed
    let counterpart_slot =
        unsafe { host::cache_ledger_obj(escrow_id.as_ptr(), escrow_id.len(), 0) };
    if counterpart_slot < 0 {
        let _ = trace_num(
            "Failed to cache counterpart escrow, error:",
            counterpart_slot as i64,
        );
        return VALIDATION_FAILED;
    }

    let counterpart_escrow = Escrow::new(counterpart_slot);

    // ENHANCED SECURITY VALIDATION: Verify counterpart escrow properties
    let _ = trace_num("Starting counterpart security validation", 0);

    // 1. WASM Validation: TEMPORARILY DISABLED
    // The FinishFunction field can be larger than the 4KB buffer limit enforced by the host.
    // TODO: Implement hash-based validation instead of retrieving the full WASM bytecode.
    // For now, we skip WASM validation and rely on other security checks.
    /*
    let counterpart_finish_function = match counterpart_escrow.get_finish_function() {
        Ok(Some(wasm)) => wasm,
        Ok(None) => {
            let _ = trace_num(
                "Counterpart escrow has no FinishFunction - security fail",
                0,
            );
            return VALIDATION_FAILED;
        }
        Err(e) => {
            let _ = trace_num("Error getting counterpart FinishFunction:", e.code() as i64);
            return e.code();
        }
    };

    // Validate that the counterpart uses a compatible WASM contract
    if !is_valid_atomic_swap2_wasm(
        &counterpart_finish_function.data[..counterpart_finish_function.len],
    ) {
        let _ = trace_num("Counterpart WASM validation failed - not atomic_swap2", 0);
        return VALIDATION_FAILED;
    }
    let _ = trace_num("Counterpart WASM validation passed", 0);
    */

    // 2. Data Field Validation: Verify counterpart's data field structure
    let counterpart_data = match counterpart_escrow.get_data() {
        Ok(data) => data,
        Err(e) => {
            let _ = trace_num("Error getting counterpart data:", e.code() as i64);
            return e.code();
        }
    };

    // For atomic_swap2 Phase 1: data must be exactly 32 bytes (first escrow keylet)
    // For atomic_swap2 Phase 2: data must be 36 bytes (keylet + timestamp)
    // We'll accept both as valid states since the counterpart might be in either phase
    if counterpart_data.len != XRPL_KEYLET_SIZE && counterpart_data.len != (XRPL_KEYLET_SIZE + 4) {
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

    // TODO: Ideally, we would validate that counterpart_data contains current escrow's keylet
    // This requires calculating current escrow's keylet from account + sequence
    // For now, we rely on the existing account reversal validation below

    let _ = trace_num("All counterpart security validations passed", 0);

    // Get current escrow's account and destination fields
    let current_escrow = current_escrow::get_current_escrow();
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

    // ATOMIC SWAP VALIDATION: Verify account reversal
    // For a valid atomic swap:
    // - Current escrow (A→B) should reference counterpart escrow (B→A)
    // - current.account should equal counterpart.destination
    // - current.destination should equal counterpart.account
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

    // All atomic swap conditions verified - allow escrow to complete
    1
}
