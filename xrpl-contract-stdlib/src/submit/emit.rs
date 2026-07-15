//! Generic transaction-emission mechanism for Smart Contracts.
//!
//! [`EmittableTx`] generalizes the build/add-field/emit sequence that
//! [`AmountSubmit`](super::amount::AmountSubmit) hand-rolls per transaction
//! type: implementors describe their transaction type and how to write their
//! own fields, and [`ContractCallContext::emit`](crate::ctx::ContractCallContext::emit)
//! drives the `build_txn` / field-writing / `emit_built_txn` sequence once.

use crate::core::types::transaction_type::TransactionType;

/// An error from building or emitting a transaction via [`EmittableTx`].
///
/// Each variant carries the raw host result code for the step that failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmitError {
    /// `build_txn` returned a negative result code.
    BuildFailed(i32),
    /// `add_txn_field` returned a negative result code while writing a field.
    FieldFailed(i32),
    /// `emit_built_txn` returned a negative result code.
    EmitFailed(i32),
}

/// A value that can be built into an XRPL transaction and emitted from a
/// Smart Contract via [`ContractCallContext::emit`](crate::ctx::ContractCallContext::emit).
pub trait EmittableTx {
    /// The XRPL transaction type to pass to `build_txn`.
    fn transaction_type(&self) -> TransactionType;

    /// Writes this value's fields onto the in-progress transaction at
    /// `txn_index` (as returned by `build_txn`), via `add_txn_field`.
    fn write_fields(&self, txn_index: i32) -> Result<(), EmitError>;
}
