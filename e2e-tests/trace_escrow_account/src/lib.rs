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

use xrpl_common_stdlib::current_tx::traits::TransactionCommonFields;
use xrpl_common_stdlib::host::trace::{trace, trace_amt, trace_hex, trace_num};
use xrpl_common_stdlib::ledger_entry_ids::accountroot_id;
use xrpl_common_stdlib::objects::traits::{AccountRootFields, LedgerObjectCommonFields};
use xrpl_common_stdlib::objects::{AccountRoot, cache_le};
use xrpl_common_stdlib::types::account_id::AccountID;
use xrpl_escrow_stdlib::current_tx::escrow_finish::{EscrowFinish, get_current_escrow_finish};

// NOTE: This is only available on WASM targets because in CI, the coverage test returns random memory (whereas locally
// this returns the bytes 0x00).
#[cfg(target_arch = "wasm32")]
use xrpl_common_stdlib::types::amount::Amount;

#[unsafe(no_mangle)]
pub extern "C" fn escrow_finish() -> i32 {
    trace("$$$$$ STARTING WASM EXECUTION $$$$$");
    trace("");

    // The transaction prompting execution of this contract.
    let escrow_finish: EscrowFinish = get_current_escrow_finish();

    // ########################################
    // [EscrowFinish Account]: Trace AccountRoot Fields.
    // ########################################
    {
        // Get the account that's finishing the escrow (our configured test account)
        let account_id: AccountID = escrow_finish.get_account().unwrap();

        // Compute the ledger entry ID for this account's AccountRoot object
        // AccountRoot ledger entry ID = 0x61 (a) + SHA512Half(account_id)
        // use xrpl_common_stdlib::ledger_entry_ids::accountroot_id;
        let accountroot_id = accountroot_id(&account_id).unwrap();

        // Try to cache the ledger object inside rippled
        let slot = cache_le(&accountroot_id).unwrap_or_else(|error| {
            trace_num("Error slotting Account object", error.code() as i64);
            panic!()
        });
        trace_num("Account object slotted at", slot as i64);

        // We use the trait-bound implementation so as not to duplicate accessor logic.
        let account = AccountRoot::new(slot);

        trace("### Step #2: Trace AccountRoot Ledger Object");
        trace("{ ");
        trace("  -- Common Fields");

        // Trace the `Flags`
        let flags = account.get_flags().unwrap();
        // Expected flags: lsfPasswordSpent (0x00010000 = 65536)
        // This flag is automatically set when the account uses its free SetRegularKey transaction
        test_utils::assert_eq!(
            flags,
            65536,
            "Expected flags to be 0x00010000 (lsfPasswordSpent)"
        );
        trace_num("  Flags:", flags as i64);

        // Trace the `LedgerEntryType`
        let ledger_entry_type = account.get_ledger_entry_type().unwrap();
        test_utils::assert_eq!(ledger_entry_type, 97); // 97 is the code for "AccountRoot"
        trace_num("  LedgerEntryType (AccountRoot):", ledger_entry_type as i64);
        trace("} ");

        trace("{ ");
        trace("  -- Account Specific Fields");

        // Trace the `Account`
        let account_id = account.account().unwrap();
        // Account is the hardcoded ledger entry ID we're looking up - just verify it's 20 bytes
        test_utils::assert_eq!(account_id.0.len(), 20);
        trace_hex("  Account:", &account_id.0);

        // Trace the `AccountTxnID` (optional - required for testing)
        let account_txn_id_opt = account.account_txn_id().unwrap();
        let account_txn_id =
            account_txn_id_opt.expect("AccountTxnID should be present for testing");
        // AccountTxnID is system-generated - just verify it's 32 bytes
        test_utils::assert_eq!(account_txn_id.0.len(), 32);
        trace_hex("  AccountTxnID:", &account_txn_id.0);

        // Trace `AMMID` (optional - only present on AMM AccountRoot entries)
        // Note: This is a regular account, not an AMM account, so AMMID should be None
        // The AMM we created has its own separate AccountRoot with an AMMID
        test_utils::assert!(
            account.amm_id().unwrap().is_none(),
            "AMMID should be None (not an AMM account)"
        );

        // Trace the `Balance` (required)
        let balance_amount = account.balance().unwrap();
        trace_amt("Balance of Account Finishing the Escrow:", &balance_amount);
        // NOTE: This is only available on WASM targets because in CI, the coverage test returns random memory
        // (whereas locally this returns the bytes 0x00).
        #[cfg(target_arch = "wasm32")]
        match balance_amount {
            Amount::XRP { num_drops } => {
                // Balance is system-generated, just verify it's reasonable
                trace_num("  Balance of Account Finishing the Escrow:", num_drops);
            }
            Amount::IOU { .. } => {
                panic!("IOU Balance encountered, but should have been XRP.")
            }
            Amount::MPT { .. } => {
                panic!("MPT Balance encountered, but should have been XRP.")
            }
        }

        // Trace and assert the `BurnedNFTokens` (optional)
        let burned_nf_tokens_opt = account.burned_nftokens().unwrap();
        let burned_nf_tokens = burned_nf_tokens_opt.unwrap_or(0);
        trace_num("  BurnedNFTokens:", burned_nf_tokens as i64);
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
        trace_hex("  Domain:", &domain.data[..domain.len]);

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
        trace_hex("  EmailHash:", &email_hash.0);

        // Trace the `FirstNFTokenSequence` (optional - required for testing)
        let first_nf_token_sequence = account
            .first_nftoken_sequence()
            .unwrap()
            .expect("FirstNFTokenSequence should be set for testing");
        trace_num("  FirstNFTokenSequence:", first_nf_token_sequence as i64);

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
        trace_hex("  MessageKey:", &message_key.data[..message_key.len]);

        // Trace the `MintedNFTokens` (optional - required for testing)
        let minted_nf_tokens = account
            .minted_nftokens()
            .unwrap()
            .expect("MintedNFTokens should be set for testing");
        // We minted exactly 1 NFToken in the test
        test_utils::assert_eq!(minted_nf_tokens, 1, "Expected 1 minted NFToken");
        trace_num("  MintedNFTokens:", minted_nf_tokens as i64);

        // Trace the `NFTokenMinter` (optional - required for testing)
        let nf_token_minter = account
            .nftoken_minter()
            .unwrap()
            .expect("NFTokenMinter should be set for testing");
        // NFTokenMinter is an AccountID - verify it's 20 bytes
        test_utils::assert_eq!(nf_token_minter.0.len(), 20);
        trace_hex("  NFTokenMinter:", &nf_token_minter.0);

        // Trace the `OwnerCount` (required)
        let owner_count = account.owner_count().unwrap();
        // OwnerCount is system-generated based on owned objects
        trace_num("  OwnerCount:", owner_count as i64);

        // Trace the `PreviousTxnID` (required)
        let previous_txn_id = account.previous_txn_id().unwrap();
        // PreviousTxnID is system-generated - just verify it's 32 bytes
        test_utils::assert_eq!(previous_txn_id.0.len(), 32);
        trace_hex("  PreviousTxnID:", &previous_txn_id.0);

        // Trace the `PreviousTxnLgrSeq` (required)
        let previous_txn_lgr_seq = account.previous_txn_lgr_seq().unwrap();
        // PreviousTxnLgrSeq is system-generated
        trace_num("  PreviousTxnLgrSeq:", previous_txn_lgr_seq as i64);

        // Trace the `RegularKey` (optional - required for testing)
        let regular_key = account
            .regular_key()
            .unwrap()
            .expect("RegularKey should be set for testing");
        // RegularKey is an AccountID - verify it's 20 bytes
        test_utils::assert_eq!(regular_key.0.len(), 20);
        trace_hex("  RegularKey:", &regular_key.0);

        // Trace the `Sequence` (required)
        let sequence = account.sequence().unwrap();
        // Sequence is system-generated
        trace_num("  Sequence:", sequence as i64);

        // Trace the `TicketCount` (optional - required for testing)
        let ticket_count = account
            .ticket_count()
            .unwrap()
            .expect("TicketCount should be set for testing");
        // We created 5 tickets in the test
        test_utils::assert_eq!(ticket_count, 5, "Expected 5 tickets");
        trace_num("  TicketCount:", ticket_count as i64);

        // Trace the `TickSize` (optional - required for testing)
        let tick_size = account
            .tick_size()
            .unwrap()
            .expect("TickSize should be set for testing");
        // TickSize was set to 5 in the test
        test_utils::assert_eq!(tick_size, 5, "Expected TickSize to be 5");
        trace_num("  TickSize:", tick_size as i64);

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
        trace_num("  TransferRate:", transfer_rate as i64);

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
        trace_hex("  WalletLocator:", &wallet_locator.0);

        trace("}");
        trace("");
    }

    trace("$$$$$ WASM EXECUTION COMPLETE $$$$$");
    1 // <-- Finish the escrow to indicate a successful outcome
}

#[cfg(test)]
mod coverage_tests {
    use super::*;

    /// Coverage test: exercises any host function categories via escrow_finish()
    ///
    /// This test runs the same logic as the integration test, but on native
    /// targets with stub host functions. It's used to measure code coverage
    /// of xrpl-common-stdlib.
    ///
    /// Note: The host functions return dummy values (from host_bindings_for_testing.rs),
    /// so this test verifies that the code *runs*, not that it's *correct*.
    /// Correctness is verified by the real integration tests against rippled.
    #[test]
    fn test_finish_exercises_all_host_functions() {
        // On non-wasm targets, escrow_finish() uses host_bindings_for_testing.rs
        // which provides stub implementations of all host functions.
        let result = escrow_finish();

        // The escrow_finish() function returns 1 on success, or a negative error code.
        // With stub host functions, we expect success (though the actual
        // behavior depends on the stub implementations).
        core::assert_eq!(result, 1, "escrow_finish() should return 1 on success");
    }
}
