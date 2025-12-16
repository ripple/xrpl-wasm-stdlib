pub mod account_root;
pub mod current_escrow;
pub mod escrow;
pub mod traits;

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
/// use xrpl_wasm_stdlib::sfield;
///
/// fn example() {
///   let slot = 0;
///   // Get a required field from a specific ledger object
///   let balance: u64 = ledger_object::get_field(slot, sfield::Balance).unwrap();
///   let account: AccountID = ledger_object::get_field(slot, sfield::Account).unwrap();
///
///   // Get an optional field from the current ledger object
///   let flags: Option<u32> = current_ledger_object::get_field_optional(sfield::Flags).unwrap();
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

pub mod current_ledger_object {
    use super::LedgerObjectFieldGetter;
    use crate::host::Result;

    /// Retrieves a field from the current ledger object.
    ///
    /// # Arguments
    ///
    /// * `field_code` - The field code identifying which field to retrieve
    ///
    /// # Returns
    ///
    /// Returns a `Result<T>` where:
    /// * `Ok(T)` - The field value for the specified field
    /// * `Err(Error)` - If the field cannot be retrieved or has unexpected size
    #[inline]
    pub fn get_field<T: LedgerObjectFieldGetter>(field_code: i32) -> Result<T> {
        T::get_from_current_ledger_obj(field_code)
    }

    /// Retrieves an optionally present field from the current ledger object.
    ///
    /// # Arguments
    ///
    /// * `field_code` - The field code identifying which field to retrieve
    ///
    /// # Returns
    ///
    /// Returns a `Result<Option<T>>` where:
    /// * `Ok(Some(T))` - The field value for the specified field
    /// * `Ok(None)` - If the field is not present
    /// * `Err(Error)` - If the field cannot be retrieved or has unexpected size
    #[inline]
    pub fn get_field_optional<T: LedgerObjectFieldGetter>(field_code: i32) -> Result<Option<T>> {
        T::get_from_current_ledger_obj_optional(field_code)
    }
}

pub mod ledger_object {
    use super::LedgerObjectFieldGetter;
    use crate::host::Result;

    /// Retrieves a field from a specified ledger object.
    ///
    /// # Arguments
    ///
    /// * `register_num` - The register number holding the ledger object to look for data in
    /// * `field_code` - The field code identifying which field to retrieve
    ///
    /// # Returns
    ///
    /// Returns a `Result<T>` where:
    /// * `Ok(T)` - The field value for the specified field
    /// * `Err(Error)` - If the field cannot be retrieved or has unexpected size
    #[inline]
    pub fn get_field<T: LedgerObjectFieldGetter>(register_num: i32, field_code: i32) -> Result<T> {
        T::get_from_ledger_obj(register_num, field_code)
    }

    /// Retrieves an optionally present field from a specified ledger object.
    ///
    /// # Arguments
    ///
    /// * `register_num` - The register number holding the ledger object to look for data in
    /// * `field_code` - The field code identifying which field to retrieve
    ///
    /// # Returns
    ///
    /// Returns a `Result<Option<T>>` where:
    /// * `Ok(Some(T))` - The field value for the specified field
    /// * `Ok(None)` - If the field is not present in the ledger object
    /// * `Err(Error)` - If the field retrieval operation failed
    #[inline]
    pub fn get_field_optional<T: LedgerObjectFieldGetter>(
        register_num: i32,
        field_code: i32,
    ) -> Result<Option<T>> {
        T::get_from_ledger_obj_optional(register_num, field_code)
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
        // Basic smoke tests for LedgerObjectFieldGetter implementations
        // These tests verify that the trait implementations compile and work with the test host.
        // Note: The test host returns buffer_len as success, so these only verify basic functionality.
        // ========================================

        #[test]
        fn test_field_getter_basic_types() {
            let mut mock = MockHostBindings::new();

            // Set up expectations for u16
            mock.expect_get_current_ledger_obj_field()
                .with(eq(sfield::LedgerEntryType), always(), eq(2))
                .returning(|_, _, _| 2);

            // Set up expectations for u32
            mock.expect_get_current_ledger_obj_field()
                .with(eq(sfield::Flags), always(), eq(4))
                .returning(|_, _, _| 4);

            // Set up expectations for u64
            mock.expect_get_current_ledger_obj_field()
                .with(eq(sfield::Balance), always(), eq(8))
                .returning(|_, _, _| 8);

            // Set the mock in thread-local storage (automatically cleans up at the end of scope)
            let _guard = setup_mock(mock);

            // Test that all basic integer types work
            assert!(u16::get_from_current_ledger_obj(sfield::LedgerEntryType).is_ok());
            assert!(u32::get_from_current_ledger_obj(sfield::Flags).is_ok());
            assert!(u64::get_from_current_ledger_obj(sfield::Balance).is_ok());
        }

        #[test]
        fn test_field_getter_xrpl_types() {
            let mut mock = MockHostBindings::new();

            // Set up expectations for AccountID (20 bytes)
            mock.expect_get_current_ledger_obj_field()
                .with(eq(sfield::Account), always(), eq(ACCOUNT_ID_SIZE))
                .returning(|_, _, _| ACCOUNT_ID_SIZE as i32);

            // Set up expectations for Amount (48 bytes max)
            mock.expect_get_current_ledger_obj_field()
                .with(eq(sfield::Amount), always(), eq(48))
                .returning(|_, _, _| 48);

            // Set up expectations for Hash128 (16 bytes)
            mock.expect_get_current_ledger_obj_field()
                .with(eq(sfield::EmailHash), always(), eq(HASH128_SIZE))
                .returning(|_, _, _| HASH128_SIZE as i32);

            // Set up expectations for Hash256 (32 bytes)
            mock.expect_get_current_ledger_obj_field()
                .with(eq(sfield::PreviousTxnID), always(), eq(HASH256_SIZE))
                .returning(|_, _, _| HASH256_SIZE as i32);

            // Set up expectations for Blob
            mock.expect_get_current_ledger_obj_field()
                .with(eq(sfield::PublicKey), always(), eq(DEFAULT_BLOB_SIZE))
                .returning(|_, _, _| DEFAULT_BLOB_SIZE as i32);

            // Set the mock in thread-local storage (automatically cleans up at the end of scope)
            let _guard = setup_mock(mock);

            // Test that XRPL-specific types work
            assert!(AccountID::get_from_current_ledger_obj(sfield::Account).is_ok());
            assert!(Amount::get_from_current_ledger_obj(sfield::Amount).is_ok());
            assert!(Hash128::get_from_current_ledger_obj(sfield::EmailHash).is_ok());
            assert!(Hash256::get_from_current_ledger_obj(sfield::PreviousTxnID).is_ok());

            let blob: Blob<DEFAULT_BLOB_SIZE> =
                Blob::get_from_current_ledger_obj(sfield::PublicKey).unwrap();
            // The test host returns buffer length as the result
            assert_eq!(blob.len, DEFAULT_BLOB_SIZE);
        }

        #[test]
        fn test_field_getter_optional_variants() {
            let mut mock = MockHostBindings::new();

            // Set up expectations for u32 Flags
            mock.expect_get_current_ledger_obj_field()
                .with(eq(sfield::Flags), always(), eq(4))
                .returning(|_, _, _| 4);

            // Set up expectations for AccountID
            mock.expect_get_current_ledger_obj_field()
                .with(eq(sfield::Account), always(), eq(ACCOUNT_ID_SIZE))
                .returning(|_, _, _| ACCOUNT_ID_SIZE as i32);

            // Set the mock in thread-local storage (automatically cleans up at the end of scope)
            let _guard = setup_mock(mock);

            // Test optional field retrieval
            let result = u32::get_from_current_ledger_obj_optional(sfield::Flags);
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());

            let result = AccountID::get_from_current_ledger_obj_optional(sfield::Account);
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());
        }

        #[test]
        fn test_field_getter_with_slot() {
            let mut mock = MockHostBindings::new();
            let slot = 0;

            // Set up expectations for u32 Flags
            mock.expect_get_ledger_obj_field()
                .with(eq(slot), eq(sfield::Flags), always(), eq(4))
                .returning(|_, _, _, _| 4);

            // Set up expectations for u64 Balance
            mock.expect_get_ledger_obj_field()
                .with(eq(slot), eq(sfield::Balance), always(), eq(8))
                .returning(|_, _, _, _| 8);

            // Set up expectations for AccountID
            mock.expect_get_ledger_obj_field()
                .with(eq(slot), eq(sfield::Account), always(), eq(ACCOUNT_ID_SIZE))
                .returning(|_, _, _, _| ACCOUNT_ID_SIZE as i32);

            // Set the mock in thread-local storage (automatically cleans up at the end of scope)
            let _guard = setup_mock(mock);

            // Test ledger object field retrieval with slot numbers
            assert!(u32::get_from_ledger_obj(slot, sfield::Flags).is_ok());
            assert!(u64::get_from_ledger_obj(slot, sfield::Balance).is_ok());
            assert!(AccountID::get_from_ledger_obj(slot, sfield::Account).is_ok());
        }

        #[test]
        fn test_field_getter_optional_with_slot() {
            let mut mock = MockHostBindings::new();
            let slot = 0;

            // Set up expectations for u32 Flags
            mock.expect_get_ledger_obj_field()
                .with(eq(slot), eq(sfield::Flags), always(), eq(4))
                .returning(|_, _, _, _| 4);

            // Set the mock in thread-local storage (automatically cleans up at the end of scope)
            let _guard = setup_mock(mock);

            // Test optional field retrieval with slot numbers
            let result = u32::get_from_ledger_obj_optional(slot, sfield::Flags);
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());
        }

        // ========================================
        // Tests for module-level convenience functions
        // ========================================

        #[test]
        fn test_current_ledger_object_module() {
            let mut mock = MockHostBindings::new();

            // Set up expectations for u32 Flags (called twice - once for required, once for optional)
            mock.expect_get_current_ledger_obj_field()
                .with(eq(sfield::Flags), always(), eq(4))
                .times(2)
                .returning(|_, _, _| 4);

            // Set up expectations for AccountID
            mock.expect_get_current_ledger_obj_field()
                .with(eq(sfield::Account), always(), eq(ACCOUNT_ID_SIZE))
                .returning(|_, _, _| ACCOUNT_ID_SIZE as i32);

            // Set the mock in thread-local storage (automatically cleans up at the end of scope)
            let _guard = setup_mock(mock);

            // Test the current_ledger_object module's convenience functions
            assert!(current_ledger_object::get_field::<u32>(sfield::Flags).is_ok());
            assert!(current_ledger_object::get_field::<AccountID>(sfield::Account).is_ok());

            let result = current_ledger_object::get_field_optional::<u32>(sfield::Flags);
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());
        }

        #[test]
        fn test_ledger_object_module() {
            let mut mock = MockHostBindings::new();
            let slot = 0;

            // Set up expectations for u16 LedgerEntryType
            mock.expect_get_ledger_obj_field()
                .with(eq(slot), eq(sfield::LedgerEntryType), always(), eq(2))
                .returning(|_, _, _, _| 2);

            // Set up expectations for u32 Flags (called twice - once for required, once for optional)
            mock.expect_get_ledger_obj_field()
                .with(eq(slot), eq(sfield::Flags), always(), eq(4))
                .times(2)
                .returning(|_, _, _, _| 4);

            // Set up expectations for u64 Balance
            mock.expect_get_ledger_obj_field()
                .with(eq(slot), eq(sfield::Balance), always(), eq(8))
                .returning(|_, _, _, _| 8);

            // Set up expectations for AccountID
            mock.expect_get_ledger_obj_field()
                .with(eq(slot), eq(sfield::Account), always(), eq(ACCOUNT_ID_SIZE))
                .returning(|_, _, _, _| ACCOUNT_ID_SIZE as i32);

            // Set up expectations for Amount
            mock.expect_get_ledger_obj_field()
                .with(eq(slot), eq(sfield::Amount), always(), eq(48))
                .returning(|_, _, _, _| 48);

            // Set up expectations for Hash128
            mock.expect_get_ledger_obj_field()
                .with(eq(slot), eq(sfield::EmailHash), always(), eq(HASH128_SIZE))
                .returning(|_, _, _, _| HASH128_SIZE as i32);

            // Set up expectations for Hash256
            mock.expect_get_ledger_obj_field()
                .with(
                    eq(slot),
                    eq(sfield::PreviousTxnID),
                    always(),
                    eq(HASH256_SIZE),
                )
                .returning(|_, _, _, _| HASH256_SIZE as i32);

            // Set up expectations for Blob<33>
            mock.expect_get_ledger_obj_field()
                .with(eq(slot), eq(sfield::PublicKey), always(), eq(33))
                .returning(|_, _, _, _| 33);

            // Set the mock in thread-local storage (automatically cleans up at the end of scope)
            let _guard = setup_mock(mock);

            // Test the ledger_object module's convenience functions
            assert!(ledger_object::get_field::<u16>(slot, sfield::LedgerEntryType).is_ok());
            assert!(ledger_object::get_field::<u32>(slot, sfield::Flags).is_ok());
            assert!(ledger_object::get_field::<u64>(slot, sfield::Balance).is_ok());
            assert!(ledger_object::get_field::<AccountID>(slot, sfield::Account).is_ok());
            assert!(ledger_object::get_field::<Amount>(slot, sfield::Amount).is_ok());
            assert!(ledger_object::get_field::<Hash128>(slot, sfield::EmailHash).is_ok());
            assert!(ledger_object::get_field::<Hash256>(slot, sfield::PreviousTxnID).is_ok());
            assert!(ledger_object::get_field::<Blob<33>>(slot, sfield::PublicKey).is_ok());

            let result = ledger_object::get_field_optional::<u32>(slot, sfield::Flags);
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

            // Set up expectations for u64 Balance
            mock.expect_get_ledger_obj_field()
                .with(eq(slot), eq(sfield::Balance), always(), eq(8))
                .returning(|_, _, _, _| 8);

            // Set up expectations for AccountID
            mock.expect_get_ledger_obj_field()
                .with(eq(slot), eq(sfield::Account), always(), eq(ACCOUNT_ID_SIZE))
                .returning(|_, _, _, _| ACCOUNT_ID_SIZE as i32);

            // Set up expectations for u32 Sequence
            mock.expect_get_ledger_obj_field()
                .with(eq(slot), eq(sfield::Sequence), always(), eq(4))
                .returning(|_, _, _, _| 4);

            // Set up expectations for u32 Flags
            mock.expect_get_ledger_obj_field()
                .with(eq(slot), eq(sfield::Flags), always(), eq(4))
                .returning(|_, _, _, _| 4);

            // Set the mock in thread-local storage (automatically cleans up at the end of scope)
            let _guard = setup_mock(mock);

            // Verify type inference works with turbofish syntax
            let _balance = get_field::<u64>(slot, sfield::Balance);
            let _account = get_field::<AccountID>(slot, sfield::Account);

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

            // Set up expectations for Hash128
            mock.expect_get_current_ledger_obj_field()
                .with(eq(sfield::EmailHash), always(), eq(HASH128_SIZE))
                .returning(|_, _, _| HASH128_SIZE as i32);

            // Set up expectations for Hash256
            mock.expect_get_current_ledger_obj_field()
                .with(eq(sfield::PreviousTxnID), always(), eq(HASH256_SIZE))
                .returning(|_, _, _| HASH256_SIZE as i32);

            // Set up expectations for AccountID
            mock.expect_get_current_ledger_obj_field()
                .with(eq(sfield::Account), always(), eq(ACCOUNT_ID_SIZE))
                .returning(|_, _, _| ACCOUNT_ID_SIZE as i32);

            // Set up expectations for Blob
            mock.expect_get_current_ledger_obj_field()
                .with(eq(sfield::PublicKey), always(), eq(PUBLIC_KEY_BUFFER_SIZE))
                .returning(|_, _, _| PUBLIC_KEY_BUFFER_SIZE as i32);

            // Set the mock in thread-local storage (automatically cleans up at the end of scope)
            let _guard = setup_mock(mock);

            // Verify that returned types have the expected sizes
            let hash128 = Hash128::get_from_current_ledger_obj(sfield::EmailHash).unwrap();
            assert_eq!(hash128.as_bytes().len(), HASH128_SIZE);

            let hash256 = Hash256::get_from_current_ledger_obj(sfield::PreviousTxnID).unwrap();
            assert_eq!(hash256.as_bytes().len(), HASH256_SIZE);

            let account = AccountID::get_from_current_ledger_obj(sfield::Account).unwrap();
            assert_eq!(account.0.len(), ACCOUNT_ID_SIZE);

            let blob: Blob<{ PUBLIC_KEY_BUFFER_SIZE }> =
                Blob::get_from_current_ledger_obj(sfield::PublicKey).unwrap();
            // In test environment, host returns buffer size as result code
            assert_eq!(blob.len, PUBLIC_KEY_BUFFER_SIZE);
            assert_eq!(blob.data.len(), PUBLIC_KEY_BUFFER_SIZE);
        }
    }
}
