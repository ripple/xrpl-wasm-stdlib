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
    ///
    /// Note: this is `AsMut<[u8]>` only, not `+ Default` — `[u8; N]: Default` is only
    /// implemented by std for a handful of small `N`, not arbitrary const generics, so
    /// zero-initialization goes through [`FieldDecoder::empty_buffer`] instead.
    type Buffer: AsMut<[u8]>;

    /// Returns a zero-initialized buffer of this type's `Buffer` size.
    fn empty_buffer() -> Self::Buffer;

    /// Decodes `Self` from exactly the bytes the host wrote (not the full, possibly
    /// zero-padded, `Buffer`).
    fn decode(bytes: &[u8]) -> Result<Self, DecodeError>;
}

/// Marker: this type can be read from the current transaction via [`crate::fields::current_tx`].
///
/// The `on_unimplemented` message turns the otherwise cryptic trait-bound error into guidance for
/// the common misuse: passing an array/object placeholder (e.g. `sfield::Memos`) to a getter.
/// Those aggregate types can't be decoded whole — navigate into them with a locator instead.
#[diagnostic::on_unimplemented(
    message = "`{Self}` cannot be read from the current transaction",
    note = "array/object fields can't be decoded directly. Navigate into them with a locator (`.field(sfield::X)...`) or use a typed accessor"
)]
pub trait FromCurrentTx: FieldDecoder {}

/// Marker: this type can be read from a ledger object via [`crate::fields::ledger_obj`] or
/// [`crate::fields::current_ledger_obj`].
///
/// See [`FromCurrentTx`] for why the `on_unimplemented` message is worth carrying.
#[diagnostic::on_unimplemented(
    message = "`{Self}` cannot be read from a ledger object",
    note = "array/object fields can't be decoded directly. Navigate into them with a locator (`.field(sfield::X)...`) or use a typed accessor"
)]
pub trait FromLedger: FieldDecoder {}

/// Turns the raw byte count a host `get_*_field` function returned (written into `buf`) into a
/// decoded `T`. Shared by the by-slot and current-object getters so the error handling — negative
/// codes, an oversized write the buffer can't have held, and decode failures — lives in one place.
#[inline]
pub(crate) fn finish_field<T: FieldDecoder>(n: i32, buf: &mut T::Buffer) -> host::Result<T> {
    if n < 0 {
        return host::Result::Err(host::Error::from_code(n));
    }
    let bytes = buf.as_mut();
    let n = n as usize;
    if n > bytes.len() {
        // A conformant host never reports writing more bytes than the buffer holds; a positive
        // count past our buffer means it described memory outside the allowed region.
        return host::Result::Err(host::Error::PointerOutOfBounds);
    }
    match T::decode(&bytes[..n]) {
        core::result::Result::Ok(value) => host::Result::Ok(value),
        core::result::Result::Err(_) => host::Result::Err(host::Error::InvalidDecoding),
    }
}

// `FieldDecoder` for the fixed-width unsigned integers the host writes directly. Each reads
// exactly its own width and reinterprets the raw bytes as the host laid them out (native-endian
// on wasm32, hence `from_ne_bytes`). All four are readable from both a transaction and a ledger
// object.

impl FieldDecoder for u8 {
    type Buffer = [u8; 1];

    #[inline]
    fn empty_buffer() -> Self::Buffer {
        [0u8; 1]
    }

    #[inline]
    fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        let array: [u8; 1] = bytes.try_into().map_err(|_| DecodeError)?;
        Ok(u8::from_ne_bytes(array))
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
    fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        let array: [u8; 2] = bytes.try_into().map_err(|_| DecodeError)?;
        Ok(u16::from_ne_bytes(array))
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
    fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        let array: [u8; 4] = bytes.try_into().map_err(|_| DecodeError)?;
        Ok(u32::from_ne_bytes(array))
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
    fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        let array: [u8; 8] = bytes.try_into().map_err(|_| DecodeError)?;
        Ok(u64::from_ne_bytes(array))
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
