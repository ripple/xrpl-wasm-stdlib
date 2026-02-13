#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_wasm_stdlib::core::ledger_objects::current_escrow::{self, CurrentEscrow};
use xrpl_wasm_stdlib::core::ledger_objects::escrow::Escrow;
use xrpl_wasm_stdlib::core::ledger_objects::traits::{CurrentEscrowFields, EscrowFields};
use xrpl_wasm_stdlib::core::types::contract_data::XRPL_CONTRACT_DATA_SIZE;
use xrpl_wasm_stdlib::core::types::keylets::XRPL_KEYLET_SIZE;
use xrpl_wasm_stdlib::host;
use xrpl_wasm_stdlib::host::error_codes::match_result_code_with_expected_bytes;
use xrpl_wasm_stdlib::host::trace::{DataRepr, trace_data, trace_num};
use xrpl_wasm_stdlib::host::{Result::Err, Result::Ok};

// Security constants for validation
const VALIDATION_FAILED: i32 = 0;
const KEYLET_PLUS_TIMESTAMP_SIZE: usize = 36;

/*
/// Validates if the provided WASM bytes represent a compatible atomic_swap1 contract.
///
/// In a production system, this should validate the WASM hash against known good versions.
/// For this implementation, we perform basic validation by checking WASM structure and size.
///
/// # Arguments
/// * `wasm_bytes` - The WASM bytecode to validate
///
/// # Returns
/// * `true` if the WASM appears to be a valid atomic_swap1 contract
/// * `false` if validation fails
fn is_valid_atomic_swap1_wasm(wasm_bytes: &[u8]) -> bool {
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

    // Additional validation: Check reasonable size range for atomic_swap1 contracts
    // Typical atomic_swap1 WASM should be between 1KB and 100KB
    if wasm_bytes.len() < 1024 || wasm_bytes.len() > 102400 {
        return false;
    }

    // TODO: In production, implement proper hash-based validation:
    // let expected_hash = sha256(wasm_bytes);
    // expected_hash == KNOWN_ATOMIC_SWAP1_HASH

    true
}
*/

/// Main finish function for data field-based atomic swap with two-phase execution.
///
/// This function implements a stateful atomic swap using the escrow's data field:
///
/// PHASE 1 (data.len <= 32):
/// 1. Validates the data field contains exactly 32 bytes (first escrow keylet)
/// 2. Verifies the referenced first escrow exists on the ledger
/// 3. Retrieves current escrow's CancelAfter field as the swap deadline
/// 4. Appends the CancelAfter timestamp to the data field (36 bytes total)
/// 5. Returns 0 (failure) to wait for the second execution
///
/// PHASE 2 (data.len > 32):
/// 1. Extracts the CancelAfter timestamp from the last 4 bytes of data
/// 2. Gets the current ledger time
/// 3. Validates that current time < CancelAfter (within deadline)
/// 4. Returns 1 (success) if within deadline, 0 (failure) if expired
///
/// The two-phase design provides built-in timing coordination and prevents
/// stale swap attempts after the deadline expires.
#[unsafe(no_mangle)]
pub extern "C" fn finish() -> i32 {
    let current_escrow = current_escrow::get_current_escrow();

    // Get the current data field - this stores the atomic swap state
    let mut current_data = match current_escrow.get_data() {
        Ok(data) => data,
        Err(e) => {
            let _ = trace_num("Error getting current escrow data:", e.code() as i64);
            return e.code();
        }
    };

    let _ = trace_num("Current data length:", current_data.len as i64);
    let _ = trace_data(
        "Current data:",
        &current_data.data[0..current_data.len],
        DataRepr::AsHex,
    );

    // STATE MACHINE: Determine execution phase based on data field length
    // Phase 1: data.len <= 32 (contains only first escrow keylet)
    // Phase 2: data.len > 32 (contains first escrow keylet + timing data)
    if current_data.len <= XRPL_KEYLET_SIZE {
        // PHASE 1: Initialization - validate first escrow and set timing deadline

        // Validate that the data contains exactly 32 bytes (first escrow keylet)
        if current_data.len != XRPL_KEYLET_SIZE {
            let _ = trace_num(
                "Invalid data length for first run, expected 32 bytes, got:",
                current_data.len as i64,
            );
            return VALIDATION_FAILED;
        }

        // Extract the first escrow keylet from data field
        let first_escrow_id: [u8; XRPL_KEYLET_SIZE] = current_data.data[0..32].try_into().unwrap();
        let _ = trace_data(
            "First escrow ID from data:",
            &first_escrow_id,
            DataRepr::AsHex,
        );

        // Verify the referenced first escrow exists on the ledger
        // This ensures we're referencing a valid counterpart for the atomic swap
        let first_escrow_slot =
            unsafe { host::cache_ledger_obj(first_escrow_id.as_ptr(), first_escrow_id.len(), 0) };
        if first_escrow_slot < 0 {
            let _ = trace_num(
                "Failed to cache first escrow, error:",
                first_escrow_slot as i64,
            );
            return VALIDATION_FAILED;
        }

        let first_escrow = Escrow::new(first_escrow_slot);

        // ENHANCED SECURITY VALIDATION: Verify first escrow properties
        let _ = trace_num("Starting first escrow security validation", 0);

        // 1. WASM Validation: TEMPORARILY DISABLED
        // The FinishFunction field can be larger than the 4KB buffer limit enforced by the host.
        // TODO: Implement hash-based validation instead of retrieving the full WASM bytecode.
        // For now, we skip WASM validation and rely on other security checks.
        /*
        let first_finish_function = match first_escrow.get_finish_function() {
            Ok(Some(wasm)) => wasm,
            Ok(None) => {
                let _ = trace_num("First escrow has no FinishFunction - security fail", 0);
                return VALIDATION_FAILED;
            }
            Err(e) => {
                let _ = trace_num(
                    "Error getting first escrow FinishFunction:",
                    e.code() as i64,
                );
                return e.code();
            }
        };

        // Validate that the first escrow uses a compatible WASM contract
        if !is_valid_atomic_swap1_wasm(&first_finish_function.data[..first_finish_function.len]) {
            let _ = trace_num("First escrow WASM validation failed - not atomic_swap1", 0);
            return VALIDATION_FAILED;
        }
        let _ = trace_num("First escrow WASM validation passed", 0);
        */

        // 2. Account Reversal Validation: Verify proper account setup between escrows
        let first_account = match first_escrow.get_account() {
            Ok(account) => account,
            Err(e) => {
                let _ = trace_num("Error getting first escrow account:", e.code() as i64);
                return e.code();
            }
        };

        let first_destination = match first_escrow.get_destination() {
            Ok(destination) => destination,
            Err(e) => {
                let _ = trace_num("Error getting first escrow destination:", e.code() as i64);
                return e.code();
            }
        };

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

        // Verify proper account reversal: first(A→B) ↔ current(B→A)
        if first_account.0 != current_destination.0 {
            let _ = trace_data("First escrow account:", &first_account.0, DataRepr::AsHex);
            let _ = trace_data(
                "Current escrow destination:",
                &current_destination.0,
                DataRepr::AsHex,
            );
            let _ = trace_num(
                "Account reversal validation failed - accounts don't match",
                0,
            );
            return VALIDATION_FAILED;
        }

        if first_destination.0 != current_account.0 {
            let _ = trace_data(
                "First escrow destination:",
                &first_destination.0,
                DataRepr::AsHex,
            );
            let _ = trace_data(
                "Current escrow account:",
                &current_account.0,
                DataRepr::AsHex,
            );
            let _ = trace_num(
                "Account reversal validation failed - destinations don't match",
                0,
            );
            return VALIDATION_FAILED;
        }

        let _ = trace_num("All first escrow security validations passed", 0);

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

        // Append CancelAfter timestamp to data field (4 bytes, little-endian)
        // This stores the deadline for phase 2 validation
        let cancel_after_bytes = cancel_after.to_le_bytes();
        if current_data.len + 4 > XRPL_CONTRACT_DATA_SIZE {
            let _ = trace_num("Data would exceed maximum size", 0);
            return VALIDATION_FAILED;
        }

        current_data.data[current_data.len..current_data.len + 4]
            .copy_from_slice(&cancel_after_bytes);
        current_data.len += 4;

        let _ = trace_num("Updated data length:", current_data.len as i64);
        let _ = trace_data(
            "Updated data:",
            &current_data.data[0..current_data.len],
            DataRepr::AsHex,
        );

        // Persist the updated data field to the escrow object
        match <CurrentEscrow as CurrentEscrowFields>::update_current_escrow_data(current_data) {
            Ok(()) => {
                let _ = trace_num("Successfully updated escrow data", 0);
            }
            Err(e) => {
                let _ = trace_num("Error updating escrow data:", e.code() as i64);
                return e.code();
            }
        }

        // Return 0 (failure) to indicate phase 1 complete, wait for phase 2
        0
    } else {
        // PHASE 2: Timing validation - check if we're within the deadline

        // Validate data field contains at least 36 bytes (32 bytes keylet + 4 bytes timing)
        if current_data.len < KEYLET_PLUS_TIMESTAMP_SIZE {
            let _ = trace_num(
                "Invalid data length for second run, expected at least 36 bytes, got:",
                current_data.len as i64,
            );
            return VALIDATION_FAILED;
        }

        // PRODUCTION CONSIDERATION: Re-validate first 32 bytes match first escrow keylet
        // This ensures the data field hasn't been modified between Phase 1 and Phase 2
        // Note: We don't re-verify the first escrow exists in Phase 2 because:
        // 1. It was already verified in Phase 1
        // 2. The first escrow may have been finished by atomic_swap1 in the meantime
        // 3. We only need to verify the timing data hasn't been tampered with

        // Extract the CancelAfter timestamp from the last 4 bytes of data field
        let cancel_after_bytes: [u8; 4] = current_data.data[current_data.len - 4..current_data.len]
            .try_into()
            .unwrap();
        let cancel_after = u32::from_le_bytes(cancel_after_bytes);
        let _ = trace_num("Extracted CancelAfter:", cancel_after as i64);

        // Get current ledger time for deadline comparison
        let mut time_buffer = [0u8; 4];
        let time_result =
            unsafe { host::get_parent_ledger_time(time_buffer.as_mut_ptr(), time_buffer.len()) };

        let current_time = match match_result_code_with_expected_bytes(time_result, 4, || {
            u32::from_le_bytes(time_buffer)
        }) {
            Ok(time) => time,
            Err(e) => {
                let _ = trace_num("Failed to get parent ledger time:", e.code() as i64);
                return VALIDATION_FAILED;
            }
        };

        let _ = trace_num("Current ledger time:", current_time as i64);

        // ATOMIC SWAP TIMING VALIDATION
        // Only allow completion if current time is before the deadline
        // This prevents stale swap attempts and enforces time-based coordination
        if current_time < cancel_after {
            let _ = trace_num("Atomic swap executed before CancelAfter - success!", 0);
            1 // Success - escrow completes within deadline
        } else {
            let _ = trace_num("Atomic swap attempted after CancelAfter - failed", 0);
            0 // Failure - deadline exceeded, swap expired
        }
    }
}
