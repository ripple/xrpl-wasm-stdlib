//! Smart Feature context primitives shared by all entry points.
//!
//! This module exposes the [`SmartFeatureContext`] trait — the narrow context
//! type implemented by feature-specific contexts (e.g., `EscrowFinishContext`)

use crate::core::current_tx::traits::TransactionCommonFields;

/// Narrow context trait shared by all Smart Escrow and Smart Contract entry points.
///
/// Provides access to the current transaction via an associated type bound to
/// [`TransactionCommonFields`]. Kept intentionally minimal.
///
/// Concrete context types (`EscrowFinishContext`, `ContractCallContext`) implement
/// this trait and expose their feature-unique host functions as inherent methods.
pub trait SmartFeatureContext {
    type Tx: TransactionCommonFields;
    fn tx(&self) -> &Self::Tx;
}
