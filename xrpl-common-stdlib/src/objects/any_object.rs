//! Untyped handle to a slot-cached ledger object.
//!
//! Typed wrappers like [`AccountRoot`](crate::objects::account_root::AccountRoot) exist to add
//! object-specific named accessors (`AccountFields`, `EscrowFields`). Object types that have no such
//! wrapper — Oracle, SignerList, NFTokenPage, RippleState — still need the common fields and, above
//! all, inner-field paths. [`LedgerObject`] is that door: wrap the raw slot
//! [`cache_le`](crate::objects::cache_le) handed back and get everything on
//! [`LedgerObjectCommonFields`](crate::objects::traits::LedgerObjectCommonFields), including
//! [`path()`](crate::objects::traits::LedgerObjectCommonFields::path).
//!
//! Adding a typed wrapper for an object later is purely additive — code written against
//! `LedgerObject` keeps working.

use crate::objects::traits::LedgerObjectCommonFields;

/// A ledger object identified only by the slot it was cached into.
///
/// ```no_run
/// use xrpl_common_stdlib::objects::LedgerObject;
/// use xrpl_common_stdlib::objects::traits::LedgerObjectCommonFields;
/// use xrpl_common_stdlib::sfield;
/// # fn demo(slot: i32) {
/// // Read PriceDataSeries[0].AssetPrice off an Oracle object, which has no typed wrapper.
/// let price = LedgerObject::new(slot)
///     .path()
///     .field(sfield::PriceDataSeries)
///     .index(0)
///     .field(sfield::AssetPrice)
///     .get::<u64>();
/// # let _ = price; }
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct LedgerObject {
    pub slot_num: i32,
}

impl LedgerObject {
    /// Wrap a slot returned by [`cache_le`](crate::objects::cache_le).
    ///
    /// The slot is not validated here — a negative slot means the caching call failed, and the
    /// caller is expected to have checked that before building a handle. Field reads through an
    /// invalid slot surface the host's error as usual.
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }
}

impl LedgerObjectCommonFields for LedgerObject {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::host::host_bindings_trait::MockHostBindings;
    use crate::host::setup_mock;
    use crate::sfield;
    use mockall::predicate::{always, eq};

    #[test]
    fn test_new_stores_slot() {
        assert_eq!(LedgerObject::new(7).get_slot_num(), 7);
    }

    #[test]
    fn test_inherits_common_fields() {
        let mut mock = MockHostBindings::new();
        mock.expect_le_field()
            .with(eq(3), eq(sfield::Flags), always(), eq(4))
            .times(1)
            .returning(|_, _, _, _| 4);
        let _guard = setup_mock(mock);

        assert!(LedgerObject::new(3).get_flags().is_ok());
    }

    #[test]
    fn test_path_reads_inner_field_through_its_slot() {
        // The whole point of this type: an inner read on an object with no typed wrapper.
        // PriceDataSeries[0].AssetPrice is three 4-byte segments = 12 bytes; u64 buffer is 8.
        let mut mock = MockHostBindings::new();
        mock.expect_le_inner()
            .with(eq(9), always(), eq(12usize), always(), eq(8usize))
            .times(1)
            .returning(|_, _, _, _, _| 8);
        let _guard = setup_mock(mock);

        let result = LedgerObject::new(9)
            .path()
            .field(sfield::PriceDataSeries)
            .index(0)
            .field(sfield::AssetPrice)
            .get::<u64>();

        assert!(result.is_ok());
    }
}
