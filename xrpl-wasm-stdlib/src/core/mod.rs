//! Core modules for XRPL transaction and ledger access.
//!
//! This namespace provides typed accessors and utilities used by smart contracts:
//! - [`constants`]: Internal helpers for buffer sizes
//! - [`current_tx`]: Read fields from the current transaction
//! - [`keylets`]: Compute keylets for various ledger entry types
//! - [`ledger_objects`]: Read fields from ledger entries (current or cached)
//! - [`types`]: Strongly-typed XRPL primitives (AccountID, Hash256, Amount, etc.)
//! - [`locator`]: Build locators for nested field access
//!
//! Start with [`current_tx::escrow_finish::EscrowFinish`] to access EscrowFinish TX fields,
//! or [`ledger_objects::current_escrow::get_current_escrow`] to access the active escrow.

pub mod constants;
pub mod current_tx;
pub mod keylets;
pub mod ledger_objects;
pub mod locator;
pub mod types;
