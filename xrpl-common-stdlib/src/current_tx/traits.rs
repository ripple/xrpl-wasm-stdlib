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

<<<<<<<< HEAD:xrpl-common-stdlib/src/current_tx/traits.rs
use crate::current_tx::{get_field, get_field_optional};
========
use crate::fields::current_tx::{get_field, get_field_optional};
>>>>>>>> c621dc8 (common reorg):xrpl-common-stdlib/src/fields/current_tx/traits.rs
use crate::host::Result;
use crate::sfield;
use crate::types::account_id::AccountID;
use crate::types::amount::Amount;
use crate::types::blob::SignatureBlob;
use crate::types::public_key::PublicKey;
use crate::types::transaction_type::TransactionType;
use crate::types::uint::Hash256;

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
    /// Returns a `Result<Option<PublicKey>>` where:
    /// * `Ok(Some(PublicKey))` - The 33-byte compressed public key for single-signature transactions
    /// * `Ok(None)` - Empty SigningPubKey field, indicating a multi-signature transaction
    /// * `Err(Error)` - If the field cannot be retrieved
    ///
    /// # Panics
    ///
    /// Panics if the field is present with a length other than 0 or 33 bytes. rippled's
    /// preflight rejects such transactions before they are applied, so this is an internal
    /// invariant violation rather than recoverable input.
    ///
    /// # Security Note
    ///
    /// The presence of this field doesn't guarantee the signature is valid. Instead, this field
    /// only provides the key claimed to be used for signing. The XRPL network performs signature
    /// validation before transaction execution.
    fn get_signing_pub_key(&self) -> Result<Option<PublicKey>> {
        get_field(sfield::SigningPubKey).and_then(|blob| match blob.len {
            0 => Result::Ok(None), // Multi-signature transaction
            33 => Result::Ok(Some(PublicKey::from(blob.data))), // Single-signature transaction
            // Unreachable in practice (see `# Panics`); fail fast if the invariant breaks.
            len => panic!("internal invariant violated: SigningPubKey has unexpected length {len} (expected 0 or 33)"),
        })
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

#[cfg(test)]
mod tests {
<<<<<<<< HEAD:xrpl-common-stdlib/src/current_tx/traits.rs
    use crate::current_tx::traits::TransactionCommonFields;
========
    use crate::fields::current_tx::traits::TransactionCommonFields;
>>>>>>>> c621dc8 (common reorg):xrpl-common-stdlib/src/fields/current_tx/traits.rs
    use crate::host::host_bindings_trait::MockHostBindings;
    use crate::sfield::SField;
    use mockall::predicate::{always, eq};

    /// Minimal concrete type implementing [`TransactionCommonFields`], used to exercise the
    /// trait's default methods without depending on any transaction-specific wrapper. The
    /// concrete wrappers (e.g. `EscrowFinish`) now live in the `xrpl-escrow-stdlib` crate, so
    /// common's own tests use a local stand-in instead.
    struct TestTransaction;
    impl TransactionCommonFields for TestTransaction {}

    /// Helper to set up a mock expectation for `get_tx_field`.
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

    mod transaction_common_fields {

        mod optional_fields {
<<<<<<<< HEAD:xrpl-common-stdlib/src/current_tx/traits.rs
            use crate::current_tx::traits::TransactionCommonFields;
            use crate::current_tx::traits::tests::TestTransaction;
            use crate::current_tx::traits::tests::expect_tx_field;
========
            use crate::fields::current_tx::traits::TransactionCommonFields;
            use crate::fields::current_tx::traits::tests::TestTransaction;
            use crate::fields::current_tx::traits::tests::expect_tx_field;
>>>>>>>> c621dc8 (common reorg):xrpl-common-stdlib/src/fields/current_tx/traits.rs
            use crate::host::error_codes::{FIELD_NOT_FOUND, INTERNAL_ERROR, INVALID_FIELD};
            use crate::host::host_bindings_trait::MockHostBindings;
            use crate::host::setup_mock;
            use crate::sfield;
            use crate::types::uint::HASH256_SIZE;
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

                let tx = TestTransaction;

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

                let tx = TestTransaction;

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

                let _guard = setup_mock(mock);

                let tx = TestTransaction;

                // Fixed-size optional fields return Err (InvalidDecoding) on a zero-length read:
                // decode's length check rejects the byte-count mismatch.
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

                let _guard = setup_mock(mock);

                let tx = TestTransaction;

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

                let _guard = setup_mock(mock);

                let tx = TestTransaction;

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
<<<<<<<< HEAD:xrpl-common-stdlib/src/current_tx/traits.rs
            use crate::current_tx::traits::TransactionCommonFields;
            use crate::current_tx::traits::tests::TestTransaction;
            use crate::current_tx::traits::tests::expect_tx_field;
========
            use crate::fields::current_tx::traits::TransactionCommonFields;
            use crate::fields::current_tx::traits::tests::TestTransaction;
            use crate::fields::current_tx::traits::tests::expect_tx_field;
>>>>>>>> c621dc8 (common reorg):xrpl-common-stdlib/src/fields/current_tx/traits.rs
            use crate::host::error_codes::{FIELD_NOT_FOUND, INTERNAL_ERROR, INVALID_FIELD};
            use crate::host::host_bindings_trait::MockHostBindings;
            use crate::host::setup_mock;
            use crate::sfield;
            use crate::types::account_id::ACCOUNT_ID_SIZE;
            use crate::types::amount::AMOUNT_SIZE;
            use crate::types::blob::SIGNATURE_BLOB_SIZE;
            use crate::types::public_key::PUBLIC_KEY_BUFFER_SIZE;
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

                let tx = TestTransaction;

                // All mandatory fields should return Ok
                assert!(tx.get_account().is_ok());
                assert!(tx.get_transaction_type().is_ok());
                assert!(tx.get_computation_allowance().is_ok());
                assert!(tx.get_fee().is_ok());
                assert!(tx.get_sequence().is_ok());
                assert!(tx.get_signing_pub_key().is_ok());
                assert!(tx.get_txn_signature().is_ok());
            }

            // A zero-length read of a mandatory fixed-size field fails `FieldDecoder::decode`'s
            // length check and surfaces as `Err(InvalidDecoding)`.

            #[test]
            fn test_get_account_errors_when_zero_length() {
                let mut mock = MockHostBindings::new();
                mock.expect_get_tx_field()
                    .with(eq(sfield::Account), always(), eq(ACCOUNT_ID_SIZE))
                    .returning(|_, _, _| 0);

                let _guard = setup_mock(mock);
                let result = TestTransaction.get_account();
                assert!(result.is_err());
                assert_eq!(
                    result.err().unwrap().code(),
                    crate::host::Error::InvalidDecoding.code()
                );
            }

            #[test]
            fn test_get_transaction_type_errors_when_zero_length() {
                let mut mock = MockHostBindings::new();
                mock.expect_get_tx_field()
                    .with(eq(sfield::TransactionType), always(), eq(2))
                    .returning(|_, _, _| 0);

                let _guard = setup_mock(mock);
                let result = TestTransaction.get_transaction_type();
                assert!(result.is_err());
                assert_eq!(
                    result.err().unwrap().code(),
                    crate::host::Error::InvalidDecoding.code()
                );
            }

            #[test]
            fn test_get_computation_allowance_errors_when_zero_length() {
                let mut mock = MockHostBindings::new();
                mock.expect_get_tx_field()
                    .with(eq(sfield::ComputationAllowance), always(), eq(4))
                    .returning(|_, _, _| 0);

                let _guard = setup_mock(mock);
                let result = TestTransaction.get_computation_allowance();
                assert!(result.is_err());
                assert_eq!(
                    result.err().unwrap().code(),
                    crate::host::Error::InvalidDecoding.code()
                );
            }

            #[test]
            fn test_get_sequence_errors_when_zero_length() {
                let mut mock = MockHostBindings::new();
                mock.expect_get_tx_field()
                    .with(eq(sfield::Sequence), always(), eq(4))
                    .returning(|_, _, _| 0);

                let _guard = setup_mock(mock);
                let result = TestTransaction.get_sequence();
                assert!(result.is_err());
                assert_eq!(
                    result.err().unwrap().code(),
                    crate::host::Error::InvalidDecoding.code()
                );
            }

            #[test]
            fn test_variable_size_fields_ok_when_zero_length() {
                let mut mock = MockHostBindings::new();

                // get_fee - returns 0 (zero length)
                mock.expect_get_tx_field()
                    .with(eq(sfield::Fee), always(), eq(AMOUNT_SIZE))
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

                let tx = TestTransaction;

                // Variable-size field (Amount) returns Ok with zero length
                let fee_result = tx.get_fee();
                assert!(fee_result.is_ok());

                // SigningPubKey is special: zero length indicates multi-signature transaction
                // and should return Ok(None), not an error
                let signing_key_result = tx.get_signing_pub_key();
                assert!(signing_key_result.is_ok());
                assert!(signing_key_result.unwrap().is_none());
            }

            #[test]
            #[should_panic]
            fn test_get_signing_pub_key_panics_on_unexpected_length() {
                let mut mock = MockHostBindings::new();

                // A SigningPubKey that is neither empty (0, multisign) nor a valid key (33)
                // can never reach a running escrow: rippled's preflight rejects it. Observing
                // such a length is an internal invariant violation and must panic.
                mock.expect_get_tx_field()
                    .with(
                        eq(sfield::SigningPubKey),
                        always(),
                        eq(PUBLIC_KEY_BUFFER_SIZE),
                    )
                    .returning(|_, _, _| 16);

                let _guard = setup_mock(mock);

                let _ = TestTransaction.get_signing_pub_key();
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

                let tx = TestTransaction;

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

                let tx = TestTransaction;

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

                let tx = TestTransaction;

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
