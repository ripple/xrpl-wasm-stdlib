// GENERATED -- do not hand-edit. Run scripts/generate-ledger-objects.sh to regenerate.

use crate::host::Result;
use crate::objects::traits::CurrentLedgerObjectCommonFields;
use crate::objects::traits::LedgerObjectCommonFields;
use crate::objects::{current_ledger_object, ledger_object};
use crate::sfield;
use crate::types::account_id::AccountID;
use crate::types::uint::Hash256;

/// Trait providing access to fields specific to TransactionProposal objects in any ledger.
pub trait TransactionProposalFields: LedgerObjectCommonFields {
    /// The PreviousTxnID field (Required).
    fn previous_txn_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The PreviousTxnLgrSeq field (Required).
    fn previous_txn_lgr_seq(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }

    /// The Owner field (Required).
    fn owner(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Owner)
    }

    /// The Expiration field (Required).
    fn expiration(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::Expiration)
    }

    /// The OwnerNode field (Required).
    fn owner_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::OwnerNode)
    }
}

/// Trait providing access to fields specific to the current TransactionProposal object.
pub trait CurrentTransactionProposalFields: CurrentLedgerObjectCommonFields {
    /// The PreviousTxnID field (Required).
    fn previous_txn_id(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::PreviousTxnID)
    }

    /// The PreviousTxnLgrSeq field (Required).
    fn previous_txn_lgr_seq(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::PreviousTxnLgrSeq)
    }

    /// The Owner field (Required).
    fn owner(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Owner)
    }

    /// The Expiration field (Required).
    fn expiration(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::Expiration)
    }

    /// The OwnerNode field (Required).
    fn owner_node(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::OwnerNode)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct TransactionProposal {
    pub(crate) slot_num: i32,
}

impl TransactionProposal {
    /// Binds this handle to a host-managed slot holding a TransactionProposal ledger object.
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }
}

impl LedgerObjectCommonFields for TransactionProposal {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl TransactionProposalFields for TransactionProposal {}

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

        let obj = TransactionProposal::new(0);

        assert!(obj.previous_txn_id().is_ok());
        assert!(obj.previous_txn_lgr_seq().is_ok());
        assert!(obj.owner().is_ok());
        assert!(obj.expiration().is_ok());
        assert!(obj.owner_node().is_ok());
    }
}
