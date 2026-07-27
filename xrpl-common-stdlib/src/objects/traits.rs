//! Generic ledger-object field accessor traits.
//!
//! Escrow-specific traits live in the `xrpl-escrow-stdlib` crate.
//! Per-ledger-entry field traits (`EscrowFields`, `AccountRootFields`, etc.) are generated in
//! `crate::objects::generated` and re-exported here, since generated code in other
//! crates/files (e.g. `xrpl-escrow-stdlib`'s `Escrow` struct) imports per-entry field traits from
//! this stable path.

pub use crate::objects::generated::{AccountRootFields, EscrowFields};

use crate::host::Result;
use crate::objects::{current_ledger_object, ledger_object};
use crate::sfield;

/// Trait providing access to common fields present in all ledger objects.
///
/// This trait defines methods to access standard fields that are common across
/// different types of ledger objects in the XRP Ledger.
pub trait LedgerObjectCommonFields {
    // NOTE: `get_ledger_index()` is not in this trait because `sfLedgerIndex` is not actually a field on a ledger
    // object (it's a synthetic field that maps to the `index` field, which is the unique ID of an object in the
    // ledger's state tree). See https://github.com/XRPLF/rippled/issues/3649 for more context.

    /// Returns the slot number (register number) where the ledger object is stored.
    ///
    /// This number is used to identify and access the specific ledger object
    /// when retrieving or modifying its fields.
    ///
    /// # Returns
    ///
    /// The slot number as an i32 value
    fn get_slot_num(&self) -> i32;

    /// Retrieves the flags field of the ledger object.
    ///
    /// # Arguments
    ///
    /// * `register_num` - The register number where the ledger object is stored
    ///
    /// # Returns
    ///
    /// The flags as a u32 value
    fn get_flags(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::Flags)
    }

    /// Retrieves the ledger entry type of the object.
    ///
    /// The value 0x0075, mapped to the string Escrow, indicates that this is an Escrow entry.
    ///
    /// # Returns
    ///
    /// The ledger entry type as a u16 value
    fn get_ledger_entry_type(&self) -> Result<u16> {
        ledger_object::get_field(self.get_slot_num(), sfield::LedgerEntryType)
    }
}

/// Trait providing access to common fields in the current ledger object.
///
/// This trait defines methods to access standard fields that are common across
/// different types of ledger objects, specifically for the current ledger object
/// being processed.
pub trait CurrentLedgerObjectCommonFields {
    // NOTE: `get_ledger_index()` is not in this trait because `sfLedgerIndex` is not actually a field on a ledger
    // object (it's a synthetic field that maps to the `index` field, which is the unique ID of an object in the
    // ledger's state tree). See https://github.com/XRPLF/rippled/issues/3649 for more context.

    /// Retrieves the flags field of the current ledger object.
    ///
    /// # Returns
    ///
    /// The flags as a u32 value
    fn get_flags(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::Flags)
    }

    /// Retrieves the ledger entry type of the current ledger object.
    ///
    /// The value 0x0075, mapped to the string Escrow, indicates that this is an Escrow entry.
    ///
    /// # Returns
    ///
    /// The ledger entry type as a u16 value
    fn get_ledger_entry_type(&self) -> Result<u16> {
        current_ledger_object::get_field(sfield::LedgerEntryType)
    }
}

// The `EscrowFields` and `AccountRootFields` traits are generated — see
// `crate::objects::generated` — and re-exported at the top of this module. The escrow `Data`-field
// accessor (`get_data`) is not representable by the generator; it lives in the hand-written
// `EscrowContractData` trait in the `xrpl-escrow-stdlib` crate.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::host::error_codes::{FIELD_NOT_FOUND, INTERNAL_ERROR, INVALID_FIELD};
    use crate::host::host_bindings_trait::MockHostBindings;
    use crate::objects::LedgerObjectFieldGetter;
    use crate::objects::account_root::AccountRoot;
    use crate::objects::generated::{AccountRootFields, EscrowFields};
    use crate::sfield::SField;
    use mockall::predicate::{always, eq};

    // ========================================
    // Test helper functions
    // ========================================

    /// Helper to set up a mock expectation for get_current_ledger_obj_field
    ///
    /// Sets up a mock expectation that will match calls with:
    /// - field: The SField with the specified CODE
    /// - size: The expected buffer size
    /// - times: How many times this expectation should be matched
    ///
    /// When a test fails, mockall will show which parameter didn't match.
    fn expect_current_field<
        T: LedgerObjectFieldGetter + Send + std::fmt::Debug + PartialEq + 'static,
        const CODE: i32,
    >(
        mock: &mut MockHostBindings,
        _field: SField<T, CODE>,
        size: usize,
        times: usize,
    ) {
        mock.expect_get_current_ledger_obj_field()
            .with(eq(CODE), always(), eq(size))
            .times(times)
            .returning(move |_, _, _| size as i32);
    }

    /// Helper to set up a mock expectation for get_ledger_obj_field
    ///
    /// Sets up a mock expectation that will match calls with:
    /// - slot: The ledger object slot number
    /// - field: The SField with the specified CODE
    /// - size: The expected buffer size
    /// - times: How many times this expectation should be matched
    ///
    /// When a test fails, mockall will show which parameter didn't match.
    fn expect_ledger_field<
        T: LedgerObjectFieldGetter + Send + std::fmt::Debug + PartialEq + 'static,
        const CODE: i32,
    >(
        mock: &mut MockHostBindings,
        slot: i32,
        _field: SField<T, CODE>,
        size: usize,
        times: usize,
    ) {
        mock.expect_get_ledger_obj_field()
            .with(eq(slot), eq(CODE), always(), eq(size))
            .times(times)
            .returning(move |_, _, _, _| size as i32);
    }

    mod ledger_object_common_fields {
        use super::*;
        use crate::host::setup_mock;

        #[test]
        fn test_mandatory_fields_return_ok() {
            let mut mock = MockHostBindings::new();

            // get_flags
            expect_ledger_field(&mut mock, 1, sfield::Flags, 4, 1);
            // get_ledger_entry_type
            expect_ledger_field(&mut mock, 1, sfield::LedgerEntryType, 2, 1);

            let _guard = setup_mock(mock);

            let account = AccountRoot::new(1);

            // All mandatory fields should return Ok
            assert!(account.get_flags().is_ok());
            assert!(account.get_ledger_entry_type().is_ok());
        }

        #[test]
        fn test_mandatory_fields_return_error_on_internal_error() {
            let mut mock = MockHostBindings::new();

            // get_flags with INTERNAL_ERROR
            mock.expect_get_ledger_obj_field()
                .with(eq(1), eq(sfield::Flags), always(), eq(4))
                .times(1)
                .returning(|_, _, _, _| INTERNAL_ERROR);

            let _guard = setup_mock(mock);

            let account = AccountRoot::new(1);
            let result = account.get_flags();

            assert!(result.is_err());
            assert_eq!(result.err().unwrap().code(), INTERNAL_ERROR);
        }

        #[test]
        fn test_get_ledger_entry_type_returns_error_on_internal_error() {
            let mut mock = MockHostBindings::new();

            mock.expect_get_ledger_obj_field()
                .with(eq(1), eq(sfield::LedgerEntryType), always(), eq(2))
                .times(1)
                .returning(|_, _, _, _| INTERNAL_ERROR);

            let _guard = setup_mock(mock);

            let account = AccountRoot::new(1);
            let result = account.get_ledger_entry_type();

            assert!(result.is_err());
            assert_eq!(result.err().unwrap().code(), INTERNAL_ERROR);
        }

        #[test]
        fn test_mandatory_fields_return_error_on_invalid_field() {
            let mut mock = MockHostBindings::new();

            // get_flags with INVALID_FIELD
            mock.expect_get_ledger_obj_field()
                .with(eq(1), eq(sfield::Flags), always(), eq(4))
                .times(1)
                .returning(|_, _, _, _| INVALID_FIELD);

            let _guard = setup_mock(mock);

            let account = AccountRoot::new(1);
            let result = account.get_flags();

            assert!(result.is_err());
            assert_eq!(result.err().unwrap().code(), INVALID_FIELD);
        }
    }

    mod escrow_fields {
        use super::*;
        use crate::host::setup_mock;
        use crate::types::blob::{CONDITION_BLOB_SIZE, WASM_BLOB_SIZE};

        struct TestLedgerObject {
            slot_num: i32,
        }
        impl LedgerObjectCommonFields for TestLedgerObject {
            fn get_slot_num(&self) -> i32 {
                self.slot_num
            }
        }
        impl EscrowFields for TestLedgerObject {}

        #[test]
        fn test_mandatory_fields_return_ok() {
            let mut mock = MockHostBindings::new();

            // get_account
            expect_ledger_field(&mut mock, 1, sfield::Account, 20, 1);
            // get_amount
            expect_ledger_field(&mut mock, 1, sfield::Amount, 48, 1);
            // get_destination
            expect_ledger_field(&mut mock, 1, sfield::Destination, 20, 1);
            // get_owner_node
            expect_ledger_field(&mut mock, 1, sfield::OwnerNode, 8, 1);
            // get_previous_txn_id
            expect_ledger_field(&mut mock, 1, sfield::PreviousTxnID, 32, 1);
            // get_previous_txn_lgr_seq
            expect_ledger_field(&mut mock, 1, sfield::PreviousTxnLgrSeq, 4, 1);

            let _guard = setup_mock(mock);

            let obj = TestLedgerObject { slot_num: 1 };

            // All mandatory fields should return Ok
            assert!(obj.get_account().is_ok());
            assert!(obj.get_amount().is_ok());
            assert!(obj.get_destination().is_ok());
            assert!(obj.get_owner_node().is_ok());
            assert!(obj.get_previous_txn_id().is_ok());
            assert!(obj.get_previous_txn_lgr_seq().is_ok());
        }

        #[test]
        fn test_optional_fields_return_some() {
            let mut mock = MockHostBindings::new();

            // get_cancel_after
            expect_ledger_field(&mut mock, 1, sfield::CancelAfter, 4, 1);
            // get_condition
            expect_ledger_field(&mut mock, 1, sfield::Condition, CONDITION_BLOB_SIZE, 1);
            // get_destination_node
            expect_ledger_field(&mut mock, 1, sfield::DestinationNode, 8, 1);
            // get_destination_tag
            expect_ledger_field(&mut mock, 1, sfield::DestinationTag, 4, 1);
            // get_finish_after
            expect_ledger_field(&mut mock, 1, sfield::FinishAfter, 4, 1);
            // get_source_tag
            expect_ledger_field(&mut mock, 1, sfield::SourceTag, 4, 1);
            // get_finish_function
            expect_ledger_field(&mut mock, 1, sfield::FinishFunction, WASM_BLOB_SIZE, 1);

            let _guard = setup_mock(mock);

            let obj = TestLedgerObject { slot_num: 1 };

            // All optional fields should return Ok(Some(...))
            assert!(obj.get_cancel_after().unwrap().is_some());
            assert!(obj.get_condition().unwrap().is_some());
            assert!(obj.get_destination_node().unwrap().is_some());
            assert!(obj.get_destination_tag().unwrap().is_some());
            assert!(obj.get_finish_after().unwrap().is_some());
            assert!(obj.get_source_tag().unwrap().is_some());
            assert!(obj.get_finish_function().unwrap().is_some());
        }

        #[test]
        fn test_optional_fields_return_none_when_field_not_found() {
            let mut mock = MockHostBindings::new();

            // get_cancel_after
            mock.expect_get_ledger_obj_field()
                .with(eq(1), eq(sfield::CancelAfter), always(), eq(4))
                .times(1)
                .returning(|_, _, _, _| FIELD_NOT_FOUND);
            // get_condition - variable size field, returns 0 for empty (Some with len=0)
            mock.expect_get_ledger_obj_field()
                .with(
                    eq(1),
                    eq(sfield::Condition),
                    always(),
                    eq(CONDITION_BLOB_SIZE),
                )
                .times(1)
                .returning(|_, _, _, _| 0);
            // get_destination_node
            mock.expect_get_ledger_obj_field()
                .with(eq(1), eq(sfield::DestinationNode), always(), eq(8))
                .times(1)
                .returning(|_, _, _, _| FIELD_NOT_FOUND);
            // get_destination_tag
            mock.expect_get_ledger_obj_field()
                .with(eq(1), eq(sfield::DestinationTag), always(), eq(4))
                .times(1)
                .returning(|_, _, _, _| FIELD_NOT_FOUND);
            // get_finish_after
            mock.expect_get_ledger_obj_field()
                .with(eq(1), eq(sfield::FinishAfter), always(), eq(4))
                .times(1)
                .returning(|_, _, _, _| FIELD_NOT_FOUND);
            // get_source_tag
            mock.expect_get_ledger_obj_field()
                .with(eq(1), eq(sfield::SourceTag), always(), eq(4))
                .times(1)
                .returning(|_, _, _, _| FIELD_NOT_FOUND);
            // get_finish_function - variable size field, returns 0 for empty (Some with len=0)
            mock.expect_get_ledger_obj_field()
                .with(
                    eq(1),
                    eq(sfield::FinishFunction),
                    always(),
                    eq(WASM_BLOB_SIZE),
                )
                .times(1)
                .returning(|_, _, _, _| 0);

            let _guard = setup_mock(mock);

            let obj = TestLedgerObject { slot_num: 1 };

            // Fixed-size optional fields should return Ok(None) when FIELD_NOT_FOUND
            assert!(obj.get_cancel_after().unwrap().is_none());
            assert!(obj.get_destination_node().unwrap().is_none());
            assert!(obj.get_destination_tag().unwrap().is_none());
            assert!(obj.get_finish_after().unwrap().is_none());
            assert!(obj.get_source_tag().unwrap().is_none());

            // Variable-size optional fields return Some with len=0 when not found
            // (they cannot distinguish between "not present" and "present with 0 bytes")
            let condition = obj.get_condition().unwrap();
            assert!(condition.is_some());
            assert_eq!(condition.unwrap().len, 0);

            let finish_function = obj.get_finish_function().unwrap();
            assert!(finish_function.is_some());
            assert_eq!(finish_function.unwrap().len, 0);
        }

        #[test]
        fn test_mandatory_fields_return_error_on_internal_error() {
            let mut mock = MockHostBindings::new();

            // get_account with INTERNAL_ERROR
            mock.expect_get_ledger_obj_field()
                .with(eq(1), eq(sfield::Account), always(), eq(20))
                .times(1)
                .returning(|_, _, _, _| INTERNAL_ERROR);

            let _guard = setup_mock(mock);

            let obj = TestLedgerObject { slot_num: 1 };
            let result = obj.get_account();

            assert!(result.is_err());
            assert_eq!(result.err().unwrap().code(), INTERNAL_ERROR);
        }

        #[test]
        fn test_mandatory_fields_return_error_on_invalid_field() {
            let mut mock = MockHostBindings::new();

            // get_account with INVALID_FIELD
            mock.expect_get_ledger_obj_field()
                .with(eq(1), eq(sfield::Account), always(), eq(20))
                .times(1)
                .returning(|_, _, _, _| INVALID_FIELD);

            let _guard = setup_mock(mock);

            let obj = TestLedgerObject { slot_num: 1 };
            let result = obj.get_account();

            assert!(result.is_err());
            assert_eq!(result.err().unwrap().code(), INVALID_FIELD);
        }
    }

    mod account_root_fields {
        use super::*;
        use crate::host::setup_mock;
        use crate::types::account_id::ACCOUNT_ID_SIZE;
        use crate::types::blob::{DOMAIN_BLOB_SIZE, PUBLIC_KEY_BLOB_SIZE};

        #[test]
        fn test_mandatory_fields_return_ok() {
            let mut mock = MockHostBindings::new();

            // get_account
            expect_ledger_field(&mut mock, 1, sfield::Account, 20, 1);
            // get_owner_count
            expect_ledger_field(&mut mock, 1, sfield::OwnerCount, 4, 1);
            // get_previous_txn_id
            expect_ledger_field(&mut mock, 1, sfield::PreviousTxnID, 32, 1);
            // get_previous_txn_lgr_seq
            expect_ledger_field(&mut mock, 1, sfield::PreviousTxnLgrSeq, 4, 1);
            // get_sequence
            expect_ledger_field(&mut mock, 1, sfield::Sequence, 4, 1);
            // get_ledger_entry_type
            expect_ledger_field(&mut mock, 1, sfield::LedgerEntryType, 2, 1);
            // get_balance
            expect_ledger_field(&mut mock, 1, sfield::Balance, 48, 1);

            let _guard = setup_mock(mock);

            let account = AccountRoot::new(1);

            // All mandatory fields should return Ok
            assert!(account.get_account().is_ok());
            assert!(account.get_owner_count().is_ok());
            assert!(account.get_previous_txn_id().is_ok());
            assert!(account.get_previous_txn_lgr_seq().is_ok());
            assert!(account.get_sequence().is_ok());
            assert!(account.get_ledger_entry_type().is_ok());
            assert!(account.get_balance().is_ok());
        }

        #[test]
        fn test_optional_fields_return_some() {
            let mut mock = MockHostBindings::new();

            // get_account_txn_id
            expect_ledger_field(&mut mock, 1, sfield::AccountTxnID, 32, 1);
            // get_ammid
            expect_ledger_field(&mut mock, 1, sfield::AMMID, 32, 1);
            // get_burned_nf_tokens
            expect_ledger_field(&mut mock, 1, sfield::BurnedNFTokens, 4, 1);
            // get_domain
            expect_ledger_field(&mut mock, 1, sfield::Domain, DOMAIN_BLOB_SIZE, 1);
            // get_email_hash
            expect_ledger_field(&mut mock, 1, sfield::EmailHash, 16, 1);
            // get_first_nf_token_sequence
            expect_ledger_field(&mut mock, 1, sfield::FirstNFTokenSequence, 4, 1);
            // get_message_key
            expect_ledger_field(&mut mock, 1, sfield::MessageKey, PUBLIC_KEY_BLOB_SIZE, 1);
            // get_minted_nf_tokens
            expect_ledger_field(&mut mock, 1, sfield::MintedNFTokens, 4, 1);
            // get_nf_token_minter
            expect_ledger_field(&mut mock, 1, sfield::NFTokenMinter, 20, 1);
            // get_regular_key
            expect_ledger_field(&mut mock, 1, sfield::RegularKey, ACCOUNT_ID_SIZE, 1);
            // get_ticket_count
            expect_ledger_field(&mut mock, 1, sfield::TicketCount, 4, 1);
            // get_tick_size
            expect_ledger_field(&mut mock, 1, sfield::TickSize, 1, 1);
            // get_transfer_rate
            expect_ledger_field(&mut mock, 1, sfield::TransferRate, 4, 1);
            // get_wallet_locator
            expect_ledger_field(&mut mock, 1, sfield::WalletLocator, 32, 1);

            let _guard = setup_mock(mock);

            let account = AccountRoot::new(1);

            // All optional fields should return Ok(Some(...))
            assert!(account.get_account_txn_id().unwrap().is_some());
            assert!(account.get_ammid().unwrap().is_some());
            assert!(account.get_burned_nf_tokens().unwrap().is_some());
            assert!(account.get_domain().unwrap().is_some());
            assert!(account.get_email_hash().unwrap().is_some());
            assert!(account.get_first_nf_token_sequence().unwrap().is_some());
            assert!(account.get_message_key().unwrap().is_some());
            assert!(account.get_minted_nf_tokens().unwrap().is_some());
            assert!(account.get_nf_token_minter().unwrap().is_some());
            assert!(account.get_regular_key().unwrap().is_some());
            assert!(account.get_ticket_count().unwrap().is_some());
            assert!(account.get_tick_size().unwrap().is_some());
            assert!(account.get_transfer_rate().unwrap().is_some());
            assert!(account.get_wallet_locator().unwrap().is_some());
        }

        #[test]
        fn test_optional_fields_return_none_when_field_not_found() {
            let mut mock = MockHostBindings::new();

            // get_account_txn_id
            mock.expect_get_ledger_obj_field()
                .with(eq(1), eq(sfield::AccountTxnID), always(), eq(32))
                .times(1)
                .returning(|_, _, _, _| FIELD_NOT_FOUND);
            // get_ammid
            mock.expect_get_ledger_obj_field()
                .with(eq(1), eq(sfield::AMMID), always(), eq(32))
                .times(1)
                .returning(|_, _, _, _| FIELD_NOT_FOUND);
            // get_burned_nf_tokens
            mock.expect_get_ledger_obj_field()
                .with(eq(1), eq(sfield::BurnedNFTokens), always(), eq(4))
                .times(1)
                .returning(|_, _, _, _| FIELD_NOT_FOUND);
            // get_domain - variable size field, returns 0 for empty (Some with len=0)
            mock.expect_get_ledger_obj_field()
                .with(eq(1), eq(sfield::Domain), always(), eq(DOMAIN_BLOB_SIZE))
                .times(1)
                .returning(|_, _, _, _| 0);
            // get_email_hash
            mock.expect_get_ledger_obj_field()
                .with(eq(1), eq(sfield::EmailHash), always(), eq(16))
                .times(1)
                .returning(|_, _, _, _| FIELD_NOT_FOUND);
            // get_first_nf_token_sequence
            mock.expect_get_ledger_obj_field()
                .with(eq(1), eq(sfield::FirstNFTokenSequence), always(), eq(4))
                .times(1)
                .returning(|_, _, _, _| FIELD_NOT_FOUND);
            // get_message_key - variable size field, returns 0 for empty (Some with len=0)
            mock.expect_get_ledger_obj_field()
                .with(
                    eq(1),
                    eq(sfield::MessageKey),
                    always(),
                    eq(PUBLIC_KEY_BLOB_SIZE),
                )
                .times(1)
                .returning(|_, _, _, _| 0);
            // get_minted_nf_tokens
            mock.expect_get_ledger_obj_field()
                .with(eq(1), eq(sfield::MintedNFTokens), always(), eq(4))
                .times(1)
                .returning(|_, _, _, _| FIELD_NOT_FOUND);
            // get_nf_token_minter
            mock.expect_get_ledger_obj_field()
                .with(eq(1), eq(sfield::NFTokenMinter), always(), eq(20))
                .times(1)
                .returning(|_, _, _, _| FIELD_NOT_FOUND);
            // get_regular_key
            mock.expect_get_ledger_obj_field()
                .with(eq(1), eq(sfield::RegularKey), always(), eq(ACCOUNT_ID_SIZE))
                .times(1)
                .returning(|_, _, _, _| FIELD_NOT_FOUND);
            // get_ticket_count
            mock.expect_get_ledger_obj_field()
                .with(eq(1), eq(sfield::TicketCount), always(), eq(4))
                .times(1)
                .returning(|_, _, _, _| FIELD_NOT_FOUND);
            // get_tick_size
            mock.expect_get_ledger_obj_field()
                .with(eq(1), eq(sfield::TickSize), always(), eq(1))
                .times(1)
                .returning(|_, _, _, _| FIELD_NOT_FOUND);
            // get_transfer_rate
            mock.expect_get_ledger_obj_field()
                .with(eq(1), eq(sfield::TransferRate), always(), eq(4))
                .times(1)
                .returning(|_, _, _, _| FIELD_NOT_FOUND);
            // get_wallet_locator
            mock.expect_get_ledger_obj_field()
                .with(eq(1), eq(sfield::WalletLocator), always(), eq(32))
                .times(1)
                .returning(|_, _, _, _| FIELD_NOT_FOUND);

            let _guard = setup_mock(mock);

            let account = AccountRoot::new(1);

            // Fixed-size optional fields should return Ok(None) when FIELD_NOT_FOUND
            assert!(account.get_account_txn_id().unwrap().is_none());
            assert!(account.get_ammid().unwrap().is_none());
            assert!(account.get_burned_nf_tokens().unwrap().is_none());
            assert!(account.get_email_hash().unwrap().is_none());
            assert!(account.get_first_nf_token_sequence().unwrap().is_none());
            assert!(account.get_minted_nf_tokens().unwrap().is_none());
            assert!(account.get_nf_token_minter().unwrap().is_none());
            assert!(account.get_regular_key().unwrap().is_none());
            assert!(account.get_ticket_count().unwrap().is_none());
            assert!(account.get_tick_size().unwrap().is_none());
            assert!(account.get_transfer_rate().unwrap().is_none());
            assert!(account.get_wallet_locator().unwrap().is_none());

            // Variable-size optional fields return Some with len=0 when not found
            // (they cannot distinguish between "not present" and "present with 0 bytes")
            let domain = account.get_domain().unwrap();
            assert!(domain.is_some());
            assert_eq!(domain.unwrap().len, 0);
            let message_key = account.get_message_key().unwrap();
            assert!(message_key.is_some());
            assert_eq!(message_key.unwrap().len, 0);
        }

        #[test]
        fn test_mandatory_fields_return_error_on_internal_error() {
            let mut mock = MockHostBindings::new();

            // get_account with INTERNAL_ERROR
            mock.expect_get_ledger_obj_field()
                .with(eq(1), eq(sfield::Account), always(), eq(20))
                .times(1)
                .returning(|_, _, _, _| INTERNAL_ERROR);

            let _guard = setup_mock(mock);

            let account = AccountRoot::new(1);
            let result = account.get_account();

            assert!(result.is_err());
            assert_eq!(result.err().unwrap().code(), INTERNAL_ERROR);
        }

        #[test]
        fn test_mandatory_fields_return_error_on_invalid_field() {
            let mut mock = MockHostBindings::new();

            // get_account with INVALID_FIELD
            mock.expect_get_ledger_obj_field()
                .with(eq(1), eq(sfield::Account), always(), eq(20))
                .times(1)
                .returning(|_, _, _, _| INVALID_FIELD);

            let _guard = setup_mock(mock);

            let account = AccountRoot::new(1);
            let result = account.get_account();

            assert!(result.is_err());
            assert_eq!(result.err().unwrap().code(), INVALID_FIELD);
        }
    }

    mod current_ledger_object_common_fields {
        use super::*;
        use crate::host::setup_mock;

        struct TestCurrentLedgerObject;
        impl CurrentLedgerObjectCommonFields for TestCurrentLedgerObject {}

        #[test]
        fn test_mandatory_fields_return_ok() {
            let mut mock = MockHostBindings::new();

            // get_flags
            expect_current_field(&mut mock, sfield::Flags, 4, 1);
            // get_ledger_entry_type
            expect_current_field(&mut mock, sfield::LedgerEntryType, 2, 1);

            let _guard = setup_mock(mock);

            let escrow = TestCurrentLedgerObject;

            // All mandatory fields should return Ok
            assert!(escrow.get_flags().is_ok());
            assert!(escrow.get_ledger_entry_type().is_ok());
        }

        #[test]
        fn test_mandatory_fields_return_error_on_internal_error() {
            let mut mock = MockHostBindings::new();

            // get_flags with INTERNAL_ERROR
            mock.expect_get_current_ledger_obj_field()
                .with(eq(sfield::Flags), always(), eq(4))
                .times(1)
                .returning(|_, _, _| INTERNAL_ERROR);

            let _guard = setup_mock(mock);

            let escrow = TestCurrentLedgerObject;
            let result = escrow.get_flags();

            assert!(result.is_err());
            assert_eq!(result.err().unwrap().code(), INTERNAL_ERROR);
        }

        #[test]
        fn test_get_ledger_entry_type_returns_error_on_internal_error() {
            let mut mock = MockHostBindings::new();

            mock.expect_get_current_ledger_obj_field()
                .with(eq(sfield::LedgerEntryType), always(), eq(2))
                .times(1)
                .returning(|_, _, _| INTERNAL_ERROR);

            let _guard = setup_mock(mock);

            let escrow = TestCurrentLedgerObject;
            let result = escrow.get_ledger_entry_type();

            assert!(result.is_err());
            assert_eq!(result.err().unwrap().code(), INTERNAL_ERROR);
        }

        #[test]
        fn test_mandatory_fields_return_error_on_invalid_field() {
            let mut mock = MockHostBindings::new();

            // get_flags with INVALID_FIELD
            mock.expect_get_current_ledger_obj_field()
                .with(eq(sfield::Flags), always(), eq(4))
                .times(1)
                .returning(|_, _, _| INVALID_FIELD);

            let _guard = setup_mock(mock);

            let escrow = TestCurrentLedgerObject;
            let result = escrow.get_flags();

            assert!(result.is_err());
            assert_eq!(result.err().unwrap().code(), INVALID_FIELD);
        }
    }
}
