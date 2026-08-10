//! Caching a ledger entry into a host slot.
//!
//! Reading any ledger object other than the one the contract is attached to is two steps: compute
//! the entry's ID (see [`crate::ledger_entry_ids`]), then ask the host to load that entry into one
//! of its cache slots. [`cache_ledger_entry`] is the second step, and the slot it returns is what
//! every slot-based handle is built from — [`LedgerObject::new`](crate::objects::LedgerObject::new)
//! for an object with no typed wrapper, `<Entry>::new` for one that has it.

use crate::host;
use crate::host::Result;
use crate::host::error_codes::match_result_code;
use crate::ledger_entry_ids::LedgerEntryIdBytes;

/// The host's sentinel for "put this entry in the next free slot" rather than replacing the entry
/// in a specific one.
const NEXT_AVAILABLE_SLOT: i32 = 0;

/// Load the ledger entry with the given ID into a host cache slot, returning that slot.
///
/// The host assigns the next free slot; its cache holds up to 255 entries at once, so a contract
/// that caches in a loop can exhaust it and see [`host::Error::SlotsFull`]. An ID that matches no
/// entry in the ledger is [`host::Error::LedgerObjNotFound`], which is how a contract asks "does
/// this object exist?".
///
/// ```no_run
/// use xrpl_common_stdlib::host::Result;
/// use xrpl_common_stdlib::ledger_entry_ids::accountroot_id;
/// use xrpl_common_stdlib::objects::{AccountRoot, AccountRootFields, cache_ledger_entry};
/// use xrpl_common_stdlib::types::account_id::AccountID;
/// # fn demo(account: &AccountID) {
/// if let Result::Ok(id) = accountroot_id(account) {
///     if let Result::Ok(slot) = cache_ledger_entry(&id) {
///         let balance = AccountRoot::new(slot).balance();
///         # let _ = balance;
///     }
/// }
/// # }
/// ```
pub fn cache_ledger_entry(entry_id: &LedgerEntryIdBytes) -> Result<i32> {
    let slot = unsafe { host::cache_le(entry_id.as_ptr(), entry_id.len(), NEXT_AVAILABLE_SLOT) };
    match_result_code(slot, || slot)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::host::error_codes::{LEDGER_OBJ_NOT_FOUND, SLOTS_FULL};
    use crate::host::host_bindings_trait::MockHostBindings;
    use crate::host::setup_mock;
    use crate::ledger_entry_ids::XRPL_LEDGER_ENTRY_ID_SIZE;
    use mockall::predicate::{always, eq};

    #[test]
    fn test_returns_the_slot_the_host_assigned() {
        let mut mock = MockHostBindings::new();
        // The whole 32-byte ID is handed over, and slot 0 means "next available", not a slot.
        mock.expect_cache_le()
            .with(always(), eq(XRPL_LEDGER_ENTRY_ID_SIZE), eq(0))
            .times(1)
            .returning(|_, _, _| 4);
        let _guard = setup_mock(mock);

        assert_eq!(cache_ledger_entry(&[0xAB; 32]).unwrap(), 4);
    }

    #[test]
    fn test_reports_a_missing_entry_as_an_error() {
        let mut mock = MockHostBindings::new();
        mock.expect_cache_le()
            .times(1)
            .returning(|_, _, _| LEDGER_OBJ_NOT_FOUND);
        let _guard = setup_mock(mock);

        assert_eq!(
            cache_ledger_entry(&[0x00; 32]).err().unwrap().code(),
            LEDGER_OBJ_NOT_FOUND
        );
    }

    #[test]
    fn test_reports_an_exhausted_cache_as_an_error() {
        let mut mock = MockHostBindings::new();
        mock.expect_cache_le()
            .times(1)
            .returning(|_, _, _| SLOTS_FULL);
        let _guard = setup_mock(mock);

        assert_eq!(
            cache_ledger_entry(&[0x11; 32]).err().unwrap().code(),
            SLOTS_FULL
        );
    }
}
