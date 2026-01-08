//! # Transaction Field Access Traits
//!
//! This module defines traits for accessing fields from XRPL transactions in a type-safe manner.
//! It provides a structured interface for retrieving both common transaction fields (shared across
//! all transaction types) and transaction-specific fields (unique to particular transaction types).
//!
//! ## Overview
//!
//! XRPL transactions contain a variety of fields, some mandatory and others optional. This module
//! organizes field access into logical groups:
//!
//! - **Common Fields**: Fields present in all XRPL transactions (Account, Fee, Sequence, etc.)
//! - **Transaction-Specific Fields**: Fields unique to specific transaction types
//!
//! ## Design Philosophy
//!
//! The trait-based design provides several benefits:
//!
//! - **Type Safety**: Each field is accessed through methods with appropriate return types
//! - **Composability**: Transaction types can implement multiple traits as needed
//! - **Zero-Cost Abstraction**: Trait methods compile down to direct host function calls
//! - **Extensibility**: New transaction types can easily implement the relevant traits
//!
//! ## Field Categories
//!
//! ### Mandatory vs. Optional Fields
//!
//! - **Mandatory fields** return `Result<T>` and will error if missing
//! - **Optional fields** return `Result<Option<T>>` and return `None` if missing
//!
//! ### Field Types
//!
//! - **AccountID**: 20-byte account identifiers
//! - **Hash256**: 256-bit cryptographic hashes
//! - **Amount**: XRP amounts (with future support for tokens)
//! - **u32**: 32-bit unsigned integers for sequence numbers, flags, etc.
//! - **Blob**: Variable-length binary data
//! - **PublicKey**: 33-byte compressed public keys
//! - **TransactionType**: Enumerated transaction type identifiers

use crate::core::current_tx::{get_field, get_field_optional};
use crate::core::types::account_id::AccountID;
use crate::core::types::amount::Amount;
use crate::core::types::blob::{
    CONDITION_BLOB_SIZE, ConditionBlob, FULFILLMENT_BLOB_SIZE, FulfillmentBlob, SignatureBlob,
};
use crate::core::types::public_key::PublicKey;
use crate::core::types::transaction_type::TransactionType;
use crate::core::types::uint::Hash256;
use crate::host::error_codes::match_result_code_optional;
use crate::host::{Error, Result, get_tx_field};
use crate::sfield;

/// Trait providing access to common fields present in all XRPL transactions.
///
/// ## Implementation Requirements
///
/// Types implementing this trait should ensure they are used only in the context of a valid
/// XRPL transaction. The trait methods assume the current transaction context is properly
/// established by the XRPL Programmability environment.
pub trait TransactionCommonFields {
    /// Retrieves the account field from the current transaction.
    ///
    /// This field identifies (Required) The unique address of the account that initiated the
    /// transaction.
    ///
    /// # Returns
    ///
    /// Returns a `Result<AccountID>` where:
    /// * `Ok(AccountID)` - The 20-byte account identifier of the transaction sender
    /// * `Err(Error)` - If the field cannot be retrieved or has an unexpected size
    fn get_account(&self) -> Result<AccountID> {
        get_field(sfield::Account)
    }

    /// Retrieves the transaction type from the current transaction.
    ///
    /// This field specifies the type of transaction. Valid transaction types include:
    /// Payment, OfferCreate, TrustSet, and many others.
    ///
    /// # Returns
    ///
    /// Returns a `Result<TransactionType>` where:
    /// * `Ok(TransactionType)` - An enumerated value representing the transaction type
    /// * `Err(Error)` - If the field cannot be retrieved or has an unexpected size
    ///
    fn get_transaction_type(&self) -> Result<TransactionType> {
        get_field(sfield::TransactionType)
    }

    /// Retrieves the computation allowance from the current transaction.
    ///
    /// This field specifies the maximum computational resources that the transaction is
    /// allowed to consume during execution in the XRPL Programmability environment.
    /// It helps prevent runaway computations and ensures network stability.
    ///
    /// # Returns
    ///
    /// Returns a `Result<u32>` where:
    /// * `Ok(u32)` - The computation allowance value in platform-defined units
    /// * `Err(Error)` - If the field cannot be retrieved or has an unexpected size
    fn get_computation_allowance(&self) -> Result<u32> {
        get_field(sfield::ComputationAllowance)
    }

    /// Retrieves the fee amount from the current transaction.
    ///
    /// This field specifies the amount of XRP (in drops) that the sender is willing to pay
    /// as a transaction fee. The fee is consumed regardless of whether the transaction
    /// succeeds or fails, and higher fees can improve transaction priority during
    /// network congestion.
    ///
    /// # Returns
    ///
    /// Returns a `Result<Amount>` where:
    /// * `Ok(Amount)` - The fee amount as an XRP amount in drops
    /// * `Err(Error)` - If the field cannot be retrieved or has an unexpected size
    ///
    /// # Note
    ///
    /// Returns XRP amounts only (for now). Future versions may support other token types
    /// when the underlying amount handling is enhanced.
    fn get_fee(&self) -> Result<Amount> {
        get_field(sfield::Fee)
    }

    /// Retrieves the sequence number from the current transaction.
    ///
    /// This field represents the sequence number of the account sending the transaction. A
    /// transaction is only valid if the Sequence number is exactly 1 greater than the previous
    /// transaction from the same account. The special case 0 means the transaction is using a
    /// Ticket instead (Added by the TicketBatch amendment).
    ///
    /// # Returns
    ///
    /// Returns a `Result<u32>` where:
    /// * `Ok(u32)` - The transaction sequence number
    /// * `Err(Error)` - If the field cannot be retrieved or has an unexpected size
    ///
    /// # Note
    ///
    /// If the transaction uses tickets instead of sequence numbers, this field may not
    /// be present. In such cases, use `get_ticket_sequence()` instead.
    fn get_sequence(&self) -> Result<u32> {
        get_field(sfield::Sequence)
    }

    /// Retrieves the account transaction ID from the current transaction.
    ///
    /// This optional field contains the hash value identifying another transaction. If provided,
    /// this transaction is only valid if the sending account's previously sent transaction matches
    /// the provided hash.
    ///
    /// # Returns
    ///
    /// Returns a `Result<Option<Hash256>>` where:
    /// * `Ok(Some(Hash256))` - The hash of the required previous transaction
    /// * `Ok(None)` - If no previous transaction requirement is specified
    /// * `Err(Error)` - If an error occurred during field retrieval
    fn get_account_txn_id(&self) -> Result<Option<Hash256>> {
        get_field_optional(sfield::AccountTxnID)
    }

    /// Retrieves the `flags` field from the current transaction.
    ///
    /// This optional field contains a bitfield of transaction-specific flags that modify
    /// the transaction's behavior.
    ///
    /// # Returns
    ///
    /// Returns a `Result<Option<u32>>` where:
    /// * `Ok(Some(u32))` - The flags bitfield if present
    /// * `Ok(None)` - If no flags are specified (equivalent to flags = 0)
    /// * `Err(Error)` - If an error occurred during field retrieval
    fn get_flags(&self) -> Result<Option<u32>> {
        get_field_optional(sfield::Flags)
    }

    /// Retrieves the last ledger sequence from the current transaction.
    ///
    /// This optional field specifies the highest ledger index this transaction can appear in.
    /// Specifying this field places a strict upper limit on how long the transaction can wait to
    /// be validated or rejected. See Reliable Transaction Submission for more details.
    ///
    /// # Returns
    ///
    /// Returns a `Result<Option<u32>>` where:
    /// * `Ok(Some(u32))` - The maximum ledger index for transaction inclusion
    /// * `Ok(None)` - If no expiration is specified (transaction never expires)
    /// * `Err(Error)` - If an error occurred during field retrieval
    fn get_last_ledger_sequence(&self) -> Result<Option<u32>> {
        get_field_optional(sfield::LastLedgerSequence)
    }

    /// Retrieves the network ID from the current transaction.
    ///
    /// This optional field identifies the network ID of the chain this transaction is intended for.
    /// MUST BE OMITTED for Mainnet and some test networks. REQUIRED on chains whose network ID is
    /// 1025 or higher.
    ///
    /// # Returns
    ///
    /// Returns a `Result<Option<u32>>` where:
    /// * `Ok(Some(u32))` - The network identifier
    /// * `Ok(None)` - If no specific network is specified (uses default network)
    /// * `Err(Error)` - If an error occurred during field retrieval
    fn get_network_id(&self) -> Result<Option<u32>> {
        get_field_optional(sfield::NetworkID)
    }

    /// Retrieves the source tag from the current transaction.
    ///
    /// This optional field is an arbitrary integer used to identify the reason for this payment, or
    /// a sender on whose behalf this transaction is made. Conventionally, a refund should specify
    /// the initial payment's SourceTag as the refund payment's DestinationTag.
    ///
    /// # Returns
    ///
    /// Returns a `Result<Option<u32>>` where:
    /// * `Ok(Some(u32))` - The source tag identifier
    /// * `Ok(None)` - If no source tag is specified
    /// * `Err(Error)` - If an error occurred during field retrieval
    fn get_source_tag(&self) -> Result<Option<u32>> {
        get_field_optional(sfield::SourceTag)
    }

    /// Retrieves the signing public key from the current transaction.
    ///
    /// This field contains the hex representation of the public key that corresponds to the
    /// private key used to sign this transaction. If an empty string, this field indicates that a
    /// multi-signature is present in the Signers field instead.
    ///
    /// # Returns
    ///
    /// Returns a `Result<PublicKey>` where:
    /// * `Ok(PublicKey)` - The 33-byte compressed public key used for signing
    /// * `Err(Error)` - If the field cannot be retrieved or has an unexpected size
    ///
    /// # Security Note
    ///
    /// The presence of this field doesn't guarantee the signature is valid. Instead, this field
    /// only provides the key claimed to be used for signing. The XRPL network performs signature
    /// validation before transaction execution.
    fn get_signing_pub_key(&self) -> Result<PublicKey> {
        get_field(sfield::SigningPubKey)
    }

    /// Retrieves the ticket sequence from the current transaction.
    ///
    /// This optional field provides the sequence number of the ticket to use in place of a
    /// Sequence number. If this is provided, Sequence must be 0. Cannot be used with AccountTxnID.
    ///
    /// # Returns
    ///
    /// Returns a `Result<Option<u32>>` where:
    /// * `Ok(Some(u32))` - The ticket sequence number if the transaction uses tickets
    /// * `Ok(None)` - If the transaction uses traditional sequence numbering
    /// * `Err(Error)` - If an error occurred during field retrieval
    ///
    /// # Note
    ///
    /// Transactions use either `Sequence` or `TicketSequence`, but not both. Check this
    /// field when `get_sequence()` fails or when implementing ticket-aware logic.
    fn get_ticket_sequence(&self) -> Result<Option<u32>> {
        get_field_optional(sfield::TicketSequence)
    }

    /// Retrieves the transaction signature from the current transaction.
    ///
    /// This mandatory field contains the signature that verifies this transaction as originating
    /// from the account it says it is from.
    ///
    /// Signatures can be either:
    /// - 64 bytes for EdDSA (Ed25519) signatures
    /// - 70-72 bytes for ECDSA (secp256k1) signatures
    ///
    /// # Returns
    ///
    /// Returns a `Result<Signature>` where:
    /// * `Ok(Signature)` - The transaction signature (up to 72 bytes)
    /// * `Err(Error)` - If the field cannot be retrieved
    ///
    /// # Security Note
    ///
    /// The signature is validated by the XRPL network before transaction execution.
    /// In the programmability context, you can access the signature for logging or
    /// analysis purposes, but signature validation has already been performed.
    fn get_txn_signature(&self) -> Result<SignatureBlob> {
        get_field(sfield::TxnSignature)
    }
}

/// Trait providing access to fields specific to EscrowFinish transactions.
///
/// This trait extends `TransactionCommonFields` with methods for retrieving fields that are
/// unique to EscrowFinish transactions. EscrowFinish transactions are used to complete
/// time-based or condition-based escrows that were previously created with EscrowCreate
/// transactions.
///
/// ## Implementation Requirements
///
/// Types implementing this trait should:
/// - Also implement `TransactionCommonFields` for access to common transaction fields
/// - Only be used in the context of processing EscrowFinish transactions
/// - Ensure proper error handling when accessing conditional fields
pub trait EscrowFinishFields: TransactionCommonFields {
    /// Retrieves the owner account from the current EscrowFinish transaction.
    ///
    /// This mandatory field identifies the XRPL account that originally created the escrow
    /// with an EscrowCreate transaction. The owner is the account that deposited the XRP
    /// into the escrow and specified the conditions for its release.
    ///
    /// # Returns
    ///
    /// Returns a `Result<AccountID>` where:
    /// * `Ok(AccountID)` - The 20-byte account identifier of the escrow owner
    /// * `Err(Error)` - If the field cannot be retrieved or has an unexpected size
    fn get_owner(&self) -> Result<AccountID> {
        get_field(sfield::Owner)
    }

    /// Retrieves the offer sequence from the current EscrowFinish transaction.
    ///
    /// This mandatory field specifies the sequence number of the original EscrowCreate
    /// transaction that created the escrow being finished. This creates a unique reference
    /// to the specific escrow object, as escrows are identified by the combination of
    /// the owner account and the sequence number of the creating transaction.
    ///
    /// # Returns
    ///
    /// Returns a `Result<u32>` where:
    /// * `Ok(u32)` - The sequence number of the EscrowCreate transaction
    /// * `Err(Error)` - If the field cannot be retrieved or has an unexpected size
    fn get_offer_sequence(&self) -> Result<u32> {
        get_field(sfield::OfferSequence)
    }

    /// Retrieves the cryptographic condition from the current EscrowFinish transaction.
    ///
    /// This optional field contains the cryptographic condition in full crypto-condition format.
    /// For PREIMAGE-SHA-256 conditions, this is 39 bytes:
    /// - 2 bytes: type tag (A025)
    /// - 2 bytes: fingerprint length tag (8020)
    /// - 32 bytes: SHA-256 hash (fingerprint)
    /// - 2 bytes: cost length tag (8101)
    /// - 1 byte: cost value (00)
    ///
    /// # Returns
    ///
    /// Returns a `Result<Option<Condition>>` where:
    /// * `Ok(Some(Condition))` - The full crypto-condition if the escrow is conditional
    /// * `Ok(None)` - If the escrow has no cryptographic condition (time-based only)
    /// * `Err(Error)` - If an error occurred during field retrieval
    fn get_condition(&self) -> Result<Option<ConditionBlob>> {
        let mut buffer = [0u8; CONDITION_BLOB_SIZE];

        let result_code =
            unsafe { get_tx_field(sfield::Condition.into(), buffer.as_mut_ptr(), buffer.len()) };

        if result_code < 0 {
            Result::Err(Error::from_code(result_code))
        } else if result_code == 0 {
            Result::Ok(None)
        } else {
            let blob = ConditionBlob {
                data: buffer,
                len: result_code as usize,
            };
            Result::Ok(Some(blob))
        }
    }

    /// Retrieves the cryptographic fulfillment from the current EscrowFinish transaction.
    ///
    /// This optional field contains the cryptographic fulfillment that satisfies the condition
    /// specified in the original EscrowCreate transaction. The fulfillment must cryptographically
    /// prove that the condition's requirements have been met. This field is only required
    /// when the escrow has an associated condition.
    ///
    /// # Returns
    ///
    /// Returns a `Result<Option<Fulfillment>>` where:
    /// * `Ok(Some(Fulfillment))` - The fulfillment data if provided
    /// * `Ok(None)` - If no fulfillment is provided (valid for unconditional escrows)
    /// * `Err(Error)` - If an error occurred during field retrieval
    ///
    /// # Fulfillment Validation
    ///
    /// The XRPL network automatically validates that:
    /// - The fulfillment satisfies the escrow's condition
    /// - The fulfillment is properly formatted according to RFC 3814
    /// - The cryptographic proof is mathematically valid
    ///
    /// # Size Limits
    ///
    /// Fulfillments are limited to 256 bytes in the current XRPL implementation.
    /// This limit ensures network performance while supporting the most practical
    /// cryptographic proof scenarios.
    fn get_fulfillment(&self) -> Result<Option<FulfillmentBlob>> {
        // Fulfillment fields are limited in rippled to 256 bytes, so we don't use `get_blob_field`
        // but instead just use a smaller buffer directly.

        let mut buffer = [0u8; FULFILLMENT_BLOB_SIZE]; // <-- 256 is the current rippled cap.

        let result_code =
            unsafe { get_tx_field(sfield::Fulfillment.into(), buffer.as_mut_ptr(), 256) };
        match_result_code_optional(result_code, || {
            let blob = FulfillmentBlob {
                data: buffer,
                len: result_code as usize,
            };
            Some(blob)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::current_tx::escrow_finish::EscrowFinish;

    #[test]
    fn test_get_condition_returns_some_with_data() {
        // When the mock host function returns a positive value (buffer length),
        // get_condition should return Ok(Some(ConditionBlob))
        let escrow = EscrowFinish;
        let result = escrow.get_condition();

        // The mock returns buffer.len() which is CONDITION_BLOB_SIZE (128)
        assert!(result.is_ok());
        let condition_opt = result.unwrap();
        assert!(condition_opt.is_some());

        let condition = condition_opt.unwrap();
        assert_eq!(condition.len, CONDITION_BLOB_SIZE);
        assert_eq!(condition.capacity(), CONDITION_BLOB_SIZE);
    }

    #[test]
    fn test_get_fulfillment_returns_some_with_data() {
        // When the mock host function returns a positive value (buffer length),
        // get_fulfillment should return Ok(Some(FulfillmentBlob))
        let escrow = EscrowFinish;
        let result = escrow.get_fulfillment();

        // The mock returns buffer.len() which is 256 (the size passed to get_tx_field)
        assert!(result.is_ok());
        let fulfillment_opt = result.unwrap();
        assert!(fulfillment_opt.is_some());

        let fulfillment = fulfillment_opt.unwrap();
        assert_eq!(fulfillment.len, 256);
        assert_eq!(fulfillment.capacity(), FULFILLMENT_BLOB_SIZE);
    }

    #[test]
    fn test_get_condition_and_fulfillment_independence() {
        // Verify that get_condition and get_fulfillment can be called independently
        let escrow = EscrowFinish;

        let condition_result = escrow.get_condition();
        let fulfillment_result = escrow.get_fulfillment();

        assert!(condition_result.is_ok());
        assert!(fulfillment_result.is_ok());

        // Verify they have different sizes
        if let (Some(condition), Some(fulfillment)) =
            (condition_result.unwrap(), fulfillment_result.unwrap())
        {
            assert_eq!(condition.capacity(), CONDITION_BLOB_SIZE);
            assert_eq!(fulfillment.capacity(), FULFILLMENT_BLOB_SIZE);
            assert_ne!(condition.capacity(), fulfillment.capacity());
        }
    }
}
