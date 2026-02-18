//! Core modules for XRPL transaction and ledger access.
//!
//! This namespace provides typed accessors and utilities used by smart contracts:
//! - [`current_tx`]: Read fields from the current transaction
//! - [`ledger_objects`]: Read fields from on-ledger objects (current or cached)
//! - [`types`]: Strongly-typed XRPL primitives (AccountID, Hash256, Amount, etc.)
//! - [`locator`]: Build locators for nested field access
//! - [`constants`]: Internal helpers for buffer sizes
//!
//! Start with [`current_tx::escrow_finish::EscrowFinish`] to access EscrowFinish TX fields,
//! or [`ledger_objects::current_escrow::get_current_escrow`] to access the active escrow.

pub mod constants;
pub mod current_tx;
pub mod keylets;
pub mod ledger_objects;
pub mod locator;
pub mod types;
