//! Context-independent decode logic for typed field values.
//!
//! Reading a field always looks the same: call a host function into a buffer, then turn the
//! bytes it wrote into a typed value. [`FieldDecoder`] captures only that second step, so a type
//! implements it once regardless of how many contexts (current transaction, ledger object, ...)
//! can produce those bytes. The marker traits below record which contexts are valid for a given
//! type at compile time; the context-specific `get_field` functions (see
//! [`crate::fields::current_tx`], [`crate::fields::ledger_obj`]) require the matching marker.

use crate::host;
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

    /// Decodes `Self` from the full `Buffer` (as written by the host, then zero-padded to the
    /// buffer's size), given `bytes_written` — the number of bytes the host actually wrote.
    ///
    /// `bytes_written` carries the length that a `&[u8]` slice would otherwise bundle inside its
    /// fat pointer — passing the whole buffer plus `bytes_written` lets fixed-layout types (e.g.
    /// `Amount`) read the padded buffer in place, with no re-slice or re-copy.
    fn decode(buf: &Self::Buffer, bytes_written: usize) -> Result<Self, DecodeError>;
}

/// Marker: this type can be read from the current transaction via [`crate::fields::current_tx`].
pub trait FromCurrentTx: FieldDecoder {}

/// Marker: this type can be read from a ledger object via [`crate::fields::ledger_obj`].
pub trait FromLedger: FieldDecoder {}

/// Shared step behind every `get_field`/`get_field_optional` in [`crate::fields::current_tx`]
/// and [`crate::fields::ledger_obj`]: turn a host result code and the buffer it (partially)
/// filled into a typed value.
///
/// Callers handle the "field not found" case themselves (only `get_field_optional` has one)
/// before reaching here; `n` is assumed to be either a real byte count or a hard error.
#[inline]
pub(crate) fn decode_result<T: FieldDecoder>(buf: &T::Buffer, n: i32) -> host::Result<T> {
    if n < 0 {
        return host::Result::Err(host::Error::from_code(n));
    }
    let n = n as usize;
    if n > buf.as_ref().len() {
        // A conformant host never reports writing more bytes than the buffer holds; a positive
        // count past our buffer means it described memory outside the allowed region.
        return host::Result::Err(host::Error::PointerOutOfBounds);
    }
    match T::decode(buf, n) {
        Ok(value) => host::Result::Ok(value),
        Err(_) => host::Result::Err(host::Error::InvalidDecoding),
    }
}

impl FieldDecoder for u8 {
    type Buffer = [u8; 1];

    #[inline]
    fn empty_buffer() -> Self::Buffer {
        [0u8; 1]
    }

    #[inline]
    fn decode(buf: &Self::Buffer, bytes_written: usize) -> Result<Self, DecodeError> {
        if bytes_written != buf.len() {
            return Err(DecodeError);
        }
        Ok(u8::from_le_bytes(*buf))
    }
}

impl FromCurrentTx for u8 {}
impl FromLedger for u8 {}

impl FieldDecoder for u16 {
    type Buffer = [u8; 2];

    #[inline]
    fn empty_buffer() -> Self::Buffer {
        [0u8; 2]
    }

    #[inline]
    fn decode(buf: &Self::Buffer, bytes_written: usize) -> Result<Self, DecodeError> {
        if bytes_written != buf.len() {
            return Err(DecodeError);
        }
        Ok(u16::from_le_bytes(*buf))
    }
}

impl FromCurrentTx for u16 {}
impl FromLedger for u16 {}

impl FieldDecoder for u32 {
    type Buffer = [u8; 4];

    #[inline]
    fn empty_buffer() -> Self::Buffer {
        [0u8; 4]
    }

    #[inline]
    fn decode(buf: &Self::Buffer, bytes_written: usize) -> Result<Self, DecodeError> {
        if bytes_written != buf.len() {
            return Err(DecodeError);
        }
        Ok(u32::from_le_bytes(*buf))
    }
}

impl FromCurrentTx for u32 {}
impl FromLedger for u32 {}

impl FieldDecoder for u64 {
    type Buffer = [u8; 8];

    #[inline]
    fn empty_buffer() -> Self::Buffer {
        [0u8; 8]
    }

    #[inline]
    fn decode(buf: &Self::Buffer, bytes_written: usize) -> Result<Self, DecodeError> {
        if bytes_written != buf.len() {
            return Err(DecodeError);
        }
        Ok(u64::from_le_bytes(*buf))
    }
}

impl FromCurrentTx for u64 {}
impl FromLedger for u64 {}

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

        fn decode(buf: &Self::Buffer, bytes_written: usize) -> Result<Self, DecodeError> {
            if bytes_written == 0 {
                return Err(DecodeError);
            }
            Ok(TxOnly(buf[0]))
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

        fn decode(buf: &Self::Buffer, bytes_written: usize) -> Result<Self, DecodeError> {
            if bytes_written == 0 {
                return Err(DecodeError);
            }
            Ok(ObjOnly(buf[0]))
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

        fn decode(buf: &Self::Buffer, bytes_written: usize) -> Result<Self, DecodeError> {
            if bytes_written == 0 {
                return Err(DecodeError);
            }
            Ok(TxAndObj(buf[0]))
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
        assert_eq!(TxOnly::decode(&[42], 1), Ok(TxOnly(42)));
    }

    #[test]
    fn decode_returns_error_on_empty_input() {
        assert_eq!(TxOnly::decode(&[0], 0), Err(DecodeError));
    }

    #[test]
    fn empty_buffer_has_expected_length() {
        let mut buffer = <TxOnly as FieldDecoder>::empty_buffer();
        assert_eq!(buffer.as_mut().len(), 1);
    }
}
