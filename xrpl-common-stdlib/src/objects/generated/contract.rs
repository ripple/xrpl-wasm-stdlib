// GENERATED -- do not hand-edit. Run scripts/generate-ledger-objects.sh to regenerate.

use crate::host::Result;
use crate::objects::traits::CurrentLedgerObjectCommonFields;
use crate::objects::traits::LedgerObjectCommonFields;
use crate::objects::{current_ledger_object, ledger_object};
use crate::sfield;
use crate::types::account_id::AccountID;
use crate::types::blob::UriBlob;
use crate::types::uint::Hash256;

/// Trait providing access to fields specific to Contract objects in any ledger.
pub trait ContractFields: LedgerObjectCommonFields {
    /// The PreviousTxnID field (Required).
    fn previous_txn_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The PreviousTxnLgrSeq field (Required).
    fn previous_txn_lgr_seq(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }

    /// The Sequence field (Required).
    fn sequence(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::Sequence)
    }

    /// The OwnerNode field (Required).
    fn owner_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::OwnerNode)
    }

    /// The Owner field (Required).
    fn owner(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Owner)
    }

    /// The ContractAccount field (Required).
    fn contract_account(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::ContractAccount)
    }

    /// The ContractHash field (Required).
    fn contract_hash(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::ContractHash)
    }

    /// The URI field (Optional).
    fn uri(&self) -> Result<Option<UriBlob>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::URI)
    }
}

/// Trait providing access to fields specific to the current Contract object.
pub trait CurrentContractFields: CurrentLedgerObjectCommonFields {
    /// The PreviousTxnID field (Required).
    fn previous_txn_id(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::PreviousTxnID)
    }

    /// The PreviousTxnLgrSeq field (Required).
    fn previous_txn_lgr_seq(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::PreviousTxnLgrSeq)
    }

    /// The Sequence field (Required).
    fn sequence(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::Sequence)
    }

    /// The OwnerNode field (Required).
    fn owner_node(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::OwnerNode)
    }

    /// The Owner field (Required).
    fn owner(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Owner)
    }

    /// The ContractAccount field (Required).
    fn contract_account(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::ContractAccount)
    }

    /// The ContractHash field (Required).
    fn contract_hash(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::ContractHash)
    }

    /// The URI field (Optional).
    fn uri(&self) -> Result<Option<UriBlob>> {
        current_ledger_object::get_field_optional(sfield::URI)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Contract {
    pub(crate) slot_num: i32,
}

impl Contract {
    /// Binds this handle to a host-managed slot holding a Contract ledger object.
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }
}

impl LedgerObjectCommonFields for Contract {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl ContractFields for Contract {}

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

        let obj = Contract::new(0);

        assert!(obj.previous_txn_id().is_ok());
        assert!(obj.previous_txn_lgr_seq().is_ok());
        assert!(obj.sequence().is_ok());
        assert!(obj.owner_node().is_ok());
        assert!(obj.owner().is_ok());
        assert!(obj.contract_account().is_ok());
        assert!(obj.contract_hash().is_ok());
        assert!(obj.uri().is_ok());
    }
}
