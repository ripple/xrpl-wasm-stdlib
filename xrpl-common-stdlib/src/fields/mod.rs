//! Typed accessors for reading fields from the current transaction and ledger objects, plus the
//! [`locator::Locator`] builder for nested field paths.
//!
//! - [`decoder`]: Context-independent `FieldDecoder` trait + per-context marker traits
//! - [`current_tx`]: Read fields from the current transaction
//! - [`ledger_obj`]: Read fields from a ledger object by slot
//! - [`current_ledger_obj`]: Read fields from the current ledger object (no slot)
//! - [`locator`]: Build locators for nested field access

pub mod current_ledger_obj;
pub mod current_tx;
pub mod decoder;
pub mod ledger_obj;
pub mod locator;
