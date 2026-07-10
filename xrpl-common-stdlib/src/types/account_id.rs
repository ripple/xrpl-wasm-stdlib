//! Account identifiers used throughout XRPL.
//!
//! This type wraps a 20-byte AccountID and is returned by many accessors.
//! See also: <https://xrpl.org/docs/references/protocol/common-fields#accountid-fields>

use crate::fields::decoder::{FieldDecoder, FromCurrentTx, FromLedger};
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

/// `FieldDecoder` for XRPL account identifiers: decodes a 20-byte buffer into an `AccountID`,
/// failing if the host wrote a different number of bytes.
impl FieldDecoder for AccountID {
    type Buffer = [u8; ACCOUNT_ID_SIZE];

    #[inline]
    fn empty_buffer() -> Self::Buffer {
        [0u8; ACCOUNT_ID_SIZE]
    }

    #[inline]
    fn decode(bytes: &[u8]) -> core::result::Result<Self, DecodeError> {
        let array: [u8; ACCOUNT_ID_SIZE] = bytes.try_into().map_err(|_| DecodeError)?;
        Ok(array.into())
    }
}

impl FromCurrentTx for AccountID {}
impl FromLedger for AccountID {}
