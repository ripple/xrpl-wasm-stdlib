//! Placeholder types for array and object SFields.
//!
//! These types are used as placeholders in SField definitions for array and object types
//! that cannot be directly retrieved from ledger objects. They are primarily used within
//! `Locator` for navigating nested structures.

/// Placeholder type for array SFields.
///
/// Array types in XRPL (like Signers, Memos, etc.) cannot be directly retrieved as complete
/// values. Instead, they are navigated with `Locator` to reach specific array elements.
///
/// This type intentionally implements neither [`crate::fields::decoder::FromCurrentTx`] nor
/// [`crate::fields::decoder::FromLedger`], so passing an aggregate SField (e.g. `sfield::Memos`)
/// to a `get_field` accessor is a *compile-time* error rather than a runtime failure.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Array;

/// Placeholder type for object SFields.
///
/// Object types in XRPL (like Memo, SignerEntry, etc.) cannot be directly retrieved as complete
/// values. Instead, they are navigated with `Locator` to reach specific object fields.
///
/// This type intentionally implements neither [`crate::fields::decoder::FromCurrentTx`] nor
/// [`crate::fields::decoder::FromLedger`], so passing an aggregate SField (e.g. `sfield::Memo`)
/// to a `get_field` accessor is a *compile-time* error rather than a runtime failure.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Object;
