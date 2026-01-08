//! Account identifiers used throughout XRPL.
//!
//! This type wraps a 20-byte AccountID and is returned by many accessors.
//! See also: <https://xrpl.org/docs/references/protocol/common-fields#accountid-fields>

use crate::core::current_tx::CurrentTxFieldGetter;
use crate::core::ledger_objects::LedgerObjectFieldGetter;
use crate::host::field_helpers::{
    get_fixed_size_field_with_expected_bytes, get_fixed_size_field_with_expected_bytes_optional,
};
use crate::host::{Result, get_current_ledger_obj_field, get_ledger_obj_field, get_tx_field};

pub const ACCOUNT_ID_SIZE: usize = 20;

/// A 20-byte account identifier on the XRP Ledger.
///
/// AccountIDs are derived from a public key and uniquely identify accounts on the ledger.
/// They are used throughout XRPL for specifying senders, receivers, issuers, and other
/// account-related fields.
///
/// ## Derived Traits
///
/// - `Copy`: Efficient for this 20-byte struct, enabling implicit copying
/// - `PartialEq, Eq`: Enable comparisons and use in hash-based collections
/// - `Debug, Clone`: Standard traits for development and consistency
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct AccountID(pub [u8; ACCOUNT_ID_SIZE]);

impl From<[u8; ACCOUNT_ID_SIZE]> for AccountID {
    fn from(value: [u8; ACCOUNT_ID_SIZE]) -> Self {
        AccountID(value)
    }
}

/// Implementation of `LedgerObjectFieldGetter` for XRPL account identifiers.
///
/// This implementation handles 20-byte account ID fields in XRPL ledger objects.
/// Account IDs uniquely identify accounts on the XRPL network and are derived
/// from public keys using cryptographic hashing.
///
/// # Buffer Management
///
/// Uses a 20-byte buffer (ACCOUNT_ID_SIZE) and validates that exactly 20 bytes
/// are returned from the host function. The buffer is converted to an AccountID
/// using the `From<[u8; 20]>` implementation.
impl LedgerObjectFieldGetter for AccountID {
    #[inline]
    fn get_from_current_ledger_obj(field_code: i32) -> Result<Self> {
        match get_fixed_size_field_with_expected_bytes::<ACCOUNT_ID_SIZE, _>(
            field_code,
            |fc, buf, size| unsafe { get_current_ledger_obj_field(fc, buf, size) },
        ) {
            Result::Ok(buffer) => Result::Ok(buffer.into()),
            Result::Err(e) => Result::Err(e),
        }
    }

    #[inline]
    fn get_from_current_ledger_obj_optional(field_code: i32) -> Result<Option<Self>> {
        match get_fixed_size_field_with_expected_bytes_optional::<ACCOUNT_ID_SIZE, _>(
            field_code,
            |fc, buf, size| unsafe { get_current_ledger_obj_field(fc, buf, size) },
        ) {
            Result::Ok(buffer) => Result::Ok(buffer.map(|b| b.into())),
            Result::Err(e) => Result::Err(e),
        }
    }

    #[inline]
    fn get_from_ledger_obj(register_num: i32, field_code: i32) -> Result<Self> {
        match get_fixed_size_field_with_expected_bytes::<ACCOUNT_ID_SIZE, _>(
            field_code,
            |fc, buf, size| unsafe { get_ledger_obj_field(register_num, fc, buf, size) },
        ) {
            Result::Ok(buffer) => Result::Ok(buffer.into()),
            Result::Err(e) => Result::Err(e),
        }
    }

    #[inline]
    fn get_from_ledger_obj_optional(register_num: i32, field_code: i32) -> Result<Option<Self>> {
        match get_fixed_size_field_with_expected_bytes_optional::<ACCOUNT_ID_SIZE, _>(
            field_code,
            |fc, buf, size| unsafe { get_ledger_obj_field(register_num, fc, buf, size) },
        ) {
            Result::Ok(buffer) => Result::Ok(buffer.map(|b| b.into())),
            Result::Err(e) => Result::Err(e),
        }
    }
}

/// Implementation of `CurrentTxFieldGetter` for XRPL account identifiers.
///
/// This implementation handles 20-byte account ID fields in XRPL transactions.
/// Account IDs identify transaction participants such as the sending account,
/// destination account, and various other account references throughout the transaction.
///
/// # Buffer Management
///
/// Uses a 20-byte buffer (ACCOUNT_ID_SIZE) and validates that exactly 20 bytes
/// are returned from the host function. The buffer is converted to an AccountID
/// using the `From<[u8; 20]>` implementation.
impl CurrentTxFieldGetter for AccountID {
    #[inline]
    fn get_from_current_tx(field_code: i32) -> Result<Self> {
        match get_fixed_size_field_with_expected_bytes::<ACCOUNT_ID_SIZE, _>(
            field_code,
            |fc, buf, size| unsafe { get_tx_field(fc, buf, size) },
        ) {
            Result::Ok(buffer) => Result::Ok(buffer.into()),
            Result::Err(e) => Result::Err(e),
        }
    }

    #[inline]
    fn get_from_current_tx_optional(field_code: i32) -> Result<Option<Self>> {
        match get_fixed_size_field_with_expected_bytes_optional::<ACCOUNT_ID_SIZE, _>(
            field_code,
            |fc, buf, size| unsafe { get_tx_field(fc, buf, size) },
        ) {
            Result::Ok(buffer) => Result::Ok(buffer.map(|b| b.into())),
            Result::Err(e) => Result::Err(e),
        }
    }
}
