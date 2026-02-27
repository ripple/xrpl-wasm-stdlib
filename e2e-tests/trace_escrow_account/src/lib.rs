//! # Trace Escrow Account Test
//!
//! This test ensures that every field on an AccountRoot ledger object can be successfully
//! traced from within a WASM smart contract.
//!
//! The test script configures an account with all possible AccountRoot fields, creates an
//! escrow with this contract as the finish condition, then finishes the escrow. This contract
//! loads the AccountRoot and traces every field to verify the WASM stdlib can access all
//! account data correctly.
#![cfg_attr(target_arch = "wasm32", no_std)]

use xrpl_wasm_stdlib::core::current_tx::escrow_finish::{EscrowFinish, get_current_escrow_finish};
use xrpl_wasm_stdlib::core::current_tx::traits::TransactionCommonFields;
use xrpl_wasm_stdlib::core::keylets::account_keylet;
use xrpl_wasm_stdlib::core::ledger_objects::account_root::AccountRoot;
use xrpl_wasm_stdlib::core::ledger_objects::traits::{AccountFields, LedgerObjectCommonFields};
use xrpl_wasm_stdlib::core::types::account_id::AccountID;
use xrpl_wasm_stdlib::host::cache_ledger_obj;
use xrpl_wasm_stdlib::host::trace::{DataRepr, trace, trace_amount, trace_data, trace_num};

// NOTE: This is only available on WASM targets because in CI, the coverage test returns random memory (whereas locally
// this returns the bytes 0x00).
#[cfg(target_arch = "wasm32")]
use xrpl_wasm_stdlib::core::types::amount::Amount;

#[unsafe(no_mangle)]
pub extern "C" fn finish() -> i32 {
    let _ = trace("$$$$$ STARTING WASM EXECUTION $$$$$");
    let _ = trace("");

    // The transaction prompting execution of this contract.
    let escrow_finish: EscrowFinish = get_current_escrow_finish();

    // ########################################
    // [EscrowFinish Account]: Trace AccountRoot Fields.
    // ########################################
    {
        // Get the account that's finishing the escrow (our configured test account)
        let account_id: AccountID = escrow_finish.get_account().unwrap();

        // Compute the keylet for this account's AccountRoot object
        // AccountRoot keylet = 0x61 (a) + SHA512Half(account_id)
        // use xrpl_wasm_stdlib::core::keylet::account_root_keylet;
        let account_keylet = account_keylet(&account_id).unwrap();

        // Try to cache the ledger object inside rippled
        let slot = unsafe { cache_ledger_obj(account_keylet.as_ptr(), 32, 0) };
        if slot < 0 {
            let _ = trace_num("Error slotting Account object", slot as i64);
            panic!()
        } else {
            let _ = trace_num("Account object slotted at", slot as i64);
        }

        // We use the trait-bound implementation so as not to duplicate accessor logic.
        let account = AccountRoot { slot_num: slot };

        let _ = trace("### Step #2: Trace AccountRoot Ledger Object");
        let _ = trace("{ ");
        let _ = trace("  -- Common Fields");

        // Trace the `Flags`
        let flags = account.get_flags().unwrap();
        // Expected flags: lsfPasswordSpent (0x00010000 = 65536)
        // This flag is automatically set when the account uses its free SetRegularKey transaction
        test_utils::assert_eq!(
            flags,
            65536,
            "Expected flags to be 0x00010000 (lsfPasswordSpent)"
        );
        let _ = trace_num("  Flags:", flags as i64);

        // Trace the `LedgerEntryType`
        let ledger_entry_type = account.ledger_entry_type().unwrap();
        test_utils::assert_eq!(ledger_entry_type, 97); // 97 is the code for "AccountRoot"
        let _ = trace_num("  LedgerEntryType (AccountRoot):", ledger_entry_type as i64);
        let _ = trace("} ");

        let _ = trace("{ ");
        let _ = trace("  -- Account Specific Fields");

        // Trace the `Account`
        let account_id = account.get_account().unwrap();
        // Account is the hardcoded keylet we're looking up - just verify it's 20 bytes
        test_utils::assert_eq!(account_id.0.len(), 20);
        let _ = trace_data("  Account:", &account_id.0, DataRepr::AsHex);

        // Trace the `AccountTxnID` (optional - required for testing)
        let account_txn_id_opt = account.account_txn_id().unwrap();
        let account_txn_id =
            account_txn_id_opt.expect("AccountTxnID should be present for testing");
        // AccountTxnID is system-generated - just verify it's 32 bytes
        test_utils::assert_eq!(account_txn_id.0.len(), 32);
        let _ = trace_data("  AccountTxnID:", &account_txn_id.0, DataRepr::AsHex);

        // Trace `AMMID` (optional - only present on AMM AccountRoot entries)
        // Note: This is a regular account, not an AMM account, so AMMID should be None
        // The AMM we created has its own separate AccountRoot with an AMMID
        test_utils::assert!(
            account.amm_id().unwrap().is_none(),
            "AMMID should be None (not an AMM account)"
        );

        // Trace the `Balance` (required)
        let balance_amount = account
            .balance()
            .unwrap()
            .expect("Balance should be present");
        let _ = trace_amount("Balance of Account Finishing the Escrow:", &balance_amount);
        // NOTE: This is only available on WASM targets because in CI, the coverage test returns random memory
        // (whereas locally this returns the bytes 0x00).
        #[cfg(target_arch = "wasm32")]
        match balance_amount {
            Amount::XRP { num_drops } => {
                // Balance is system-generated, just verify it's reasonable
                let _ = trace_num("  Balance of Account Finishing the Escrow:", num_drops);
            }
            Amount::IOU { .. } => {
                panic!("IOU Balance encountered, but should have been XRP.")
            }
            Amount::MPT { .. } => {
                panic!("MPT Balance encountered, but should have been XRP.")
            }
        }

        // Trace and assert the `BurnedNFTokens` (optional)
        let burned_nf_tokens_opt = account.burned_nf_tokens().unwrap();
        let burned_nf_tokens = burned_nf_tokens_opt.unwrap_or(0);
        let _ = trace_num("  BurnedNFTokens:", burned_nf_tokens as i64);
        test_utils::assert_eq!(burned_nf_tokens, 0, "Expected 0 burned NFTokens");

        // Trace the `Domain` (optional - required for testing)
        let domain_opt = account.domain().unwrap();
        let domain = domain_opt.expect("Domain should be set for testing");
        // Domain should be "example.com" in hex: 6578616D706C652E636F6D
        let expected_domain = b"example.com";
        test_utils::assert_eq!(domain.len, expected_domain.len(), "Domain length mismatch");
        test_utils::assert_eq!(
            &domain.data[..domain.len],
            &expected_domain[..],
            "Domain should be 'example.com'"
        );
        let _ = trace_data("  Domain:", &domain.data[..domain.len], DataRepr::AsHex);

        // Trace the `EmailHash` (optional - required for testing)
        let email_hash_opt = account.email_hash().unwrap();
        let email_hash = email_hash_opt.expect("EmailHash should be set for testing");
        // EmailHash should be MD5 of "hello": 5D41402ABC4B2A76B9719D911017C592
        test_utils::assert_eq!(email_hash.0.len(), 16);
        let expected_email_hash: [u8; 16] = [
            0x5D, 0x41, 0x40, 0x2A, 0xBC, 0x4B, 0x2A, 0x76, 0xB9, 0x71, 0x9D, 0x91, 0x10, 0x17,
            0xC5, 0x92,
        ];
        test_utils::assert_eq!(
            email_hash.0,
            expected_email_hash,
            "EmailHash should be MD5 of 'hello'"
        );
        let _ = trace_data("  EmailHash:", &email_hash.0, DataRepr::AsHex);

        // Trace the `FirstNFTokenSequence` (optional - required for testing)
        let first_nf_token_sequence = account
            .first_nf_token_sequence()
            .unwrap()
            .expect("FirstNFTokenSequence should be set for testing");
        let _ = trace_num("  FirstNFTokenSequence:", first_nf_token_sequence as i64);

        // Trace the `MessageKey` (optional - required for testing)
        let message_key_opt = account.message_key().unwrap();
        let message_key = message_key_opt.expect("MessageKey should be set for testing");
        // MessageKey should be: 03AB40A0490F9B7ED8DF29D246BF2D6269820A0EE7742ACDD457BEA7C7D0931EDB
        test_utils::assert_eq!(message_key.len, 33, "MessageKey should be 33 bytes");
        let expected_message_key: [u8; 33] = [
            0x03, 0xAB, 0x40, 0xA0, 0x49, 0x0F, 0x9B, 0x7E, 0xD8, 0xDF, 0x29, 0xD2, 0x46, 0xBF,
            0x2D, 0x62, 0x69, 0x82, 0x0A, 0x0E, 0xE7, 0x74, 0x2A, 0xCD, 0xD4, 0x57, 0xBE, 0xA7,
            0xC7, 0xD0, 0x93, 0x1E, 0xDB,
        ];
        test_utils::assert_eq!(
            &message_key.data[..message_key.len],
            &expected_message_key,
            "MessageKey mismatch"
        );
        let _ = trace_data(
            "  MessageKey:",
            &message_key.data[..message_key.len],
            DataRepr::AsHex,
        );

        // Trace the `MintedNFTokens` (optional - required for testing)
        let minted_nf_tokens = account
            .minted_nf_tokens()
            .unwrap()
            .expect("MintedNFTokens should be set for testing");
        // We minted exactly 1 NFToken in the test
        test_utils::assert_eq!(minted_nf_tokens, 1, "Expected 1 minted NFToken");
        let _ = trace_num("  MintedNFTokens:", minted_nf_tokens as i64);

        // Trace the `NFTokenMinter` (optional - required for testing)
        let nf_token_minter = account
            .nf_token_minter()
            .unwrap()
            .expect("NFTokenMinter should be set for testing");
        // NFTokenMinter is an AccountID - verify it's 20 bytes
        test_utils::assert_eq!(nf_token_minter.0.len(), 20);
        let _ = trace_data("  NFTokenMinter:", &nf_token_minter.0, DataRepr::AsHex);

        // Trace the `OwnerCount` (required)
        let owner_count = account.owner_count().unwrap();
        // OwnerCount is system-generated based on owned objects
        let _ = trace_num("  OwnerCount:", owner_count as i64);

        // Trace the `PreviousTxnID` (required)
        let previous_txn_id = account.previous_txn_id().unwrap();
        // PreviousTxnID is system-generated - just verify it's 32 bytes
        test_utils::assert_eq!(previous_txn_id.0.len(), 32);
        let _ = trace_data("  PreviousTxnID:", &previous_txn_id.0, DataRepr::AsHex);

        // Trace the `PreviousTxnLgrSeq` (required)
        let previous_txn_lgr_seq = account.previous_txn_lgr_seq().unwrap();
        // PreviousTxnLgrSeq is system-generated
        let _ = trace_num("  PreviousTxnLgrSeq:", previous_txn_lgr_seq as i64);

        // Trace the `RegularKey` (optional - required for testing)
        let regular_key = account
            .regular_key()
            .unwrap()
            .expect("RegularKey should be set for testing");
        // RegularKey is an AccountID - verify it's 20 bytes
        test_utils::assert_eq!(regular_key.0.len(), 20);
        let _ = trace_data("  RegularKey:", &regular_key.0, DataRepr::AsHex);

        // Trace the `Sequence` (required)
        let sequence = account.sequence().unwrap();
        // Sequence is system-generated
        let _ = trace_num("  Sequence:", sequence as i64);

        // Trace the `TicketCount` (optional - required for testing)
        let ticket_count = account
            .ticket_count()
            .unwrap()
            .expect("TicketCount should be set for testing");
        // We created 5 tickets in the test
        test_utils::assert_eq!(ticket_count, 5, "Expected 5 tickets");
        let _ = trace_num("  TicketCount:", ticket_count as i64);

        // Trace the `TickSize` (optional - required for testing)
        let tick_size = account
            .tick_size()
            .unwrap()
            .expect("TickSize should be set for testing");
        // TickSize was set to 5 in the test
        test_utils::assert_eq!(tick_size, 5, "Expected TickSize to be 5");
        let _ = trace_num("  TickSize:", tick_size as i64);

        // Trace the `TransferRate` (optional - required for testing)
        let transfer_rate = account
            .transfer_rate()
            .unwrap()
            .expect("TransferRate should be set for testing");
        // TransferRate was set to 1002000000 (0.2% fee) in the test
        test_utils::assert_eq!(
            transfer_rate,
            1002000000,
            "Expected TransferRate to be 1002000000"
        );
        let _ = trace_num("  TransferRate:", transfer_rate as i64);

        // Trace the `WalletLocator` (optional - required for testing)
        let wallet_locator = account
            .wallet_locator()
            .unwrap()
            .expect("WalletLocator should be set for testing");
        // WalletLocator should be all 0xAA bytes (32 bytes)
        test_utils::assert_eq!(wallet_locator.0.len(), 32);
        let expected_wallet_locator = [0xAA; 32];
        test_utils::assert_eq!(
            wallet_locator.0,
            expected_wallet_locator,
            "WalletLocator should be all 0xAA bytes"
        );
        let _ = trace_data("  WalletLocator:", &wallet_locator.0, DataRepr::AsHex);

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
