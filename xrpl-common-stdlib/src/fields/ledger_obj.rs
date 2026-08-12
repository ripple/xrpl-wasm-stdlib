//! # Ledger Object Field Retrieval Module (by slot)
//!
//! Typed accessors for reading fields from a ledger object that has been cached into a slot (via
//! [`cache_ledger_entry`](crate::objects::cache_ledger_entry)). `get_field` and
//! `get_field_optional` are generic over any type
//! implementing [`crate::fields::decoder::FromLedger`] — see [`crate::fields::decoder`] for
//! how a type opts into that.

use crate::fields::decoder::{FromLedger, decode_host_result};
use crate::host::{Error, Result, le_field};
use crate::sfield::SField;
use crate::types::blob::Blob;
use core::mem::MaybeUninit;

/// Retrieves a field from the ledger object cached in `slot` using an SField constant.
///
/// # Returns
///
/// Returns a `Result<T>` where:
/// * `Ok(T)` - The field value for the specified field
/// * `Err(Error)` - If the field cannot be retrieved, has unexpected size, or fails to decode
#[inline]
pub fn get_field<T: FromLedger, const CODE: i32>(slot: i32, _: SField<T, CODE>) -> Result<T> {
    let mut buf = T::empty_buffer();
    let n = {
        let slice = buf.as_mut();
        unsafe { le_field(slot, CODE, slice.as_mut_ptr(), slice.len()) }
    };
    decode_host_result::<T>(buf, n)
}

/// Retrieves an optionally present field from the ledger object cached in `slot`.
///
/// # Returns
///
/// Returns a `Result<Option<T>>` where:
/// * `Ok(Some(T))` - The field value for the specified field
/// * `Ok(None)` - If the field is not present (i.e., result_code == FIELD_NOT_FOUND)
/// * `Err(Error)` - If the field cannot be retrieved, has unexpected size, or fails to decode
#[inline]
pub fn get_field_optional<T: FromLedger, const CODE: i32>(
    slot: i32,
    field: SField<T, CODE>,
) -> Result<Option<T>> {
    match get_field(slot, field) {
        Result::Ok(value) => Result::Ok(Some(value)),
        Result::Err(Error::FieldNotFound) => Result::Ok(None),
        Result::Err(e) => Result::Err(e),
    }
}

// --- `Blob<N>`-specific accessors ------------------------------------------------------------
//
// Why these exist (rather than just calling the generic `get_field`/`get_field_optional` above,
// which `Blob<N>` is also eligible for via `FromLedger`):
//
// The generic path allocates `T::empty_buffer()` as its own local, has the host write into it,
// and then hands that buffer to `T::decode` to build the returned `T`. For small fixed-size
// types (`u32`, `AccountID`, ...) that "move" from the local buffer into the returned value is
// reliably optimized away. For `Blob<N>` it is not: `Blob<N>` is `{ data: [u8; N], len: usize }`,
// a different memory shape than the bare `[u8; N]` buffer `get_field` allocates, so building the
// `Blob` from that buffer is a genuine field-by-field reconstruction, not just a reinterpret. We
// verified with `wasm2wat` against this crate's actual release profile (`opt-level = "s"`,
// `lto = true`, `codegen-units = 1`) that for `WasmBlob` (N = 4096) this reconstruction compiles
// to a real ~4092-byte `memcpy` per read — `-Os` does not fuse it away the way `-O2`/`-O3` might.
//
// The fix is to give the host nothing to reconstruct from: allocate the `Blob<N>` itself first,
// and point the host straight at its `data` field, so the bytes land in their final resting
// place on the very first (and only) write. There is then no second, distinctly-addressed buffer
// for the optimizer to have to notice is redundant — the copy is structurally absent rather than
// hoped-away.
//
// This is deliberately NOT folded into the generic `get_field`/`get_field_optional`/`FieldDecoder`
// machinery above, which is shared by every other field type (`u8..u64`, `AccountID`, `Amount`,
// `UInt<N>`, `TransactionType`) where the buffer is small enough that the copy is already
// negligible/elided. Scoping the fix to `Blob<N>` alone avoids adding raw-pointer plumbing to
// code paths that don't need it.

/// Retrieves a `Blob<N>` field from the ledger object cached in `slot`, writing the host's bytes
/// directly into the returned `Blob<N>`'s own storage. See the comment above this function for
/// why `Blob<N>` gets a dedicated accessor instead of using [`get_field`].
#[inline]
pub fn get_blob_field<const N: usize, const CODE: i32>(
    slot: i32,
    _: SField<Blob<N>, CODE>,
) -> Result<Blob<N>> {
    let mut blob = MaybeUninit::<Blob<N>>::uninit();
    // SAFETY: `data_ptr` points at the `data` field inside `blob`'s own (uninitialized) storage;
    // `blob` outlives this pointer and no other reference to it exists yet.
    let data_ptr = unsafe { core::ptr::addr_of_mut!((*blob.as_mut_ptr()).data) } as *mut u8;
    // Zero the destination *before* the host call: a result code of 0 is a legitimate "empty
    // field" success (not an error), and `Blob::data` is `pub`, so callers may read past `len`
    // directly. Zeroing in place (rather than in a separate scratch buffer) costs the same one
    // `memset` the old code paid anyway, just at the final address instead of a temporary one.
    unsafe { core::ptr::write_bytes(data_ptr, 0u8, N) };
    let n = unsafe { le_field(slot, CODE, data_ptr, N) };
    if n < 0 {
        return Result::Err(Error::from_code(n));
    }
    if n as usize > N {
        // A conformant host never reports writing more bytes than the buffer holds.
        return Result::Err(Error::PointerOutOfBounds);
    }
    // SAFETY: `data` was fully zeroed above and then (partially) overwritten by the host, so
    // every byte is initialized; `len` is set right before `assume_init`, completing the value.
    unsafe {
        core::ptr::addr_of_mut!((*blob.as_mut_ptr()).len).write(n as usize);
        Result::Ok(blob.assume_init())
    }
}

/// Optional variant of [`get_blob_field`]: returns `Ok(None)` if the field is not present,
/// otherwise behaves identically (including the direct-write zero-copy behavior). Implemented
/// in terms of [`get_blob_field`] itself rather than duplicating its body — `Error::FieldNotFound`
/// round-trips exactly through `Error::from_code`/`.code()` (both are just `FIELD_NOT_FOUND`),
/// so translating that one error case into `None` is all this needs to do.
#[inline]
pub fn get_blob_field_optional<const N: usize, const CODE: i32>(
    slot: i32,
    field: SField<Blob<N>, CODE>,
) -> Result<Option<Blob<N>>> {
    match get_blob_field(slot, field) {
        Result::Ok(blob) => Result::Ok(Some(blob)),
        Result::Err(Error::FieldNotFound) => Result::Ok(None),
        Result::Err(e) => Result::Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::{get_blob_field, get_blob_field_optional, get_field, get_field_optional};
    use crate::host::error_codes::{FIELD_NOT_FOUND, SOME_ERROR};
    use crate::host::host_bindings_trait::MockHostBindings;
    use crate::host::setup_mock;
    use crate::sfield;
    use crate::types::account_id::{ACCOUNT_ID_SIZE, AccountID};
    use crate::types::number::Number;
    use mockall::predicate::{always, eq};

    const SLOT: i32 = 3;

    fn expect_ledger_obj_field(
        mock: &mut MockHostBindings,
        slot: i32,
        field_code: i32,
        size: usize,
        times: usize,
    ) {
        mock.expect_le_field()
            .with(eq(slot), eq(field_code), always(), eq(size))
            .times(times)
            .returning(move |_, _, _, _| size as i32);
    }

    #[test]
    fn test_get_field_success() {
        let mut mock = MockHostBindings::new();
        expect_ledger_obj_field(&mut mock, SLOT, sfield::Sequence.into(), 4, 1);
        expect_ledger_obj_field(&mut mock, SLOT, sfield::Account.into(), ACCOUNT_ID_SIZE, 1);
        let _guard = setup_mock(mock);

        assert!(get_field::<u32, _>(SLOT, sfield::Sequence).is_ok());
        assert!(get_field::<AccountID, _>(SLOT, sfield::Account).is_ok());
    }

    #[test]
    fn test_get_field_decodes_stnumber_field() {
        // An `STI_NUMBER` field is 12 bytes; the mock writes a full-width value so this also
        // exercises `Number`'s buffer size and its `FromLedger` marker.
        const VALUE: [u8; 12] = [
            0x00, 0x03, 0x8D, 0x7E, 0xA4, 0xC6, 0x80, 0x00, 0xFF, 0xFF, 0xFF, 0xF1,
        ];
        let mut mock = MockHostBindings::new();
        mock.expect_le_field()
            .with(
                eq(SLOT),
                eq::<i32>(sfield::AssetsTotal.into()),
                always(),
                eq(VALUE.len()),
            )
            .times(1)
            .returning(|_, _, out, out_len| {
                unsafe { out.copy_from_nonoverlapping(VALUE.as_ptr(), VALUE.len()) }
                out_len as i32
            });
        let _guard = setup_mock(mock);

        assert_eq!(
            get_field(SLOT, sfield::AssetsTotal).unwrap(),
            Number::from(VALUE)
        );
    }

    #[test]
    fn test_get_field_optional_returns_none_on_field_not_found() {
        let mut mock = MockHostBindings::new();
        mock.expect_le_field()
            .with(
                eq(SLOT),
                eq::<i32>(sfield::SourceTag.into()),
                always(),
                eq(4),
            )
            .times(1)
            .returning(|_, _, _, _| FIELD_NOT_FOUND);
        let _guard = setup_mock(mock);

        let result = get_field_optional::<u32, _>(SLOT, sfield::SourceTag);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_get_field_optional_returns_some_when_present() {
        let mut mock = MockHostBindings::new();
        expect_ledger_obj_field(&mut mock, SLOT, sfield::SourceTag.into(), 4, 1);
        let _guard = setup_mock(mock);

        let result = get_field_optional::<u32, _>(SLOT, sfield::SourceTag);
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[test]
    fn test_get_field_returns_decode_error_on_byte_mismatch() {
        let mut mock = MockHostBindings::new();
        mock.expect_le_field()
            .with(
                eq(SLOT),
                eq::<i32>(sfield::Sequence.into()),
                always(),
                eq(4),
            )
            .times(1)
            .returning(|_, _, _, _| 3);
        let _guard = setup_mock(mock);

        let result = get_field::<u32, _>(SLOT, sfield::Sequence);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().code(),
            crate::host::Error::InvalidDecoding.code()
        );
    }

    #[test]
    fn test_get_field_returns_err_on_internal_error() {
        let mut mock = MockHostBindings::new();
        mock.expect_le_field()
            .with(eq(SLOT), eq::<i32>(sfield::Flags.into()), always(), eq(4))
            .times(1)
            .returning(|_, _, _, _| SOME_ERROR);
        let _guard = setup_mock(mock);

        let result = get_field::<u32, _>(SLOT, sfield::Flags);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().code(), SOME_ERROR);
    }

    #[test]
    fn test_get_field_returns_err_when_host_reports_oversized_write() {
        // A conformant host can't write past the buffer it was handed; a positive count larger
        // than the buffer is reported as PointerOutOfBounds.
        let mut mock = MockHostBindings::new();
        mock.expect_le_field()
            .with(
                eq(SLOT),
                eq::<i32>(sfield::Sequence.into()),
                always(),
                eq(4),
            )
            .times(1)
            .returning(|_, _, _, _| 8); // claims 8 bytes into a 4-byte u32 buffer
        let _guard = setup_mock(mock);

        let result = get_field::<u32, _>(SLOT, sfield::Sequence);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().code(),
            crate::host::Error::PointerOutOfBounds.code()
        );
    }

    #[test]
    fn test_get_blob_field_writes_bytes_directly_into_blob_data() {
        let mut mock = MockHostBindings::new();
        mock.expect_le_field()
            .with(
                eq(SLOT),
                eq::<i32>(sfield::Condition.into()),
                always(),
                eq(128),
            )
            .times(1)
            .returning(|_, _, buf, size| {
                // Simulate the host writing 128 bytes of non-zero data.
                let slice = unsafe { core::slice::from_raw_parts_mut(buf, size) };
                slice.fill(0xAB);
                size as i32
            });
        let _guard = setup_mock(mock);

        let blob = get_blob_field(SLOT, sfield::Condition).unwrap();
        assert_eq!(blob.len(), 128);
        assert!(blob.as_slice().iter().all(|&b| b == 0xAB));
    }

    #[test]
    fn test_get_blob_field_zeroes_tail_when_host_writes_fewer_bytes() {
        // A short write (e.g. an empty/undersized field) must leave the tail zeroed, not
        // uninitialized -- `Blob::data` is `pub`, so callers may read past `len` directly.
        let mut mock = MockHostBindings::new();
        mock.expect_le_field()
            .with(
                eq(SLOT),
                eq::<i32>(sfield::Condition.into()),
                always(),
                eq(128),
            )
            .times(1)
            .returning(|_, _, buf, _size| {
                let slice = unsafe { core::slice::from_raw_parts_mut(buf, 10) };
                slice.fill(0xFF);
                10
            });
        let _guard = setup_mock(mock);

        let blob = get_blob_field(SLOT, sfield::Condition).unwrap();
        assert_eq!(blob.len(), 10);
        assert_eq!(blob.data[9], 0xFF);
        assert_eq!(blob.data[10], 0);
        assert_eq!(blob.data[127], 0);
    }

    #[test]
    fn test_get_blob_field_returns_err_on_internal_error() {
        let mut mock = MockHostBindings::new();
        mock.expect_le_field()
            .with(
                eq(SLOT),
                eq::<i32>(sfield::Condition.into()),
                always(),
                eq(128),
            )
            .times(1)
            .returning(|_, _, _, _| SOME_ERROR);
        let _guard = setup_mock(mock);

        let result = get_blob_field(SLOT, sfield::Condition);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().code(), SOME_ERROR);
    }

    #[test]
    fn test_get_blob_field_returns_err_when_host_reports_oversized_write() {
        let mut mock = MockHostBindings::new();
        mock.expect_le_field()
            .with(
                eq(SLOT),
                eq::<i32>(sfield::Condition.into()),
                always(),
                eq(128),
            )
            .times(1)
            .returning(|_, _, _, _| 129); // claims 129 bytes into a 128-byte buffer
        let _guard = setup_mock(mock);

        let result = get_blob_field(SLOT, sfield::Condition);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().code(),
            crate::host::Error::PointerOutOfBounds.code()
        );
    }

    #[test]
    fn test_get_blob_field_optional_returns_none_on_field_not_found() {
        let mut mock = MockHostBindings::new();
        mock.expect_le_field()
            .with(
                eq(SLOT),
                eq::<i32>(sfield::Condition.into()),
                always(),
                eq(128),
            )
            .times(1)
            .returning(|_, _, _, _| FIELD_NOT_FOUND);
        let _guard = setup_mock(mock);

        let result = get_blob_field_optional(SLOT, sfield::Condition);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_get_blob_field_optional_returns_some_when_present() {
        let mut mock = MockHostBindings::new();
        expect_ledger_obj_field(&mut mock, SLOT, sfield::Condition.into(), 128, 1);
        let _guard = setup_mock(mock);

        let result = get_blob_field_optional(SLOT, sfield::Condition);
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }
}
