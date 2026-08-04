// GENERATED -- do not hand-edit. Run scripts/generate-ledger-objects.sh to regenerate.

use crate::host::Result;
use crate::objects::traits::CurrentLedgerObjectCommonFields;
use crate::objects::traits::LedgerObjectCommonFields;
use crate::objects::{current_ledger_object, ledger_object};
use crate::sfield;
use crate::types::account_id::AccountID;
use crate::types::uint::Hash256;

/// Trait providing access to fields specific to Delegate objects in any ledger.
pub trait DelegateFields: LedgerObjectCommonFields {
    /// The account delegating permissions to another, also called the _delegating account_.
    fn account(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Account)
    }

    /// The account receiving permissions, also called the _delegate_.
    fn authorize(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Authorize)
    }

    /// A hint indicating which page of the delegating account's owner directory links to this
    /// object, in case the directory consists of multiple pages.
    fn owner_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::OwnerNode)
    }

    /// A hint indicating which page of the delegate's owner directory links to this object, in case
    /// the directory consists of multiple pages.
    fn destination_node(&self) -> Result<Option<u64>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::DestinationNode)
    }

    /// The identifying hash of the transaction that most recently modified this object.
    fn previous_txn_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// object.
    fn previous_txn_lgr_seq(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }
}

/// Trait providing access to fields specific to the current Delegate object.
pub trait CurrentDelegateFields: CurrentLedgerObjectCommonFields {
    /// The account delegating permissions to another, also called the _delegating account_.
    fn account(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Account)
    }

    /// The account receiving permissions, also called the _delegate_.
    fn authorize(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Authorize)
    }

    /// A hint indicating which page of the delegating account's owner directory links to this
    /// object, in case the directory consists of multiple pages.
    fn owner_node(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::OwnerNode)
    }

    /// A hint indicating which page of the delegate's owner directory links to this object, in case
    /// the directory consists of multiple pages.
    fn destination_node(&self) -> Result<Option<u64>> {
        current_ledger_object::get_field_optional(sfield::DestinationNode)
    }

    /// The identifying hash of the transaction that most recently modified this object.
    fn previous_txn_id(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// object.
    fn previous_txn_lgr_seq(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::PreviousTxnLgrSeq)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Delegate {
    pub(crate) slot_num: i32,
}

impl Delegate {
    /// Binds this handle to a host-managed slot holding a Delegate ledger object.
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }
}

impl LedgerObjectCommonFields for Delegate {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl DelegateFields for Delegate {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::host::host_bindings_trait::MockHostBindings;
    use crate::host::setup_mock;
    use crate::objects::test_utils::*;

    #[test]
    fn read_all_fields() {
        let mut mock = MockHostBindings::new();
        mock_all_fields_present(&mut mock);
        let _guard = setup_mock(mock);

        let obj = Delegate::new(0);

        assert!(obj.account().is_ok());
        assert!(obj.authorize().is_ok());
        assert!(obj.owner_node().is_ok());
        assert!(obj.previous_txn_id().is_ok());
        assert!(obj.previous_txn_lgr_seq().is_ok());
        assert!(obj.destination_node().is_ok());
    }

    #[test]
    fn optional_fields_none() {
        let mut mock = MockHostBindings::new();
        mock_all_fields_not_found(&mut mock);
        let _guard = setup_mock(mock);

        let obj = Delegate::new(0);

        assert!(obj.destination_node().unwrap().is_none());
    }
}
