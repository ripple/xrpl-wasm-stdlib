//! Placeholder types for array and object SFields.
//!
//! These types are used as placeholders in SField definitions for array and object types
//! that cannot be directly retrieved from ledger objects. They are primarily used within
//! `Location` for navigating nested structures.

use crate::host::Result;

/// Placeholder type for array SFields.
///
/// Array types in XRPL (like Signers, Memos, etc.) cannot be directly retrieved
/// as complete values. Instead, they are used within `Location` to navigate to
/// specific array elements.
///
/// This type implements `FieldGetter` as a no-op to satisfy the trait bound,
/// but should not be used to actually retrieve values.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Array;

/// Placeholder type for object SFields.
///
/// Object types in XRPL (like Memo, SignerEntry, etc.) cannot be directly retrieved
/// as complete values. Instead, they are used within `Location` to navigate to
/// specific object fields.
///
/// This type implements `FieldGetter` as a no-op to satisfy the trait bound,
/// but should not be used to actually retrieve values.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Object;

// Implement FieldGetter for Array and Object as no-ops
// These are placeholder types and should not be used for actual field retrieval
use crate::core::ledger_objects::LedgerObjectFieldGetter;

impl LedgerObjectFieldGetter for Array {
    #[inline]
    fn get_from_current_ledger_obj(_field_code: i32) -> Result<Self> {
        // This should never be called - Array is a placeholder type
        unreachable!("Array is a placeholder type and cannot be retrieved from ledger objects")
    }

    #[inline]
    fn get_from_ledger_obj(_slot: i32, _field_code: i32) -> Result<Self> {
        // This should never be called - Array is a placeholder type
        unreachable!("Array is a placeholder type and cannot be retrieved from ledger objects")
    }

    #[inline]
    fn get_from_current_ledger_obj_optional(_field_code: i32) -> Result<Option<Self>> {
        // This should never be called - Array is a placeholder type
        unreachable!("Array is a placeholder type and cannot be retrieved from ledger objects")
    }

    #[inline]
    fn get_from_ledger_obj_optional(_slot: i32, _field_code: i32) -> Result<Option<Self>> {
        // This should never be called - Array is a placeholder type
        unreachable!("Array is a placeholder type and cannot be retrieved from ledger objects")
    }
}

impl LedgerObjectFieldGetter for Object {
    #[inline]
    fn get_from_current_ledger_obj(_field_code: i32) -> Result<Self> {
        // This should never be called - Object is a placeholder type
        unreachable!("Object is a placeholder type and cannot be retrieved from ledger objects")
    }

    #[inline]
    fn get_from_ledger_obj(_slot: i32, _field_code: i32) -> Result<Self> {
        // This should never be called - Object is a placeholder type
        unreachable!("Object is a placeholder type and cannot be retrieved from ledger objects")
    }

    #[inline]
    fn get_from_current_ledger_obj_optional(_field_code: i32) -> Result<Option<Self>> {
        // This should never be called - Object is a placeholder type
        unreachable!("Object is a placeholder type and cannot be retrieved from ledger objects")
    }

    #[inline]
    fn get_from_ledger_obj_optional(_slot: i32, _field_code: i32) -> Result<Option<Self>> {
        // This should never be called - Object is a placeholder type
        unreachable!("Object is a placeholder type and cannot be retrieved from ledger objects")
    }
}
