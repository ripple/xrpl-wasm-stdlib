use crate::core::ledger_objects::{current_ledger_object, ledger_object};
use crate::core::types::account_id::AccountID;
use crate::core::types::amount::Amount;
use crate::core::types::blob::{
    Blob, CONDITION_BLOB_SIZE, ConditionBlob, DEFAULT_BLOB_SIZE, UriBlob,
};
use crate::core::types::contract_data::{ContractData, XRPL_CONTRACT_DATA_SIZE};
use crate::core::types::public_key::PUBLIC_KEY_BUFFER_SIZE;
use crate::core::types::uint::{Hash128, Hash256};

/// This module provides traits for interacting with XRP Ledger objects.
///
/// It defines common interfaces for accessing and manipulating different types of ledger objects,
/// particularly focusing on Escrow objects. The traits provide methods to get and set various
/// fields of ledger objects, with separate traits for current ledger objects and general ledger objects.
use crate::host::error_codes::{
    match_result_code, match_result_code_optional, match_result_code_with_expected_bytes,
};
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
        let mut buffer = [0u8; CONDITION_BLOB_SIZE];

        let result_code = unsafe {
            get_current_ledger_obj_field(sfield::Condition, buffer.as_mut_ptr(), buffer.len())
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
    fn get_finish_function(&self) -> Result<Option<Blob<{ DEFAULT_BLOB_SIZE }>>> {
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

        let result_code =
            unsafe { get_current_ledger_obj_field(sfield::Data, data.as_mut_ptr(), data.len()) };

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
        match_result_code_with_expected_bytes(result_code, data.len, || ())
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
                sfield::Amount,
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
                sfield::Condition,
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
    fn get_finish_function(&self) -> Result<Option<Blob<{ DEFAULT_BLOB_SIZE }>>> {
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
                sfield::Data,
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
    fn domain(&self) -> Result<Option<UriBlob>> {
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
    fn message_key(&self) -> Result<Option<Blob<{ PUBLIC_KEY_BUFFER_SIZE }>>> {
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
    // NOTE: Will be replaced fully by the next PR that adds new unit tests.
}
