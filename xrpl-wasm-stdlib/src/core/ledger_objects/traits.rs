use crate::core::ledger_objects::{current_ledger_object, ledger_object};
use crate::core::types::account_id::AccountID;
use crate::core::types::amount::Amount;
use crate::core::types::blob::{CONDITION_BLOB_SIZE, ConditionBlob, StandardBlob};
use crate::core::types::contract_data::{ContractData, XRPL_CONTRACT_DATA_SIZE};
use crate::core::types::uint::{Hash128, Hash256};

/// This module provides traits for interacting with XRP Ledger objects.
///
/// It defines common interfaces for accessing and manipulating different types of ledger objects,
/// particularly focusing on Escrow objects. The traits provide methods to get and set various
/// fields of ledger objects, with separate traits for current ledger objects and general ledger objects.
use crate::host::error_codes::{match_result_code, match_result_code_optional};
use crate::host::{Error, get_current_ledger_obj_field, get_ledger_obj_field, update_data};
use crate::host::{Result, Result::Err, Result::Ok};
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
        current_ledger_object::get_field(sfield::LedgerEntryType)
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

/// Trait providing access to fields specific to Escrow objects in the current ledger.
///
/// This trait extends `CurrentLedgerObjectCommonFields` and provides methods to access
/// fields that are specific to Escrow objects in the current ledger being processed.
pub trait CurrentEscrowFields: CurrentLedgerObjectCommonFields {
    /// The address of the owner (sender) of this escrow. This is the account that provided the XRP
    /// and gets it back if the escrow is canceled.
    fn get_account(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Account)
    }

    /// The amount currently held in the escrow (could be XRP, IOU, or MPT).
    fn get_amount(&self) -> Result<Amount> {
        current_ledger_object::get_field(sfield::Amount)
    }

    /// The escrow can be canceled if and only if this field is present and the time it specifies
    /// has passed. Specifically, this is specified as seconds since the Ripple Epoch and it
    /// "has passed" if it's earlier than the close time of the previous validated ledger.
    fn get_cancel_after(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::CancelAfter)
    }

    /// A PREIMAGE-SHA-256 crypto-condition in full crypto-condition format. If present, the EscrowFinish
    /// transaction must contain a fulfillment that satisfies this condition.
    fn get_condition(&self) -> Result<Option<ConditionBlob>> {
        let mut buffer = ConditionBlob::new();

        let result_code = unsafe {
            get_current_ledger_obj_field(
                sfield::Condition.into(),
                buffer.data.as_mut_ptr(),
                buffer.capacity(),
            )
        };

        match_result_code_optional(result_code, || {
            buffer.len = result_code as usize;
            (result_code > 0).then_some(buffer)
        })
    }

    /// The destination address where the XRP is paid if the escrow is successful.
    fn get_destination(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Destination)
    }

    /// A hint indicating which page of the destination's owner directory links to this object, in
    /// case the directory consists of multiple pages. Omitted on escrows created before enabling the fix1523 amendment.
    fn get_destination_node(&self) -> Result<Option<u64>> {
        current_ledger_object::get_field_optional(sfield::DestinationNode)
    }

    /// An arbitrary tag to further specify the destination for this escrow, such as a hosted
    /// recipient at the destination address.
    fn get_destination_tag(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::DestinationTag)
    }

    /// The time, in seconds since the Ripple Epoch, after which this escrow can be finished. Any
    /// EscrowFinish transaction before this time fails. (Specifically, this is compared with the
    /// close time of the previous validated ledger.)
    fn get_finish_after(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::FinishAfter)
    }

    // TODO: Implement this function.
    // /// The value 0x0075, mapped to the string Escrow, indicates that this is an Escrow entry.
    // fn get_ledger_entry_type(&self) -> Result<LedgerEntryType> {
    //     return Ok(LedgerEntryType::Escrow);
    // }

    /// A hint indicating which page of the sender's owner directory links to this entry, in case
    /// the directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::OwnerNode)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::PreviousTxnLgrSeq)
    }

    /// An arbitrary tag to further specify the source for this escrow, such as a hosted recipient
    /// at the owner's address.
    fn get_source_tag(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::SourceTag)
    }

    /// The WASM code that is executing.
    fn get_finish_function(&self) -> Result<Option<StandardBlob>> {
        current_ledger_object::get_field_optional(sfield::FinishFunction)
    }

    /// Retrieves the contract `data` from the current escrow object.
    ///
    /// This function fetches the `data` field from the current ledger object and returns it as a
    /// ContractData structure. The data is read into a fixed-size buffer of XRPL_CONTRACT_DATA_SIZE.
    ///
    /// # Returns
    ///
    /// Returns a `Result<ContractData>` where:
    /// * `Ok(ContractData)` - Contains the retrieved data and its actual length
    /// * `Err(Error)` - If the retrieval operation failed
    fn get_data(&self) -> Result<ContractData> {
        let mut data: [u8; XRPL_CONTRACT_DATA_SIZE] = [0; XRPL_CONTRACT_DATA_SIZE];

        let result_code = unsafe {
            get_current_ledger_obj_field(sfield::Data.into(), data.as_mut_ptr(), data.len())
        };

        match result_code {
            code if code >= 0 => Ok(ContractData {
                data,
                len: code as usize,
            }),
            code => Err(Error::from_code(code)),
        }
    }

    /// Updates the contract data in the current escrow object.
    ///
    /// # Arguments
    ///
    /// * `data` - The contract data to update
    ///
    /// # Returns
    ///
    /// Returns a `Result<()>` where:
    /// * `Ok(())` - The data was successfully updated
    /// * `Err(Error)` - If the update operation failed
    fn update_current_escrow_data(data: ContractData) -> Result<()> {
        // TODO: Make sure rippled always deletes any existing data bytes in rippled, and sets the new
        // length to be `data.len` (e.g., if the developer writes 2 bytes, then that's the new
        // length and any old bytes are lost).
        let result_code = unsafe { update_data(data.data.as_ptr(), data.len) };
        match_result_code(result_code, || ())
    }
}

/// Trait providing access to fields specific to Escrow objects in any ledger.
///
/// This trait extends `LedgerObjectCommonFields` and provides methods to access
/// fields that are specific to Escrow objects in any ledger, not just the current one.
/// Each method requires a register number to identify which ledger object to access.
pub trait EscrowFields: LedgerObjectCommonFields {
    /// The address of the owner (sender) of this escrow. This is the account that provided the XRP
    /// and gets it back if the escrow is canceled.
    fn get_account(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Account)
    }

    /// The amount of XRP, in drops, currently held in the escrow.
    fn get_amount(&self) -> Result<Amount> {
        // Create a buffer large enough for any Amount type
        const BUFFER_SIZE: usize = 48usize;
        let mut buffer = [0u8; BUFFER_SIZE];

        let result_code = unsafe {
            get_ledger_obj_field(
                self.get_slot_num(),
                sfield::Amount.into(),
                buffer.as_mut_ptr(),
                buffer.len(),
            )
        };

        match_result_code(result_code, || Amount::from(buffer))
    }

    /// The escrow can be canceled if and only if this field is present and the time it specifies
    /// has passed. Specifically, this is specified as seconds since the Ripple Epoch and it
    /// "has passed" if it's earlier than the close time of the previous validated ledger.
    fn get_cancel_after(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::CancelAfter)
    }

    /// A PREIMAGE-SHA-256 crypto-condition in full crypto-condition format. If present, the EscrowFinish
    /// transaction must contain a fulfillment that satisfies this condition.
    fn get_condition(&self) -> Result<Option<ConditionBlob>> {
        let mut buffer = [0u8; CONDITION_BLOB_SIZE];

        let result_code = unsafe {
            get_ledger_obj_field(
                self.get_slot_num(),
                sfield::Condition.into(),
                buffer.as_mut_ptr(),
                buffer.len(),
            )
        };

        match_result_code_optional(result_code, || {
            if result_code > 0 {
                let blob = ConditionBlob {
                    data: buffer,
                    len: result_code as usize,
                };
                Some(blob)
            } else {
                None
            }
        })
    }

    /// The destination address where the XRP is paid if the escrow is successful.
    fn get_destination(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Destination)
    }

    /// A hint indicating which page of the destination's owner directory links to this object, in
    /// case the directory consists of multiple pages. Omitted on escrows created before enabling the fix1523 amendment.
    fn get_destination_node(&self) -> Result<Option<u64>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::DestinationNode)
    }

    /// An arbitrary tag to further specify the destination for this escrow, such as a hosted
    /// recipient at the destination address.
    fn get_destination_tag(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::DestinationTag)
    }

    /// The time, in seconds since the Ripple Epoch, after which this escrow can be finished. Any
    /// EscrowFinish transaction before this time fails. (Specifically, this is compared with the
    /// close time of the previous validated ledger.)
    fn get_finish_after(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::FinishAfter)
    }

    // TODO: Implement this function.
    // /// The value 0x0075, mapped to the string Escrow, indicates that this is an Escrow entry.
    // fn get_ledger_entry_type(&self) -> Result<LedgerEntryType> {
    //     return Ok(LedgerEntryType::Escrow);
    // }

    /// A hint indicating which page of the sender's owner directory links to this entry, in case
    /// the directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::OwnerNode)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }

    /// An arbitrary tag to further specify the source for this escrow, such as a hosted recipient
    /// at the owner's address.
    fn get_source_tag(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::SourceTag)
    }

    /// The WASM code that is executing.
    fn get_finish_function(&self) -> Result<Option<StandardBlob>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::FinishFunction)
    }

    /// Retrieves the contract data from the specified ledger object.
    ///
    /// This function fetches the `data` field from the ledger object at the specified register
    /// and returns it as a ContractData structure. The data is read into a fixed-size buffer
    /// of XRPL_CONTRACT_DATA_SIZE.
    ///
    /// # Arguments
    ///
    /// * `register_num` - The register number where the ledger object is stored
    ///
    /// # Returns
    ///
    /// Returns a `Result<ContractData>` where:
    /// * `Ok(ContractData)` - Contains the retrieved data and its actual length
    /// * `Err(Error)` - If the retrieval operation failed
    fn get_data(&self) -> Result<ContractData> {
        let mut data: [u8; XRPL_CONTRACT_DATA_SIZE] = [0; XRPL_CONTRACT_DATA_SIZE];

        let result_code = unsafe {
            get_ledger_obj_field(
                self.get_slot_num(),
                sfield::Data.into(),
                data.as_mut_ptr(),
                data.len(),
            )
        };

        match result_code {
            code if code >= 0 => Ok(ContractData {
                data,
                len: code as usize,
            }),
            code => Err(Error::from_code(code)),
        }
    }
}

/// Trait providing access to fields specific to AccountRoot objects in any ledger.
///
/// This trait extends `LedgerObjectCommonFields` and provides methods to access
/// fields that are specific to Escrow objects in any ledger, not just the current one.
/// Each method requires a register number to identify which ledger object to access.
pub trait AccountFields: LedgerObjectCommonFields {
    /// The identifying address of the account.
    fn get_account(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Account)
    }

    /// AccountTxnID field for the account.
    fn account_txn_id(&self) -> Result<Option<Hash256>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::AccountTxnID)
    }

    /// The ledger entry ID of the corresponding AMM ledger entry. Set during account creation; cannot be modified.
    /// If present, indicates that this is a special AMM AccountRoot; always omitted on non-AMM accounts.
    /// (Added by the AMM amendment)
    fn amm_id(&self) -> Result<Option<Hash256>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::AMMID)
    }

    /// The account's current XRP balance in drops.
    fn balance(&self) -> Result<Option<Amount>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::Balance)
    }

    /// How many total of this account's issued non-fungible tokens have been burned.
    /// This number is always equal or less than MintedNFTokens.
    fn burned_nf_tokens(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::BurnedNFTokens)
    }

    /// A domain associated with this account. In JSON, this is the hexadecimal for the ASCII representation of the
    /// domain. Cannot be more than 256 bytes in length.
    fn domain(&self) -> Result<Option<StandardBlob>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::Domain)
    }

    /// The MD5 hash of an email address. Clients can use this to look up an avatar through services such as Gravatar.
    fn email_hash(&self) -> Result<Option<Hash128>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::EmailHash)
    }

    /// The account's Sequence Number at the time it minted its first non-fungible-token.
    /// (Added by the fixNFTokenRemint amendment)
    fn first_nf_token_sequence(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::FirstNFTokenSequence)
    }

    /// The value 0x0061, mapped to the string AccountRoot, indicates that this is an AccountRoot object.
    fn ledger_entry_type(&self) -> Result<u16> {
        ledger_object::get_field(self.get_slot_num(), sfield::LedgerEntryType)
    }

    /// A public key that may be used to send encrypted messages to this account. In JSON, uses hexadecimal.
    /// Must be exactly 33 bytes, with the first byte indicating the key type: 0x02 or 0x03 for secp256k1 keys,
    /// 0xED for Ed25519 keys.
    fn message_key(&self) -> Result<Option<StandardBlob>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::MessageKey)
    }

    /// How many total non-fungible tokens have been minted by and on behalf of this account.
    /// (Added by the NonFungibleTokensV1_1 amendment)
    fn minted_nf_tokens(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::MintedNFTokens)
    }

    /// Another account that can mint non-fungible tokens on behalf of this account.
    /// (Added by the NonFungibleTokensV1_1 amendment)
    fn nf_token_minter(&self) -> Result<Option<AccountID>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::NFTokenMinter)
    }

    /// The number of objects this account owns in the ledger, which contributes to its owner reserve.
    fn owner_count(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::OwnerCount)
    }

    /// The identifying hash of the transaction that most recently modified this object.
    fn previous_txn_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this object.
    fn previous_txn_lgr_seq(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }

    /// The address of a key pair that can be used to sign transactions for this account instead of the master key.
    /// Use a SetRegularKey transaction to change this value.
    fn regular_key(&self) -> Result<Option<AccountID>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::RegularKey)
    }

    /// The sequence number of the next valid transaction for this account.
    fn sequence(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::Sequence)
    }

    /// How many Tickets this account owns in the ledger. This is updated automatically to ensure that
    /// the account stays within the hard limit of 250 Tickets at a time. This field is omitted if the account has zero
    /// Tickets. (Added by the TicketBatch amendment.)
    fn ticket_count(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::TicketCount)
    }

    /// How many significant digits to use for exchange rates of Offers involving currencies issued by this address.
    /// Valid values are 3 to 15, inclusive. (Added by the TickSize amendment.)
    fn tick_size(&self) -> Result<Option<u8>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::TickSize)
    }

    /// A transfer fee to charge other users for sending currency issued by this account to each other.
    fn transfer_rate(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::TransferRate)
    }

    /// An arbitrary 256-bit value that users can set.
    fn wallet_locator(&self) -> Result<Option<Hash256>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::WalletLocator)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::ledger_objects::LedgerObjectFieldGetter;
    use crate::core::ledger_objects::account_root::AccountRoot;
    use crate::host::error_codes::{FIELD_NOT_FOUND, INTERNAL_ERROR, INVALID_FIELD};
    use crate::host::host_bindings_trait::MockHostBindings;
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
            expect_current_field(&mut mock, sfield::LedgerEntryType, 2, 1);

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

            mock.expect_get_current_ledger_obj_field()
                .with(eq(sfield::LedgerEntryType), always(), eq(2))
                .times(1)
                .returning(|_, _, _| INTERNAL_ERROR);

            let _guard = setup_mock(mock);

            // Note: slot is ignored for this test, but required to instantiate the struct.
            let account = AccountRoot { slot_num: -1 };
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
    }

    mod account_fields {
        use super::*;
        use crate::host::setup_mock;

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
            assert!(account.owner_count().is_ok());
            assert!(account.previous_txn_id().is_ok());
            assert!(account.previous_txn_lgr_seq().is_ok());
            assert!(account.sequence().is_ok());
            assert!(account.ledger_entry_type().is_ok());
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
            // domain - StandardBlob uses 1024 bytes
            expect_ledger_field(&mut mock, 1, sfield::Domain, 1024, 1);
            // email_hash
            expect_ledger_field(&mut mock, 1, sfield::EmailHash, 16, 1);
            // first_nf_token_sequence
            expect_ledger_field(&mut mock, 1, sfield::FirstNFTokenSequence, 4, 1);
            // message_key - StandardBlob uses 1024 bytes
            expect_ledger_field(&mut mock, 1, sfield::MessageKey, 1024, 1);
            // minted_nf_tokens
            expect_ledger_field(&mut mock, 1, sfield::MintedNFTokens, 4, 1);
            // nf_token_minter
            expect_ledger_field(&mut mock, 1, sfield::NFTokenMinter, 20, 1);
            // regular_key
            expect_ledger_field(&mut mock, 1, sfield::RegularKey, 20, 1);
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
            assert!(account.account_txn_id().unwrap().is_some());
            assert!(account.amm_id().unwrap().is_some());
            assert!(account.balance().unwrap().is_some());
            assert!(account.burned_nf_tokens().unwrap().is_some());
            assert!(account.domain().unwrap().is_some());
            assert!(account.email_hash().unwrap().is_some());
            assert!(account.first_nf_token_sequence().unwrap().is_some());
            assert!(account.message_key().unwrap().is_some());
            assert!(account.minted_nf_tokens().unwrap().is_some());
            assert!(account.nf_token_minter().unwrap().is_some());
            assert!(account.regular_key().unwrap().is_some());
            assert!(account.ticket_count().unwrap().is_some());
            assert!(account.tick_size().unwrap().is_some());
            assert!(account.transfer_rate().unwrap().is_some());
            assert!(account.wallet_locator().unwrap().is_some());
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
            // domain - variable size field, returns 0 for empty (Some with len=0) - StandardBlob uses 1024 bytes
            mock.expect_get_ledger_obj_field()
                .with(eq(1), eq(sfield::Domain), always(), eq(1024))
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
            // message_key - variable size field, returns 0 for empty (Some with len=0) - StandardBlob uses 1024 bytes
            mock.expect_get_ledger_obj_field()
                .with(eq(1), eq(sfield::MessageKey), always(), eq(1024))
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
                .with(eq(1), eq(sfield::RegularKey), always(), eq(20))
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
            assert!(account.account_txn_id().unwrap().is_none());
            assert!(account.amm_id().unwrap().is_none());
            assert!(account.burned_nf_tokens().unwrap().is_none());
            assert!(account.email_hash().unwrap().is_none());
            assert!(account.first_nf_token_sequence().unwrap().is_none());
            assert!(account.minted_nf_tokens().unwrap().is_none());
            assert!(account.nf_token_minter().unwrap().is_none());
            assert!(account.regular_key().unwrap().is_none());
            assert!(account.ticket_count().unwrap().is_none());
            assert!(account.tick_size().unwrap().is_none());
            assert!(account.transfer_rate().unwrap().is_none());
            assert!(account.wallet_locator().unwrap().is_none());

            // Variable-size optional fields return Some with len=0 when not found
            // (they cannot distinguish between "not present" and "present with 0 bytes")
            let balance = account.balance().unwrap();
            assert!(balance.is_some());
            let domain = account.domain().unwrap();
            assert!(domain.is_some());
            assert_eq!(domain.unwrap().len, 0);
            let message_key = account.message_key().unwrap();
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
        use crate::core::ledger_objects::current_escrow::CurrentEscrow;
        use crate::host::setup_mock;

        #[test]
        fn test_mandatory_fields_return_ok() {
            let mut mock = MockHostBindings::new();

            // get_flags
            expect_current_field(&mut mock, sfield::Flags, 4, 1);
            // get_ledger_entry_type
            expect_current_field(&mut mock, sfield::LedgerEntryType, 2, 1);

            let _guard = setup_mock(mock);

            let escrow = CurrentEscrow;

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

            let escrow = CurrentEscrow;
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

            let escrow = CurrentEscrow;
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

            let escrow = CurrentEscrow;
            let result = escrow.get_flags();

            assert!(result.is_err());
            assert_eq!(result.err().unwrap().code(), INVALID_FIELD);
        }
    }

    mod current_escrow_fields {
        use super::*;
        use crate::core::ledger_objects::current_escrow::CurrentEscrow;
        use crate::host::setup_mock;

        #[test]
        fn test_mandatory_fields_return_ok() {
            let mut mock = MockHostBindings::new();

            // get_account
            expect_current_field(&mut mock, sfield::Account, 20, 1);
            // get_amount
            expect_current_field(&mut mock, sfield::Amount, 48, 1);
            // get_destination
            expect_current_field(&mut mock, sfield::Destination, 20, 1);
            // get_owner_node
            expect_current_field(&mut mock, sfield::OwnerNode, 8, 1);
            // get_previous_txn_id
            expect_current_field(&mut mock, sfield::PreviousTxnID, 32, 1);
            // get_previous_txn_lgr_seq
            expect_current_field(&mut mock, sfield::PreviousTxnLgrSeq, 4, 1);
            // get_data (mandatory for escrow)
            expect_current_field(&mut mock, sfield::Data, 4096, 1);

            let _guard = setup_mock(mock);

            let escrow = CurrentEscrow;

            // All mandatory fields should return Ok
            assert!(escrow.get_account().is_ok());
            assert!(escrow.get_amount().is_ok());
            assert!(escrow.get_destination().is_ok());
            assert!(escrow.get_owner_node().is_ok());
            assert!(escrow.get_previous_txn_id().is_ok());
            assert!(escrow.get_previous_txn_lgr_seq().is_ok());
            assert!(escrow.get_data().is_ok());
        }

        #[test]
        fn test_optional_fields_return_some() {
            let mut mock = MockHostBindings::new();

            // get_cancel_after
            expect_current_field(&mut mock, sfield::CancelAfter, 4, 1);
            // get_condition
            expect_current_field(&mut mock, sfield::Condition, 128, 1);
            // get_destination_node
            expect_current_field(&mut mock, sfield::DestinationNode, 8, 1);
            // get_destination_tag
            expect_current_field(&mut mock, sfield::DestinationTag, 4, 1);
            // get_finish_after
            expect_current_field(&mut mock, sfield::FinishAfter, 4, 1);
            // get_source_tag
            expect_current_field(&mut mock, sfield::SourceTag, 4, 1);
            // get_finish_function - StandardBlob uses 1024 bytes
            expect_current_field(&mut mock, sfield::FinishFunction, 1024, 1);

            let _guard = setup_mock(mock);

            let escrow = CurrentEscrow;

            // All optional fields should return Ok(Some(...))
            assert!(escrow.get_cancel_after().unwrap().is_some());
            assert!(escrow.get_condition().unwrap().is_some());
            assert!(escrow.get_destination_node().unwrap().is_some());
            assert!(escrow.get_destination_tag().unwrap().is_some());
            assert!(escrow.get_finish_after().unwrap().is_some());
            assert!(escrow.get_source_tag().unwrap().is_some());
            assert!(escrow.get_finish_function().unwrap().is_some());
        }

        #[test]
        fn test_optional_fields_return_none_when_field_not_found() {
            let mut mock = MockHostBindings::new();

            // get_cancel_after
            mock.expect_get_current_ledger_obj_field()
                .with(eq(sfield::CancelAfter), always(), eq(4))
                .times(1)
                .returning(|_, _, _| FIELD_NOT_FOUND);
            // get_condition - returns 0 for None
            mock.expect_get_current_ledger_obj_field()
                .with(eq(sfield::Condition), always(), eq(128))
                .times(1)
                .returning(|_, _, _| 0);
            // get_destination_node
            mock.expect_get_current_ledger_obj_field()
                .with(eq(sfield::DestinationNode), always(), eq(8))
                .times(1)
                .returning(|_, _, _| FIELD_NOT_FOUND);
            // get_destination_tag
            mock.expect_get_current_ledger_obj_field()
                .with(eq(sfield::DestinationTag), always(), eq(4))
                .times(1)
                .returning(|_, _, _| FIELD_NOT_FOUND);
            // get_finish_after
            mock.expect_get_current_ledger_obj_field()
                .with(eq(sfield::FinishAfter), always(), eq(4))
                .times(1)
                .returning(|_, _, _| FIELD_NOT_FOUND);
            // get_source_tag
            mock.expect_get_current_ledger_obj_field()
                .with(eq(sfield::SourceTag), always(), eq(4))
                .times(1)
                .returning(|_, _, _| FIELD_NOT_FOUND);
            // get_finish_function - variable size field, returns 0 for empty (Some with len=0) - StandardBlob uses 1024 bytes
            mock.expect_get_current_ledger_obj_field()
                .with(eq(sfield::FinishFunction), always(), eq(1024))
                .times(1)
                .returning(|_, _, _| 0);

            let _guard = setup_mock(mock);

            let escrow = CurrentEscrow;

            // Fixed-size optional fields should return Ok(None) when FIELD_NOT_FOUND
            assert!(escrow.get_cancel_after().unwrap().is_none());
            assert!(escrow.get_condition().unwrap().is_none());
            assert!(escrow.get_destination_node().unwrap().is_none());
            assert!(escrow.get_destination_tag().unwrap().is_none());
            assert!(escrow.get_finish_after().unwrap().is_none());
            assert!(escrow.get_source_tag().unwrap().is_none());

            // Variable-size optional fields return Some with len=0 when not found
            let finish_function = escrow.get_finish_function().unwrap();
            assert!(finish_function.is_some());
            assert_eq!(finish_function.unwrap().len, 0);
        }

        #[test]
        fn test_mandatory_fields_return_error_on_internal_error() {
            let mut mock = MockHostBindings::new();

            // get_account with INTERNAL_ERROR
            mock.expect_get_current_ledger_obj_field()
                .with(eq(sfield::Account), always(), eq(20))
                .times(1)
                .returning(|_, _, _| INTERNAL_ERROR);

            let _guard = setup_mock(mock);

            let escrow = CurrentEscrow;
            let result = escrow.get_account();

            assert!(result.is_err());
            assert_eq!(result.err().unwrap().code(), INTERNAL_ERROR);
        }

        #[test]
        fn test_get_data_returns_error_on_internal_error() {
            let mut mock = MockHostBindings::new();

            mock.expect_get_current_ledger_obj_field()
                .with(eq(sfield::Data), always(), eq(4096))
                .times(1)
                .returning(|_, _, _| INTERNAL_ERROR);

            let _guard = setup_mock(mock);

            let escrow = CurrentEscrow;
            let result = escrow.get_data();

            assert!(result.is_err());
            assert_eq!(result.err().unwrap().code(), INTERNAL_ERROR);
        }

        #[test]
        fn test_mandatory_fields_return_error_on_invalid_field() {
            let mut mock = MockHostBindings::new();

            // get_account with INVALID_FIELD
            mock.expect_get_current_ledger_obj_field()
                .with(eq(sfield::Account), always(), eq(20))
                .times(1)
                .returning(|_, _, _| INVALID_FIELD);

            let _guard = setup_mock(mock);

            let escrow = CurrentEscrow;
            let result = escrow.get_account();

            assert!(result.is_err());
            assert_eq!(result.err().unwrap().code(), INVALID_FIELD);
        }
    }

    mod escrow_fields {
        use super::*;
        use crate::core::ledger_objects::escrow::Escrow;
        use crate::host::setup_mock;

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
            // get_data (mandatory for escrow)
            expect_ledger_field(&mut mock, 1, sfield::Data, 4096, 1);

            let _guard = setup_mock(mock);

            let escrow = Escrow { slot_num: 1 };

            // All mandatory fields should return Ok
            assert!(escrow.get_account().is_ok());
            assert!(escrow.get_amount().is_ok());
            assert!(escrow.get_destination().is_ok());
            assert!(escrow.get_owner_node().is_ok());
            assert!(escrow.get_previous_txn_id().is_ok());
            assert!(escrow.get_previous_txn_lgr_seq().is_ok());
            assert!(escrow.get_data().is_ok());
        }

        #[test]
        fn test_optional_fields_return_some() {
            let mut mock = MockHostBindings::new();

            // get_cancel_after
            expect_ledger_field(&mut mock, 1, sfield::CancelAfter, 4, 1);
            // get_condition
            expect_ledger_field(&mut mock, 1, sfield::Condition, 128, 1);
            // get_destination_node
            expect_ledger_field(&mut mock, 1, sfield::DestinationNode, 8, 1);
            // get_destination_tag
            expect_ledger_field(&mut mock, 1, sfield::DestinationTag, 4, 1);
            // get_finish_after
            expect_ledger_field(&mut mock, 1, sfield::FinishAfter, 4, 1);
            // get_source_tag
            expect_ledger_field(&mut mock, 1, sfield::SourceTag, 4, 1);
            // get_finish_function - StandardBlob uses 1024 bytes
            expect_ledger_field(&mut mock, 1, sfield::FinishFunction, 1024, 1);

            let _guard = setup_mock(mock);

            let escrow = Escrow { slot_num: 1 };

            // All optional fields should return Ok(Some(...))
            assert!(escrow.get_cancel_after().unwrap().is_some());
            assert!(escrow.get_condition().unwrap().is_some());
            assert!(escrow.get_destination_node().unwrap().is_some());
            assert!(escrow.get_destination_tag().unwrap().is_some());
            assert!(escrow.get_finish_after().unwrap().is_some());
            assert!(escrow.get_source_tag().unwrap().is_some());
            assert!(escrow.get_finish_function().unwrap().is_some());
        }

        #[test]
        fn test_optional_fields_return_none_when_field_not_found() {
            let mut mock = MockHostBindings::new();

            // get_cancel_after
            mock.expect_get_ledger_obj_field()
                .with(eq(1), eq(sfield::CancelAfter), always(), eq(4))
                .times(1)
                .returning(|_, _, _, _| FIELD_NOT_FOUND);
            // get_condition - returns 0 for None
            mock.expect_get_ledger_obj_field()
                .with(eq(1), eq(sfield::Condition), always(), eq(128))
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
            // get_finish_function - variable size field, returns 0 for empty (Some with len=0) - StandardBlob uses 1024 bytes
            mock.expect_get_ledger_obj_field()
                .with(eq(1), eq(sfield::FinishFunction), always(), eq(1024))
                .times(1)
                .returning(|_, _, _, _| 0);

            let _guard = setup_mock(mock);

            let escrow = Escrow { slot_num: 1 };

            // Fixed-size optional fields should return Ok(None) when FIELD_NOT_FOUND
            assert!(escrow.get_cancel_after().unwrap().is_none());
            assert!(escrow.get_condition().unwrap().is_none());
            assert!(escrow.get_destination_node().unwrap().is_none());
            assert!(escrow.get_destination_tag().unwrap().is_none());
            assert!(escrow.get_finish_after().unwrap().is_none());
            assert!(escrow.get_source_tag().unwrap().is_none());

            // Variable-size optional fields return Some with len=0 when not found
            let finish_function = escrow.get_finish_function().unwrap();
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

            let escrow = Escrow { slot_num: 1 };
            let result = escrow.get_account();

            assert!(result.is_err());
            assert_eq!(result.err().unwrap().code(), INTERNAL_ERROR);
        }

        #[test]
        fn test_get_data_returns_error_on_internal_error() {
            let mut mock = MockHostBindings::new();

            mock.expect_get_ledger_obj_field()
                .with(eq(1), eq(sfield::Data), always(), eq(4096))
                .times(1)
                .returning(|_, _, _, _| INTERNAL_ERROR);

            let _guard = setup_mock(mock);

            let escrow = Escrow { slot_num: 1 };
            let result = escrow.get_data();

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

            let escrow = Escrow { slot_num: 1 };
            let result = escrow.get_account();

            assert!(result.is_err());
            assert_eq!(result.err().unwrap().code(), INVALID_FIELD);
        }
    }
}
