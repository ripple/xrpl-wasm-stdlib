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
use crate::sfield::SField;

/// Trait for types that can be retrieved from ledger object fields.
///
/// This trait provides a unified interface for retrieving typed data from XRPL ledger objects.
///
/// ## Error Handling
///
/// - Required field methods return `Result<T>` and error if the field is missing.
/// - Optional field methods return `Result<Option<T>>` and return `None` if the field is missing.
/// - All methods return appropriate errors for buffer size mismatches or other retrieval failures.
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
    fn get_from_current_ledger_obj<const CODE: i32>(field: SField<Self, CODE>) -> Result<Self>;

    /// Get an optional field from the current ledger object.
    ///
    /// # Arguments
    ///
    /// * `field` - The SField identifying which field to retrieve
    ///
    /// # Returns
    ///
    /// Returns a `Result<Option<Self>>` where:
    /// * `Ok(Some(Self))` - The field value for the specified field
    /// * `Ok(None)` - If the field is not present
    /// * `Err(Error)` - If the field cannot be retrieved or has unexpected size
    fn get_from_current_ledger_obj_optional<const CODE: i32>(
        field: SField<Self, CODE>,
    ) -> Result<Option<Self>>;

    /// Get a required field from a specific ledger object.
    ///
    /// # Arguments
    ///
    /// * `register_num` - The register number holding the ledger object
    /// * `field` - The SField identifying which field to retrieve
    ///
    /// # Returns
    ///
    /// Returns a `Result<Self>` where:
    /// * `Ok(Self)` - The field value for the specified field
    /// * `Err(Error)` - If the field cannot be retrieved or has unexpected size
    fn get_from_ledger_obj<const CODE: i32>(
        register_num: i32,
        field: SField<Self, CODE>,
    ) -> Result<Self>;

    /// Get an optional field from a specific ledger object.
    ///
    /// # Arguments
    ///
    /// * `register_num` - The register number holding the ledger object
    /// * `field` - The SField identifying which field to retrieve
    ///
    /// # Returns
    ///
    /// Returns a `Result<Option<Self>>` where:
    /// * `Ok(Some(Self))` - The field value for the specified field
    /// * `Ok(None)` - If the field is not present in the ledger object
    /// * `Err(Error)` - If the field retrieval operation failed
    fn get_from_ledger_obj_optional<const CODE: i32>(
        register_num: i32,
        field: SField<Self, CODE>,
    ) -> Result<Option<Self>>;
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
    fn get_from_current_ledger_obj<const CODE: i32>(field: SField<Self, CODE>) -> Result<Self> {
        let mut value = core::mem::MaybeUninit::<T>::uninit();
        let result_code = unsafe {
            get_current_ledger_obj_field(i32::from(field), value.as_mut_ptr().cast(), T::SIZE)
        };
        match_result_code_with_expected_bytes(result_code, T::SIZE, || unsafe {
            value.assume_init()
        })
    }

    #[inline]
    fn get_from_current_ledger_obj_optional<const CODE: i32>(
        field: SField<Self, CODE>,
    ) -> Result<Option<Self>> {
        let mut value = core::mem::MaybeUninit::<T>::uninit();
        let result_code = unsafe {
            get_current_ledger_obj_field(i32::from(field), value.as_mut_ptr().cast(), T::SIZE)
        };
        match_result_code_with_expected_bytes_optional(result_code, T::SIZE, || {
            Some(unsafe { value.assume_init() })
        })
    }

    #[inline]
    fn get_from_ledger_obj<const CODE: i32>(
        register_num: i32,
        field: SField<Self, CODE>,
    ) -> Result<Self> {
        let mut value = core::mem::MaybeUninit::<T>::uninit();
        let result_code = unsafe {
            get_ledger_obj_field(
                register_num,
                i32::from(field),
                value.as_mut_ptr().cast(),
                T::SIZE,
            )
        };
        match_result_code_with_expected_bytes(result_code, T::SIZE, || unsafe {
            value.assume_init()
        })
    }

    #[inline]
    fn get_from_ledger_obj_optional<const CODE: i32>(
        register_num: i32,
        field: SField<Self, CODE>,
    ) -> Result<Option<Self>> {
        let mut value = core::mem::MaybeUninit::<T>::uninit();
        let result_code = unsafe {
            get_ledger_obj_field(
                register_num,
                i32::from(field),
                value.as_mut_ptr().cast(),
                T::SIZE,
            )
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
    fn get_from_current_ledger_obj<const CODE: i32>(field: SField<Self, CODE>) -> Result<Self> {
        let mut buffer = core::mem::MaybeUninit::<[u8; HASH160_SIZE]>::uninit();
        let result_code = unsafe {
            get_current_ledger_obj_field(i32::from(field), buffer.as_mut_ptr().cast(), HASH160_SIZE)
        };
        match_result_code_with_expected_bytes(result_code, HASH160_SIZE, || {
            Hash160::from(unsafe { buffer.assume_init() })
        })
    }

    #[inline]
    fn get_from_current_ledger_obj_optional<const CODE: i32>(
        field: SField<Self, CODE>,
    ) -> Result<Option<Self>> {
        let mut buffer = core::mem::MaybeUninit::<[u8; HASH160_SIZE]>::uninit();
        let result_code = unsafe {
            get_current_ledger_obj_field(i32::from(field), buffer.as_mut_ptr().cast(), HASH160_SIZE)
        };
        match_result_code_with_expected_bytes_optional(result_code, HASH160_SIZE, || {
            Some(Hash160::from(unsafe { buffer.assume_init() }))
        })
    }

    #[inline]
    fn get_from_ledger_obj<const CODE: i32>(
        register_num: i32,
        field: SField<Self, CODE>,
    ) -> Result<Self> {
        let mut buffer = core::mem::MaybeUninit::<[u8; HASH160_SIZE]>::uninit();
        let result_code = unsafe {
            get_ledger_obj_field(
                register_num,
                i32::from(field),
                buffer.as_mut_ptr().cast(),
                HASH160_SIZE,
            )
        };
        match_result_code_with_expected_bytes(result_code, HASH160_SIZE, || {
            Hash160::from(unsafe { buffer.assume_init() })
        })
    }

    #[inline]
    fn get_from_ledger_obj_optional<const CODE: i32>(
        register_num: i32,
        field: SField<Self, CODE>,
    ) -> Result<Option<Self>> {
        let mut buffer = core::mem::MaybeUninit::<[u8; HASH160_SIZE]>::uninit();
        let result_code = unsafe {
            get_ledger_obj_field(
                register_num,
                i32::from(field),
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
    fn get_from_current_ledger_obj<const CODE: i32>(field: SField<Self, CODE>) -> Result<Self> {
        let mut buffer = core::mem::MaybeUninit::<[u8; HASH192_SIZE]>::uninit();
        let result_code = unsafe {
            get_current_ledger_obj_field(i32::from(field), buffer.as_mut_ptr().cast(), HASH192_SIZE)
        };
        match_result_code_with_expected_bytes(result_code, HASH192_SIZE, || {
            Hash192::from(unsafe { buffer.assume_init() })
        })
    }

    #[inline]
    fn get_from_current_ledger_obj_optional<const CODE: i32>(
        field: SField<Self, CODE>,
    ) -> Result<Option<Self>> {
        let mut buffer = core::mem::MaybeUninit::<[u8; HASH192_SIZE]>::uninit();
        let result_code = unsafe {
            get_current_ledger_obj_field(i32::from(field), buffer.as_mut_ptr().cast(), HASH192_SIZE)
        };
        match_result_code_with_expected_bytes_optional(result_code, HASH192_SIZE, || {
            Some(Hash192::from(unsafe { buffer.assume_init() }))
        })
    }

    #[inline]
    fn get_from_ledger_obj<const CODE: i32>(
        register_num: i32,
        field: SField<Self, CODE>,
    ) -> Result<Self> {
        let mut buffer = core::mem::MaybeUninit::<[u8; HASH192_SIZE]>::uninit();
        let result_code = unsafe {
            get_ledger_obj_field(
                register_num,
                i32::from(field),
                buffer.as_mut_ptr().cast(),
                HASH192_SIZE,
            )
        };
        match_result_code_with_expected_bytes(result_code, HASH192_SIZE, || {
            Hash192::from(unsafe { buffer.assume_init() })
        })
    }

    #[inline]
    fn get_from_ledger_obj_optional<const CODE: i32>(
        register_num: i32,
        field: SField<Self, CODE>,
    ) -> Result<Option<Self>> {
        let mut buffer = core::mem::MaybeUninit::<[u8; HASH192_SIZE]>::uninit();
        let result_code = unsafe {
            get_ledger_obj_field(
                register_num,
                i32::from(field),
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
        field: SField<T, CODE>,
    ) -> Result<T> {
        T::get_from_current_ledger_obj(field)
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
        field: SField<T, CODE>,
    ) -> Result<Option<T>> {
        T::get_from_current_ledger_obj_optional(field)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::core::types::account_id::{ACCOUNT_ID_SIZE, AccountID};
        use crate::core::types::amount::{AMOUNT_SIZE, Amount};
        use crate::core::types::blob::{Blob, PUBLIC_KEY_BLOB_SIZE};
        use crate::core::types::uint::{HASH128_SIZE, HASH256_SIZE, Hash128, Hash256};
        use crate::host::host_bindings_trait::MockHostBindings;
        use crate::host::setup_mock;
        use crate::sfield::{self, SField};
        use mockall::predicate::{always, eq};

        // ========================================
        // Shared test helper functions
        // ========================================

        fn expect_current_field<
            T: LedgerObjectFieldGetter + Send + std::fmt::Debug + PartialEq + 'static,
            const CODE: i32,
        >(
            mock: &mut MockHostBindings,
            field: SField<T, CODE>,
            size: usize,
            times: usize,
        ) {
            mock.expect_get_current_ledger_obj_field()
                .with(eq(field), always(), eq(size))
                .times(times)
                .returning(move |_, _, _| size as i32);
        }

        // ========================================
        // Tests for current_ledger_object module
        // ========================================

        #[test]
        fn test_current_basic_types() {
            let mut mock = MockHostBindings::new();

            expect_current_field(&mut mock, sfield::LedgerEntryType, 2, 1);
            expect_current_field(&mut mock, sfield::Flags, 4, 1);
            expect_current_field(&mut mock, sfield::OwnerNode, 8, 1);

            let _guard = setup_mock(mock);

            assert!(u16::get_from_current_ledger_obj(sfield::LedgerEntryType).is_ok());
            assert!(u32::get_from_current_ledger_obj(sfield::Flags).is_ok());
            assert!(u64::get_from_current_ledger_obj(sfield::OwnerNode).is_ok());
        }

        #[test]
        fn test_current_xrpl_types() {
            let mut mock = MockHostBindings::new();

            expect_current_field(&mut mock, sfield::Account, ACCOUNT_ID_SIZE, 1);
            expect_current_field(&mut mock, sfield::Amount, AMOUNT_SIZE, 1);
            expect_current_field(&mut mock, sfield::EmailHash, HASH128_SIZE, 1);
            expect_current_field(&mut mock, sfield::PreviousTxnID, HASH256_SIZE, 1);
            expect_current_field(&mut mock, sfield::PublicKey, PUBLIC_KEY_BLOB_SIZE, 1);

            let _guard = setup_mock(mock);

            assert!(AccountID::get_from_current_ledger_obj(sfield::Account).is_ok());
            assert!(Amount::get_from_current_ledger_obj(sfield::Amount).is_ok());
            assert!(Hash128::get_from_current_ledger_obj(sfield::EmailHash).is_ok());
            assert!(Hash256::get_from_current_ledger_obj(sfield::PreviousTxnID).is_ok());

            let blob: Blob<33> = Blob::get_from_current_ledger_obj(sfield::PublicKey).unwrap();
            assert_eq!(blob.len, 33);
        }

        #[test]
        fn test_current_optional_fields() {
            let mut mock = MockHostBindings::new();

            expect_current_field(&mut mock, sfield::Flags, 4, 1);
            expect_current_field(&mut mock, sfield::Account, ACCOUNT_ID_SIZE, 1);

            let _guard = setup_mock(mock);

            let result = u32::get_from_current_ledger_obj_optional(sfield::Flags);
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());

            let result = AccountID::get_from_current_ledger_obj_optional(sfield::Account);
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());
        }

        #[test]
        fn test_current_module_convenience_functions() {
            let mut mock = MockHostBindings::new();

            expect_current_field(&mut mock, sfield::Flags, 4, 2);
            expect_current_field(&mut mock, sfield::Account, ACCOUNT_ID_SIZE, 1);

            let _guard = setup_mock(mock);

            assert!(get_field(sfield::Flags).is_ok());
            assert!(get_field(sfield::Account).is_ok());

            let result = get_field_optional(sfield::Flags);
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());
        }
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
        field: SField<T, CODE>,
    ) -> Result<T> {
        T::get_from_ledger_obj(register_num, field)
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
        field: SField<T, CODE>,
    ) -> Result<Option<T>> {
        T::get_from_ledger_obj_optional(register_num, field)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::core::types::account_id::{ACCOUNT_ID_SIZE, AccountID};
        use crate::core::types::amount::{AMOUNT_SIZE, Amount};
        use crate::core::types::blob::{Blob, PUBLIC_KEY_BLOB_SIZE};
        use crate::core::types::uint::{HASH128_SIZE, HASH256_SIZE, Hash128, Hash256};
        use crate::host::host_bindings_trait::MockHostBindings;
        use crate::host::setup_mock;
        use crate::sfield::{self, SField};
        use mockall::predicate::{always, eq};

        // ========================================
        // Shared test helper functions
        // ========================================

        fn expect_ledger_field<
            T: LedgerObjectFieldGetter + Send + std::fmt::Debug + PartialEq + 'static,
            const CODE: i32,
        >(
            mock: &mut MockHostBindings,
            slot: i32,
            field: SField<T, CODE>,
            size: usize,
            times: usize,
        ) {
            mock.expect_get_ledger_obj_field()
                .with(eq(slot), eq(field), always(), eq(size))
                .times(times)
                .returning(move |_, _, _, _| size as i32);
        }

        // ========================================
        // Tests for ledger_object module
        // ========================================

        #[test]
        fn test_ledger_basic_types() {
            let mut mock = MockHostBindings::new();
            let slot = 0;

            expect_ledger_field(&mut mock, slot, sfield::LedgerEntryType, 2, 1);
            expect_ledger_field(&mut mock, slot, sfield::Flags, 4, 1);
            expect_ledger_field(&mut mock, slot, sfield::OwnerNode, 8, 1);

            let _guard = setup_mock(mock);

            assert!(u16::get_from_ledger_obj(slot, sfield::LedgerEntryType).is_ok());
            assert!(u32::get_from_ledger_obj(slot, sfield::Flags).is_ok());
            assert!(u64::get_from_ledger_obj(slot, sfield::OwnerNode).is_ok());
        }

        #[test]
        fn test_ledger_xrpl_types() {
            let mut mock = MockHostBindings::new();
            let slot = 0;

            expect_ledger_field(&mut mock, slot, sfield::Account, ACCOUNT_ID_SIZE, 1);
            expect_ledger_field(&mut mock, slot, sfield::Amount, AMOUNT_SIZE, 1);
            expect_ledger_field(&mut mock, slot, sfield::EmailHash, HASH128_SIZE, 1);
            expect_ledger_field(&mut mock, slot, sfield::PreviousTxnID, HASH256_SIZE, 1);
            expect_ledger_field(&mut mock, slot, sfield::PublicKey, PUBLIC_KEY_BLOB_SIZE, 1);

            let _guard = setup_mock(mock);

            assert!(AccountID::get_from_ledger_obj(slot, sfield::Account).is_ok());
            assert!(Amount::get_from_ledger_obj(slot, sfield::Amount).is_ok());
            assert!(Hash128::get_from_ledger_obj(slot, sfield::EmailHash).is_ok());
            assert!(Hash256::get_from_ledger_obj(slot, sfield::PreviousTxnID).is_ok());

            let blob: Blob<33> = Blob::get_from_ledger_obj(slot, sfield::PublicKey).unwrap();
            assert_eq!(blob.len, 33);
        }

        #[test]
        fn test_ledger_optional_fields() {
            let mut mock = MockHostBindings::new();
            let slot = 0;

            expect_ledger_field(&mut mock, slot, sfield::Flags, 4, 1);
            expect_ledger_field(&mut mock, slot, sfield::Account, ACCOUNT_ID_SIZE, 1);

            let _guard = setup_mock(mock);

            let result = u32::get_from_ledger_obj_optional(slot, sfield::Flags);
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());

            let result = AccountID::get_from_ledger_obj_optional(slot, sfield::Account);
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());
        }

        #[test]
        fn test_ledger_module_convenience_functions() {
            let mut mock = MockHostBindings::new();
            let slot = 0;

            expect_ledger_field(&mut mock, slot, sfield::Flags, 4, 2);
            expect_ledger_field(&mut mock, slot, sfield::Account, ACCOUNT_ID_SIZE, 1);

            let _guard = setup_mock(mock);

            assert!(get_field(slot, sfield::Flags).is_ok());
            assert!(get_field(slot, sfield::Account).is_ok());

            let result = get_field_optional(slot, sfield::Flags);
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());
        }
    }
}
