// GENERATED -- do not hand-edit. Run scripts/generate-ledger-objects.sh to regenerate.

use crate::host::Result;
use crate::objects::traits::CurrentLedgerObjectCommonFields;
use crate::objects::traits::LedgerObjectCommonFields;
use crate::objects::{current_ledger_object, ledger_object};
use crate::sfield;
use crate::types::account_id::AccountID;
use crate::types::uint::Hash256;

/// Trait providing access to fields specific to Ticket objects in any ledger.
pub trait TicketFields: LedgerObjectCommonFields {
    /// The account that owns this Ticket.
    fn get_account(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Account)
    }

    /// A hint indicating which page of the owner directory links to this entry, in case the directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::OwnerNode)
    }

    /// The [Sequence Number][] this Ticket sets aside.
    fn get_ticket_sequence(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::TicketSequence)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The [index of the ledger][Ledger Index] that contains the transaction that most recently modified this entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }
}

/// Trait providing access to fields specific to the current Ticket object.
pub trait CurrentTicketFields: CurrentLedgerObjectCommonFields {
    /// The account that owns this Ticket.
    fn get_account(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Account)
    }

    /// A hint indicating which page of the owner directory links to this entry, in case the directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::OwnerNode)
    }

    /// The [Sequence Number][] this Ticket sets aside.
    fn get_ticket_sequence(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::TicketSequence)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::PreviousTxnID)
    }

    /// The [index of the ledger][Ledger Index] that contains the transaction that most recently modified this entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::PreviousTxnLgrSeq)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Ticket {
    pub(crate) slot_num: i32,
}

impl Ticket {
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }

    /// Loads the Ticket ledger object identified by the given keylet arguments,
    /// caching it in a host-managed slot.
    pub fn load(owner: &AccountID, seq: u32) -> Result<Self> {
        let keylet = match crate::keylets::ticket_keylet(owner, seq) {
            Result::Ok(k) => k,
            Result::Err(e) => return Result::Err(e),
        };
        let slot = unsafe { crate::host::cache_ledger_obj(keylet.as_ptr(), keylet.len(), 0) };
        if slot < 0 {
            return Result::Err(crate::host::Error::from_code(slot));
        }
        Result::Ok(Self { slot_num: slot })
    }
}

impl LedgerObjectCommonFields for Ticket {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl TicketFields for Ticket {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::host::host_bindings_trait::MockHostBindings;
    use crate::host::setup_mock;
    use crate::objects::test_support::*;

    #[test]
    fn read_all_fields() {
        let mut mock = MockHostBindings::new();
        mock_all_fields_present(&mut mock);
        let _guard = setup_mock(mock);

        let obj = Ticket::new(0);

        assert!(obj.get_account().is_ok());
        assert!(obj.get_owner_node().is_ok());
        assert!(obj.get_ticket_sequence().is_ok());
        assert!(obj.get_previous_txn_id().is_ok());
        assert!(obj.get_previous_txn_lgr_seq().is_ok());
    }

    #[test]
    fn load_success() {
        let mut mock = MockHostBindings::new();
        mock_ticket_keylet_success(&mut mock);
        mock_cache_ledger_obj_success(&mut mock, 7);
        let _guard = setup_mock(mock);

        let result = Ticket::load(&sample::account_id(), sample::seq());
        assert!(result.is_ok());
    }

    #[test]
    fn load_cache_error() {
        use crate::host::error_codes::INTERNAL_ERROR;

        let mut mock = MockHostBindings::new();
        mock_ticket_keylet_success(&mut mock);
        mock_cache_ledger_obj_error(&mut mock, INTERNAL_ERROR);
        let _guard = setup_mock(mock);

        let result = Ticket::load(&sample::account_id(), sample::seq());
        assert!(result.is_err());
    }
}
