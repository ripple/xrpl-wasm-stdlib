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
use crate::core::types::blob::{ConditionBlob, FulfillmentBlob, SignatureBlob};
use crate::core::types::public_key::PublicKey;
use crate::core::types::transaction_type::TransactionType;
use crate::core::types::uint::Hash256;
use crate::host::error_codes::match_result_code_optional;
use crate::host::{Result, get_tx_field};
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
        let mut buffer = ConditionBlob::new();
        let result_code = unsafe {
            get_tx_field(
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
        let mut buffer = FulfillmentBlob::new();
        let result_code = unsafe {
            get_tx_field(
                sfield::Fulfillment.into(),
                buffer.data.as_mut_ptr(),
                buffer.capacity(),
            )
        };
        match_result_code_optional(result_code, || {
            buffer.len = result_code as usize;
            (result_code > 0).then_some(buffer)
        })
    }
}

#[cfg(test)]
mod tests {

    use crate::host::host_bindings_trait::MockHostBindings;
    use crate::sfield::SField;
    use mockall::predicate::{always, eq};
    // ========================================
    // Test helper functions
    // ========================================

    /// Helper to set up a mock expectation for get_tx_field
    fn expect_tx_field<T: Send + std::fmt::Debug + PartialEq + 'static, const CODE: i32>(
        mock: &mut MockHostBindings,
        field: SField<T, CODE>,
        size: usize,
        times: usize,
    ) {
        mock.expect_get_tx_field()
            .with(eq(field), always(), eq(size))
            .times(times)
            .returning(move |_, _, _| size as i32);
    }

    mod escrow_finish_fields {

        mod optional_fields {
            use crate::core::current_tx::escrow_finish::EscrowFinish;
            use crate::core::current_tx::traits::EscrowFinishFields;
            use crate::core::current_tx::traits::tests::expect_tx_field;
            use crate::core::types::blob::{CONDITION_BLOB_SIZE, FULFILLMENT_BLOB_SIZE};
            use crate::host::error_codes::{FIELD_NOT_FOUND, INTERNAL_ERROR, INVALID_FIELD};
            use crate::host::host_bindings_trait::MockHostBindings;
            use crate::host::setup_mock;
            use crate::sfield;

            use crate::sfield::{Condition, Fulfillment};
            use mockall::predicate::{always, eq};

            #[test]
            fn test_optional_fields_return_some() {
                let mut mock = MockHostBindings::new();

                // get_condition
                expect_tx_field(&mut mock, Condition, CONDITION_BLOB_SIZE, 1);
                // get_fulfillment
                expect_tx_field(&mut mock, Fulfillment, FULFILLMENT_BLOB_SIZE, 1);

                let _guard = setup_mock(mock);

                let escrow = EscrowFinish;

                // All optional fields should return Ok(Some(...))
                let condition = escrow.get_condition().unwrap();
                assert!(condition.is_some());
                assert_eq!(condition.unwrap().len, CONDITION_BLOB_SIZE);

                let fulfillment = escrow.get_fulfillment().unwrap();
                assert!(fulfillment.is_some());
                assert_eq!(fulfillment.unwrap().len, FULFILLMENT_BLOB_SIZE);
            }

            #[test]
            fn test_optional_fields_return_none_when_zero_length() {
                let mut mock = MockHostBindings::new();

                // get_condition - returns None when result code is 0
                mock.expect_get_tx_field()
                    .with(eq(sfield::Condition), always(), eq(CONDITION_BLOB_SIZE))
                    .times(1)
                    .returning(|_, _, _| 0);
                // get_fulfillment - returns None when result code is 0
                mock.expect_get_tx_field()
                    .with(eq(sfield::Fulfillment), always(), eq(FULFILLMENT_BLOB_SIZE))
                    .times(1)
                    .returning(|_, _, _| 0);

                let _guard = setup_mock(mock);

                let escrow = EscrowFinish;

                // Variable-size optional fields return None when result code is 0 (not present)
                assert!(escrow.get_condition().unwrap().is_none());
                assert!(escrow.get_fulfillment().unwrap().is_none());
            }

            #[test]
            fn test_optional_fields_return_error_on_internal_error() {
                let mut mock = MockHostBindings::new();

                // get_condition
                mock.expect_get_tx_field()
                    .with(eq(sfield::Condition), always(), eq(CONDITION_BLOB_SIZE))
                    .times(1)
                    .returning(|_, _, _| INTERNAL_ERROR);
                // get_fulfillment
                mock.expect_get_tx_field()
                    .with(eq(sfield::Fulfillment), always(), eq(FULFILLMENT_BLOB_SIZE))
                    .times(1)
                    .returning(|_, _, _| INTERNAL_ERROR);

                let _guard = setup_mock(mock);

                let escrow = EscrowFinish;

                // Optional fields should also return Err on INTERNAL_ERROR
                let condition_result = escrow.get_condition();
                assert!(condition_result.is_err());
                assert_eq!(condition_result.err().unwrap().code(), INTERNAL_ERROR);

                let fulfillment_result = escrow.get_fulfillment();
                assert!(fulfillment_result.is_err());
                assert_eq!(fulfillment_result.err().unwrap().code(), INTERNAL_ERROR);
            }

            #[test]
            fn test_optional_fields_return_error_on_field_not_found() {
                let mut mock = MockHostBindings::new();

                // get_condition
                mock.expect_get_tx_field()
                    .with(eq(Condition), always(), eq(CONDITION_BLOB_SIZE))
                    .times(1)
                    .returning(|_, _, _| FIELD_NOT_FOUND);
                // get_fulfillment
                mock.expect_get_tx_field()
                    .with(eq(Fulfillment), always(), eq(FULFILLMENT_BLOB_SIZE))
                    .times(1)
                    .returning(|_, _, _| FIELD_NOT_FOUND);

                let _guard = setup_mock(mock);

                let escrow = EscrowFinish;

                // Optional fields return Err on FIELD_NOT_FOUND (not None)
                let condition_result = escrow.get_condition();
                assert!(condition_result.is_err());
                assert_eq!(condition_result.err().unwrap().code(), FIELD_NOT_FOUND);

                let fulfillment_result = escrow.get_fulfillment();
                assert!(fulfillment_result.is_err());
                assert_eq!(fulfillment_result.err().unwrap().code(), FIELD_NOT_FOUND);
            }

            #[test]
            fn test_optional_fields_return_error_on_invalid_field() {
                let mut mock = MockHostBindings::new();

                // get_condition
                mock.expect_get_tx_field()
                    .with(eq(sfield::Condition), always(), eq(CONDITION_BLOB_SIZE))
                    .times(1)
                    .returning(|_, _, _| INVALID_FIELD);
                // get_fulfillment
                mock.expect_get_tx_field()
                    .with(eq(sfield::Fulfillment), always(), eq(FULFILLMENT_BLOB_SIZE))
                    .times(1)
                    .returning(|_, _, _| INVALID_FIELD);

                let _guard = setup_mock(mock);

                let escrow = EscrowFinish;

                // Optional fields should also return Err on INVALID_FIELD
                let condition_result = escrow.get_condition();
                assert!(condition_result.is_err());
                assert_eq!(condition_result.err().unwrap().code(), INVALID_FIELD);

                let fulfillment_result = escrow.get_fulfillment();
                assert!(fulfillment_result.is_err());
                assert_eq!(fulfillment_result.err().unwrap().code(), INVALID_FIELD);
            }
        }

        mod mandatory_fields {
            use crate::core::current_tx::escrow_finish::EscrowFinish;
            use crate::core::current_tx::traits::EscrowFinishFields;
            use crate::core::current_tx::traits::tests::expect_tx_field;
            use crate::core::types::account_id::ACCOUNT_ID_SIZE;
            use crate::host::error_codes::{FIELD_NOT_FOUND, INTERNAL_ERROR, INVALID_FIELD};
            use crate::host::host_bindings_trait::MockHostBindings;
            use crate::host::setup_mock;
            use crate::sfield;
            use mockall::predicate::{always, eq};

            #[test]
            fn test_mandatory_fields_return_ok() {
                let mut mock = MockHostBindings::new();

                // get_owner
                expect_tx_field(&mut mock, sfield::Owner, ACCOUNT_ID_SIZE, 1);
                // get_offer_sequence
                expect_tx_field(&mut mock, sfield::OfferSequence, 4, 1);

                let _guard = setup_mock(mock);

                let escrow = EscrowFinish;

                // All mandatory fields should return Ok
                assert!(escrow.get_owner().is_ok());
                assert!(escrow.get_offer_sequence().is_ok());
            }

            #[test]
            fn test_mandatory_fields_return_error_when_zero_length() {
                let mut mock = MockHostBindings::new();

                // get_owner - returns 0 (zero length)
                mock.expect_get_tx_field()
                    .with(eq(sfield::Owner), always(), eq(ACCOUNT_ID_SIZE))
                    .times(1)
                    .returning(|_, _, _| 0);
                // get_offer_sequence - returns 0 (zero length)
                mock.expect_get_tx_field()
                    .with(eq(sfield::OfferSequence), always(), eq(4))
                    .times(1)
                    .returning(|_, _, _| 0);

                let _guard = setup_mock(mock);

                let escrow = EscrowFinish;

                // Mandatory fields should return Err when zero length (INTERNAL_ERROR due to byte mismatch)
                let owner_result = escrow.get_owner();
                assert!(owner_result.is_err());

                let offer_seq_result = escrow.get_offer_sequence();
                assert!(offer_seq_result.is_err());
            }

            #[test]
            fn test_mandatory_fields_return_error_on_field_not_found() {
                let mut mock = MockHostBindings::new();

                // get_owner
                mock.expect_get_tx_field()
                    .with(eq(sfield::Owner), always(), eq(ACCOUNT_ID_SIZE))
                    .times(1)
                    .returning(|_, _, _| FIELD_NOT_FOUND);
                // get_offer_sequence
                mock.expect_get_tx_field()
                    .with(eq(sfield::OfferSequence), always(), eq(4))
                    .times(1)
                    .returning(|_, _, _| FIELD_NOT_FOUND);

                let _guard = setup_mock(mock);

                let escrow = EscrowFinish;

                // All mandatory fields should return Err on FIELD_NOT_FOUND
                let owner_result = escrow.get_owner();
                assert!(owner_result.is_err());
                assert_eq!(owner_result.err().unwrap().code(), FIELD_NOT_FOUND);

                let offer_seq_result = escrow.get_offer_sequence();
                assert!(offer_seq_result.is_err());
                assert_eq!(offer_seq_result.err().unwrap().code(), FIELD_NOT_FOUND);
            }

            #[test]
            fn test_mandatory_fields_return_error_on_internal_error() {
                let mut mock = MockHostBindings::new();

                // get_owner
                mock.expect_get_tx_field()
                    .with(eq(sfield::Owner), always(), eq(ACCOUNT_ID_SIZE))
                    .times(1)
                    .returning(|_, _, _| INTERNAL_ERROR);
                // get_offer_sequence
                mock.expect_get_tx_field()
                    .with(eq(sfield::OfferSequence), always(), eq(4))
                    .times(1)
                    .returning(|_, _, _| INTERNAL_ERROR);

                let _guard = setup_mock(mock);

                let escrow = EscrowFinish;

                // All mandatory fields should return Err on INTERNAL_ERROR
                let owner_result = escrow.get_owner();
                assert!(owner_result.is_err());
                assert_eq!(owner_result.err().unwrap().code(), INTERNAL_ERROR);

                let offer_seq_result = escrow.get_offer_sequence();
                assert!(offer_seq_result.is_err());
                assert_eq!(offer_seq_result.err().unwrap().code(), INTERNAL_ERROR);
            }

            #[test]
            fn test_mandatory_fields_return_error_on_invalid_field() {
                let mut mock = MockHostBindings::new();

                // get_owner
                mock.expect_get_tx_field()
                    .with(eq(sfield::Owner), always(), eq(ACCOUNT_ID_SIZE))
                    .times(1)
                    .returning(|_, _, _| INVALID_FIELD);
                // get_offer_sequence
                mock.expect_get_tx_field()
                    .with(eq(sfield::OfferSequence), always(), eq(4))
                    .times(1)
                    .returning(|_, _, _| INVALID_FIELD);

                let _guard = setup_mock(mock);

                let escrow = EscrowFinish;

                // All mandatory fields should return Err on INVALID_FIELD
                let owner_result = escrow.get_owner();
                assert!(owner_result.is_err());
                assert_eq!(owner_result.err().unwrap().code(), INVALID_FIELD);

                let offer_seq_result = escrow.get_offer_sequence();
                assert!(offer_seq_result.is_err());
                assert_eq!(offer_seq_result.err().unwrap().code(), INVALID_FIELD);
            }
        }
    }

    mod transaction_common_fields {

        mod optional_fields {
            use crate::core::current_tx::escrow_finish::EscrowFinish;
            use crate::core::current_tx::traits::TransactionCommonFields;
            use crate::core::current_tx::traits::tests::expect_tx_field;
            use crate::core::types::uint::HASH256_SIZE;
            use crate::host::error_codes::{FIELD_NOT_FOUND, INTERNAL_ERROR, INVALID_FIELD};
            use crate::host::host_bindings_trait::MockHostBindings;
            use crate::host::setup_mock;
            use crate::sfield;
            use mockall::predicate::{always, eq};

            #[test]
            fn test_optional_fields_return_some() {
                let mut mock = MockHostBindings::new();

                // get_account_txn_id
                expect_tx_field(&mut mock, sfield::AccountTxnID, HASH256_SIZE, 1);
                // get_flags
                expect_tx_field(&mut mock, sfield::Flags, 4, 1);
                // get_last_ledger_sequence
                expect_tx_field(&mut mock, sfield::LastLedgerSequence, 4, 1);
                // get_network_id
                expect_tx_field(&mut mock, sfield::NetworkID, 4, 1);
                // get_source_tag
                expect_tx_field(&mut mock, sfield::SourceTag, 4, 1);
                // get_ticket_sequence
                expect_tx_field(&mut mock, sfield::TicketSequence, 4, 1);

                let _guard = setup_mock(mock);

                let tx = EscrowFinish;

                // All optional fields should return Ok(Some(...))
                assert!(tx.get_account_txn_id().unwrap().is_some());
                assert!(tx.get_flags().unwrap().is_some());
                assert!(tx.get_last_ledger_sequence().unwrap().is_some());
                assert!(tx.get_network_id().unwrap().is_some());
                assert!(tx.get_source_tag().unwrap().is_some());
                assert!(tx.get_ticket_sequence().unwrap().is_some());
            }

            #[test]
            fn test_optional_fields_return_none_when_field_not_found() {
                let mut mock = MockHostBindings::new();

                // get_account_txn_id
                mock.expect_get_tx_field()
                    .with(eq(sfield::AccountTxnID), always(), eq(HASH256_SIZE))
                    .times(1)
                    .returning(|_, _, _| FIELD_NOT_FOUND);
                // get_flags
                mock.expect_get_tx_field()
                    .with(eq(sfield::Flags), always(), eq(4))
                    .times(1)
                    .returning(|_, _, _| FIELD_NOT_FOUND);
                // get_last_ledger_sequence
                mock.expect_get_tx_field()
                    .with(eq(sfield::LastLedgerSequence), always(), eq(4))
                    .times(1)
                    .returning(|_, _, _| FIELD_NOT_FOUND);
                // get_network_id
                mock.expect_get_tx_field()
                    .with(eq(sfield::NetworkID), always(), eq(4))
                    .times(1)
                    .returning(|_, _, _| FIELD_NOT_FOUND);
                // get_source_tag
                mock.expect_get_tx_field()
                    .with(eq(sfield::SourceTag), always(), eq(4))
                    .times(1)
                    .returning(|_, _, _| FIELD_NOT_FOUND);
                // get_ticket_sequence
                mock.expect_get_tx_field()
                    .with(eq(sfield::TicketSequence), always(), eq(4))
                    .times(1)
                    .returning(|_, _, _| FIELD_NOT_FOUND);

                let _guard = setup_mock(mock);

                let tx = EscrowFinish;

                // Fixed-size optional fields should return Ok(None) when FIELD_NOT_FOUND
                assert!(tx.get_account_txn_id().unwrap().is_none());
                assert!(tx.get_flags().unwrap().is_none());
                assert!(tx.get_last_ledger_sequence().unwrap().is_none());
                assert!(tx.get_network_id().unwrap().is_none());
                assert!(tx.get_source_tag().unwrap().is_none());
                assert!(tx.get_ticket_sequence().unwrap().is_none());
            }

            #[test]
            fn test_optional_fields_return_none_when_zero_length() {
                let mut mock = MockHostBindings::new();

                // get_account_txn_id - returns 0 (zero length)
                mock.expect_get_tx_field()
                    .with(eq(sfield::AccountTxnID), always(), eq(HASH256_SIZE))
                    .times(1)
                    .returning(|_, _, _| 0);
                // get_flags - returns 0 (zero length)
                mock.expect_get_tx_field()
                    .with(eq(sfield::Flags), always(), eq(4))
                    .times(1)
                    .returning(|_, _, _| 0);
                // get_last_ledger_sequence - returns 0 (zero length)
                mock.expect_get_tx_field()
                    .with(eq(sfield::LastLedgerSequence), always(), eq(4))
                    .times(1)
                    .returning(|_, _, _| 0);
                // get_network_id - returns 0 (zero length)
                mock.expect_get_tx_field()
                    .with(eq(sfield::NetworkID), always(), eq(4))
                    .times(1)
                    .returning(|_, _, _| 0);
                // get_source_tag - returns 0 (zero length)
                mock.expect_get_tx_field()
                    .with(eq(sfield::SourceTag), always(), eq(4))
                    .times(1)
                    .returning(|_, _, _| 0);
                // get_ticket_sequence - returns 0 (zero length)
                mock.expect_get_tx_field()
                    .with(eq(sfield::TicketSequence), always(), eq(4))
                    .times(1)
                    .returning(|_, _, _| 0);

                // Mock trace_num calls (2 calls per field for byte mismatch: expected + actual)
                mock.expect_trace_num()
                    .with(always(), always(), always())
                    .returning(|_, _, _| 0)
                    .times(12); // 6 fields * 2 calls each

                let _guard = setup_mock(mock);

                let tx = EscrowFinish;

                // Fixed-size optional fields should return Err when zero length (byte mismatch)
                assert!(tx.get_account_txn_id().is_err());
                assert!(tx.get_flags().is_err());
                assert!(tx.get_last_ledger_sequence().is_err());
                assert!(tx.get_network_id().is_err());
                assert!(tx.get_source_tag().is_err());
                assert!(tx.get_ticket_sequence().is_err());
            }

            #[test]
            fn test_optional_fields_return_error_on_internal_error() {
                let mut mock = MockHostBindings::new();

                // get_account_txn_id
                mock.expect_get_tx_field()
                    .with(eq(sfield::AccountTxnID), always(), eq(HASH256_SIZE))
                    .times(1)
                    .returning(|_, _, _| INTERNAL_ERROR);
                // get_flags
                mock.expect_get_tx_field()
                    .with(eq(sfield::Flags), always(), eq(4))
                    .times(1)
                    .returning(|_, _, _| INTERNAL_ERROR);
                // get_last_ledger_sequence
                mock.expect_get_tx_field()
                    .with(eq(sfield::LastLedgerSequence), always(), eq(4))
                    .times(1)
                    .returning(|_, _, _| INTERNAL_ERROR);
                // get_network_id
                mock.expect_get_tx_field()
                    .with(eq(sfield::NetworkID), always(), eq(4))
                    .times(1)
                    .returning(|_, _, _| INTERNAL_ERROR);
                // get_source_tag
                mock.expect_get_tx_field()
                    .with(eq(sfield::SourceTag), always(), eq(4))
                    .times(1)
                    .returning(|_, _, _| INTERNAL_ERROR);
                // get_ticket_sequence
                mock.expect_get_tx_field()
                    .with(eq(sfield::TicketSequence), always(), eq(4))
                    .times(1)
                    .returning(|_, _, _| INTERNAL_ERROR);

                // Mock trace_num calls (1 call per field for error codes)
                mock.expect_trace_num()
                    .with(always(), always(), always())
                    .returning(|_, _, _| 0)
                    .times(6); // 6 fields * 1 call each

                let _guard = setup_mock(mock);

                let tx = EscrowFinish;

                // Optional fields should return Err on INTERNAL_ERROR
                let account_txn_id_result = tx.get_account_txn_id();
                assert!(account_txn_id_result.is_err());
                assert_eq!(account_txn_id_result.err().unwrap().code(), INTERNAL_ERROR);

                let flags_result = tx.get_flags();
                assert!(flags_result.is_err());
                assert_eq!(flags_result.err().unwrap().code(), INTERNAL_ERROR);

                let last_ledger_seq_result = tx.get_last_ledger_sequence();
                assert!(last_ledger_seq_result.is_err());
                assert_eq!(last_ledger_seq_result.err().unwrap().code(), INTERNAL_ERROR);

                let network_id_result = tx.get_network_id();
                assert!(network_id_result.is_err());
                assert_eq!(network_id_result.err().unwrap().code(), INTERNAL_ERROR);

                let source_tag_result = tx.get_source_tag();
                assert!(source_tag_result.is_err());
                assert_eq!(source_tag_result.err().unwrap().code(), INTERNAL_ERROR);

                let ticket_seq_result = tx.get_ticket_sequence();
                assert!(ticket_seq_result.is_err());
                assert_eq!(ticket_seq_result.err().unwrap().code(), INTERNAL_ERROR);
            }

            #[test]
            fn test_optional_fields_return_error_on_invalid_field() {
                let mut mock = MockHostBindings::new();

                // get_account_txn_id
                mock.expect_get_tx_field()
                    .with(eq(sfield::AccountTxnID), always(), eq(HASH256_SIZE))
                    .times(1)
                    .returning(|_, _, _| INVALID_FIELD);
                // get_flags
                mock.expect_get_tx_field()
                    .with(eq(sfield::Flags), always(), eq(4))
                    .times(1)
                    .returning(|_, _, _| INVALID_FIELD);
                // get_last_ledger_sequence
                mock.expect_get_tx_field()
                    .with(eq(sfield::LastLedgerSequence), always(), eq(4))
                    .times(1)
                    .returning(|_, _, _| INVALID_FIELD);
                // get_network_id
                mock.expect_get_tx_field()
                    .with(eq(sfield::NetworkID), always(), eq(4))
                    .times(1)
                    .returning(|_, _, _| INVALID_FIELD);
                // get_source_tag
                mock.expect_get_tx_field()
                    .with(eq(sfield::SourceTag), always(), eq(4))
                    .times(1)
                    .returning(|_, _, _| INVALID_FIELD);
                // get_ticket_sequence
                mock.expect_get_tx_field()
                    .with(eq(sfield::TicketSequence), always(), eq(4))
                    .times(1)
                    .returning(|_, _, _| INVALID_FIELD);

                // Mock trace_num calls (1 call per field for error codes)
                mock.expect_trace_num()
                    .with(always(), always(), always())
                    .returning(|_, _, _| 0)
                    .times(6); // 6 fields * 1 call each

                let _guard = setup_mock(mock);

                let tx = EscrowFinish;

                // Optional fields should return Err on INVALID_FIELD
                let account_txn_id_result = tx.get_account_txn_id();
                assert!(account_txn_id_result.is_err());
                assert_eq!(account_txn_id_result.err().unwrap().code(), INVALID_FIELD);

                let flags_result = tx.get_flags();
                assert!(flags_result.is_err());
                assert_eq!(flags_result.err().unwrap().code(), INVALID_FIELD);

                let last_ledger_seq_result = tx.get_last_ledger_sequence();
                assert!(last_ledger_seq_result.is_err());
                assert_eq!(last_ledger_seq_result.err().unwrap().code(), INVALID_FIELD);

                let network_id_result = tx.get_network_id();
                assert!(network_id_result.is_err());
                assert_eq!(network_id_result.err().unwrap().code(), INVALID_FIELD);

                let source_tag_result = tx.get_source_tag();
                assert!(source_tag_result.is_err());
                assert_eq!(source_tag_result.err().unwrap().code(), INVALID_FIELD);

                let ticket_seq_result = tx.get_ticket_sequence();
                assert!(ticket_seq_result.is_err());
                assert_eq!(ticket_seq_result.err().unwrap().code(), INVALID_FIELD);
            }
        }

        mod required_fields {
            use crate::core::current_tx::escrow_finish::EscrowFinish;
            use crate::core::current_tx::traits::TransactionCommonFields;
            use crate::core::current_tx::traits::tests::expect_tx_field;
            use crate::core::types::account_id::ACCOUNT_ID_SIZE;
            use crate::core::types::amount::AMOUNT_SIZE;
            use crate::core::types::blob::SIGNATURE_BLOB_SIZE;
            use crate::core::types::public_key::PUBLIC_KEY_BUFFER_SIZE;
            use crate::host::error_codes::{FIELD_NOT_FOUND, INTERNAL_ERROR, INVALID_FIELD};
            use crate::host::host_bindings_trait::MockHostBindings;
            use crate::host::setup_mock;
            use crate::sfield;
            use mockall::predicate::{always, eq};

            #[test]
            fn test_mandatory_fields_return_ok() {
                let mut mock = MockHostBindings::new();

                // get_account
                expect_tx_field(&mut mock, sfield::Account, ACCOUNT_ID_SIZE, 1);
                // get_transaction_type
                expect_tx_field(&mut mock, sfield::TransactionType, 2, 1);
                // get_computation_allowance
                expect_tx_field(&mut mock, sfield::ComputationAllowance, 4, 1);
                // get_fee
                expect_tx_field(&mut mock, sfield::Fee, AMOUNT_SIZE, 1);
                // get_sequence
                expect_tx_field(&mut mock, sfield::Sequence, 4, 1);
                // get_signing_pub_key
                expect_tx_field(&mut mock, sfield::SigningPubKey, PUBLIC_KEY_BUFFER_SIZE, 1);
                // get_txn_signature
                expect_tx_field(&mut mock, sfield::TxnSignature, SIGNATURE_BLOB_SIZE, 1);

                let _guard = setup_mock(mock);

                let tx = EscrowFinish;

                // All mandatory fields should return Ok
                assert!(tx.get_account().is_ok());
                assert!(tx.get_transaction_type().is_ok());
                assert!(tx.get_computation_allowance().is_ok());
                assert!(tx.get_fee().is_ok());
                assert!(tx.get_sequence().is_ok());
                assert!(tx.get_signing_pub_key().is_ok());
                assert!(tx.get_txn_signature().is_ok());
            }

            #[test]
            fn test_mandatory_fields_return_error_when_zero_length() {
                let mut mock = MockHostBindings::new();

                // get_account - returns 0 (zero length)
                mock.expect_get_tx_field()
                    .with(eq(sfield::Account), always(), eq(ACCOUNT_ID_SIZE))
                    .times(1)
                    .returning(|_, _, _| 0);
                // get_transaction_type - returns 0 (zero length)
                mock.expect_get_tx_field()
                    .with(eq(sfield::TransactionType), always(), eq(2))
                    .times(1)
                    .returning(|_, _, _| 0);
                // get_computation_allowance - returns 0 (zero length)
                mock.expect_get_tx_field()
                    .with(eq(sfield::ComputationAllowance), always(), eq(4))
                    .times(1)
                    .returning(|_, _, _| 0);
                // get_fee - returns 0 (zero length)
                mock.expect_get_tx_field()
                    .with(eq(sfield::Fee), always(), eq(AMOUNT_SIZE))
                    .times(1)
                    .returning(|_, _, _| 0);
                // get_sequence - returns 0 (zero length)
                mock.expect_get_tx_field()
                    .with(eq(sfield::Sequence), always(), eq(4))
                    .times(1)
                    .returning(|_, _, _| 0);
                // get_signing_pub_key - returns 0 (zero length)
                mock.expect_get_tx_field()
                    .with(
                        eq(sfield::SigningPubKey),
                        always(),
                        eq(PUBLIC_KEY_BUFFER_SIZE),
                    )
                    .times(1)
                    .returning(|_, _, _| 0);

                let _guard = setup_mock(mock);

                let tx = EscrowFinish;

                // Fixed-size mandatory fields should return Err when zero length (byte mismatch)
                let account_result = tx.get_account();
                assert!(account_result.is_err());

                let tx_type_result = tx.get_transaction_type();
                assert!(tx_type_result.is_err());

                let comp_allow_result = tx.get_computation_allowance();
                assert!(comp_allow_result.is_err());

                // Variable-size field (Amount) returns Ok with zero length
                let fee_result = tx.get_fee();
                assert!(fee_result.is_ok());

                let seq_result = tx.get_sequence();
                assert!(seq_result.is_err());

                let signing_key_result = tx.get_signing_pub_key();
                assert!(signing_key_result.is_err());
            }

            #[test]
            fn test_mandatory_fields_return_error_on_field_not_found() {
                let mut mock = MockHostBindings::new();

                // get_account
                mock.expect_get_tx_field()
                    .with(eq(sfield::Account), always(), eq(ACCOUNT_ID_SIZE))
                    .times(1)
                    .returning(|_, _, _| FIELD_NOT_FOUND);
                // get_transaction_type
                mock.expect_get_tx_field()
                    .with(eq(sfield::TransactionType), always(), eq(2))
                    .times(1)
                    .returning(|_, _, _| FIELD_NOT_FOUND);
                // get_computation_allowance
                mock.expect_get_tx_field()
                    .with(eq(sfield::ComputationAllowance), always(), eq(4))
                    .times(1)
                    .returning(|_, _, _| FIELD_NOT_FOUND);
                // get_fee
                mock.expect_get_tx_field()
                    .with(eq(sfield::Fee), always(), eq(AMOUNT_SIZE))
                    .times(1)
                    .returning(|_, _, _| FIELD_NOT_FOUND);
                // get_sequence
                mock.expect_get_tx_field()
                    .with(eq(sfield::Sequence), always(), eq(4))
                    .times(1)
                    .returning(|_, _, _| FIELD_NOT_FOUND);
                // get_signing_pub_key
                mock.expect_get_tx_field()
                    .with(
                        eq(sfield::SigningPubKey),
                        always(),
                        eq(PUBLIC_KEY_BUFFER_SIZE),
                    )
                    .times(1)
                    .returning(|_, _, _| FIELD_NOT_FOUND);

                let _guard = setup_mock(mock);

                let tx = EscrowFinish;

                // All mandatory fields should return Err on FIELD_NOT_FOUND
                let account_result = tx.get_account();
                assert!(account_result.is_err());
                assert_eq!(account_result.err().unwrap().code(), FIELD_NOT_FOUND);

                let tx_type_result = tx.get_transaction_type();
                assert!(tx_type_result.is_err());
                assert_eq!(tx_type_result.err().unwrap().code(), FIELD_NOT_FOUND);

                let comp_allow_result = tx.get_computation_allowance();
                assert!(comp_allow_result.is_err());
                assert_eq!(comp_allow_result.err().unwrap().code(), FIELD_NOT_FOUND);

                let fee_result = tx.get_fee();
                assert!(fee_result.is_err());
                assert_eq!(fee_result.err().unwrap().code(), FIELD_NOT_FOUND);

                let seq_result = tx.get_sequence();
                assert!(seq_result.is_err());
                assert_eq!(seq_result.err().unwrap().code(), FIELD_NOT_FOUND);

                let signing_key_result = tx.get_signing_pub_key();
                assert!(signing_key_result.is_err());
                assert_eq!(signing_key_result.err().unwrap().code(), FIELD_NOT_FOUND);
            }

            #[test]
            fn test_mandatory_fields_return_error_on_internal_error() {
                let mut mock = MockHostBindings::new();

                // get_account
                mock.expect_get_tx_field()
                    .with(eq(sfield::Account), always(), eq(ACCOUNT_ID_SIZE))
                    .times(1)
                    .returning(|_, _, _| INTERNAL_ERROR);
                // get_transaction_type
                mock.expect_get_tx_field()
                    .with(eq(sfield::TransactionType), always(), eq(2))
                    .times(1)
                    .returning(|_, _, _| INTERNAL_ERROR);
                // get_computation_allowance
                mock.expect_get_tx_field()
                    .with(eq(sfield::ComputationAllowance), always(), eq(4))
                    .times(1)
                    .returning(|_, _, _| INTERNAL_ERROR);
                // get_fee
                mock.expect_get_tx_field()
                    .with(eq(sfield::Fee), always(), eq(AMOUNT_SIZE))
                    .times(1)
                    .returning(|_, _, _| INTERNAL_ERROR);
                // get_sequence
                mock.expect_get_tx_field()
                    .with(eq(sfield::Sequence), always(), eq(4))
                    .times(1)
                    .returning(|_, _, _| INTERNAL_ERROR);
                // get_signing_pub_key
                mock.expect_get_tx_field()
                    .with(
                        eq(sfield::SigningPubKey),
                        always(),
                        eq(PUBLIC_KEY_BUFFER_SIZE),
                    )
                    .times(1)
                    .returning(|_, _, _| INTERNAL_ERROR);

                let _guard = setup_mock(mock);

                let tx = EscrowFinish;

                // All mandatory fields should return Err on INTERNAL_ERROR
                let account_result = tx.get_account();
                assert!(account_result.is_err());
                assert_eq!(account_result.err().unwrap().code(), INTERNAL_ERROR);

                let tx_type_result = tx.get_transaction_type();
                assert!(tx_type_result.is_err());
                assert_eq!(tx_type_result.err().unwrap().code(), INTERNAL_ERROR);

                let comp_allow_result = tx.get_computation_allowance();
                assert!(comp_allow_result.is_err());
                assert_eq!(comp_allow_result.err().unwrap().code(), INTERNAL_ERROR);

                let fee_result = tx.get_fee();
                assert!(fee_result.is_err());
                assert_eq!(fee_result.err().unwrap().code(), INTERNAL_ERROR);

                let seq_result = tx.get_sequence();
                assert!(seq_result.is_err());
                assert_eq!(seq_result.err().unwrap().code(), INTERNAL_ERROR);

                let signing_key_result = tx.get_signing_pub_key();
                assert!(signing_key_result.is_err());
                assert_eq!(signing_key_result.err().unwrap().code(), INTERNAL_ERROR);
            }

            #[test]
            fn test_mandatory_fields_return_error_on_invalid_field() {
                let mut mock = MockHostBindings::new();

                // get_account
                mock.expect_get_tx_field()
                    .with(eq(sfield::Account), always(), eq(ACCOUNT_ID_SIZE))
                    .times(1)
                    .returning(|_, _, _| INVALID_FIELD);
                // get_transaction_type
                mock.expect_get_tx_field()
                    .with(eq(sfield::TransactionType), always(), eq(2))
                    .times(1)
                    .returning(|_, _, _| INVALID_FIELD);
                // get_computation_allowance
                mock.expect_get_tx_field()
                    .with(eq(sfield::ComputationAllowance), always(), eq(4))
                    .times(1)
                    .returning(|_, _, _| INVALID_FIELD);
                // get_fee
                mock.expect_get_tx_field()
                    .with(eq(sfield::Fee), always(), eq(AMOUNT_SIZE))
                    .times(1)
                    .returning(|_, _, _| INVALID_FIELD);
                // get_sequence
                mock.expect_get_tx_field()
                    .with(eq(sfield::Sequence), always(), eq(4))
                    .times(1)
                    .returning(|_, _, _| INVALID_FIELD);
                // get_signing_pub_key
                mock.expect_get_tx_field()
                    .with(
                        eq(sfield::SigningPubKey),
                        always(),
                        eq(PUBLIC_KEY_BUFFER_SIZE),
                    )
                    .times(1)
                    .returning(|_, _, _| INVALID_FIELD);

                let _guard = setup_mock(mock);

                let tx = EscrowFinish;

                // All mandatory fields should return Err on INVALID_FIELD
                let account_result = tx.get_account();
                assert!(account_result.is_err());
                assert_eq!(account_result.err().unwrap().code(), INVALID_FIELD);

                let tx_type_result = tx.get_transaction_type();
                assert!(tx_type_result.is_err());
                assert_eq!(tx_type_result.err().unwrap().code(), INVALID_FIELD);

                let comp_allow_result = tx.get_computation_allowance();
                assert!(comp_allow_result.is_err());
                assert_eq!(comp_allow_result.err().unwrap().code(), INVALID_FIELD);

                let fee_result = tx.get_fee();
                assert!(fee_result.is_err());
                assert_eq!(fee_result.err().unwrap().code(), INVALID_FIELD);

                let seq_result = tx.get_sequence();
                assert!(seq_result.is_err());
                assert_eq!(seq_result.err().unwrap().code(), INVALID_FIELD);

                let signing_key_result = tx.get_signing_pub_key();
                assert!(signing_key_result.is_err());
                assert_eq!(signing_key_result.err().unwrap().code(), INVALID_FIELD);
            }
        }
    }
}
