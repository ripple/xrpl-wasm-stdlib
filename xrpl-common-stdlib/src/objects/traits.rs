//! Generic ledger-object field accessor traits.
//!
//! Escrow-specific traits live in the `xrpl-escrow-stdlib` crate.

use crate::fields::locator::LedgerPathBuilder;
use crate::fields::{current_ledger_obj, ledger_obj};
use crate::host::Result;
use crate::sfield;
use crate::types::account_id::AccountID;
use crate::types::amount::Amount;
use crate::types::blob::{PublicKeyBlob, UriBlob};
use crate::types::uint::{Hash128, Hash256};

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

    /// Starts a nested-field path rooted at this ledger object (read by slot).
    ///
    /// Use this to reach into arrays and inner objects that the flat getters can't return whole.
    /// Chain [`field`](LedgerPathBuilder::field) / [`index`](LedgerPathBuilder::index), then
    /// [`get::<T>()`](LedgerPathBuilder::get).
    ///
    /// ```no_run
    /// use xrpl_common_stdlib::objects::traits::LedgerObjectCommonFields;
    /// use xrpl_common_stdlib::sfield;
    /// # fn demo(obj: &impl LedgerObjectCommonFields) {
    /// let signer = obj.path()
    ///     .field(sfield::SignerEntries)
    ///     .index(0)
    ///     .field(sfield::Account)
    ///     .get::<xrpl_common_stdlib::types::account_id::AccountID>();
    /// # let _ = signer; }
    /// ```
    fn path(&self) -> LedgerPathBuilder {
        LedgerPathBuilder::for_ledger_obj(self.get_slot_num())
    }

    /// Retrieves the flags field of the ledger object.
    ///
    /// # Returns
    ///
    /// The flags as a u32 value
    fn get_flags(&self) -> Result<u32> {
        ledger_obj::get_field(self.get_slot_num(), sfield::Flags)
    }

    /// Retrieves the ledger entry type of the object.
    ///
    /// This value identifies which kind of ledger object this is (e.g. AccountRoot, Escrow, etc.).
    /// See the `LedgerEntryType` enum for the full list of values.
    ///
    /// # Returns
    ///
    /// The ledger entry type as a u16 value
    fn get_ledger_entry_type(&self) -> Result<u16> {
        ledger_obj::get_field(self.get_slot_num(), sfield::LedgerEntryType)
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
        current_ledger_obj::get_field(sfield::Flags)
    }

    /// Retrieves the ledger entry type of the current ledger object.
    ///
    /// This value identifies which kind of ledger object this is (e.g. AccountRoot, Escrow, etc.).
    /// See the `LedgerEntryType` enum for the full list of values.
    ///
    /// # Returns
    ///
    /// The ledger entry type as a u16 value
    fn get_ledger_entry_type(&self) -> Result<u16> {
        current_ledger_obj::get_field(sfield::LedgerEntryType)
    }

    /// Starts a nested-field path rooted at the current ledger object (no slot).
    ///
    /// Use this to reach into arrays and inner objects that the flat getters can't return whole.
    /// Chain [`field`](LedgerPathBuilder::field) / [`index`](LedgerPathBuilder::index), then
    /// [`get::<T>()`](LedgerPathBuilder::get).
    ///
    /// ```no_run
    /// use xrpl_common_stdlib::objects::traits::CurrentLedgerObjectCommonFields;
    /// use xrpl_common_stdlib::sfield;
    /// # fn demo(obj: &impl CurrentLedgerObjectCommonFields) {
    /// let amount = obj.path()
    ///     .field(sfield::Amount)
    ///     .get::<xrpl_common_stdlib::types::amount::Amount>();
    /// # let _ = amount; }
    /// ```
    fn path(&self) -> LedgerPathBuilder {
        LedgerPathBuilder::for_current_ledger_obj()
    }
}

/// Trait providing access to fields specific to AccountRoot objects in any ledger.
///
/// This trait extends `LedgerObjectCommonFields` and provides methods to access
/// fields that are specific to AccountRoot objects in any ledger, not just the current one.
/// Each method requires a register number to identify which ledger object to access.
pub trait AccountFields: LedgerObjectCommonFields {
    /// The identifying address of the account.
    fn get_account(&self) -> Result<AccountID> {
        ledger_obj::get_field(self.get_slot_num(), sfield::Account)
    }

    /// AccountTxnID field for the account.
    fn get_account_txn_id(&self) -> Result<Option<Hash256>> {
        ledger_obj::get_field_optional(self.get_slot_num(), sfield::AccountTxnID)
    }

    /// The ledger entry ID of the corresponding AMM ledger entry. Set during account creation; cannot be modified.
    /// If present, indicates that this is a special AMM AccountRoot; always omitted on non-AMM accounts.
    /// (Added by the AMM amendment)
    fn get_amm_id(&self) -> Result<Option<Hash256>> {
        ledger_obj::get_field_optional(self.get_slot_num(), sfield::AMMID)
    }

    /// The account's current XRP balance in drops.
    fn get_balance(&self) -> Result<Option<Amount>> {
        ledger_obj::get_field_optional(self.get_slot_num(), sfield::Balance)
    }

    /// How many total of this account's issued non-fungible tokens have been burned.
    /// This number is always equal or less than MintedNFTokens.
    fn get_burned_nf_tokens(&self) -> Result<Option<u32>> {
        ledger_obj::get_field_optional(self.get_slot_num(), sfield::BurnedNFTokens)
    }

    /// A domain associated with this account. In JSON, this is the hexadecimal for the ASCII representation of the
    /// domain. Cannot be more than 256 bytes in length.
    fn get_domain(&self) -> Result<Option<UriBlob>> {
        ledger_obj::get_field_optional(self.get_slot_num(), sfield::Domain)
    }

    /// The MD5 hash of an email address. Clients can use this to look up an avatar through services such as Gravatar.
    fn get_email_hash(&self) -> Result<Option<Hash128>> {
        ledger_obj::get_field_optional(self.get_slot_num(), sfield::EmailHash)
    }

    /// The account's Sequence Number at the time it minted its first non-fungible-token.
    /// (Added by the fixNFTokenRemint amendment)
    fn get_first_nf_token_sequence(&self) -> Result<Option<u32>> {
        ledger_obj::get_field_optional(self.get_slot_num(), sfield::FirstNFTokenSequence)
    }

    /// A public key that may be used to send encrypted messages to this account. In JSON, uses hexadecimal.
    /// Must be exactly 33 bytes, with the first byte indicating the key type: 0x02 or 0x03 for secp256k1 keys,
    /// 0xED for Ed25519 keys.
    // TODO: See https://github.com/ripple/xrpl-wasm-stdlib/issues/106
    fn get_message_key(&self) -> Result<Option<PublicKeyBlob>> {
        ledger_obj::get_field_optional(self.get_slot_num(), sfield::MessageKey)
    }

    /// How many total non-fungible tokens have been minted by and on behalf of this account.
    /// (Added by the NonFungibleTokensV1_1 amendment)
    fn get_minted_nf_tokens(&self) -> Result<Option<u32>> {
        ledger_obj::get_field_optional(self.get_slot_num(), sfield::MintedNFTokens)
    }

    /// Another account that can mint non-fungible tokens on behalf of this account.
    /// (Added by the NonFungibleTokensV1_1 amendment)
    fn get_nf_token_minter(&self) -> Result<Option<AccountID>> {
        ledger_obj::get_field_optional(self.get_slot_num(), sfield::NFTokenMinter)
    }

    /// The number of objects this account owns in the ledger, which contributes to its owner reserve.
    fn get_owner_count(&self) -> Result<u32> {
        ledger_obj::get_field(self.get_slot_num(), sfield::OwnerCount)
    }

    /// The identifying hash of the transaction that most recently modified this object.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        ledger_obj::get_field(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this object.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        ledger_obj::get_field(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }

    /// The address of a key pair that can be used to sign transactions for this account instead of the master key.
    /// Use a SetRegularKey transaction to change this value.
    fn get_regular_key(&self) -> Result<Option<AccountID>> {
        ledger_obj::get_field_optional(self.get_slot_num(), sfield::RegularKey)
    }

    /// The sequence number of the next valid transaction for this account.
    fn get_sequence(&self) -> Result<u32> {
        ledger_obj::get_field(self.get_slot_num(), sfield::Sequence)
    }

    /// How many Tickets this account owns in the ledger. This is updated automatically to ensure that
    /// the account stays within the hard limit of 250 Tickets at a time. This field is omitted if the account has zero
    /// Tickets. (Added by the TicketBatch amendment.)
    fn get_ticket_count(&self) -> Result<Option<u32>> {
        ledger_obj::get_field_optional(self.get_slot_num(), sfield::TicketCount)
    }

    /// How many significant digits to use for exchange rates of Offers involving currencies issued by this address.
    /// Valid values are 3 to 15, inclusive. (Added by the TickSize amendment.)
    fn get_tick_size(&self) -> Result<Option<u8>> {
        ledger_obj::get_field_optional(self.get_slot_num(), sfield::TickSize)
    }

    /// A transfer fee to charge other users for sending currency issued by this account to each other.
    fn get_transfer_rate(&self) -> Result<Option<u32>> {
        ledger_obj::get_field_optional(self.get_slot_num(), sfield::TransferRate)
    }

    /// An arbitrary 256-bit value that users can set.
    fn get_wallet_locator(&self) -> Result<Option<Hash256>> {
        ledger_obj::get_field_optional(self.get_slot_num(), sfield::WalletLocator)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::host::error_codes::{FIELD_NOT_FOUND, INTERNAL_ERROR, INVALID_FIELD};
    use crate::host::host_bindings_trait::MockHostBindings;
    use crate::objects::account_root::AccountRoot;
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
    fn expect_current_field<T, const CODE: i32>(
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
    fn expect_ledger_field<T, const CODE: i32>(
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

            let account = AccountRoot { slot_num: 1 };

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

            let account = AccountRoot { slot_num: 1 };
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

            let account = AccountRoot { slot_num: 1 };
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

            let account = AccountRoot { slot_num: 1 };
            let result = account.get_flags();

            assert!(result.is_err());
            assert_eq!(result.err().unwrap().code(), INVALID_FIELD);
        }

        #[test]
        fn test_path_roots_a_by_slot_builder() {
            let mut mock = MockHostBindings::new();
            // SignerEntries[0] -> two 4-byte segments = 8 bytes; the object's slot is threaded through.
            mock.expect_get_ledger_obj_nested_field()
                .with(eq(1), always(), eq(8usize), always(), eq(4usize))
                .times(1)
                .returning(|_, _, _, _, _| 4);
            let _guard = setup_mock(mock);

            let account = AccountRoot { slot_num: 1 };
            let result = account
                .path()
                .field(sfield::SignerEntries)
                .index(0)
                .get::<u32>();
            assert!(result.is_ok());
        }
    }

    mod account_fields {
        use super::*;
        use crate::host::setup_mock;
        use crate::types::account_id::ACCOUNT_ID_SIZE;
        use crate::types::blob::{DOMAIN_BLOB_SIZE, PUBLIC_KEY_BLOB_SIZE};

        #[test]
        fn test_mandatory_fields_return_ok() {
            let mut mock = MockHostBindings::new();

            // get_account
            expect_ledger_field(&mut mock, 1, sfield::Account, 20, 1);
            // owner_count
            expect_ledger_field(&mut mock, 1, sfield::OwnerCount, 4, 1);
            // previous_txn_id
            expect_ledger_field(&mut mock, 1, sfield::PreviousTxnID, 32, 1);
            // previous_txn_lgr_seq
            expect_ledger_field(&mut mock, 1, sfield::PreviousTxnLgrSeq, 4, 1);
            // sequence
            expect_ledger_field(&mut mock, 1, sfield::Sequence, 4, 1);
            // ledger_entry_type
            expect_ledger_field(&mut mock, 1, sfield::LedgerEntryType, 2, 1);

            let _guard = setup_mock(mock);

            let account = AccountRoot { slot_num: 1 };

            // All mandatory fields should return Ok
            assert!(account.get_account().is_ok());
            assert!(account.get_owner_count().is_ok());
            assert!(account.get_previous_txn_id().is_ok());
            assert!(account.get_previous_txn_lgr_seq().is_ok());
            assert!(account.get_sequence().is_ok());
            assert!(account.get_ledger_entry_type().is_ok());
        }

        #[test]
        fn test_optional_fields_return_some() {
            let mut mock = MockHostBindings::new();

            // account_txn_id
            expect_ledger_field(&mut mock, 1, sfield::AccountTxnID, 32, 1);
            // amm_id
            expect_ledger_field(&mut mock, 1, sfield::AMMID, 32, 1);
            // balance
            expect_ledger_field(&mut mock, 1, sfield::Balance, 48, 1);
            // burned_nf_tokens
            expect_ledger_field(&mut mock, 1, sfield::BurnedNFTokens, 4, 1);
            // domain
            expect_ledger_field(&mut mock, 1, sfield::Domain, DOMAIN_BLOB_SIZE, 1);
            // email_hash
            expect_ledger_field(&mut mock, 1, sfield::EmailHash, 16, 1);
            // first_nf_token_sequence
            expect_ledger_field(&mut mock, 1, sfield::FirstNFTokenSequence, 4, 1);
            // message_key
            expect_ledger_field(&mut mock, 1, sfield::MessageKey, PUBLIC_KEY_BLOB_SIZE, 1);
            // minted_nf_tokens
            expect_ledger_field(&mut mock, 1, sfield::MintedNFTokens, 4, 1);
            // nf_token_minter
            expect_ledger_field(&mut mock, 1, sfield::NFTokenMinter, 20, 1);
            // regular_key
            expect_ledger_field(&mut mock, 1, sfield::RegularKey, ACCOUNT_ID_SIZE, 1);
            // ticket_count
            expect_ledger_field(&mut mock, 1, sfield::TicketCount, 4, 1);
            // tick_size
            expect_ledger_field(&mut mock, 1, sfield::TickSize, 1, 1);
            // transfer_rate
            expect_ledger_field(&mut mock, 1, sfield::TransferRate, 4, 1);
            // wallet_locator
            expect_ledger_field(&mut mock, 1, sfield::WalletLocator, 32, 1);

            let _guard = setup_mock(mock);

            let account = AccountRoot { slot_num: 1 };

            // All optional fields should return Ok(Some(...))
            assert!(account.get_account_txn_id().unwrap().is_some());
            assert!(account.get_amm_id().unwrap().is_some());
            assert!(account.get_balance().unwrap().is_some());
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

            // account_txn_id
            mock.expect_get_ledger_obj_field()
                .with(eq(1), eq(sfield::AccountTxnID), always(), eq(32))
                .times(1)
                .returning(|_, _, _, _| FIELD_NOT_FOUND);
            // amm_id
            mock.expect_get_ledger_obj_field()
                .with(eq(1), eq(sfield::AMMID), always(), eq(32))
                .times(1)
                .returning(|_, _, _, _| FIELD_NOT_FOUND);
            // balance - variable size field, returns 0 for empty (Some with len=0)
            mock.expect_get_ledger_obj_field()
                .with(eq(1), eq(sfield::Balance), always(), eq(48))
                .times(1)
                .returning(|_, _, _, _| 0);
            // burned_nf_tokens
            mock.expect_get_ledger_obj_field()
                .with(eq(1), eq(sfield::BurnedNFTokens), always(), eq(4))
                .times(1)
                .returning(|_, _, _, _| FIELD_NOT_FOUND);
            // domain - variable size field, returns 0 for empty (Some with len=0)
            mock.expect_get_ledger_obj_field()
                .with(eq(1), eq(sfield::Domain), always(), eq(DOMAIN_BLOB_SIZE))
                .times(1)
                .returning(|_, _, _, _| 0);
            // email_hash
            mock.expect_get_ledger_obj_field()
                .with(eq(1), eq(sfield::EmailHash), always(), eq(16))
                .times(1)
                .returning(|_, _, _, _| FIELD_NOT_FOUND);
            // first_nf_token_sequence
            mock.expect_get_ledger_obj_field()
                .with(eq(1), eq(sfield::FirstNFTokenSequence), always(), eq(4))
                .times(1)
                .returning(|_, _, _, _| FIELD_NOT_FOUND);
            // message_key - variable size field, returns 0 for empty (Some with len=0)
            mock.expect_get_ledger_obj_field()
                .with(
                    eq(1),
                    eq(sfield::MessageKey),
                    always(),
                    eq(PUBLIC_KEY_BLOB_SIZE),
                )
                .times(1)
                .returning(|_, _, _, _| 0);
            // minted_nf_tokens
            mock.expect_get_ledger_obj_field()
                .with(eq(1), eq(sfield::MintedNFTokens), always(), eq(4))
                .times(1)
                .returning(|_, _, _, _| FIELD_NOT_FOUND);
            // nf_token_minter
            mock.expect_get_ledger_obj_field()
                .with(eq(1), eq(sfield::NFTokenMinter), always(), eq(20))
                .times(1)
                .returning(|_, _, _, _| FIELD_NOT_FOUND);
            // regular_key
            mock.expect_get_ledger_obj_field()
                .with(eq(1), eq(sfield::RegularKey), always(), eq(ACCOUNT_ID_SIZE))
                .times(1)
                .returning(|_, _, _, _| FIELD_NOT_FOUND);
            // ticket_count
            mock.expect_get_ledger_obj_field()
                .with(eq(1), eq(sfield::TicketCount), always(), eq(4))
                .times(1)
                .returning(|_, _, _, _| FIELD_NOT_FOUND);
            // tick_size
            mock.expect_get_ledger_obj_field()
                .with(eq(1), eq(sfield::TickSize), always(), eq(1))
                .times(1)
                .returning(|_, _, _, _| FIELD_NOT_FOUND);
            // transfer_rate
            mock.expect_get_ledger_obj_field()
                .with(eq(1), eq(sfield::TransferRate), always(), eq(4))
                .times(1)
                .returning(|_, _, _, _| FIELD_NOT_FOUND);
            // wallet_locator
            mock.expect_get_ledger_obj_field()
                .with(eq(1), eq(sfield::WalletLocator), always(), eq(32))
                .times(1)
                .returning(|_, _, _, _| FIELD_NOT_FOUND);

            let _guard = setup_mock(mock);

            let account = AccountRoot { slot_num: 1 };

            // Fixed-size optional fields should return Ok(None) when FIELD_NOT_FOUND
            assert!(account.get_account_txn_id().unwrap().is_none());
            assert!(account.get_amm_id().unwrap().is_none());
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
            let balance = account.get_balance().unwrap();
            assert!(balance.is_some());
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

            let account = AccountRoot { slot_num: 1 };
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

            let account = AccountRoot { slot_num: 1 };
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

        #[test]
        fn test_path_roots_a_current_obj_builder() {
            let mut mock = MockHostBindings::new();
            mock.expect_get_current_ledger_obj_nested_field()
                .with(always(), eq(4usize), always(), eq(4usize))
                .times(1)
                .returning(|_, _, _, _| 4);
            let _guard = setup_mock(mock);

            let escrow = TestCurrentLedgerObject;
            let result = escrow.path().field(sfield::Flags).get::<u32>();
            assert!(result.is_ok());
        }
    }
}
