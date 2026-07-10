//! # Current Ledger Object Field Retrieval Module
//!
//! Typed accessors for reading fields from the ledger object currently being processed (the one
//! the contract is attached to), without needing a slot. `get_field` and `get_field_optional` are
//! generic over any type implementing [`crate::fields::decoder::FromLedger`] — see
//! [`crate::fields::decoder`] for how a type opts into that.
//!
//! This is the no-slot counterpart to [`crate::fields::ledger_obj`]: the decode logic is identical,
//! only the host function differs (`get_current_ledger_obj_field` instead of `get_ledger_obj_field`).

use crate::fields::decoder::{FromLedger, finish_field};
use crate::host::error_codes::FIELD_NOT_FOUND;
use crate::host::{Result, get_current_ledger_obj_field};
use crate::sfield::SField;

/// Retrieves a field from the current ledger object using an SField constant.
///
/// # Returns
///
/// Returns a `Result<T>` where:
/// * `Ok(T)` - The field value for the specified field
/// * `Err(Error)` - If the field cannot be retrieved, has unexpected size, or fails to decode
#[inline]
pub fn get_field<T: FromLedger, const CODE: i32>(_: SField<T, CODE>) -> Result<T> {
    let mut buf = T::empty_buffer();
    let n = {
        let slice = buf.as_mut();
        unsafe { get_current_ledger_obj_field(CODE, slice.as_mut_ptr(), slice.len()) }
    };
    finish_field::<T>(n, &mut buf)
}

/// Retrieves an optionally present field from the current ledger object.
///
/// # Returns
///
/// Returns a `Result<Option<T>>` where:
/// * `Ok(Some(T))` - The field value for the specified field
/// * `Ok(None)` - If the field is not present (i.e., result_code == FIELD_NOT_FOUND)
/// * `Err(Error)` - If the field cannot be retrieved, has unexpected size, or fails to decode
#[inline]
pub fn get_field_optional<T: FromLedger, const CODE: i32>(_: SField<T, CODE>) -> Result<Option<T>> {
    let mut buf = T::empty_buffer();
    let n = {
        let slice = buf.as_mut();
        unsafe { get_current_ledger_obj_field(CODE, slice.as_mut_ptr(), slice.len()) }
    };
    if n == FIELD_NOT_FOUND {
        return Result::Ok(None);
    }
    finish_field::<T>(n, &mut buf).map(Some)
}

#[cfg(test)]
mod tests {
    use super::{get_field, get_field_optional};
    use crate::host::error_codes::{FIELD_NOT_FOUND, INTERNAL_ERROR};
    use crate::host::host_bindings_trait::MockHostBindings;
    use crate::host::setup_mock;
    use crate::sfield;
    use crate::types::account_id::{ACCOUNT_ID_SIZE, AccountID};
    use mockall::predicate::{always, eq};

    fn expect_current_field(
        mock: &mut MockHostBindings,
        field_code: i32,
        size: usize,
        times: usize,
    ) {
        mock.expect_get_current_ledger_obj_field()
            .with(eq(field_code), always(), eq(size))
            .times(times)
            .returning(move |_, _, _| size as i32);
    }

    #[test]
    fn test_get_field_success() {
        let mut mock = MockHostBindings::new();
        expect_current_field(&mut mock, sfield::Flags.into(), 4, 1);
        expect_current_field(&mut mock, sfield::Account.into(), ACCOUNT_ID_SIZE, 1);
        let _guard = setup_mock(mock);

        assert!(get_field::<u32, _>(sfield::Flags).is_ok());
        assert!(get_field::<AccountID, _>(sfield::Account).is_ok());
    }

    #[test]
    fn test_get_field_optional_returns_none_on_field_not_found() {
        let mut mock = MockHostBindings::new();
        mock.expect_get_current_ledger_obj_field()
            .with(eq::<i32>(sfield::SourceTag.into()), always(), eq(4))
            .times(1)
            .returning(|_, _, _| FIELD_NOT_FOUND);
        let _guard = setup_mock(mock);

        let result = get_field_optional::<u32, _>(sfield::SourceTag);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_get_field_optional_returns_some_when_present() {
        let mut mock = MockHostBindings::new();
        expect_current_field(&mut mock, sfield::SourceTag.into(), 4, 1);
        let _guard = setup_mock(mock);

        let result = get_field_optional::<u32, _>(sfield::SourceTag);
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[test]
    fn test_get_field_returns_decode_error_on_byte_mismatch() {
        let mut mock = MockHostBindings::new();
        mock.expect_get_current_ledger_obj_field()
            .with(eq::<i32>(sfield::Sequence.into()), always(), eq(4))
            .times(1)
            .returning(|_, _, _| 3);
        let _guard = setup_mock(mock);

        let result = get_field::<u32, _>(sfield::Sequence);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().code(),
            crate::host::Error::InvalidDecoding.code()
        );
    }

    #[test]
    fn test_get_field_returns_err_on_internal_error() {
        let mut mock = MockHostBindings::new();
        mock.expect_get_current_ledger_obj_field()
            .with(eq::<i32>(sfield::Flags.into()), always(), eq(4))
            .times(1)
            .returning(|_, _, _| INTERNAL_ERROR);
        let _guard = setup_mock(mock);

        let result = get_field::<u32, _>(sfield::Flags);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().code(), INTERNAL_ERROR);
    }

    #[test]
    fn test_get_field_returns_err_when_host_reports_oversized_write() {
        // A conformant host can't write past the buffer it was handed; a positive count larger
        // than the buffer is reported as PointerOutOfBounds.
        let mut mock = MockHostBindings::new();
        mock.expect_get_current_ledger_obj_field()
            .with(eq::<i32>(sfield::Sequence.into()), always(), eq(4))
            .times(1)
            .returning(|_, _, _| 8); // claims 8 bytes into a 4-byte u32 buffer
        let _guard = setup_mock(mock);

        let result = get_field::<u32, _>(sfield::Sequence);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().code(),
            crate::host::Error::PointerOutOfBounds.code()
        );
    }
}
