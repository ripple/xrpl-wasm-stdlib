//! Typed accessors for reading fields from the current transaction, plus the
//! [`locator::Locator`] builder for nested field paths.
//!
//! - [`current_tx`]: Read fields from the current transaction
//! - [`locator`]: Build locators for nested field access

pub mod current_tx;
pub mod locator;
