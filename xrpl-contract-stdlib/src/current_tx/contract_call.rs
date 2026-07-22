//! # ContractCall
//!
//! This module provides functionality for handling ContractCall transactions within the
//! XRPL Programmability environment.

use crate::current_tx::traits::ContractCallFields;
use xrpl_wasm_stdlib::core::current_tx::traits::TransactionCommonFields;

/// Represents an ContractCall transaction in the XRPL Programmability environment.
///
/// This zero-sized type serves as a marker for ContractCall transactions and provides
/// access to transaction-specific fields through trait implementations. The structure
/// implements both common transaction fields (available to all transaction types) and
/// ContractCall-specific fields.
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
/// ## ContractCall-Specific Fields (via `ContractCallFields`)
/// - ContractAccount (the account of the contract being invoked)
/// - ContractID (the identifier of the contract being invoked)
///
/// # Zero-Cost Abstraction
///
/// This structure has no runtime overhead as it contains no data fields. All field
/// access is performed through the trait methods, which directly call the underlying
/// host functions to retrieve data from the current transaction context.
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
#[repr(C)]
pub struct ContractCall;

/// Implementation of common transaction fields for ContractCall transactions.
///
/// This implementation provides access to standard XRPL transaction fields that are
/// present in all transaction types, such as Account, Fee, Sequence, and others.
/// The methods are provided by the `TransactionCommonFields` trait.
impl TransactionCommonFields for ContractCall {}

/// Implementation of ContractCall-specific transaction fields.
///
/// This implementation provides access to fields that are specific to ContractCall
/// transactions, such as Owner, OfferSequence, Condition, and Fulfillment.
/// The methods are provided by the `ContractCallFields` trait.
impl ContractCallFields for ContractCall {}

/// Creates a new ContractCall transaction handler for the current transaction context.
///
/// This function returns an `ContractCall` instance that can be used to access fields
/// from the current XRPL transaction. The function assumes that the current transaction
/// is indeed an ContractCall transaction - using this with other transaction types
/// may result in unexpected behavior or errors when accessing type-specific fields.
///
/// # Returns
///
/// Returns an `ContractCall` struct that provides access to both common transaction
/// fields and ContractCall-specific fields through its trait implementations.
///
/// # Safety
///
/// This function is safe to call, but the returned object should only be used when
/// the current transaction context is guaranteed to be an ContractCall transaction.
/// The XRPL Programmability environment ensures this context is correct when the
/// smart contract is invoked in response to an ContractCall transaction.
///
/// # Performance
///
/// This function has zero runtime cost as it simply returns a zero-sized type.
/// All actual field access happens lazily when trait methods are called.
#[inline]
pub fn get_current_contract_call() -> ContractCall {
    ContractCall
}
