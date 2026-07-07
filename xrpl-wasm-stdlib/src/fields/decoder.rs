//! Field decoding traits for XRPL transaction and ledger object fields.
//!
//! This module defines a lightweight, host-independent decoding contract for types that
//! are constructed directly from a raw byte buffer.

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
    impl FromCurrentTx for TxOnly {}

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
    impl FromLedger for ObjOnly {}

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
    impl FromCurrentTx for TxAndObj {}
    impl FromLedger for TxAndObj {}

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
