// GENERATED -- do not hand-edit. Run scripts/generate-ledger-objects.sh to regenerate.

use crate::host::Result;
use crate::objects::traits::CurrentLedgerObjectCommonFields;
use crate::objects::traits::LedgerObjectCommonFields;
use crate::objects::{current_ledger_object, ledger_object};
use crate::sfield;
use crate::types::blob::StandardBlob;
use crate::types::uint::Hash256;

/// Trait providing access to fields specific to ContractSource objects in any ledger.
pub trait ContractSourceFields: LedgerObjectCommonFields {
    /// The PreviousTxnID field (Required).
    fn previous_txn_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The PreviousTxnLgrSeq field (Required).
    fn previous_txn_lgr_seq(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }

    /// The ContractHash field (Required).
    fn contract_hash(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::ContractHash)
    }

    /// The ContractCode field (Required).
    fn contract_code(&self) -> Result<StandardBlob> {
        ledger_object::get_field(self.get_slot_num(), sfield::ContractCode)
    }

    /// The ReferenceCount field (Required).
    fn reference_count(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::ReferenceCount)
    }
}

/// Trait providing access to fields specific to the current ContractSource object.
pub trait CurrentContractSourceFields: CurrentLedgerObjectCommonFields {
    /// The PreviousTxnID field (Required).
    fn previous_txn_id(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::PreviousTxnID)
    }

    /// The PreviousTxnLgrSeq field (Required).
    fn previous_txn_lgr_seq(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::PreviousTxnLgrSeq)
    }

    /// The ContractHash field (Required).
    fn contract_hash(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::ContractHash)
    }

    /// The ContractCode field (Required).
    fn contract_code(&self) -> Result<StandardBlob> {
        current_ledger_object::get_field(sfield::ContractCode)
    }

    /// The ReferenceCount field (Required).
    fn reference_count(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::ReferenceCount)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ContractSource {
    pub(crate) slot_num: i32,
}

impl ContractSource {
    /// Binds this handle to a host-managed slot holding a ContractSource ledger object.
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }
}

impl LedgerObjectCommonFields for ContractSource {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl ContractSourceFields for ContractSource {}

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

        let obj = ContractSource::new(0);

        assert!(obj.previous_txn_id().is_ok());
        assert!(obj.previous_txn_lgr_seq().is_ok());
        assert!(obj.contract_hash().is_ok());
        assert!(obj.contract_code().is_ok());
        assert!(obj.reference_count().is_ok());
    }
}
