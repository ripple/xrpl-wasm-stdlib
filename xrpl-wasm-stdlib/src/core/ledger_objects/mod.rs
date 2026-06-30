pub mod account_root;
pub mod array_object;
pub mod current_escrow;
pub mod escrow;
pub mod traits;

use crate::core::types::uint::{HASH160_SIZE, HASH192_SIZE, Hash160, Hash192};
use crate::host::error_codes::{
    match_result_code_with_expected_bytes, match_result_code_with_expected_bytes_optional,
};
use crate::host::{Result, get_current_ledger_obj_field, get_ledger_obj_field};

/// Trait for types that can be retrieved from ledger object fields.
///
/// This trait provides a unified interface for retrieving typed data from XRPL ledger objects,
/// replacing the previous collection of type-specific functions with a generic, type-safe approach.
///
/// ## Supported Types
///
/// The following types implement this trait:
/// - `u8` - 8-bit unsigned integers (1 byte)
/// - `u16` - 16-bit unsigned integers (2 bytes)
/// - `u32` - 32-bit unsigned integers (4 bytes)
/// - `u64` - 64-bit unsigned integers (8 bytes)
/// - `AccountID` - 20-byte account identifiers
/// - `Amount` - XRP amounts and token amounts (variable size, up to 48 bytes)
/// - `Hash128` - 128-bit cryptographic hashes (16 bytes)
/// - `Hash256` - 256-bit cryptographic hashes (32 bytes)
/// - `Blob<N>` - Variable-length binary data (generic over buffer size `N`)
///
/// ## Usage Patterns
///
/// ```rust,no_run
/// use xrpl_wasm_stdlib::core::ledger_objects::{ledger_object, current_ledger_object};
/// use xrpl_wasm_stdlib::core::types::account_id::AccountID;
/// use xrpl_wasm_stdlib::core::types::amount::Amount;
/// use xrpl_wasm_stdlib::sfield;
///
/// fn example() {
///   let slot = 0;
///   // Get a required field from a specific ledger object
///   let balance = ledger_object::get_field(slot, sfield::Balance.into()).unwrap();
///   let account = ledger_object::get_field(slot, sfield::Account.into()).unwrap();
///
///   // Get an optional field from the current ledger object
///   let flags = current_ledger_object::get_field_optional(sfield::Flags).unwrap();
/// }
/// ```
///
/// ## Error Handling
///
/// - Required field methods return `Result<T>` and error if the field is missing
/// - Optional field methods return `Result<Option<T>>` and return `None` if the field is missing
/// - All methods return appropriate errors for buffer size mismatches or other retrieval failures
///
/// ## Safety Considerations
///
/// - All implementations use appropriately sized buffers for their data types
/// - Buffer sizes are validated against expected field sizes where applicable
/// - Unsafe operations are contained within the host function calls
pub trait LedgerObjectFieldGetter: Sized {
    /// Get a required field from the current ledger object.
    ///
    /// # Arguments
    ///
    /// * `field_code` - The field code identifying which field to retrieve
    ///
    /// # Returns
    ///
    /// Returns a `Result<Self>` where:
    /// * `Ok(Self)` - The field value for the specified field
    /// * `Err(Error)` - If the field cannot be retrieved or has unexpected size
    fn get_from_current_ledger_obj(field_code: i32) -> Result<Self>;

    /// Get an optional field from the current ledger object.
    ///
    /// # Arguments
    ///
    /// * `field_code` - The field code identifying which field to retrieve
    ///
    /// # Returns
    ///
    /// Returns a `Result<Option<Self>>` where:
    /// * `Ok(Some(Self))` - The field value for the specified field
    /// * `Ok(None)` - If the field is not present
    /// * `Err(Error)` - If the field cannot be retrieved or has unexpected size
    fn get_from_current_ledger_obj_optional(field_code: i32) -> Result<Option<Self>>;

    /// Get a required field from a specific ledger object.
    ///
    /// # Arguments
    ///
    /// * `register_num` - The register number holding the ledger object
    /// * `field_code` - The field code identifying which field to retrieve
    ///
    /// # Returns
    ///
    /// Returns a `Result<Self>` where:
    /// * `Ok(Self)` - The field value for the specified field
    /// * `Err(Error)` - If the field cannot be retrieved or has unexpected size
    fn get_from_ledger_obj(register_num: i32, field_code: i32) -> Result<Self>;

    /// Get an optional field from a specific ledger object.
    ///
    /// # Arguments
    ///
    /// * `register_num` - The register number holding the ledger object
    /// * `field_code` - The field code identifying which field to retrieve
    ///
    /// # Returns
    ///
    /// Returns a `Result<Option<Self>>` where:
    /// * `Ok(Some(Self))` - The field value for the specified field
    /// * `Ok(None)` - If the field is not present in the ledger object
    /// * `Err(Error)` - If the field retrieval operation failed
    fn get_from_ledger_obj_optional(register_num: i32, field_code: i32) -> Result<Option<Self>>;
}

/// Trait for types that can be retrieved as fixed-size fields from ledger objects.
///
/// This trait enables a generic implementation of `LedgerObjectFieldGetter` for all fixed-size
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

/// Generic implementation of `LedgerObjectFieldGetter` for all fixed-size unsigned integer types.
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
impl<T: FixedSizeFieldType> LedgerObjectFieldGetter for T {
    #[inline]
    fn get_from_current_ledger_obj(field_code: i32) -> Result<Self> {
        let mut value = core::mem::MaybeUninit::<T>::uninit();
        let result_code =
            unsafe { get_current_ledger_obj_field(field_code, value.as_mut_ptr().cast(), T::SIZE) };
        match_result_code_with_expected_bytes(result_code, T::SIZE, || unsafe {
            value.assume_init()
        })
    }

    #[inline]
    fn get_from_current_ledger_obj_optional(field_code: i32) -> Result<Option<Self>> {
        let mut value = core::mem::MaybeUninit::<T>::uninit();
        let result_code =
            unsafe { get_current_ledger_obj_field(field_code, value.as_mut_ptr().cast(), T::SIZE) };
        match_result_code_with_expected_bytes_optional(result_code, T::SIZE, || {
            Some(unsafe { value.assume_init() })
        })
    }

    #[inline]
    fn get_from_ledger_obj(register_num: i32, field_code: i32) -> Result<Self> {
        let mut value = core::mem::MaybeUninit::<T>::uninit();
        let result_code = unsafe {
            get_ledger_obj_field(register_num, field_code, value.as_mut_ptr().cast(), T::SIZE)
        };
        match_result_code_with_expected_bytes(result_code, T::SIZE, || unsafe {
            value.assume_init()
        })
    }

    #[inline]
    fn get_from_ledger_obj_optional(register_num: i32, field_code: i32) -> Result<Option<Self>> {
        let mut value = core::mem::MaybeUninit::<T>::uninit();
        let result_code = unsafe {
            get_ledger_obj_field(register_num, field_code, value.as_mut_ptr().cast(), T::SIZE)
        };
        match_result_code_with_expected_bytes_optional(result_code, T::SIZE, || {
            Some(unsafe { value.assume_init() })
        })
    }
}

/// Implementation of `LedgerObjectFieldGetter` for 160-bit cryptographic hashes.
///
/// This implementation handles 20-byte hash fields in XRPL ledger objects.
/// Hash160 values are used for various cryptographic operations and identifiers.
///
/// # Buffer Management
///
/// Uses a 20-byte buffer (HASH160_SIZE) and validates that exactly 20 bytes
/// are returned from the host function to ensure data integrity.
impl LedgerObjectFieldGetter for Hash160 {
    #[inline]
    fn get_from_current_ledger_obj(field_code: i32) -> Result<Self> {
        let mut buffer = core::mem::MaybeUninit::<[u8; HASH160_SIZE]>::uninit();
        let result_code = unsafe {
            get_current_ledger_obj_field(field_code, buffer.as_mut_ptr().cast(), HASH160_SIZE)
        };
        match_result_code_with_expected_bytes(result_code, HASH160_SIZE, || {
            Hash160::from(unsafe { buffer.assume_init() })
        })
    }

    #[inline]
    fn get_from_current_ledger_obj_optional(field_code: i32) -> Result<Option<Self>> {
        let mut buffer = core::mem::MaybeUninit::<[u8; HASH160_SIZE]>::uninit();
        let result_code = unsafe {
            get_current_ledger_obj_field(field_code, buffer.as_mut_ptr().cast(), HASH160_SIZE)
        };
        match_result_code_with_expected_bytes_optional(result_code, HASH160_SIZE, || {
            Some(Hash160::from(unsafe { buffer.assume_init() }))
        })
    }

    #[inline]
    fn get_from_ledger_obj(register_num: i32, field_code: i32) -> Result<Self> {
        let mut buffer = core::mem::MaybeUninit::<[u8; HASH160_SIZE]>::uninit();
        let result_code = unsafe {
            get_ledger_obj_field(
                register_num,
                field_code,
                buffer.as_mut_ptr().cast(),
                HASH160_SIZE,
            )
        };
        match_result_code_with_expected_bytes(result_code, HASH160_SIZE, || {
            Hash160::from(unsafe { buffer.assume_init() })
        })
    }

    #[inline]
    fn get_from_ledger_obj_optional(register_num: i32, field_code: i32) -> Result<Option<Self>> {
        let mut buffer = core::mem::MaybeUninit::<[u8; HASH160_SIZE]>::uninit();
        let result_code = unsafe {
            get_ledger_obj_field(
                register_num,
                field_code,
                buffer.as_mut_ptr().cast(),
                HASH160_SIZE,
            )
        };
        match_result_code_with_expected_bytes_optional(result_code, HASH160_SIZE, || {
            Some(Hash160::from(unsafe { buffer.assume_init() }))
        })
    }
}

/// Implementation of `LedgerObjectFieldGetter` for 192-bit cryptographic hashes.
///
/// This implementation handles 24-byte hash fields in XRPL ledger objects.
/// Hash192 values are used for various cryptographic operations and identifiers.
///
/// # Buffer Management
///
/// Uses a 24-byte buffer (HASH192_SIZE) and validates that exactly 24 bytes
/// are returned from the host function to ensure data integrity.
impl LedgerObjectFieldGetter for Hash192 {
    #[inline]
    fn get_from_current_ledger_obj(field_code: i32) -> Result<Self> {
        let mut buffer = core::mem::MaybeUninit::<[u8; HASH192_SIZE]>::uninit();
        let result_code = unsafe {
            get_current_ledger_obj_field(field_code, buffer.as_mut_ptr().cast(), HASH192_SIZE)
        };
        match_result_code_with_expected_bytes(result_code, HASH192_SIZE, || {
            Hash192::from(unsafe { buffer.assume_init() })
        })
    }

    #[inline]
    fn get_from_current_ledger_obj_optional(field_code: i32) -> Result<Option<Self>> {
        let mut buffer = core::mem::MaybeUninit::<[u8; HASH192_SIZE]>::uninit();
        let result_code = unsafe {
            get_current_ledger_obj_field(field_code, buffer.as_mut_ptr().cast(), HASH192_SIZE)
        };
        match_result_code_with_expected_bytes_optional(result_code, HASH192_SIZE, || {
            Some(Hash192::from(unsafe { buffer.assume_init() }))
        })
    }

    #[inline]
    fn get_from_ledger_obj(register_num: i32, field_code: i32) -> Result<Self> {
        let mut buffer = core::mem::MaybeUninit::<[u8; HASH192_SIZE]>::uninit();
        let result_code = unsafe {
            get_ledger_obj_field(
                register_num,
                field_code,
                buffer.as_mut_ptr().cast(),
                HASH192_SIZE,
            )
        };
        match_result_code_with_expected_bytes(result_code, HASH192_SIZE, || {
            Hash192::from(unsafe { buffer.assume_init() })
        })
    }

    #[inline]
    fn get_from_ledger_obj_optional(register_num: i32, field_code: i32) -> Result<Option<Self>> {
        let mut buffer = core::mem::MaybeUninit::<[u8; HASH192_SIZE]>::uninit();
        let result_code = unsafe {
            get_ledger_obj_field(
                register_num,
                field_code,
                buffer.as_mut_ptr().cast(),
                HASH192_SIZE,
            )
        };
        match_result_code_with_expected_bytes_optional(result_code, HASH192_SIZE, || {
            Some(Hash192::from(unsafe { buffer.assume_init() }))
        })
    }
}

pub mod current_ledger_object {
    use super::LedgerObjectFieldGetter;
    use crate::host::Result;
    use crate::sfield::SField;

    /// Retrieves a field from the current ledger object.
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
    /// use xrpl_wasm_stdlib::core::ledger_objects::current_ledger_object;
    /// use xrpl_wasm_stdlib::sfield;
    ///
    /// // Type is automatically inferred from the SField constant
    /// let flags = current_ledger_object::get_field(sfield::Flags).unwrap();  // u32
    /// let balance = current_ledger_object::get_field(sfield::Balance).unwrap();  // u64
    /// ```
    #[inline]
    pub fn get_field<T: LedgerObjectFieldGetter, const CODE: i32>(
        _field: SField<T, CODE>,
    ) -> Result<T> {
        T::get_from_current_ledger_obj(CODE)
    }

    /// Retrieves an optionally present field from the current ledger object.
    ///
    /// # Arguments
    ///
    /// * `field` - An SField constant that encodes both the field code and expected type
    ///
    /// # Returns
    ///
    /// Returns a `Result<Option<T>>` where:
    /// * `Ok(Some(T))` - The field value for the specified field
    /// * `Ok(None)` - If the field is not present
    /// * `Err(Error)` - If the field cannot be retrieved or has unexpected size
    #[inline]
    pub fn get_field_optional<T: LedgerObjectFieldGetter, const CODE: i32>(
        _field: SField<T, CODE>,
    ) -> Result<Option<T>> {
        T::get_from_current_ledger_obj_optional(CODE)
    }
}

pub mod ledger_object {
    use super::LedgerObjectFieldGetter;
    use crate::host::Result;
    use crate::sfield::SField;

    /// Retrieves a field from a specified ledger object.
    ///
    /// # Arguments
    ///
    /// * `register_num` - The register number holding the ledger object to look for data in
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
    /// use xrpl_wasm_stdlib::core::ledger_objects::ledger_object;
    /// use xrpl_wasm_stdlib::sfield;
    ///
    /// // Type is automatically inferred from the SField constant
    /// let balance = ledger_object::get_field(0, sfield::Balance).unwrap();  // u64
    /// let account = ledger_object::get_field(0, sfield::Account).unwrap();  // AccountID
    /// ```
    #[inline]
    pub fn get_field<T: LedgerObjectFieldGetter, const CODE: i32>(
        register_num: i32,
        _field: SField<T, CODE>,
    ) -> Result<T> {
        T::get_from_ledger_obj(register_num, CODE)
    }

    /// Retrieves an optionally present field from a specified ledger object.
    ///
    /// # Arguments
    ///
    /// * `register_num` - The register number holding the ledger object to look for data in
    /// * `field` - An SField constant that encodes both the field code and expected type
    ///
    /// # Returns
    ///
    /// Returns a `Result<Option<T>>` where:
    /// * `Ok(Some(T))` - The field value for the specified field
    /// * `Ok(None)` - If the field is not present in the ledger object
    /// * `Err(Error)` - If the field retrieval operation failed
    #[inline]
    pub fn get_field_optional<T: LedgerObjectFieldGetter, const CODE: i32>(
        register_num: i32,
        _field: SField<T, CODE>,
    ) -> Result<Option<T>> {
        T::get_from_ledger_obj_optional(register_num, CODE)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::core::ledger_objects::{current_ledger_object, ledger_object};
        use crate::core::types::account_id::{ACCOUNT_ID_SIZE, AccountID};
        use crate::core::types::amount::{AMOUNT_SIZE, Amount};
        use crate::core::types::blob::{Blob, DEFAULT_BLOB_SIZE};
        use crate::core::types::currency::{CURRENCY_SIZE, Currency};
        use crate::core::types::issue::Issue;
        use crate::core::types::public_key::PUBLIC_KEY_BUFFER_SIZE;
        use crate::core::types::uint::{
            HASH128_SIZE, HASH160_SIZE, HASH192_SIZE, HASH256_SIZE, Hash128, Hash160, Hash192,
            Hash256,
        };
        use crate::host::host_bindings_trait::MockHostBindings;
        use crate::host::setup_mock;
        use crate::sfield;
        use mockall::predicate::{always, eq};

        // ========================================
        // Test helper functions
        // ========================================

        /// Helper to set up a mock expectation for get_current_ledger_obj_field.
        ///
        /// Zero-fills the output buffer before returning. This is required because
        /// `get_variable_size_field` and the fixed-size getters allocate the buffer
        /// via `MaybeUninit` and call `assume_init` after the host call returns —
        /// leaving the buffer uninitialized would be UB.
        fn expect_current_field(
            mock: &mut MockHostBindings,
            field_code: i32,
            size: usize,
            times: usize,
        ) {
            mock.expect_get_current_ledger_obj_field()
                .with(eq(field_code), always(), eq(size))
                .times(times)
                .returning(move |_, buf, buf_size| {
                    unsafe { core::ptr::write_bytes(buf, 0, buf_size) };
                    size as i32
                });
        }

        /// Helper to set up a mock expectation for get_ledger_obj_field.
        /// Zero-fills the output buffer; see `expect_current_field` for rationale.
        fn expect_ledger_field(
            mock: &mut MockHostBindings,
            slot: i32,
            field_code: i32,
            size: usize,
            times: usize,
        ) {
            mock.expect_get_ledger_obj_field()
                .with(eq(slot), eq(field_code), always(), eq(size))
                .times(times)
                .returning(move |_, _, buf, buf_size| {
                    unsafe { core::ptr::write_bytes(buf, 0, buf_size) };
                    size as i32
                });
        }

        /// Like `expect_current_field`, but the host writes fewer bytes than the
        /// buffer holds — used for variable-size fields (e.g. `Issue` uses a 40-byte
        /// buffer but returns 20 bytes for the XRP variant).
        fn expect_current_field_short(
            mock: &mut MockHostBindings,
            field_code: i32,
            buf_size: usize,
            returned: i32,
        ) {
            mock.expect_get_current_ledger_obj_field()
                .with(eq(field_code), always(), eq(buf_size))
                .times(1)
                .returning(move |_, _, _| returned);
        }

        /// Like `expect_ledger_field`, but the host writes fewer bytes than the
        /// buffer holds. See `expect_current_field_short`.
        fn expect_ledger_field_short(
            mock: &mut MockHostBindings,
            slot: i32,
            field_code: i32,
            buf_size: usize,
            returned: i32,
        ) {
            mock.expect_get_ledger_obj_field()
                .with(eq(slot), eq(field_code), always(), eq(buf_size))
                .times(1)
                .returning(move |_, _, _, _| returned);
        }

        // ========================================
        // Basic smoke tests for LedgerObjectFieldGetter implementations
        // These tests verify that the trait implementations compile and work with the test host.
        // Note: The test host returns buffer_len as success, so these only verify basic functionality.
        // ========================================

        #[test]
        fn test_field_getter_basic_types() {
            let mut mock = MockHostBindings::new();

            expect_current_field(&mut mock, sfield::LedgerEntryType.into(), 2, 1);
            expect_current_field(&mut mock, sfield::Flags.into(), 4, 1);
            expect_current_field(&mut mock, sfield::Balance.into(), 8, 1);

            let _guard = setup_mock(mock);

            // Test that all basic integer types work
            assert!(u16::get_from_current_ledger_obj(sfield::LedgerEntryType.into()).is_ok());
            assert!(u32::get_from_current_ledger_obj(sfield::Flags.into()).is_ok());
            assert!(u64::get_from_current_ledger_obj(sfield::Balance.into()).is_ok());
        }

        #[test]
        fn test_field_getter_xrpl_types() {
            let mut mock = MockHostBindings::new();

            expect_current_field(&mut mock, sfield::Account.into(), ACCOUNT_ID_SIZE, 1);
            expect_current_field(&mut mock, sfield::Amount.into(), AMOUNT_SIZE, 1);
            expect_current_field(&mut mock, sfield::EmailHash.into(), HASH128_SIZE, 1);
            expect_current_field(&mut mock, sfield::PreviousTxnID.into(), HASH256_SIZE, 1);
            expect_current_field(&mut mock, sfield::PublicKey.into(), DEFAULT_BLOB_SIZE, 1);
            expect_current_field(&mut mock, sfield::TakerPaysCurrency.into(), HASH160_SIZE, 1);
            expect_current_field(&mut mock, sfield::MPTokenIssuanceID.into(), HASH192_SIZE, 1);
            expect_current_field(&mut mock, sfield::BaseAsset.into(), CURRENCY_SIZE, 1);
            expect_current_field_short(&mut mock, sfield::Asset.into(), 40, 20);

            let _guard = setup_mock(mock);

            // Test that XRPL-specific types work
            assert!(AccountID::get_from_current_ledger_obj(sfield::Account.into()).is_ok());
            assert!(Amount::get_from_current_ledger_obj(sfield::Amount.into()).is_ok());
            assert!(Hash128::get_from_current_ledger_obj(sfield::EmailHash.into()).is_ok());
            assert!(Hash256::get_from_current_ledger_obj(sfield::PreviousTxnID.into()).is_ok());
            assert!(Hash160::get_from_current_ledger_obj(sfield::TakerPaysCurrency.into()).is_ok());
            assert!(Hash192::get_from_current_ledger_obj(sfield::MPTokenIssuanceID.into()).is_ok());
            assert!(Currency::get_from_current_ledger_obj(sfield::BaseAsset.into()).is_ok());
            assert!(Issue::get_from_current_ledger_obj(sfield::Asset.into()).is_ok());

            let blob: Blob<DEFAULT_BLOB_SIZE> =
                Blob::get_from_current_ledger_obj(sfield::PublicKey.into()).unwrap();
            // The test host returns buffer length as the result
            assert_eq!(blob.len, DEFAULT_BLOB_SIZE);
        }

        #[test]
        fn test_field_getter_optional_variants() {
            let mut mock = MockHostBindings::new();

            expect_current_field(&mut mock, sfield::Flags.into(), 4, 1);
            expect_current_field(&mut mock, sfield::Account.into(), ACCOUNT_ID_SIZE, 1);
            expect_current_field(&mut mock, sfield::TakerPaysCurrency.into(), HASH160_SIZE, 1);
            expect_current_field(&mut mock, sfield::MPTokenIssuanceID.into(), HASH192_SIZE, 1);
            expect_current_field(&mut mock, sfield::Amount.into(), AMOUNT_SIZE, 1);
            expect_current_field(&mut mock, sfield::BaseAsset.into(), CURRENCY_SIZE, 1);
            expect_current_field(&mut mock, sfield::EmailHash.into(), HASH128_SIZE, 1);
            expect_current_field(&mut mock, sfield::PreviousTxnID.into(), HASH256_SIZE, 1);
            expect_current_field(&mut mock, sfield::PublicKey.into(), DEFAULT_BLOB_SIZE, 1);
            expect_current_field_short(&mut mock, sfield::Asset.into(), 40, 20);

            let _guard = setup_mock(mock);

            // Test optional field retrieval
            let result = u32::get_from_current_ledger_obj_optional(sfield::Flags.into());
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());

            let result = AccountID::get_from_current_ledger_obj_optional(sfield::Account.into());
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());

            let result =
                Hash160::get_from_current_ledger_obj_optional(sfield::TakerPaysCurrency.into());
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());

            let result =
                Hash192::get_from_current_ledger_obj_optional(sfield::MPTokenIssuanceID.into());
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());

            let result = Amount::get_from_current_ledger_obj_optional(sfield::Amount.into());
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());

            let result = Currency::get_from_current_ledger_obj_optional(sfield::BaseAsset.into());
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());

            let result = Hash128::get_from_current_ledger_obj_optional(sfield::EmailHash.into());
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());

            let result =
                Hash256::get_from_current_ledger_obj_optional(sfield::PreviousTxnID.into());
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());

            let result = Blob::<DEFAULT_BLOB_SIZE>::get_from_current_ledger_obj_optional(
                sfield::PublicKey.into(),
            );
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());

            let result = Issue::get_from_current_ledger_obj_optional(sfield::Asset.into());
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());
        }

        #[test]
        fn test_field_getter_with_slot() {
            let mut mock = MockHostBindings::new();
            let slot = 0;

            expect_ledger_field(&mut mock, slot, sfield::Flags.into(), 4, 1);
            expect_ledger_field(&mut mock, slot, sfield::Balance.into(), 8, 1);
            expect_ledger_field(&mut mock, slot, sfield::Account.into(), ACCOUNT_ID_SIZE, 1);
            expect_ledger_field(
                &mut mock,
                slot,
                sfield::TakerPaysCurrency.into(),
                HASH160_SIZE,
                1,
            );
            expect_ledger_field(
                &mut mock,
                slot,
                sfield::MPTokenIssuanceID.into(),
                HASH192_SIZE,
                1,
            );
            expect_ledger_field(&mut mock, slot, sfield::Amount.into(), AMOUNT_SIZE, 1);
            expect_ledger_field(&mut mock, slot, sfield::BaseAsset.into(), CURRENCY_SIZE, 1);
            expect_ledger_field(&mut mock, slot, sfield::EmailHash.into(), HASH128_SIZE, 1);
            expect_ledger_field(
                &mut mock,
                slot,
                sfield::PreviousTxnID.into(),
                HASH256_SIZE,
                1,
            );
            expect_ledger_field(
                &mut mock,
                slot,
                sfield::PublicKey.into(),
                DEFAULT_BLOB_SIZE,
                1,
            );
            expect_ledger_field_short(&mut mock, slot, sfield::Asset.into(), 40, 20);

            let _guard = setup_mock(mock);

            // Test ledger object field retrieval with slot numbers
            assert!(u32::get_from_ledger_obj(slot, sfield::Flags.into()).is_ok());
            assert!(u64::get_from_ledger_obj(slot, sfield::Balance.into()).is_ok());
            assert!(AccountID::get_from_ledger_obj(slot, sfield::Account.into()).is_ok());
            assert!(Hash160::get_from_ledger_obj(slot, sfield::TakerPaysCurrency.into()).is_ok());
            assert!(Hash192::get_from_ledger_obj(slot, sfield::MPTokenIssuanceID.into()).is_ok());
            assert!(Amount::get_from_ledger_obj(slot, sfield::Amount.into()).is_ok());
            assert!(Currency::get_from_ledger_obj(slot, sfield::BaseAsset.into()).is_ok());
            assert!(Hash128::get_from_ledger_obj(slot, sfield::EmailHash.into()).is_ok());
            assert!(Hash256::get_from_ledger_obj(slot, sfield::PreviousTxnID.into()).is_ok());
            assert!(
                Blob::<DEFAULT_BLOB_SIZE>::get_from_ledger_obj(slot, sfield::PublicKey.into())
                    .is_ok()
            );
            assert!(Issue::get_from_ledger_obj(slot, sfield::Asset.into()).is_ok());
        }

        #[test]
        fn test_field_getter_optional_with_slot() {
            let mut mock = MockHostBindings::new();
            let slot = 0;

            expect_ledger_field(&mut mock, slot, sfield::Flags.into(), 4, 1);
            expect_ledger_field(
                &mut mock,
                slot,
                sfield::TakerPaysCurrency.into(),
                HASH160_SIZE,
                1,
            );
            expect_ledger_field(
                &mut mock,
                slot,
                sfield::MPTokenIssuanceID.into(),
                HASH192_SIZE,
                1,
            );
            expect_ledger_field(&mut mock, slot, sfield::Amount.into(), AMOUNT_SIZE, 1);
            expect_ledger_field(&mut mock, slot, sfield::BaseAsset.into(), CURRENCY_SIZE, 1);
            expect_ledger_field(&mut mock, slot, sfield::EmailHash.into(), HASH128_SIZE, 1);
            expect_ledger_field(
                &mut mock,
                slot,
                sfield::PreviousTxnID.into(),
                HASH256_SIZE,
                1,
            );
            expect_ledger_field(
                &mut mock,
                slot,
                sfield::PublicKey.into(),
                DEFAULT_BLOB_SIZE,
                1,
            );
            expect_ledger_field_short(&mut mock, slot, sfield::Asset.into(), 40, 20);

            let _guard = setup_mock(mock);

            // Test optional field retrieval with slot numbers
            let result = u32::get_from_ledger_obj_optional(slot, sfield::Flags.into());
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());

            let result =
                Hash160::get_from_ledger_obj_optional(slot, sfield::TakerPaysCurrency.into());
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());

            let result =
                Hash192::get_from_ledger_obj_optional(slot, sfield::MPTokenIssuanceID.into());
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());
            let result = Amount::get_from_ledger_obj_optional(slot, sfield::Amount.into());
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());

            let result = Currency::get_from_ledger_obj_optional(slot, sfield::BaseAsset.into());
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());

            let result = Hash128::get_from_ledger_obj_optional(slot, sfield::EmailHash.into());
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());

            let result = Hash256::get_from_ledger_obj_optional(slot, sfield::PreviousTxnID.into());
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());

            let result = Blob::<DEFAULT_BLOB_SIZE>::get_from_ledger_obj_optional(
                slot,
                sfield::PublicKey.into(),
            );
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());

            let result = Issue::get_from_ledger_obj_optional(slot, sfield::Asset.into());
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());
        }

        // ========================================
        // Value-level tests: verify Issue variant detection by populating
        // the mock buffer with known bytes (not just checking `is_ok()`).
        // ========================================

        #[test]
        fn test_issue_decodes_xrp_variant() {
            let mut mock = MockHostBindings::new();
            mock.expect_get_current_ledger_obj_field()
                .with(eq::<i32>(sfield::Asset.into()), always(), eq(40))
                .times(1)
                .returning(|_, buf, _| {
                    // 20 zero bytes → XRP
                    unsafe { core::ptr::write_bytes(buf, 0, 20) };
                    20
                });

            let _guard = setup_mock(mock);

            let issue = Issue::get_from_current_ledger_obj(sfield::Asset.into()).unwrap();
            assert!(matches!(issue, Issue::XRP(_)));
        }

        #[test]
        fn test_issue_decodes_mpt_variant() {
            let mut mock = MockHostBindings::new();
            mock.expect_get_current_ledger_obj_field()
                .with(eq::<i32>(sfield::Asset.into()), always(), eq(40))
                .times(1)
                .returning(|_, buf, _| {
                    // 4 bytes seq=42 (big-endian) + 20 bytes issuer=0xAB → MPT
                    let slice = unsafe { core::slice::from_raw_parts_mut(buf, 24) };
                    slice[0..4].copy_from_slice(&42u32.to_be_bytes());
                    slice[4..24].fill(0xAB);
                    24
                });

            let _guard = setup_mock(mock);

            let issue = Issue::get_from_current_ledger_obj(sfield::Asset.into()).unwrap();
            match issue {
                Issue::MPT(mpt) => {
                    assert_eq!(mpt.mpt_id().get_sequence_num(), 42);
                    assert_eq!(mpt.mpt_id().get_issuer(), AccountID::from([0xAB; 20]));
                }
                _ => panic!("expected MPT variant"),
            }
        }

        #[test]
        fn test_issue_decodes_iou_variant() {
            let mut mock = MockHostBindings::new();
            mock.expect_get_current_ledger_obj_field()
                .with(eq::<i32>(sfield::Asset.into()), always(), eq(40))
                .times(1)
                .returning(|_, buf, _| {
                    // 20 bytes currency=0xCC + 20 bytes issuer=0xDD → IOU
                    let slice = unsafe { core::slice::from_raw_parts_mut(buf, 40) };
                    slice[0..20].fill(0xCC);
                    slice[20..40].fill(0xDD);
                    40
                });

            let _guard = setup_mock(mock);

            let issue = Issue::get_from_current_ledger_obj(sfield::Asset.into()).unwrap();
            match issue {
                Issue::IOU(iou) => {
                    let bytes = iou.as_bytes();
                    assert_eq!(&bytes[..20], &[0xCC; 20]);
                    assert_eq!(&bytes[20..], &[0xDD; 20]);
                }
                _ => panic!("expected IOU variant"),
            }
        }

        // ========================================
        // Value-level tests: verify Amount variant detection by populating
        // the mock buffer with known flag bits + payload (not just `is_ok()`).
        // The 48-byte buffer is always passed to `Amount::from_bytes` in full,
        // so the mock must initialize all 48 bytes (MaybeUninit) even though
        // only a prefix is meaningful per variant.
        // ========================================

        #[test]
        fn test_amount_decodes_xrp_variant() {
            let mut mock = MockHostBindings::new();
            mock.expect_get_current_ledger_obj_field()
                .with(eq::<i32>(sfield::Amount.into()), always(), eq(48))
                .times(1)
                .returning(|_, buf, size| {
                    // XRP positive 1000 drops: byte0 = 0x40 (positive bit, XRP type),
                    // remaining 7 bytes hold the drop amount big-endian.
                    let slice = unsafe { core::slice::from_raw_parts_mut(buf, size) };
                    slice.fill(0);
                    let mut be = 1000u64.to_be_bytes();
                    be[0] |= 0x40; // set positive flag in top bits
                    slice[0..8].copy_from_slice(&be);
                    8
                });

            let _guard = setup_mock(mock);

            let amount = Amount::get_from_current_ledger_obj(sfield::Amount.into()).unwrap();
            assert!(matches!(amount, Amount::XRP { num_drops: 1000 }));
        }

        #[test]
        fn test_amount_decodes_mpt_variant() {
            let mut mock = MockHostBindings::new();
            mock.expect_get_current_ledger_obj_field()
                .with(eq::<i32>(sfield::Amount.into()), always(), eq(48))
                .times(1)
                .returning(|_, buf, size| {
                    // MPT positive: byte0 bit7=0 (not IOU), bit6=1 (positive), bit5=1 (MPT)
                    // bytes[1..9]  = num_units big-endian
                    // bytes[9..33] = MptId (4-byte seq + 20-byte issuer)
                    let slice = unsafe { core::slice::from_raw_parts_mut(buf, size) };
                    slice.fill(0);
                    slice[0] = 0x60;
                    slice[1..9].copy_from_slice(&100u64.to_be_bytes());
                    slice[9..13].copy_from_slice(&7u32.to_be_bytes());
                    slice[13..33].fill(0xAB);
                    33
                });

            let _guard = setup_mock(mock);

            let amount = Amount::get_from_current_ledger_obj(sfield::Amount.into()).unwrap();
            match amount {
                Amount::MPT {
                    num_units,
                    is_positive,
                    mpt_id,
                } => {
                    assert_eq!(num_units, 100);
                    assert!(is_positive);
                    assert_eq!(mpt_id.get_sequence_num(), 7);
                    assert_eq!(mpt_id.get_issuer(), AccountID::from([0xAB; 20]));
                }
                _ => panic!("expected MPT variant"),
            }
        }

        #[test]
        fn test_amount_decodes_iou_variant() {
            let mut mock = MockHostBindings::new();
            mock.expect_get_current_ledger_obj_field()
                .with(eq::<i32>(sfield::Amount.into()), always(), eq(48))
                .times(1)
                .returning(|_, buf, size| {
                    // IOU: byte0 bit7=1; bytes[0..8]=XFloat (opaque, content
                    // doesn't matter for variant detection), bytes[8..28]=currency,
                    // bytes[28..48]=issuer.
                    let slice = unsafe { core::slice::from_raw_parts_mut(buf, size) };
                    slice.fill(0);
                    slice[0] = 0x80;
                    slice[8..28].fill(0xCC);
                    slice[28..48].fill(0xDD);
                    48
                });
            mock.expect_float_from_stamount().times(1).returning(
                |_, _, out_buff, out_buff_len, _| {
                    unsafe { out_buff.copy_from_nonoverlapping([0u8; 12].as_ptr(), 12) }
                    out_buff_len as i32
                },
            );

            let _guard = setup_mock(mock);

            let amount = Amount::get_from_current_ledger_obj(sfield::Amount.into()).unwrap();
            match amount {
                Amount::IOU {
                    issuer, currency, ..
                } => {
                    assert_eq!(issuer, AccountID::from([0xDD; 20]));
                    assert_eq!(currency, Currency::from([0xCC; 20]));
                }
                _ => panic!("expected IOU variant"),
            }
        }

        // ========================================
        // Tests for module-level convenience functions
        // ========================================

        #[test]
        fn test_current_ledger_object_module() {
            let mut mock = MockHostBindings::new();

            expect_current_field(&mut mock, sfield::Flags.into(), 4, 2);
            expect_current_field(&mut mock, sfield::Account.into(), ACCOUNT_ID_SIZE, 1);

            let _guard = setup_mock(mock);

            // Test the current_ledger_object module's convenience functions
            assert!(current_ledger_object::get_field(sfield::Flags).is_ok());
            assert!(current_ledger_object::get_field(sfield::Account).is_ok());

            let result = current_ledger_object::get_field_optional(sfield::Flags);
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());
        }

        #[test]
        fn test_ledger_object_module() {
            let mut mock = MockHostBindings::new();
            let slot = 0;

            expect_ledger_field(&mut mock, slot, sfield::LedgerEntryType.into(), 2, 1);
            expect_ledger_field(&mut mock, slot, sfield::Flags.into(), 4, 2);
            expect_ledger_field(&mut mock, slot, sfield::Balance.into(), AMOUNT_SIZE, 1);
            expect_ledger_field(&mut mock, slot, sfield::Account.into(), ACCOUNT_ID_SIZE, 1);
            expect_ledger_field(&mut mock, slot, sfield::Amount.into(), AMOUNT_SIZE, 1);
            expect_ledger_field(&mut mock, slot, sfield::EmailHash.into(), HASH128_SIZE, 1);
            expect_ledger_field(
                &mut mock,
                slot,
                sfield::PreviousTxnID.into(),
                HASH256_SIZE,
                1,
            );
            expect_ledger_field(
                &mut mock,
                slot,
                sfield::PublicKey.into(),
                DEFAULT_BLOB_SIZE,
                1,
            );

            let _guard = setup_mock(mock);

            // Test the ledger_object module's convenience functions
            assert!(ledger_object::get_field(slot, sfield::LedgerEntryType).is_ok());
            assert!(ledger_object::get_field(slot, sfield::Flags).is_ok());
            assert!(ledger_object::get_field(slot, sfield::Balance).is_ok());
            assert!(ledger_object::get_field(slot, sfield::Account).is_ok());
            assert!(ledger_object::get_field(slot, sfield::Amount).is_ok());
            assert!(ledger_object::get_field(slot, sfield::EmailHash).is_ok());
            assert!(ledger_object::get_field(slot, sfield::PreviousTxnID).is_ok());
            assert!(ledger_object::get_field(slot, sfield::PublicKey).is_ok());

            let result = ledger_object::get_field_optional(slot, sfield::Flags);
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());
        }

        // ========================================
        // Type inference and compilation tests
        // ========================================

        #[test]
        fn test_type_inference() {
            let mut mock = MockHostBindings::new();
            let slot = 0;

            expect_ledger_field(&mut mock, slot, sfield::Balance.into(), AMOUNT_SIZE, 1);
            expect_ledger_field(&mut mock, slot, sfield::Account.into(), ACCOUNT_ID_SIZE, 1);
            expect_ledger_field(&mut mock, slot, sfield::Sequence.into(), 4, 1);
            expect_ledger_field(&mut mock, slot, sfield::Flags.into(), 4, 1);

            let _guard = setup_mock(mock);

            // Verify type inference works with turbofish syntax
            let _balance = get_field(slot, sfield::Balance);
            let _account = get_field(slot, sfield::Account);

            // Verify type inference works with type annotations
            let _sequence: Result<u32> = get_field(slot, sfield::Sequence);
            let _flags: Result<u32> = get_field(slot, sfield::Flags);
        }

        // ========================================
        // Data size verification tests
        // ========================================

        #[test]
        fn test_type_sizes() {
            let mut mock = MockHostBindings::new();

            expect_current_field(&mut mock, sfield::EmailHash.into(), HASH128_SIZE, 1);
            expect_current_field(&mut mock, sfield::PreviousTxnID.into(), HASH256_SIZE, 1);
            expect_current_field(&mut mock, sfield::Account.into(), ACCOUNT_ID_SIZE, 1);
            expect_current_field(
                &mut mock,
                sfield::PublicKey.into(),
                PUBLIC_KEY_BUFFER_SIZE,
                1,
            );

            let _guard = setup_mock(mock);

            // Verify that returned types have the expected sizes
            let hash128 = Hash128::get_from_current_ledger_obj(sfield::EmailHash.into()).unwrap();
            assert_eq!(hash128.as_bytes().len(), HASH128_SIZE);

            let hash256 =
                Hash256::get_from_current_ledger_obj(sfield::PreviousTxnID.into()).unwrap();
            assert_eq!(hash256.as_bytes().len(), HASH256_SIZE);

            let account = AccountID::get_from_current_ledger_obj(sfield::Account.into()).unwrap();
            assert_eq!(account.0.len(), ACCOUNT_ID_SIZE);

            let blob: Blob<{ PUBLIC_KEY_BUFFER_SIZE }> =
                Blob::get_from_current_ledger_obj(sfield::PublicKey.into()).unwrap();
            // In test environment, host returns buffer size as result code
            assert_eq!(blob.len, PUBLIC_KEY_BUFFER_SIZE);
            assert_eq!(blob.data.len(), PUBLIC_KEY_BUFFER_SIZE);
        }

        // ========================================
        // Array and Object unreachable tests
        // These verify that Array and Object types panic when accessed
        // through the public field access API, as they should only be
        // used for navigation via the Location/Locator system.
        // ========================================

        #[test]
        #[should_panic]
        fn test_array_get_field_panics() {
            let _ = current_ledger_object::get_field(sfield::Signers);
        }

        #[test]
        #[should_panic]
        fn test_array_get_field_optional_panics() {
            let _ = current_ledger_object::get_field_optional(sfield::Signers);
        }

        #[test]
        #[should_panic]
        fn test_array_get_field_with_slot_panics() {
            let _ = ledger_object::get_field(0, sfield::Signers);
        }

        #[test]
        #[should_panic]
        fn test_array_get_field_optional_with_slot_panics() {
            let _ = ledger_object::get_field_optional(0, sfield::Signers);
        }

        #[test]
        #[should_panic]
        fn test_object_get_field_panics() {
            let _ = current_ledger_object::get_field(sfield::Memo);
        }

        #[test]
        #[should_panic]
        fn test_object_get_field_optional_panics() {
            let _ = current_ledger_object::get_field_optional(sfield::Memo);
        }

        #[test]
        #[should_panic]
        fn test_object_get_field_with_slot_panics() {
            let _ = ledger_object::get_field(0, sfield::Memo);
        }

        #[test]
        #[should_panic]
        fn test_object_get_field_optional_with_slot_panics() {
            let _ = ledger_object::get_field_optional(0, sfield::Memo);
        }
    }
}
