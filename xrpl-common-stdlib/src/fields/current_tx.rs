//! # Current Transaction Retrieval Module
//!
//! This module provides utilities for retrieving typed fields from the current XRPL transaction
//! within the context of XRPL Programmability. It offers a safe, type-safe
//! interface over the low-level host functions for accessing transaction data, such as from an
//! `EscrowFinish` transaction.
//!
//! ## Overview
//!
//! When processing XRPL transactions in a permissionless programmability environment, you often
//! need to extract specific fields like account IDs, hashes, public keys, and other data. This
//! module provides convenient wrapper functions that handle the low-level buffer management
//! and error handling required to safely retrieve these fields.
//!
//! `get_field` and `get_field_optional` are generic over any type implementing
//! [`crate::fields::decoder::FromCurrentTx`] — see [`crate::fields::decoder`] for how a type
//! opts into that.
//!
//! ## Optional vs Required Fields
//!
//! - **Required** (`get_field`): Returns an error if the field is missing.
//! - **Optional** (`get_field_optional`): Returns `Ok(None)` if the field is missing.
//!
//! Concrete transaction wrappers (e.g., `EscrowFinish`) live in their respective
//! companion crates (`xrpl-escrow-stdlib` for escrow flows).

pub mod traits;

use crate::fields::decoder::{FromCurrentTx, finish_field};
use crate::host::error_codes::FIELD_NOT_FOUND;
use crate::host::{Result, get_tx_field};
use crate::sfield::SField;

/// Retrieves a field from the current transaction using an SField constant.
///
/// # Arguments
///
/// * `field` - An SField constant that encodes both the field code and expected type
///
/// # Returns
///
/// Returns a `Result<T>` where:
/// * `Ok(T)` - The field value for the specified field
/// * `Err(Error)` - If the field cannot be retrieved, has unexpected size, or fails to decode
///
/// # Example
///
/// ```rust,no_run
/// use xrpl_common_stdlib::fields::current_tx::get_field;
/// use xrpl_common_stdlib::sfield;
///
/// // Type is automatically inferred from the SField constant
/// let sequence = get_field(sfield::Sequence).unwrap();  // u32
/// let account = get_field(sfield::Account).unwrap();  // AccountID
/// ```
#[inline]
pub fn get_field<T: FromCurrentTx, const CODE: i32>(_: SField<T, CODE>) -> Result<T> {
    let mut buf = T::empty_buffer();
    let n = {
        let slice = buf.as_mut();
        unsafe { get_tx_field(CODE, slice.as_mut_ptr(), slice.len()) }
    };
    finish_field::<T>(n, &mut buf)
}

/// Retrieves an optionally present field from the current transaction using an SField constant.
///
/// # Arguments
///
/// * `field` - An SField constant that encodes both the field code and expected type
///
/// # Returns
///
/// Returns a `Result<Option<T>>` where:
/// * `Ok(Some(T))` - The field value for the specified field
/// * `Ok(None)` - If the field is not present (i.e., result_code == FIELD_NOT_FOUND)
/// * `Err(Error)` - If the field cannot be retrieved, has unexpected size, or fails to decode
///
/// # Example
///
/// ```rust,no_run
/// use xrpl_common_stdlib::fields::current_tx::get_field_optional;
/// use xrpl_common_stdlib::sfield;
///
/// // Type is automatically inferred from the SField constant
/// let flags = get_field_optional(sfield::Flags).unwrap();  // Option<u32>
/// let source_tag = get_field_optional(sfield::SourceTag).unwrap();  // Option<u32>
/// ```
#[inline]
pub fn get_field_optional<T: FromCurrentTx, const CODE: i32>(
    _: SField<T, CODE>,
) -> Result<Option<T>> {
    let mut buf = T::empty_buffer();
    let n = {
        let slice = buf.as_mut();
        unsafe { get_tx_field(CODE, slice.as_mut_ptr(), slice.len()) }
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

    fn expect_tx_field(mock: &mut MockHostBindings, field_code: i32, size: usize, times: usize) {
        mock.expect_get_tx_field()
            .with(eq(field_code), always(), eq(size))
            .times(times)
            .returning(move |_, _, _| size as i32);
    }

    #[test]
    fn test_get_field_success() {
        let mut mock = MockHostBindings::new();
        expect_tx_field(&mut mock, sfield::Sequence.into(), 4, 1);
        expect_tx_field(&mut mock, sfield::Account.into(), ACCOUNT_ID_SIZE, 1);
        let _guard = setup_mock(mock);

        assert!(get_field::<u32, _>(sfield::Sequence).is_ok());
        assert!(get_field::<AccountID, _>(sfield::Account).is_ok());
    }

    #[test]
    fn test_get_field_optional_returns_none_on_field_not_found() {
        let mut mock = MockHostBindings::new();
        mock.expect_get_tx_field()
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
        expect_tx_field(&mut mock, sfield::SourceTag.into(), 4, 1);
        let _guard = setup_mock(mock);

        let result = get_field_optional::<u32, _>(sfield::SourceTag);
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[test]
    fn test_get_field_returns_decode_error_on_byte_mismatch() {
        // u32's FieldDecoder requires exactly 4 bytes; a shorter write fails the length check
        // and surfaces as InvalidDecoding.
        let mut mock = MockHostBindings::new();
        mock.expect_get_tx_field()
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
        mock.expect_get_tx_field()
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
        mock.expect_get_tx_field()
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
