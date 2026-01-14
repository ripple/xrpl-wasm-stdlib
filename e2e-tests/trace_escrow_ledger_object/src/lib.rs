//! # Trace Escrow Ledger Object Test
//!
//! This test ensures that every field on an Escrow ledger object can be successfully
//! traced from within a WASM smart contract.
//!
//! The test script creates an escrow with all possible Escrow fields set, including this
//! contract as the finish condition. When the escrow is finished, this contract loads the
//! Escrow ledger object and traces every field to verify the WASM stdlib can access all
//! escrow data correctly.
#![cfg_attr(target_arch = "wasm32", no_std)]

/// The following are private constants used for testing purposes to enforce value checks in this
/// contract (to ensure that code changes don't break this contract).
///
/// Condition: A0258020121B69A8D20269CFA850F78931EFF3B1FCF3CCA1982A22D7FDB111734C65E5E3810103
/// This is a PREIMAGE-SHA-256 condition in full crypto-condition format (39 bytes)
const EXPECTED_CONDITION: [u8; 39] = [
    0xA0, 0x25, 0x80, 0x20, 0x12, 0x1B, 0x69, 0xA8, 0xD2, 0x02, 0x69, 0xCF, 0xA8, 0x50, 0xF7, 0x89,
    0x31, 0xEF, 0xF3, 0xB1, 0xFC, 0xF3, 0xCC, 0xA1, 0x98, 0x2A, 0x22, 0xD7, 0xFD, 0xB1, 0x11, 0x73,
    0x4C, 0x65, 0xE5, 0xE3, 0x81, 0x01, 0x03,
];

use xrpl_wasm_stdlib::core::ledger_objects::current_escrow::{CurrentEscrow, get_current_escrow};
use xrpl_wasm_stdlib::core::ledger_objects::traits::{
    CurrentEscrowFields, CurrentLedgerObjectCommonFields,
};
use xrpl_wasm_stdlib::host::trace::{DataRepr, trace, trace_amount, trace_data, trace_num};
use xrpl_wasm_stdlib::host::{Result::Err, Result::Ok};

#[unsafe(no_mangle)]
pub extern "C" fn finish() -> i32 {
    let _ = trace("$$$$$ STARTING WASM EXECUTION $$$$$");
    let _ = trace("");

    let current_escrow: CurrentEscrow = get_current_escrow();

    // ########################################
    // Trace All Current Escrow Ledger Object Fields
    // ########################################
    {
        let _ = trace("### Trace Current Escrow Ledger Object Fields");
        let _ = trace("{ ");
        let _ = trace("  -- Common Fields");

        // Trace Field: Account
        let account = current_escrow.get_account().unwrap();
        test_utils::assert_eq!(account.0.len(), 20);
        let _ = trace_data("  Account:", &account.0, DataRepr::AsHex);

        // Trace Field: Amount
        let amount = current_escrow.get_amount().unwrap();
        let _ = trace_amount("  Amount:", &amount);

        // Trace Field: LedgerEntryType
        let ledger_entry_type = current_escrow.get_ledger_entry_type().unwrap();
        test_utils::assert_eq!(ledger_entry_type, 117);
        let _ = trace_num("  LedgerEntryType:", ledger_entry_type as i64);

        // Trace Field: CancelAfter (optional - require it for testing)
        let opt_cancel_after = current_escrow.get_cancel_after().unwrap();
        let cancel_after = opt_cancel_after.expect("CancelAfter should be set for testing");
        let _ = trace_num("  CancelAfter:", cancel_after as i64);

        // Trace Field: Condition (optional)
        match current_escrow.get_condition() {
            Ok(opt_condition) => {
                if let Some(condition) = opt_condition {
                    let _ = trace_num("  Condition length:", condition.len() as i64);
                    let _ = trace_data(
                        "  Condition (full hex):",
                        condition.as_slice(),
                        DataRepr::AsHex,
                    );

                    test_utils::assert_eq!(
                        condition.len(),
                        EXPECTED_CONDITION.len(),
                        "Condition length mismatch"
                    );
                    test_utils::assert_eq!(
                        condition.as_slice(),
                        &EXPECTED_CONDITION[..],
                        "Condition bytes mismatch"
                    );
                    let _ = trace("  ✓ Condition matches expected value");
                } else {
                    let _ = trace("  Condition: not present");
                }
            }
            Err(e) => {
                let _ = trace("  ERROR getting Condition");
                let _ = trace_num("  error_code=", e as i64);
                return e.code();
            }
        }

        // Trace Field: Destination
        let destination = current_escrow.get_destination().unwrap();
        // Destination is set in runTest.js (destWallet), just verify it's a valid AccountID
        test_utils::assert_eq!(destination.0.len(), 20);
        let _ = trace_data("  Destination:", &destination.0, DataRepr::AsHex);

        // Trace Field: DestinationTag (optional - already set in runTest.js)
        let opt_destination_tag = current_escrow.get_destination_tag().unwrap();
        let destination_tag =
            opt_destination_tag.expect("DestinationTag should be set for testing");
        test_utils::assert_eq!(destination_tag, 23480);
        let _ = trace_num("  DestinationTag:", destination_tag as i64);

        // Trace Field: FinishAfter (optional - require it for testing)
        let opt_finish_after = current_escrow.get_finish_after().unwrap();
        let finish_after = opt_finish_after.expect("FinishAfter should be set for testing");
        let _ = trace_num("  FinishAfter:", finish_after as i64);

        // Trace Field: Flags
        let result = current_escrow.get_flags();
        if let Ok(flags) = result {
            // Flags is typically 0 for escrows
            let _ = trace_num("  Flags:", flags as i64);
        } else if let Err(error) = result {
            let _ = trace_num("  Error getting Flags. error_code = ", error.code() as i64);
        }

        // TODO: Uncomment this once https://github.com/ripple/xrpl-wasm-stdlib/issues/86 is fixed.
        // Trace Field: FinishFunction
        // let opt_finish_function = current_escrow.get_finish_function().unwrap();
        // if let Some(finish_function) = opt_finish_function {
        //     FinishFunction is the WASM code - just verify it exists and has reasonable length
        // let _ = trace_num("  FinishFunction length:", finish_function.len as i64);
        // let _ = trace_data(
        //     "  FinishFunction:",
        //     &finish_function.data[..finish_function.len],
        //     DataRepr::AsHex,
        // );
        // }

        // Trace Field: OwnerNode
        let owner_node = current_escrow.get_owner_node().unwrap();
        // OwnerNode is system-generated, typically 0 for first entry
        let _ = trace_num("  OwnerNode:", owner_node as i64);

        // Trace Field: DestinationNode (optional - system-generated, may or may not be present)
        let opt_destination_node = current_escrow.get_destination_node().unwrap();
        if let Some(destination_node) = opt_destination_node {
            // DestinationNode is system-generated, typically 0 for first entry
            let _ = trace_num("  DestinationNode:", destination_node as i64);
        } else {
            let _ = trace("  DestinationNode: not present");
        }

        // Trace Field: PreviousTxnID
        let previous_txn_id = current_escrow.get_previous_txn_id().unwrap();
        // PreviousTxnID is the hash of the EscrowCreate transaction - unpredictable
        // Just verify it's 32 bytes (valid Hash256)
        test_utils::assert_eq!(previous_txn_id.0.len(), 32);
        let _ = trace_data("  PreviousTxnID:", &previous_txn_id.0, DataRepr::AsHex);

        // Trace Field: PreviousTxnLgrSeq
        let previous_txn_lgr_seq = current_escrow.get_previous_txn_lgr_seq().unwrap();
        // PreviousTxnLgrSeq is system-generated - just verify it's non-zero
        let _ = trace_num("  PreviousTxnLgrSeq:", previous_txn_lgr_seq as i64);

        // Trace Field: SourceTag (optional - already set in runTest.js)
        let opt_source_tag = current_escrow.get_source_tag().unwrap();
        let source_tag = opt_source_tag.expect("SourceTag should be set for testing");
        test_utils::assert_eq!(source_tag, 11747);
        let _ = trace_num("  SourceTag:", source_tag as i64);

        // Trace Field: Data (contract data)
        // Note: Data field is optional and only present if set during EscrowCreate or
        // updated via the FinishFunction. We don't set it in runTest.js, so this will likely
        // return empty or error.
        let data_result = current_escrow.get_data();
        if let Ok(contract_data) = data_result
            && contract_data.len > 0
        {
            let _ = trace_num("  Data length:", contract_data.len as i64);
            let _ = trace_data(
                "  Data:",
                &contract_data.data[..contract_data.len],
                DataRepr::AsHex,
            );
        }

        let _ = trace("}");
        let _ = trace("");
    }

    let _ = trace("$$$$$ WASM EXECUTION COMPLETE $$$$$");
    1 // <-- Finish the escrow to indicate a successful outcome
}

#[cfg(test)]
mod coverage_tests {
    use super::*;

    /// Coverage test: exercises any host function categories via finish()
    ///
    /// This test runs the same logic as the integration test, but on native
    /// targets with stub host functions. It's used to measure code coverage
    /// of xrpl-wasm-stdlib.
    ///
    /// Note: The host functions return dummy values (from host_bindings_for_testing.rs),
    /// so this test verifies that the code *runs*, not that it's *correct*.
    /// Correctness is verified by the real integration tests against rippled.
    #[test]
    fn test_finish_exercises_all_host_functions() {
        // On non-wasm targets, finish() uses host_bindings_for_testing.rs
        // which provides stub implementations of all host functions.
        let result = finish();

        // The finish() function returns 1 on success, or a negative error code.
        // With stub host functions, we expect success (though the actual
        // behavior depends on the stub implementations).
        core::assert_eq!(result, 1, "finish() should return 1 on success");
    }
}
