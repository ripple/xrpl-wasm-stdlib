//! Context-independent decode logic for typed field values.
//!
//! Reading a field always looks the same: call a host function into a buffer, then turn the
//! bytes it wrote into a typed value. [`FieldDecoder`] captures only that second step, so a type
//! implements it once regardless of how many contexts (current transaction, ledger object, ...)
//! can produce those bytes. The marker traits below record which contexts are valid for a given
//! type at compile time; the context-specific `get_field` functions (see
//! [`crate::fields::current_tx`], [`crate::fields::ledger_obj`]) require the matching marker.

use crate::types::decode_error::DecodeError;

/// Decodes a typed value from the raw bytes a host function wrote.
pub trait FieldDecoder: Sized {
    /// The buffer a `get_field` caller allocates before invoking the host function. Each type
    /// picks its own size (an associated type, not a `const`, so this stays on stable Rust).
    // TODO: once `generic_const_exprs` stabilises (tracking issue rust-lang/rust#76560), replace
    // this with `const SIZE: usize` and change the bound to `[u8; Self::SIZE]`, removing the
    // need for `empty_buffer()` entirely.
    type Buffer: AsMut<[u8]> + AsRef<[u8]>;

    /// Returns a zero-initialized buffer of this type's `Buffer` size.
    fn empty_buffer() -> Self::Buffer;

    /// Decodes `Self` from exactly the bytes the host wrote (not the full, possibly
    /// zero-padded, `Buffer`).
    fn decode(bytes: &[u8]) -> Result<Self, DecodeError>;
}

/// Marker: this type can be read from the current transaction via [`crate::fields::current_tx`].
pub trait FromCurrentTx: FieldDecoder {}

/// Marker: this type can be read from a ledger object via [`crate::fields::ledger_obj`].
pub trait FromLedger: FieldDecoder {}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq)]
    struct TxOnly(u8);

    impl FieldDecoder for TxOnly {
        type Buffer = [u8; 1];

        fn empty_buffer() -> Self::Buffer {
            [0u8; 1]
        }

        fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
            bytes.first().copied().map(TxOnly).ok_or(DecodeError)
        }
    }
    impl FromCurrentTx for TxOnly {}

    #[derive(Debug, PartialEq, Eq)]
    struct ObjOnly(u8);

    impl FieldDecoder for ObjOnly {
        type Buffer = [u8; 1];

        fn empty_buffer() -> Self::Buffer {
            [0u8; 1]
        }

        fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
            bytes.first().copied().map(ObjOnly).ok_or(DecodeError)
        }
    }
    impl FromLedger for ObjOnly {}

    #[derive(Debug, PartialEq, Eq)]
    struct TxAndObj(u8);

    impl FieldDecoder for TxAndObj {
        type Buffer = [u8; 1];

        fn empty_buffer() -> Self::Buffer {
            [0u8; 1]
        }

        fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
            bytes.first().copied().map(TxAndObj).ok_or(DecodeError)
        }
    }
    impl FromCurrentTx for TxAndObj {}
    impl FromLedger for TxAndObj {}

    // These take no arguments and are never called; if a type didn't actually implement
    // the trait, the crate would fail to compile. The negative direction (a type that
    // implements only one marker being rejected by the other) is covered by the
    // `tests/decoder_compile_fail.rs` trybuild cases.
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
        assert_eq!(TxOnly::decode(&[42]), Ok(TxOnly(42)));
    }

    #[test]
    fn decode_returns_error_on_empty_input() {
        assert_eq!(TxOnly::decode(&[]), Err(DecodeError));
    }

    #[test]
    fn empty_buffer_has_expected_length() {
        let mut buffer = <TxOnly as FieldDecoder>::empty_buffer();
        assert_eq!(buffer.as_mut().len(), 1);
    }
}
