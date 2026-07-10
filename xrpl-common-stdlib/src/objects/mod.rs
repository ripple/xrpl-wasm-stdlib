//! Ledger-object domain types and their field-accessor traits.
//!
//! Reading fields off a ledger object goes through [`crate::fields::ledger_obj`] (by slot) and
//! [`crate::fields::current_ledger_obj`] (the current object). The traits here
//! ([`traits::LedgerObjectCommonFields`], [`traits::AccountFields`], ...) build ergonomic,
//! per-object-type accessors on top of those primitives.

pub mod account_root;
pub mod array;
pub mod object;
pub mod traits;
