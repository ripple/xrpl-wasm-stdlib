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

/// Fulfillment: A0058003736868
/// This is a PREIMAGE-SHA-256 fulfillment (7 bytes) for preimage "shh"
const EXPECTED_FULFILLMENT: [u8; 7] = [0xA0, 0x05, 0x80, 0x03, 0x73, 0x68, 0x68];

use xrpl_wasm_stdlib::core::current_tx::escrow_finish::{EscrowFinish, get_current_escrow_finish};
use xrpl_wasm_stdlib::core::current_tx::traits::{EscrowFinishFields, TransactionCommonFields};
use xrpl_wasm_stdlib::core::locator::Locator;
use xrpl_wasm_stdlib::core::types::account_id::AccountID;
use xrpl_wasm_stdlib::core::types::transaction_type::TransactionType;
use xrpl_wasm_stdlib::host;
use xrpl_wasm_stdlib::host::trace::{
    DataRepr, trace, trace_account, trace_account_buf, trace_amount, trace_data, trace_num,
};
use xrpl_wasm_stdlib::sfield;

#[unsafe(no_mangle)]
pub extern "C" fn finish() -> i32 {
    let _ = trace("$$$$$ STARTING WASM EXECUTION $$$$$");
    let _ = trace("");

    // The transaction prompting execution of this contract.
    let escrow_finish: EscrowFinish = get_current_escrow_finish();

    // ########################################
    // Trace All EscrowFinish Fields
    // ########################################
    {
        let _ = trace("### Trace All EscrowFinish Fields");
        let _ = trace("{ ");
        let _ = trace("  -- Common Fields");

        // Trace Field: Account
        let account = escrow_finish.get_account().unwrap();
        // Account is the wallet that submitted the EscrowFinish - verify it's 20 bytes
        test_utils::assert_eq!(account.0.len(), 20);
        let _ = trace_account("  Account:", &account);

        // Trace Field: TransactionType
        let transaction_type: TransactionType = escrow_finish.get_transaction_type().unwrap();
        test_utils::assert_eq!(transaction_type, TransactionType::EscrowFinish);
        let tx_type_bytes: [u8; 2] = transaction_type.into();
        let _ = trace_data(
            "  TransactionType (EscrowFinish):",
            &tx_type_bytes,
            DataRepr::AsHex,
        );

        // Trace Field: ComputationAllowance
        let computation_allowance: u32 = escrow_finish.get_computation_allowance().unwrap();
        test_utils::assert_eq!(computation_allowance, 1000000);
        // ComputationAllowance is set in the transaction - just verify it's reasonable
        let _ = trace_num("  ComputationAllowance:", computation_allowance as i64);

        // Trace Field: Fee
        let fee = escrow_finish.get_fee().unwrap();
        // Fee is system-calculated, just trace it
        let _ = trace_amount("  Fee:", &fee);

        // Trace Field: Sequence
        let sequence: u32 = escrow_finish.get_sequence().unwrap();
        test_utils::assert!(sequence > 0);
        // Sequence is system-generated based on account state
        let _ = trace_num("  Sequence:", sequence as i64);

        // Trace Field: AccountTxnID (optional)
        let opt_account_txn_id = escrow_finish.get_account_txn_id().unwrap();
        if let Some(account_txn_id) = opt_account_txn_id {
            // AccountTxnID is optional - if present, verify it's 32 bytes
            test_utils::assert_eq!(account_txn_id.0.len(), 32);
            let _ = trace_data("  AccountTxnID:", &account_txn_id.0, DataRepr::AsHex);
        }

        // Trace Field: Flags (optional)
        let opt_flags = escrow_finish.get_flags().unwrap();
        if let Some(flags) = opt_flags {
            // Flags are transaction-specific, just trace the value
            let _ = trace_num("  Flags:", flags as i64);
        }

        // Trace Field: LastLedgerSequence (optional)
        let opt_last_ledger_sequence = escrow_finish.get_last_ledger_sequence().unwrap();
        if let Some(last_ledger_sequence) = opt_last_ledger_sequence {
            // LastLedgerSequence is optional, just trace it
            let _ = trace_num("  LastLedgerSequence:", last_ledger_sequence as i64);
        }

        // Trace Field: NetworkID (optional)
        let opt_network_id = escrow_finish.get_network_id().unwrap();
        if let Some(network_id) = opt_network_id {
            // NetworkID identifies the chain, just trace it
            let _ = trace_num("  NetworkID:", network_id as i64);
        }

        // Trace Field: SourceTag (optional - require it for testing)
        let opt_source_tag = escrow_finish.get_source_tag().unwrap();
        let source_tag = opt_source_tag.expect("SourceTag should be set for testing");
        let _ = trace_num("  SourceTag:", source_tag as i64);

        // Trace Field: SigningPubKey (required)
        // For multi-signed transactions, SigningPubKey must be empty (0 bytes)
        let signing_pub_key_result = escrow_finish.get_signing_pub_key();
        match signing_pub_key_result {
            host::Result::Ok(signing_pub_key) => {
                let _ = trace_num("  SigningPubKey length:", signing_pub_key.0.len() as i64);
                // For multi-signed transactions, SigningPubKey should be empty
                // But let's not assert for now, just trace it
                let _ = trace_data("  SigningPubKey:", &signing_pub_key.0, DataRepr::AsHex);
            }
            host::Result::Err(e) => {
                let _ = trace_num("  Error getting SigningPubKey, error_code = ", e as i64);
            }
        }

        // Trace Field: TicketSequence (optional)
        let opt_ticket_sequence = escrow_finish.get_ticket_sequence().unwrap();
        if let Some(ticket_sequence) = opt_ticket_sequence {
            // TicketSequence is used instead of Sequence for ticket-based transactions
            let _ = trace_num("  TicketSequence:", ticket_sequence as i64);
        }

        // Memos array (optional) - require at least one memo for testing
        let array_len = unsafe { host::get_tx_array_len(sfield::Memos.into()) };
        test_utils::assert!(
            array_len > 0,
            "At least one Memo should be present for testing"
        );
        let _ = trace_num("  Memos array len:", array_len as i64);

        for i in 0..array_len {
            let mut memo_buf = [0u8; 1024];
            let mut locator = Locator::new();
            locator.pack(sfield::Memos);
            locator.pack(i);
            locator.pack(sfield::Memo);
            locator.pack(sfield::MemoType);
            let output_len = unsafe {
                host::get_tx_nested_field(
                    locator.as_ptr(),
                    locator.num_packed_bytes(),
                    memo_buf.as_mut_ptr(),
                    memo_buf.len(),
                )
            };
            let _ = trace_num("    Memo #:", i as i64);
            if output_len > 0 {
                let _ = trace_data(
                    "      MemoType:",
                    &memo_buf[..output_len as usize],
                    DataRepr::AsHex,
                );
            }

            locator.repack_last(sfield::MemoData);
            let output_len = unsafe {
                host::get_tx_nested_field(
                    locator.as_ptr(),
                    locator.num_packed_bytes(),
                    memo_buf.as_mut_ptr(),
                    memo_buf.len(),
                )
            };
            if output_len > 0 {
                let _ = trace_data(
                    "      MemoData:",
                    &memo_buf[..output_len as usize],
                    DataRepr::AsHex,
                );
            }

            locator.repack_last(sfield::MemoFormat);
            let output_len = unsafe {
                host::get_tx_nested_field(
                    locator.as_ptr(),
                    locator.num_packed_bytes(),
                    memo_buf.as_mut_ptr(),
                    memo_buf.len(),
                )
            };
            if output_len > 0 {
                let _ = trace_data(
                    "      MemoFormat:",
                    &memo_buf[..output_len as usize],
                    DataRepr::AsHex,
                );
            }
        }

        // Signers array (optional) - require at least one signer for testing
        // TODO: Use this logic to fix https://github.com/ripple/xrpl-wasm-stdlib/issues/90
        let array_len = unsafe { host::get_tx_array_len(sfield::Signers.into()) };
        #[cfg(target_arch = "wasm32")]
        assert!(
            array_len > 0,
            "At least one Signer should be present for testing"
        );
        let _ = trace_num("  Signers array len:", array_len as i64);

        for i in 0..array_len {
            let mut buf = [0x00; 128];
            let mut locator = Locator::new();
            locator.pack(sfield::Signers);
            locator.pack(i);
            // Try without Signer wrapper - maybe the structure is different
            locator.pack(sfield::Account);
            let output_len = unsafe {
                host::get_tx_nested_field(
                    locator.as_ptr(),
                    locator.num_packed_bytes(),
                    buf.as_mut_ptr(),
                    buf.len(),
                )
            };
            if output_len < 0 {
                let _ = trace_num("  cannot get Account, error:", output_len as i64);
                panic!()
            }
            let _ = trace_num("    Signer #:", i as i64);
            // Account should be 20 bytes
            let _ = trace_num("     Account length:", output_len as i64);
            if output_len == 20 {
                let _ = trace_account_buf("     Account:", &buf[..20].try_into().unwrap());
            } else {
                let _ = trace_data(
                    "     Account (unexpected length):",
                    &buf[..output_len as usize],
                    DataRepr::AsHex,
                );
                panic!()
            }

            locator.repack_last(sfield::TxnSignature);
            let output_len = unsafe {
                host::get_tx_nested_field(
                    locator.as_ptr(),
                    locator.num_packed_bytes(),
                    buf.as_mut_ptr(),
                    buf.len(),
                )
            };
            if output_len < 0 {
                let _ = trace_num("  cannot get TxnSignature, error:", output_len as i64);
                panic!()
            }
            let _ = trace_data(
                "     TxnSignature:",
                &buf[..output_len as usize],
                DataRepr::AsHex,
            );

            locator.repack_last(sfield::SigningPubKey);
            let output_len = unsafe {
                host::get_tx_nested_field(
                    locator.as_ptr(),
                    locator.num_packed_bytes(),
                    buf.as_mut_ptr(),
                    buf.len(),
                )
            };
            if output_len < 0 {
                let _ = trace_num(
                    "     Error getting SigningPubKey. error_code = ",
                    output_len as i64,
                );
                panic!()
            }
            // SigningPubKey should be 33 bytes (compressed public key)
            let _ = trace_num("     SigningPubKey length:", output_len as i64);
            let _ = trace_data(
                "     SigningPubKey:",
                &buf[..output_len as usize],
                DataRepr::AsHex,
            );
        }

        // TxnSignature - only present for single-signed transactions
        // Multi-signed transactions use Signers array instead
        match escrow_finish.get_txn_signature() {
            host::Result::Ok(txn_signature) => {
                let _ = trace("  TxnSignature (single-signed):");
                let _ = trace_num("    Length:", txn_signature.len() as i64);
                let _ = trace_data(
                    "    Data:",
                    &txn_signature.data[..txn_signature.len()],
                    DataRepr::AsHex,
                );
            }
            host::Result::Err(_) => {
                let _ = trace("  TxnSignature not present (multi-signed transaction)");
            }
        }

        let _ = trace("  -- EscrowFinish Fields");

        // Trace Field: Owner (required)
        let owner: AccountID = escrow_finish.get_owner().unwrap();
        // Owner is the account that created the escrow - verify it's 20 bytes
        test_utils::assert_eq!(owner.0.len(), 20);
        let _ = trace_account("  Owner:", &owner);

        // Trace Field: OfferSequence (required)
        let offer_sequence: u32 = escrow_finish.get_offer_sequence().unwrap();
        // OfferSequence is the sequence number of the EscrowCreate transaction
        let _ = trace_num("  OfferSequence:", offer_sequence as i64);

        // Trace Field: Condition (optional)
        match escrow_finish.get_condition() {
            host::Result::Ok(opt_condition) => {
                if let Some(condition) = opt_condition {
                    let _ = trace_num("  Condition length:", condition.len() as i64);
                    let _ = trace_data(
                        "  Condition (full hex):",
                        condition.as_slice(),
                        DataRepr::AsHex,
                    );

                    // Assert the condition matches the expected value
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
            host::Result::Err(e) => {
                let _ = trace("  ERROR getting Condition");
                let _ = trace_num("  error_code=", e as i64);
                return e.code();
            }
        }

        // Trace Field: Fulfillment (optional)
        // NOTE: When an escrow has both Condition and FinishFunction, you cannot provide Fulfillment
        // in the EscrowFinish transaction (causes temMALFORMED). The FinishFunction validates the condition.
        let opt_fulfillment = escrow_finish.get_fulfillment().unwrap();
        if let Some(fulfillment) = opt_fulfillment {
            let _ = trace_num("  Fulfillment length:", fulfillment.len() as i64);
            let _ = trace_data(
                "  Fulfillment (hex):",
                fulfillment.as_slice(),
                DataRepr::AsHex,
            );

            // Assert the fulfillment matches the expected value
            test_utils::assert_eq!(
                fulfillment.len(),
                EXPECTED_FULFILLMENT.len(),
                "Fulfillment length mismatch"
            );
            test_utils::assert_eq!(
                fulfillment.as_slice(),
                &EXPECTED_FULFILLMENT[..],
                "Fulfillment bytes mismatch"
            );
            let _ = trace("  ✓ Fulfillment matches expected value");

            // Verify the fulfillment format
            // The Condition field in XRPL is just the 32-byte hash (not the full crypto-condition)
            // The Fulfillment should be a PREIMAGE-SHA-256 fulfillment in crypto-condition format
            let _ = trace("  Verifying Fulfillment format...");

            // For PREIMAGE-SHA-256 fulfillment format: A002 80XX <preimage>
            // A002 = PREIMAGE-SHA-256 fulfillment type
            // 80XX = preimage length (variable length encoding)
            // For empty preimage: A002 8000
            let fulfillment_data = fulfillment.as_slice();
            if fulfillment.len() >= 4 {
                let _ = trace_data(
                    "    Fulfillment type tag:",
                    &fulfillment_data[0..2],
                    DataRepr::AsHex,
                );
                let _ = trace_data(
                    "    Preimage length encoding:",
                    &fulfillment_data[2..4],
                    DataRepr::AsHex,
                );

                // Parse the preimage length
                if fulfillment_data[2] == 0x80 {
                    let preimage_len = fulfillment_data[3] as usize;
                    let _ = trace_num("    Preimage length:", preimage_len as i64);

                    if preimage_len == 0 {
                        let _ = trace("    Preimage is empty (0 bytes)");
                        let _ = trace(
                            "    Expected SHA-256 of empty string: E3B0C44298FC1C149AFBF4C8996FB92427AE41E4649B934CA495991B7852B855",
                        );
                    } else if fulfillment.len() >= 4 + preimage_len {
                        let _ = trace_data(
                            "    Preimage (hex):",
                            &fulfillment_data[4..4 + preimage_len],
                            DataRepr::AsHex,
                        );
                    }
                }
            }
        } else {
            let _ = trace("  Fulfillment: not present (FinishFunction validates condition)");
        }

        // As part of https://github.com/ripple/xrpl-wasm-stdlib/issues/91, we had the concept (for a minute) of a
        // `vector_256` struct to represent a full `Vector256` field. In that design, all bytes of this kind of field
        // would be loaded to get any portion particular value of the vector. This felt both inefficient but also
        // deviated from the Locator style we're employing in this library for array fields (e.g., Memos, Signers, etc).
        // See https://github.com/ripple/xrpl-wasm-stdlib/issues/108 for the issue that tracks fixing this particular
        // portion of this test (Note: this portion of the test will need to be rewritten using the Locator style).
        // CredentialIDs (Vector256 - array of 256-bit hashes)
        // let opt_credential_ids = escrow_finish.get_credential_ids().unwrap();
        // if let Some(credential_ids) = opt_credential_ids {
        //     let _ = trace_num("  Number of CredentialIDs:", credential_ids.len() as i64);
        //     for i in 0..credential_ids.len() {
        //         let cred_id = credential_ids.get(i).unwrap();
        //         let _ = trace_num("  CredentialID index:", i as i64);
        //         let _ = trace_data("    CredentialID:", cred_id.as_bytes(), DataRepr::AsHex);
        //     }
        // } else {
        //     let _ = trace("  No CredentialIDs present");
        // }

        let _ = trace("}");
        let _ = trace(""); // Newline
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

        // The finish() function returns 1 on success or a negative error code.
        // With stub host functions, we expect success (though the actual
        // behavior depends on the stub implementations).
        core::assert_eq!(result, 1, "finish() should return 1 on success");
    }
}
