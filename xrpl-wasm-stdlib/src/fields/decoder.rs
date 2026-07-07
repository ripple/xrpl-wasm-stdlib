//! Field decoding traits for XRPL transaction and ledger object fields.
//!
//! This module defines a lightweight, host-independent decoding contract for types that
//! are constructed directly from a raw byte buffer.
//!
//! # Examples
//!
//! ```
//! use xrpl_wasm_stdlib::fields::decoder::{FieldDecoder, FromCurrentTx};
//! use xrpl_wasm_stdlib::host::Error;
//!
//! /// A toy field: a single boolean flag byte (0 or 1).
//! struct Flag(bool);
//!
//! impl FieldDecoder for Flag {
//!     type Buffer = [u8; 1];
//!
//!     fn decode(bytes: &[u8]) -> Result<Self, Error> {
//!         match bytes {
//!             [0] => Ok(Flag(false)),
//!             [1] => Ok(Flag(true)),
//!             _ => Err(Error::InvalidDecoding),
//!         }
//!     }
//! }
//!
//! // Declares that `Flag` can be decoded from the currently executing transaction.
//! // `FromLedger` is implemented the same way for fields readable from ledger objects.
//! impl FromCurrentTx for Flag {}
//!
//! let flag = Flag::decode(&[1]).unwrap();
//! assert!(flag.0);
//!
//! assert!(Flag::decode(&[2]).is_err());
//! ```

use crate::host::Error;

/// Decodes a fixed-format XRPL field from its raw byte representation.
pub trait FieldDecoder: Sized {
    /// A stack buffer sized to hold this field's raw bytes, used by generic host-field
    /// getters to read into before calling [`decode`](FieldDecoder::decode).
    type Buffer: AsMut<[u8]> + Default;

    /// Decodes `Self` from `bytes`, returning an error if the bytes are malformed.
    fn decode(bytes: &[u8]) -> Result<Self, Error>;
}

/// Marker trait for fields that can be decoded from the currently executing transaction.
pub trait FromCurrentTx: FieldDecoder {}

/// Marker trait for fields that can be decoded from a ledger object.
pub trait FromLedger: FieldDecoder {}

/// Implements [`FromCurrentTx`] and/or [`FromLedger`] for a type, based on which field
/// sources it supports.
///
/// # Examples
///
/// ```ignore
/// field_source!(MyType : tx);        // decodable from the current transaction only
/// field_source!(MyType : obj);       // decodable from a ledger object only
/// field_source!(MyType : tx, obj);   // decodable from either source
/// ```
#[allow(unused_macros)]
macro_rules! field_source {
    ($ty:ty : tx) => {
        impl FromCurrentTx for $ty {}
    };
    ($ty:ty : obj) => {
        impl FromLedger for $ty {}
    };
    ($ty:ty : tx, obj) => {
        impl FromCurrentTx for $ty {}
        impl FromLedger for $ty {}
    };
    ($ty:ty : obj, tx) => {
        impl FromCurrentTx for $ty {}
        impl FromLedger for $ty {}
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq)]
    struct TxOnly(u8);

    impl FieldDecoder for TxOnly {
        type Buffer = [u8; 1];

        fn decode(bytes: &[u8]) -> Result<Self, Error> {
            bytes
                .first()
                .copied()
                .map(TxOnly)
                .ok_or(Error::FieldNotFound)
        }
    }
    field_source!(TxOnly : tx);

    #[derive(Debug, PartialEq, Eq)]
    struct ObjOnly(u8);

    impl FieldDecoder for ObjOnly {
        type Buffer = [u8; 1];

        fn decode(bytes: &[u8]) -> Result<Self, Error> {
            bytes
                .first()
                .copied()
                .map(ObjOnly)
                .ok_or(Error::FieldNotFound)
        }
    }
    field_source!(ObjOnly : obj);

    #[derive(Debug, PartialEq, Eq)]
    struct TxAndObj(u8);

    impl FieldDecoder for TxAndObj {
        type Buffer = [u8; 1];

        fn decode(bytes: &[u8]) -> Result<Self, Error> {
            bytes
                .first()
                .copied()
                .map(TxAndObj)
                .ok_or(Error::FieldNotFound)
        }
    }
    field_source!(TxAndObj : tx, obj);

    #[derive(Debug, PartialEq, Eq)]
    struct ObjAndTx(u8);

    impl FieldDecoder for ObjAndTx {
        type Buffer = [u8; 1];

        fn decode(bytes: &[u8]) -> Result<Self, Error> {
            bytes
                .first()
                .copied()
                .map(ObjAndTx)
                .ok_or(Error::FieldNotFound)
        }
    }
    field_source!(ObjAndTx : obj, tx);

    // These take no arguments and are never called; if a type didn't actually implement
    // the trait, the crate would fail to compile.
    fn assert_from_current_tx<T: FromCurrentTx>() {}
    fn assert_from_ledger<T: FromLedger>() {}

    #[test]
    fn tx_only_implements_from_current_tx_only() {
        assert_from_current_tx::<TxOnly>();
    }

    #[test]
    fn obj_only_implements_from_ledger_only() {
        assert_from_ledger::<ObjOnly>();
    }

    #[test]
    fn tx_and_obj_implements_both() {
        assert_from_current_tx::<TxAndObj>();
        assert_from_ledger::<TxAndObj>();
    }

    #[test]
    fn obj_and_tx_implements_both() {
        assert_from_current_tx::<ObjAndTx>();
        assert_from_ledger::<ObjAndTx>();
    }

    #[test]
    fn decode_returns_value_on_success() {
        let decoded = TxOnly::decode(&[42]).unwrap();
        assert_eq!(decoded, TxOnly(42));
    }

    #[test]
    fn decode_returns_error_on_empty_input() {
        let result = TxOnly::decode(&[]);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code(), Error::FieldNotFound.code());
    }

    #[test]
    fn buffer_type_has_expected_length() {
        let mut buffer = <TxOnly as FieldDecoder>::Buffer::default();
        assert_eq!(buffer.as_mut().len(), 1);
    }
}
