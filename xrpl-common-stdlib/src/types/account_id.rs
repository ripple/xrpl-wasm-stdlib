//! Account identifiers used throughout XRPL.
//!
//! This type wraps a 20-byte AccountID and is returned by many accessors.
//! See also: <https://xrpl.org/docs/references/protocol/common-fields#accountid-fields>

use crate::fields::decoder::{FieldDecoder, FromCurrentTx, FromLedger, decode_exact};
use crate::host::field_helpers::{
    get_fixed_size_field_with_expected_bytes, get_fixed_size_field_with_expected_bytes_optional,
};
use crate::host::{Result, get_current_ledger_obj_field, get_ledger_obj_field};
use crate::objects::LedgerObjectFieldGetter;
use crate::sfield::SField;
use crate::types::decode_error::DecodeError;

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

/// The account being authorized in a keylet that pairs an owner account with an authorized
/// account (e.g. [`delegate_keylet`](crate::keylets::delegate_keylet) and
/// [`deposit_preauth_keylet`](crate::keylets::deposit_preauth_keylet)).
///
/// Both parameters of those functions are accounts, so wrapping the authorized one in a
/// distinct type stops a caller from silently swapping owner and authorized account — the
/// swap becomes a compile error instead of a wrong-but-valid keylet. Mirrors the
/// `sfield::Authorize` field.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Authorize(pub AccountID);

impl From<AccountID> for Authorize {
    fn from(value: AccountID) -> Self {
        Authorize(value)
    }
}

/// The issuer of a credential in [`credential_keylet`](crate::keylets::credential_keylet).
///
/// The subject and issuer are both accounts; wrapping the issuer in a distinct type makes an
/// accidental subject/issuer swap a compile error. Mirrors the `sfield::Issuer` field.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Issuer(pub AccountID);

impl From<AccountID> for Issuer {
    fn from(value: AccountID) -> Self {
        Issuer(value)
    }
}

/// The destination account of a payment channel in
/// [`paychan_keylet`](crate::keylets::paychan_keylet).
///
/// The source and destination are both accounts; wrapping the destination in a distinct type
/// makes an accidental source/destination swap a compile error. Mirrors the
/// `sfield::Destination` field.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Destination(pub AccountID);

impl From<AccountID> for Destination {
    fn from(value: AccountID) -> Self {
        Destination(value)
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
    fn get_from_current_ledger_obj<const CODE: i32>(field: SField<Self, CODE>) -> Result<Self> {
        get_fixed_size_field_with_expected_bytes::<ACCOUNT_ID_SIZE, _>(
            i32::from(field),
            |fc, buf, size| unsafe { get_current_ledger_obj_field(fc, buf, size) },
        )
        .map(|buffer| buffer.into())
    }

    #[inline]
    fn get_from_current_ledger_obj_optional<const CODE: i32>(
        field: SField<Self, CODE>,
    ) -> Result<Option<Self>> {
        get_fixed_size_field_with_expected_bytes_optional::<ACCOUNT_ID_SIZE, _>(
            i32::from(field),
            |fc, buf, size| unsafe { get_current_ledger_obj_field(fc, buf, size) },
        )
        .map(|buffer| buffer.map(|b| b.into()))
    }

    #[inline]
    fn get_from_ledger_obj<const CODE: i32>(
        register_num: i32,
        field: SField<Self, CODE>,
    ) -> Result<Self> {
        get_fixed_size_field_with_expected_bytes::<ACCOUNT_ID_SIZE, _>(
            i32::from(field),
            |fc, buf, size| unsafe { get_ledger_obj_field(register_num, fc, buf, size) },
        )
        .map(|buffer| buffer.into())
    }

    #[inline]
    fn get_from_ledger_obj_optional<const CODE: i32>(
        register_num: i32,
        field: SField<Self, CODE>,
    ) -> Result<Option<Self>> {
        get_fixed_size_field_with_expected_bytes_optional::<ACCOUNT_ID_SIZE, _>(
            i32::from(field),
            |fc, buf, size| unsafe { get_ledger_obj_field(register_num, fc, buf, size) },
        )
        .map(|buffer| buffer.map(|b| b.into()))
    }
}

/// `FieldDecoder` for XRPL account identifiers: decodes a 20-byte buffer into an `AccountID`,
/// failing if the host wrote a different number of bytes.
impl FieldDecoder for AccountID {
    type Buffer = [u8; ACCOUNT_ID_SIZE];

    #[inline]
    fn empty_buffer() -> Self::Buffer {
        [0u8; ACCOUNT_ID_SIZE]
    }

    #[inline]
    fn decode(buf: Self::Buffer, bytes_written: usize) -> core::result::Result<Self, DecodeError> {
        decode_exact(buf, bytes_written)
    }
}

impl FromCurrentTx for AccountID {}
impl FromLedger for AccountID {}
