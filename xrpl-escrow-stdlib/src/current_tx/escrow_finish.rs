//! # EscrowFinish
//!
//! This module provides functionality for handling EscrowFinish transactions within the
//! XRPL Programmability environment.

use xrpl_wasm_stdlib::core::current_tx::traits::TransactionCommonFields;

use crate::current_tx::traits::EscrowFinishFields;

/// Represents an EscrowFinish transaction in the XRPL Programmability environment.
///
/// This zero-sized type serves as a marker for EscrowFinish transactions and provides
/// access to transaction-specific fields through trait implementations. The structure
/// implements both common transaction fields (available to all transaction types) and
/// escrow-finish-specific fields.
///
/// # Field Access
///
/// Through the implemented traits, this structure provides access to:
///
/// ## Common Transaction Fields (via `TransactionCommonFields`)
/// - Account (transaction sender)
/// - Fee (transaction cost in drops)
/// - Sequence (account sequence number)
/// - LastLedgerSequence (transaction expiration)
/// - And other standard XRPL transaction fields
///
/// ## EscrowFinish-Specific Fields (via `EscrowFinishFields`)
/// - Owner (account that created the escrow)
/// - OfferSequence (sequence number of the EscrowCreate transaction)
/// - Condition (cryptographic condition, if present)
/// - Fulfillment (cryptographic fulfillment, if present)
///
/// # Zero-Cost Abstraction
///
/// This structure has no runtime overhead as it contains no data fields. All field
/// access is performed through the trait methods, which directly call the underlying
/// host functions to retrieve data from the current transaction context.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct EscrowFinish;

/// Implementation of common transaction fields for EscrowFinish transactions.
///
/// This implementation provides access to standard XRPL transaction fields that are
/// present in all transaction types, such as Account, Fee, Sequence, and others.
/// The methods are provided by the `TransactionCommonFields` trait.
impl TransactionCommonFields for EscrowFinish {}

/// Implementation of EscrowFinish-specific transaction fields.
///
/// This implementation provides access to fields that are specific to EscrowFinish
/// transactions, such as Owner, OfferSequence, Condition, and Fulfillment.
/// The methods are provided by the `EscrowFinishFields` trait.
impl EscrowFinishFields for EscrowFinish {}

/// Creates a new EscrowFinish transaction handler for the current transaction context.
///
/// This function returns an `EscrowFinish` instance that can be used to access fields
/// from the current XRPL transaction. The function assumes that the current transaction
/// is indeed an EscrowFinish transaction - using this with other transaction types
/// may result in unexpected behavior or errors when accessing type-specific fields.
///
/// # Returns
///
/// Returns an `EscrowFinish` struct that provides access to both common transaction
/// fields and EscrowFinish-specific fields through its trait implementations.
///
/// # Safety
///
/// This function is safe to call, but the returned object should only be used when
/// the current transaction context is guaranteed to be an EscrowFinish transaction.
/// The XRPL Programmability environment ensures this context is correct when the
/// smart contract is invoked in response to an EscrowFinish transaction.
///
/// # Performance
///
/// This function has zero runtime cost as it simply returns a zero-sized type.
/// All actual field access happens lazily when trait methods are called.
///
/// # Example
///
/// ```no_run
/// use xrpl_escrow_stdlib::current_tx::escrow_finish::EscrowFinish;
/// use xrpl_escrow_stdlib::current_tx::traits::EscrowFinishFields;
/// use xrpl_wasm_stdlib::core::current_tx::traits::TransactionCommonFields;
/// let tx = EscrowFinish;
/// let owner = tx.get_owner().unwrap_or_panic();
/// let offer_seq = tx.get_offer_sequence().unwrap_or_panic();
/// let condition = tx.get_condition().unwrap_or_panic(); // Option<_>
/// ```
#[inline]
pub fn get_current_escrow_finish() -> EscrowFinish {
    EscrowFinish
}
