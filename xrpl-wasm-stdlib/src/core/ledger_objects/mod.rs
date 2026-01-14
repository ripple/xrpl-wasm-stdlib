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
        use crate::core::types::amount::Amount;
        use crate::core::types::blob::{Blob, DEFAULT_BLOB_SIZE};
        use crate::core::types::public_key::PUBLIC_KEY_BUFFER_SIZE;
        use crate::core::types::uint::{HASH128_SIZE, HASH256_SIZE, Hash128, Hash256};
        use crate::host::host_bindings_trait::MockHostBindings;
        use crate::host::setup_mock;
        use crate::sfield;
        use mockall::predicate::{always, eq};

        // ========================================
        // Test helper functions
        // ========================================

        /// Helper to set up a mock expectation for get_current_ledger_obj_field
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

        /// Helper to set up a mock expectation for get_ledger_obj_field
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
                .returning(move |_, _, _, _| size as i32);
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
            assert!(u16::get_from_current_ledger_obj(65537i32).is_ok());
            assert!(u32::get_from_current_ledger_obj(65538i32).is_ok());
            assert!(u64::get_from_current_ledger_obj(393216i32).is_ok());
        }

        #[test]
        fn test_field_getter_xrpl_types() {
            let mut mock = MockHostBindings::new();

            expect_current_field(&mut mock, sfield::Account.into(), ACCOUNT_ID_SIZE, 1);
            expect_current_field(&mut mock, sfield::Amount.into(), 48, 1);
            expect_current_field(&mut mock, sfield::EmailHash.into(), HASH128_SIZE, 1);
            expect_current_field(&mut mock, sfield::PreviousTxnID.into(), HASH256_SIZE, 1);
            expect_current_field(&mut mock, sfield::PublicKey.into(), DEFAULT_BLOB_SIZE, 1);

            let _guard = setup_mock(mock);

            // Test that XRPL-specific types work
            assert!(AccountID::get_from_current_ledger_obj(524289i32).is_ok());
            assert!(Amount::get_from_current_ledger_obj(393216i32).is_ok());
            assert!(Hash128::get_from_current_ledger_obj(262145i32).is_ok());
            assert!(Hash256::get_from_current_ledger_obj(262146i32).is_ok());

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

            let _guard = setup_mock(mock);

            // Test optional field retrieval
            let result = u32::get_from_current_ledger_obj_optional(65538i32);
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());

            let result = AccountID::get_from_current_ledger_obj_optional(524289i32);
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

            let _guard = setup_mock(mock);

            // Test ledger object field retrieval with slot numbers
            assert!(u32::get_from_ledger_obj(slot, sfield::Flags.into()).is_ok());
            assert!(u64::get_from_ledger_obj(slot, sfield::Balance.into()).is_ok());
            assert!(AccountID::get_from_ledger_obj(slot, sfield::Account.into()).is_ok());
        }

        #[test]
        fn test_field_getter_optional_with_slot() {
            let mut mock = MockHostBindings::new();
            let slot = 0;

            expect_ledger_field(&mut mock, slot, sfield::Flags.into(), 4, 1);

            let _guard = setup_mock(mock);

            // Test optional field retrieval with slot numbers
            let result = u32::get_from_ledger_obj_optional(slot, sfield::Flags.into());
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());
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
            expect_ledger_field(&mut mock, slot, sfield::Balance.into(), 8, 1);
            expect_ledger_field(&mut mock, slot, sfield::Account.into(), ACCOUNT_ID_SIZE, 1);
            expect_ledger_field(&mut mock, slot, sfield::Amount.into(), 48, 1);
            expect_ledger_field(&mut mock, slot, sfield::EmailHash.into(), HASH128_SIZE, 1);
            expect_ledger_field(
                &mut mock,
                slot,
                sfield::PreviousTxnID.into(),
                HASH256_SIZE,
                1,
            );
            expect_ledger_field(&mut mock, slot, sfield::PublicKey.into(), 33, 1);

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

            expect_ledger_field(&mut mock, slot, sfield::Balance.into(), 8, 1);
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
            let hash128 = Hash128::get_from_current_ledger_obj(262145i32).unwrap();
            assert_eq!(hash128.as_bytes().len(), HASH128_SIZE);

            let hash256 = Hash256::get_from_current_ledger_obj(262146i32).unwrap();
            assert_eq!(hash256.as_bytes().len(), HASH256_SIZE);

            let account = AccountID::get_from_current_ledger_obj(524289i32).unwrap();
            assert_eq!(account.0.len(), ACCOUNT_ID_SIZE);

            let blob: Blob<{ PUBLIC_KEY_BUFFER_SIZE }> =
                Blob::get_from_current_ledger_obj(sfield::PublicKey.into()).unwrap();
            // In test environment, host returns buffer size as result code
            assert_eq!(blob.len, PUBLIC_KEY_BUFFER_SIZE);
            assert_eq!(blob.data.len(), PUBLIC_KEY_BUFFER_SIZE);
        }
    }
}
