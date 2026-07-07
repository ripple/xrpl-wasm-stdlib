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
//! ## Field Types Supported
//!
//! - **AccountID**: 20-byte account identifiers
//! - **u32**: 32-bit unsigned integers
//! - **Hash256**: 256-bit cryptographic hashes
//! - **PublicKey**: 33-byte public keys
//! - **Blob**: Variable-length binary data
//!
//! ## Optional vs Required Fields
//!
//! The module provides both optional and required variants for field retrieval:
//!
//! - **Required variants** (e.g., `get_u32_field`): Return an error if the field is missing
//! - **Optional variants** (e.g., `get_optional_u32_field`): Return `None` if the field is missing
//!
//! ## Error Handling
//!
//! All functions return `Result<T>` or `Result<Option<T>>` types that encapsulate
//! the custom error handling required for the XRPL Programmability environment.
//!
//! ## Safety Considerations
//!
//! - All functions use fixed-size buffers appropriate for their data types
//! - Buffer sizes are validated against expected field sizes
//! - Unsafe operations are contained within the low-level host function calls
//! - Memory safety is ensured through proper buffer management
//! - Field codes are validated by the underlying host functions
//!
//! ## Performance Notes
//!
//! - All functions are marked `#[inline]` to minimize call overhead
//! - Buffer allocations are stack-based and have minimal cost
//! - Host function calls are the primary performance bottleneck
//!
//! Concrete transaction wrappers (e.g., `EscrowFinish`) live in their respective
//! companion crates (`xrpl-escrow-stdlib` for escrow flows).

pub mod traits;

use crate::host::error_codes::{
    match_result_code_with_expected_bytes, match_result_code_with_expected_bytes_optional,
};
use crate::host::{Result, get_tx_field};
use crate::sfield::SField;

/// Trait for types that can be retrieved from current transaction fields.
///
/// This trait provides a unified interface for retrieving typed data from the current
/// XRPL transaction being processed, replacing the previous collection of type-specific
/// functions with a generic, type-safe approach.
///
/// ## Supported Types
///
/// The following types implement this trait:
/// - `u32` - 32-bit unsigned integers for sequence numbers, flags, timestamps
/// - `AccountID` - 20-byte account identifiers for transaction participants
/// - `Amount` - XRP amounts and token amounts for transaction values
/// - `Hash256` - 256-bit hashes for transaction IDs and references
/// - `PublicKey` - 33-byte compressed public keys for cryptographic operations
/// - `Blob<N>` - Variable-length binary data (generic over buffer size `N`)
///
/// ## Usage Patterns
///
/// ```rust,no_run
<<<<<<<< HEAD:xrpl-common-stdlib/src/current_tx/mod.rs
<<<<<<<< HEAD:xrpl-common-stdlib/src/current_tx/mod.rs
/// use xrpl_common_stdlib::current_tx::{get_field, get_field_optional};
/// use xrpl_common_stdlib::types::account_id::AccountID;
/// use xrpl_common_stdlib::types::amount::Amount;
========
/// use xrpl_common_stdlib::core::current_tx::{get_field, get_field_optional};
/// use xrpl_common_stdlib::core::types::account_id::AccountID;
/// use xrpl_common_stdlib::core::types::amount::Amount;
>>>>>>>> 38f2382 (renames, import fixes):xrpl-common-stdlib/src/core/current_tx/mod.rs
========
/// use xrpl_common_stdlib::fields::current_tx::{get_field, get_field_optional};
/// use xrpl_common_stdlib::types::account_id::AccountID;
/// use xrpl_common_stdlib::types::amount::Amount;
>>>>>>>> c621dc8 (common reorg):xrpl-common-stdlib/src/fields/current_tx/mod.rs
/// use xrpl_common_stdlib::sfield;
/// # fn example() {
///   // Get required fields from the current transaction
///   let account: AccountID = get_field(sfield::Account).unwrap();
///   let sequence: u32 = get_field(sfield::Sequence).unwrap();
///   let fee: Amount = get_field(sfield::Fee).unwrap();
///
///   // Get optional fields from the current transaction
///   let flags: Option<u32> = get_field_optional(sfield::Flags).unwrap();
/// # }
/// ```
///
/// ## Error Handling
///
/// - Required field methods return `Result<T>` and error if the field is missing
/// - Optional field methods return `Result<Option<T>>` and return `None` if the field is missing
/// - All methods return appropriate errors for buffer size mismatches or other retrieval failures
///
/// ## Transaction Context
///
/// This trait operates on the "current transaction" - the transaction currently being
/// processed in the XRPL Programmability environment. The transaction context is
/// established by the XRPL host environment before calling into WASM code.
///
/// ## Safety Considerations
///
/// - All implementations use appropriately sized buffers for their data types
/// - Buffer sizes are validated against expected field sizes where applicable
/// - Unsafe operations are contained within the host function calls
/// - Transaction field access is validated by the host environment
pub trait CurrentTxFieldGetter: Sized {
    /// Get a required field from the current transaction.
    ///
    /// This method retrieves a field that must be present in the transaction.
    /// If the field is missing, an error is returned.
    ///
    /// # Arguments
    ///
    /// * `field` - The SField identifying which field to retrieve
    ///
    /// # Returns
    ///
    /// Returns a `Result<Self>` where:
    /// * `Ok(Self)` - The field value for the specified field
    /// * `Err(Error::FieldNotFound)` - If the field is not present in the transaction
    /// * `Err(Error)` - If the field cannot be retrieved or has unexpected size
    fn get_from_current_tx<const CODE: i32>(field: SField<Self, CODE>) -> Result<Self>;

    /// Get an optional field from the current transaction.
    ///
    /// This method retrieves a field that may or may not be present in the transaction.
    /// If the field is missing, `None` is returned rather than an error.
    ///
    /// # Arguments
    ///
    /// * `field` - The SField identifying which field to retrieve
    ///
    /// # Returns
    ///
    /// Returns a `Result<Option<Self>>` where:
    /// * `Ok(Some(Self))` - The field value for the specified field
    /// * `Ok(None)` - If the field is not present in the transaction (i.e., result_code == FIELD_NOT_FOUND)
    /// * `Err(Error)` - If the field cannot be retrieved or has unexpected size
    fn get_from_current_tx_optional<const CODE: i32>(
        field: SField<Self, CODE>,
    ) -> Result<Option<Self>>;
}

/// Trait for types that can be retrieved as fixed-size fields from transactions.
///
/// This trait enables a generic implementation of `CurrentTxFieldGetter` for all fixed-size
/// unsigned integer types (u8, u16, u32, u64). Types implementing this trait must
/// have a known, constant size in bytes.
///
/// # Implementing Types
///
/// - `u8` - 1 byte
/// - `u16` - 2 bytes
/// - `u32` - 4 bytes
/// - `u64` - 8 bytes
trait FixedSizeFieldType: Sized {
    /// The size of this type in bytes
    const SIZE: usize;
}

impl FixedSizeFieldType for u8 {
    const SIZE: usize = 1;
}

impl FixedSizeFieldType for u16 {
    const SIZE: usize = 2;
}

impl FixedSizeFieldType for u32 {
    const SIZE: usize = 4;
}

impl FixedSizeFieldType for u64 {
    const SIZE: usize = 8;
}

/// Generic implementation of `CurrentTxFieldGetter` for all fixed-size unsigned integer types.
///
/// This single implementation handles u8, u16, u32, and u64 by leveraging the
/// `FixedSizeFieldType` trait. The implementation:
/// - Allocates a buffer of the appropriate size
/// - Calls the host function to retrieve the field
/// - Validates that the returned byte count matches the expected size
/// - Converts the buffer to the target type
///
/// # Buffer Management
///
/// Uses `MaybeUninit` for efficient stack allocation without initialization overhead.
/// The buffer size is determined at compile-time via the `SIZE` constant.
impl<T: FixedSizeFieldType> CurrentTxFieldGetter for T {
    #[inline]
    fn get_from_current_tx<const CODE: i32>(field: SField<Self, CODE>) -> Result<Self> {
        let mut value = core::mem::MaybeUninit::<T>::uninit();
        let result_code =
            unsafe { get_tx_field(i32::from(field), value.as_mut_ptr().cast(), T::SIZE) };
        match_result_code_with_expected_bytes(result_code, T::SIZE, || unsafe {
            value.assume_init()
        })
    }

    #[inline]
    fn get_from_current_tx_optional<const CODE: i32>(
        field: SField<Self, CODE>,
    ) -> Result<Option<Self>> {
        let mut value = core::mem::MaybeUninit::<T>::uninit();
        let result_code =
            unsafe { get_tx_field(i32::from(field), value.as_mut_ptr().cast(), T::SIZE) };
        match_result_code_with_expected_bytes_optional(result_code, T::SIZE, || {
            Some(unsafe { value.assume_init() })
        })
    }
}

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
/// * `Err(Error)` - If the field cannot be retrieved or has unexpected size
///
/// # Example
///
/// ```rust,no_run
<<<<<<<< HEAD:xrpl-common-stdlib/src/current_tx/mod.rs
<<<<<<<< HEAD:xrpl-common-stdlib/src/current_tx/mod.rs
/// use xrpl_common_stdlib::current_tx::get_field;
========
/// use xrpl_common_stdlib::core::current_tx::get_field;
>>>>>>>> 38f2382 (renames, import fixes):xrpl-common-stdlib/src/core/current_tx/mod.rs
========
/// use xrpl_common_stdlib::fields::current_tx::get_field;
>>>>>>>> c621dc8 (common reorg):xrpl-common-stdlib/src/fields/current_tx/mod.rs
/// use xrpl_common_stdlib::sfield;
///
/// // Type is automatically inferred from the SField constant
/// let sequence = get_field(sfield::Sequence).unwrap();  // u32
/// let account = get_field(sfield::Account).unwrap();  // AccountID
/// ```
#[inline]
pub fn get_field<T: CurrentTxFieldGetter, const CODE: i32>(field: SField<T, CODE>) -> Result<T> {
    T::get_from_current_tx(field)
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
/// * `Err(Error)` - If the field cannot be retrieved or has unexpected size
///
/// # Example
///
/// ```rust,no_run
<<<<<<<< HEAD:xrpl-common-stdlib/src/current_tx/mod.rs
<<<<<<<< HEAD:xrpl-common-stdlib/src/current_tx/mod.rs
/// use xrpl_common_stdlib::current_tx::get_field_optional;
========
/// use xrpl_common_stdlib::core::current_tx::get_field_optional;
>>>>>>>> 38f2382 (renames, import fixes):xrpl-common-stdlib/src/core/current_tx/mod.rs
========
/// use xrpl_common_stdlib::fields::current_tx::get_field_optional;
>>>>>>>> c621dc8 (common reorg):xrpl-common-stdlib/src/fields/current_tx/mod.rs
/// use xrpl_common_stdlib::sfield;
///
/// // Type is automatically inferred from the SField constant
/// let flags = get_field_optional(sfield::Flags).unwrap();  // Option<u32>
/// let source_tag = get_field_optional(sfield::SourceTag).unwrap();  // Option<u32>
/// ```
#[inline]
pub fn get_field_optional<T: CurrentTxFieldGetter, const CODE: i32>(
    field: SField<T, CODE>,
) -> Result<Option<T>> {
    T::get_from_current_tx_optional(field)
}

#[cfg(test)]
mod tests {
    use super::{CurrentTxFieldGetter, get_field, get_field_optional};
    use crate::host::error_codes::{FIELD_NOT_FOUND, INTERNAL_ERROR};
    use crate::host::host_bindings_trait::MockHostBindings;
    use crate::host::setup_mock;
    use crate::sfield;
    use crate::types::account_id::{ACCOUNT_ID_SIZE, AccountID};
    use crate::types::amount::{AMOUNT_SIZE, Amount};
    use crate::types::blob::{Blob, DEFAULT_BLOB_SIZE, PUBLIC_KEY_BLOB_SIZE, PublicKeyBlob};
    use crate::types::transaction_type::TransactionType;
    use crate::types::uint::{HASH256_SIZE, Hash256};
    use mockall::predicate::{always, eq};

    fn expect_tx_field(mock: &mut MockHostBindings, field_code: i32, size: usize, times: usize) {
        mock.expect_get_tx_field()
            .with(eq(field_code), always(), eq(size))
            .times(times)
            .returning(move |_, _, _| size as i32);
    }

    fn expect_tx_field_not_found(mock: &mut MockHostBindings, field_code: i32, size: usize) {
        mock.expect_get_tx_field()
            .with(eq(field_code), always(), eq(size))
            .times(1)
            .returning(|_, _, _| FIELD_NOT_FOUND);
    }

    // One fixed-size (u32) and one variable-size (AccountID) type are sufficient here;
    // success-path coverage for all supported types lives in the per-type getter tests above.
    #[test]
    fn test_optional_field_getter_returns_some_when_field_present() {
        let mut mock = MockHostBindings::new();

        expect_tx_field(&mut mock, sfield::SourceTag.into(), 4, 1);
        expect_tx_field(&mut mock, sfield::Destination.into(), ACCOUNT_ID_SIZE, 1);

        let _guard = setup_mock(mock);

        let result = u32::get_from_current_tx_optional(sfield::SourceTag);
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());

        let result = AccountID::get_from_current_tx_optional(sfield::Destination);
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[test]
    fn test_optional_field_getter_returns_none_when_field_not_found() {
        let mut mock = MockHostBindings::new();

        expect_tx_field_not_found(&mut mock, sfield::SourceTag.into(), 4);
        expect_tx_field_not_found(&mut mock, sfield::Destination.into(), ACCOUNT_ID_SIZE);

        let _guard = setup_mock(mock);

        let result = u32::get_from_current_tx_optional(sfield::SourceTag);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());

        let result = AccountID::get_from_current_tx_optional(sfield::Destination);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_required_field_getter_returns_err_when_field_not_found() {
        let mut mock = MockHostBindings::new();

        expect_tx_field_not_found(&mut mock, sfield::Sequence.into(), 4);

        let _guard = setup_mock(mock);

        assert!(u32::get_from_current_tx(sfield::Sequence).is_err());
    }

    #[test]
    #[should_panic]
    fn test_field_getter_panics_on_size_mismatch() {
        let mut mock = MockHostBindings::new();
        mock.expect_get_tx_field()
            .with(eq::<i32>(sfield::Sequence.into()), always(), eq(4))
            .times(1)
            .returning(|_, _, _| 2); // host returns fewer bytes than expected

        let _guard = setup_mock(mock);

        let _ = u32::get_from_current_tx(sfield::Sequence);
    }

    // get_field / get_field_optional are thin wrappers over get_from_current_tx / get_from_current_tx_optional,
    // so exercising u32 and AccountID here is sufficient; per-type coverage lives in the getter tests above.
    #[test]
    fn test_get_field_and_get_field_optional_convenience_fns() {
        let mut mock = MockHostBindings::new();

        expect_tx_field(&mut mock, sfield::Sequence.into(), 4, 1);
        expect_tx_field(&mut mock, sfield::Account.into(), ACCOUNT_ID_SIZE, 1);
        expect_tx_field(&mut mock, sfield::SourceTag.into(), 4, 1);

        let _guard = setup_mock(mock);

        assert!(get_field::<u32, _>(sfield::Sequence).is_ok());
        assert!(get_field::<AccountID, _>(sfield::Account).is_ok());

        let result = get_field_optional::<u32, _>(sfield::SourceTag);
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[test]
    fn test_get_field_returns_err_on_internal_error() {
        let mut mock = MockHostBindings::new();
        mock.expect_get_tx_field()
            .with(eq::<i32>(sfield::Flags.into()), always(), eq(4))
            .times(1)
            .returning(|_, _, _| INTERNAL_ERROR);

        let _guard = setup_mock(mock);

        assert!(get_field::<u32, _>(sfield::Flags).is_err());
    }

    #[test]
    fn test_u8_field_getter() {
        let mut mock = MockHostBindings::new();
        expect_tx_field(&mut mock, sfield::Generic.into(), 1, 1);
        let _guard = setup_mock(mock);
        assert!(u8::get_from_current_tx(sfield::Generic).is_ok());
    }

    #[test]
    fn test_u16_field_getter() {
        let mut mock = MockHostBindings::new();
        expect_tx_field(&mut mock, sfield::SignerWeight.into(), 2, 1);
        let _guard = setup_mock(mock);
        assert!(u16::get_from_current_tx(sfield::SignerWeight).is_ok());
    }

    #[test]
    fn test_u64_field_getter() {
        let mut mock = MockHostBindings::new();
        expect_tx_field(&mut mock, sfield::IndexNext.into(), 8, 1);
        let _guard = setup_mock(mock);
        assert!(u64::get_from_current_tx(sfield::IndexNext).is_ok());
    }

    #[test]
    fn test_account_id_field_getter() {
        let mut mock = MockHostBindings::new();
        expect_tx_field(&mut mock, sfield::Account.into(), ACCOUNT_ID_SIZE, 1);
        let _guard = setup_mock(mock);
        assert!(AccountID::get_from_current_tx(sfield::Account).is_ok());
    }

    #[test]
    fn test_hash256_field_getter() {
        let mut mock = MockHostBindings::new();
        expect_tx_field(&mut mock, sfield::PreviousTxnID.into(), HASH256_SIZE, 1);
        let _guard = setup_mock(mock);
        assert!(Hash256::get_from_current_tx(sfield::PreviousTxnID).is_ok());
    }

    #[test]
    fn test_amount_field_getter() {
        let mut mock = MockHostBindings::new();
        expect_tx_field(&mut mock, sfield::Fee.into(), AMOUNT_SIZE, 1);
        let _guard = setup_mock(mock);
        assert!(Amount::get_from_current_tx(sfield::Fee).is_ok());
    }

    #[test]
    fn test_public_key_blob_field_getter() {
        let mut mock = MockHostBindings::new();
        expect_tx_field(
            &mut mock,
            sfield::SigningPubKey.into(),
            PUBLIC_KEY_BLOB_SIZE,
            1,
        );
        let _guard = setup_mock(mock);
        assert!(PublicKeyBlob::get_from_current_tx(sfield::SigningPubKey).is_ok());
    }

    #[test]
    fn test_transaction_type_field_getter() {
        let mut mock = MockHostBindings::new();
        expect_tx_field(&mut mock, sfield::TransactionType.into(), 2, 1);
        let _guard = setup_mock(mock);
        assert!(TransactionType::get_from_current_tx(sfield::TransactionType).is_ok());
    }

    #[test]
    fn test_blob_field_getter() {
        let mut mock = MockHostBindings::new();
        expect_tx_field(&mut mock, sfield::MemoData.into(), DEFAULT_BLOB_SIZE, 1);
        let _guard = setup_mock(mock);
        assert!(Blob::<DEFAULT_BLOB_SIZE>::get_from_current_tx(sfield::MemoData).is_ok());
    }

    #[test]
    fn test_get_tx_field_pipeline_routes_bytes_to_amount() {
        let mut mock = MockHostBindings::new();
        mock.expect_get_tx_field()
            .with(eq::<i32>(sfield::Amount.into()), always(), eq(AMOUNT_SIZE))
            .times(1)
            .returning(|_, buf, size| {
                let slice = unsafe { core::slice::from_raw_parts_mut(buf, size) };
                slice.fill(0);
                let mut be = 1000u64.to_be_bytes();
                be[0] |= 0x40;
                slice[0..8].copy_from_slice(&be);
                8
            });

        let _guard = setup_mock(mock);

        let amount = Amount::get_from_current_tx(sfield::Amount).unwrap();
        assert!(matches!(amount, Amount::XRP { num_drops: 1000 }));
    }
}
