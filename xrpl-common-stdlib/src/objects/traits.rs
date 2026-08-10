//! Generic ledger-object field accessor traits.
//!
//! The base traits `LedgerObjectCommonFields` / `CurrentLedgerObjectCommonFields` are
//! hand-written here (no field is common to every ledger entry). Every per-ledger-entry
//! field trait (`AccountRootFields`, `EscrowFields`, `OracleFields`, etc.) is generated in
//! `crate::objects::generated` and re-exported from `crate::objects`; a couple are also
//! re-exported here for a stable `objects::traits::*` import path.

pub use crate::objects::generated::{AccountRootFields, EscrowFields};

use crate::fields::locator::LedgerPathBuilder;
use crate::host::Result;
use crate::objects::{current_ledger_object, ledger_object};
use crate::sfield;

/// Trait providing access to common fields present in all ledger objects.
///
/// This trait defines methods to access standard fields that are common across
/// different types of ledger objects in the XRP Ledger.
pub trait LedgerObjectCommonFields {
    // NOTE: `get_ledger_index()` is not in this trait because `sfLedgerIndex` is not actually a field on a ledger
    // object (it's a synthetic field that maps to the `index` field, which is the unique ID of an object in the
    // ledger's state tree). See https://github.com/XRPLF/rippled/issues/3649 for more context.

    /// Returns the slot number (register number) where the ledger object is stored.
    ///
    /// This number is used to identify and access the specific ledger object
    /// when retrieving or modifying its fields.
    ///
    /// # Returns
    ///
    /// The slot number as an i32 value
    fn get_slot_num(&self) -> i32;

    /// Starts a nested-field path rooted at this ledger object, read through its slot.
    ///
    /// Use this to reach into arrays and inner objects that the flat getters below can't return
    /// whole (e.g. `SignerEntries[0].Account`). Chain [`field`](LedgerPathBuilder::field) /
    /// [`index`](LedgerPathBuilder::index), then [`get::<T>()`](LedgerPathBuilder::get).
    ///
    /// ```no_run
    /// use xrpl_common_stdlib::objects::traits::LedgerObjectCommonFields;
    /// use xrpl_common_stdlib::sfield;
    /// use xrpl_common_stdlib::types::account_id::AccountID;
    /// # fn demo(obj: &impl LedgerObjectCommonFields) {
    /// let signer = obj.path()
    ///     .field(sfield::SignerEntries)
    ///     .index(0)
    ///     .field(sfield::Account)
    ///     .get::<AccountID>();
    /// # let _ = signer; }
    /// ```
    fn path(&self) -> LedgerPathBuilder {
        LedgerPathBuilder::for_ledger_obj(self.get_slot_num())
    }

    /// Retrieves the flags field of the ledger object.
    ///
    /// # Arguments
    ///
    /// * `register_num` - The register number where the ledger object is stored
    ///
    /// # Returns
    ///
    /// The flags as a u32 value
    fn get_flags(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::Flags)
    }

    /// Retrieves the ledger entry type of the object.
    ///
    /// The value 0x0075, mapped to the string Escrow, indicates that this is an Escrow entry.
    ///
    /// # Returns
    ///
    /// The ledger entry type as a u16 value
    fn get_ledger_entry_type(&self) -> Result<u16> {
        ledger_object::get_field(self.get_slot_num(), sfield::LedgerEntryType)
    }
}

/// Trait providing access to common fields in the current ledger object.
///
/// This trait defines methods to access standard fields that are common across
/// different types of ledger objects, specifically for the current ledger object
/// being processed.
pub trait CurrentLedgerObjectCommonFields {
    // NOTE: `get_ledger_index()` is not in this trait because `sfLedgerIndex` is not actually a field on a ledger
    // object (it's a synthetic field that maps to the `index` field, which is the unique ID of an object in the
    // ledger's state tree). See https://github.com/XRPLF/rippled/issues/3649 for more context.

    /// Starts a nested-field path rooted at the current ledger object (no slot).
    ///
    /// Use this to reach into arrays and inner objects that the flat getters below can't return
    /// whole. Chain [`field`](LedgerPathBuilder::field) / [`index`](LedgerPathBuilder::index), then
    /// [`get::<T>()`](LedgerPathBuilder::get).
    ///
    /// ```no_run
    /// use xrpl_common_stdlib::objects::traits::CurrentLedgerObjectCommonFields;
    /// use xrpl_common_stdlib::sfield;
    /// use xrpl_common_stdlib::types::amount::Amount;
    /// # fn demo(obj: &impl CurrentLedgerObjectCommonFields) {
    /// let amount = obj.path().field(sfield::Amount).get::<Amount>();
    /// # let _ = amount; }
    /// ```
    fn path(&self) -> LedgerPathBuilder {
        LedgerPathBuilder::for_current_ledger_obj()
    }

    /// Retrieves the flags field of the current ledger object.
    ///
    /// # Returns
    ///
    /// The flags as a u32 value
    fn get_flags(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::Flags)
    }

    /// Retrieves the ledger entry type of the current ledger object.
    ///
    /// The value 0x0075, mapped to the string Escrow, indicates that this is an Escrow entry.
    ///
    /// # Returns
    ///
    /// The ledger entry type as a u16 value
    fn get_ledger_entry_type(&self) -> Result<u16> {
        current_ledger_object::get_field(sfield::LedgerEntryType)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fields::decoder::FromLedger;
    use crate::host::error_codes::{INTERNAL_ERROR, INVALID_FIELD};
    use crate::host::host_bindings_trait::MockHostBindings;
    use crate::objects::AccountRoot;
    use crate::sfield::SField;
    use mockall::predicate::{always, eq};

    // ========================================
    // Test helper functions
    // ========================================

    /// Helper to set up a mock expectation for home_le_field
    ///
    /// Sets up a mock expectation that will match calls with:
    /// - field: The SField with the specified CODE
    /// - size: The expected buffer size
    /// - times: How many times this expectation should be matched
    ///
    /// When a test fails, mockall will show which parameter didn't match.
    fn expect_current_field<
        T: FromLedger + Send + std::fmt::Debug + PartialEq + 'static,
        const CODE: i32,
    >(
        mock: &mut MockHostBindings,
        _field: SField<T, CODE>,
        size: usize,
        times: usize,
    ) {
        mock.expect_home_le_field()
            .with(eq(CODE), always(), eq(size))
            .times(times)
            .returning(move |_, _, _| size as i32);
    }

    /// Helper to set up a mock expectation for le_field
    ///
    /// Sets up a mock expectation that will match calls with:
    /// - slot: The ledger object slot number
    /// - field: The SField with the specified CODE
    /// - size: The expected buffer size
    /// - times: How many times this expectation should be matched
    ///
    /// When a test fails, mockall will show which parameter didn't match.
    fn expect_ledger_field<
        T: FromLedger + Send + std::fmt::Debug + PartialEq + 'static,
        const CODE: i32,
    >(
        mock: &mut MockHostBindings,
        slot: i32,
        _field: SField<T, CODE>,
        size: usize,
        times: usize,
    ) {
        mock.expect_le_field()
            .with(eq(slot), eq(CODE), always(), eq(size))
            .times(times)
            .returning(move |_, _, _, _| size as i32);
    }

    mod ledger_object_common_fields {
        use super::*;
        use crate::host::setup_mock;

        #[test]
        fn test_mandatory_fields_return_ok() {
            let mut mock = MockHostBindings::new();

            // get_flags
            expect_ledger_field(&mut mock, 1, sfield::Flags, 4, 1);
            // get_ledger_entry_type
            expect_ledger_field(&mut mock, 1, sfield::LedgerEntryType, 2, 1);

            let _guard = setup_mock(mock);

            let account = AccountRoot::new(1);

            // All mandatory fields should return Ok
            assert!(account.get_flags().is_ok());
            assert!(account.get_ledger_entry_type().is_ok());
        }

        #[test]
        fn test_mandatory_fields_return_error_on_internal_error() {
            let mut mock = MockHostBindings::new();

            // get_flags with INTERNAL_ERROR
            mock.expect_le_field()
                .with(eq(1), eq(sfield::Flags), always(), eq(4))
                .times(1)
                .returning(|_, _, _, _| INTERNAL_ERROR);

            let _guard = setup_mock(mock);

            let account = AccountRoot::new(1);
            let result = account.get_flags();

            assert!(result.is_err());
            assert_eq!(result.err().unwrap().code(), INTERNAL_ERROR);
        }

        #[test]
        fn test_get_ledger_entry_type_returns_error_on_internal_error() {
            let mut mock = MockHostBindings::new();

            mock.expect_le_field()
                .with(eq(1), eq(sfield::LedgerEntryType), always(), eq(2))
                .times(1)
                .returning(|_, _, _, _| INTERNAL_ERROR);

            let _guard = setup_mock(mock);

            let account = AccountRoot::new(1);
            let result = account.get_ledger_entry_type();

            assert!(result.is_err());
            assert_eq!(result.err().unwrap().code(), INTERNAL_ERROR);
        }

        #[test]
        fn test_mandatory_fields_return_error_on_invalid_field() {
            let mut mock = MockHostBindings::new();

            // get_flags with INVALID_FIELD
            mock.expect_le_field()
                .with(eq(1), eq(sfield::Flags), always(), eq(4))
                .times(1)
                .returning(|_, _, _, _| INVALID_FIELD);

            let _guard = setup_mock(mock);

            let account = AccountRoot::new(1);
            let result = account.get_flags();

            assert!(result.is_err());
            assert_eq!(result.err().unwrap().code(), INVALID_FIELD);
        }
    }

    mod current_ledger_object_common_fields {
        use super::*;
        use crate::host::setup_mock;

        struct TestCurrentLedgerObject;
        impl CurrentLedgerObjectCommonFields for TestCurrentLedgerObject {}

        #[test]
        fn test_mandatory_fields_return_ok() {
            let mut mock = MockHostBindings::new();

            // get_flags
            expect_current_field(&mut mock, sfield::Flags, 4, 1);
            // get_ledger_entry_type
            expect_current_field(&mut mock, sfield::LedgerEntryType, 2, 1);

            let _guard = setup_mock(mock);

            let escrow = TestCurrentLedgerObject;

            // All mandatory fields should return Ok
            assert!(escrow.get_flags().is_ok());
            assert!(escrow.get_ledger_entry_type().is_ok());
        }

        #[test]
        fn test_mandatory_fields_return_error_on_internal_error() {
            let mut mock = MockHostBindings::new();

            // get_flags with INTERNAL_ERROR
            mock.expect_home_le_field()
                .with(eq(sfield::Flags), always(), eq(4))
                .times(1)
                .returning(|_, _, _| INTERNAL_ERROR);

            let _guard = setup_mock(mock);

            let escrow = TestCurrentLedgerObject;
            let result = escrow.get_flags();

            assert!(result.is_err());
            assert_eq!(result.err().unwrap().code(), INTERNAL_ERROR);
        }

        #[test]
        fn test_get_ledger_entry_type_returns_error_on_internal_error() {
            let mut mock = MockHostBindings::new();

            mock.expect_home_le_field()
                .with(eq(sfield::LedgerEntryType), always(), eq(2))
                .times(1)
                .returning(|_, _, _| INTERNAL_ERROR);

            let _guard = setup_mock(mock);

            let escrow = TestCurrentLedgerObject;
            let result = escrow.get_ledger_entry_type();

            assert!(result.is_err());
            assert_eq!(result.err().unwrap().code(), INTERNAL_ERROR);
        }

        #[test]
        fn test_mandatory_fields_return_error_on_invalid_field() {
            let mut mock = MockHostBindings::new();

            // get_flags with INVALID_FIELD
            mock.expect_home_le_field()
                .with(eq(sfield::Flags), always(), eq(4))
                .times(1)
                .returning(|_, _, _| INVALID_FIELD);

            let _guard = setup_mock(mock);

            let escrow = TestCurrentLedgerObject;
            let result = escrow.get_flags();

            assert!(result.is_err());
            assert_eq!(result.err().unwrap().code(), INVALID_FIELD);
        }
    }
}
