// GENERATED -- do not hand-edit. Run scripts/generate-ledger-objects.sh to regenerate.

use crate::host::Result;
use crate::objects::traits::CurrentLedgerObjectCommonFields;
use crate::objects::traits::LedgerObjectCommonFields;
use crate::objects::{current_ledger_object, ledger_object};
use crate::sfield;
use crate::types::uint::Hash256;

/// Trait providing access to fields specific to NFTokenPage objects in any ledger.
pub trait NFTokenPageFields: LedgerObjectCommonFields {
    /// The locator of the previous page, if any. Details about this field and how it should be used are outlined below.
    fn previous_page_min(&self) -> Result<Option<Hash256>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::PreviousPageMin)
    }

    /// The locator of the next page, if any. Details about this field and how it should be used are outlined below.
    fn next_page_min(&self) -> Result<Option<Hash256>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::NextPageMin)
    }

    /// Identifies the transaction ID of the transaction that most recently modified this NFTokenPage object.
    fn previous_txn_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The sequence of the ledger that contains the transaction that most recently modified this NFTokenPage object.
    fn previous_txn_lgr_seq(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }
}

/// Trait providing access to fields specific to the current NFTokenPage object.
pub trait CurrentNFTokenPageFields: CurrentLedgerObjectCommonFields {
    /// The locator of the previous page, if any. Details about this field and how it should be used are outlined below.
    fn previous_page_min(&self) -> Result<Option<Hash256>> {
        current_ledger_object::get_field_optional(sfield::PreviousPageMin)
    }

    /// The locator of the next page, if any. Details about this field and how it should be used are outlined below.
    fn next_page_min(&self) -> Result<Option<Hash256>> {
        current_ledger_object::get_field_optional(sfield::NextPageMin)
    }

    /// Identifies the transaction ID of the transaction that most recently modified this NFTokenPage object.
    fn previous_txn_id(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::PreviousTxnID)
    }

    /// The sequence of the ledger that contains the transaction that most recently modified this NFTokenPage object.
    fn previous_txn_lgr_seq(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::PreviousTxnLgrSeq)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct NFTokenPage {
    pub(crate) slot_num: i32,
}

impl NFTokenPage {
    /// Binds this handle to a host-managed slot holding a NFTokenPage ledger object.
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }
}

impl LedgerObjectCommonFields for NFTokenPage {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl NFTokenPageFields for NFTokenPage {}

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

        let obj = NFTokenPage::new(0);

        assert!(obj.previous_txn_id().is_ok());
        assert!(obj.previous_txn_lgr_seq().is_ok());
        assert!(obj.previous_page_min().is_ok());
        assert!(obj.next_page_min().is_ok());
    }

    #[test]
    fn optional_fields_none() {
        let mut mock = MockHostBindings::new();
        mock_all_fields_not_found(&mut mock);
        let _guard = setup_mock(mock);

        let obj = NFTokenPage::new(0);

        assert!(obj.previous_page_min().unwrap().is_none());
        assert!(obj.next_page_min().unwrap().is_none());
    }
}
