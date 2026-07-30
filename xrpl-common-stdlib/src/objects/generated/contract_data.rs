// GENERATED -- do not hand-edit. Run scripts/generate-ledger-objects.sh to regenerate.

/// Placeholder buffer size for fields whose XRPL wire type has no genuine Rust
/// mapping yet (VECTOR256, XCHAIN_BRIDGE, NUMBER, INT32, ...). Such getters return
/// raw, unparsed bytes; see the summary at the top of `generated/mod.rs`.
const RAW_UNMAPPED_FIELD_SIZE: usize = 512;

use crate::host::Result;
use crate::host::error_codes::match_result_code;
use crate::host::get_current_ledger_obj_field;
use crate::host::get_ledger_obj_field;
use crate::objects::traits::CurrentLedgerObjectCommonFields;
use crate::objects::traits::LedgerObjectCommonFields;
use crate::objects::{current_ledger_object, ledger_object};
use crate::sfield;
use crate::types::account_id::AccountID;
use crate::types::uint::Hash256;

/// Trait providing access to fields specific to ContractData objects in any ledger.
pub trait ContractDataFields: LedgerObjectCommonFields {
    /// The PreviousTxnID field (Required).
    fn previous_txn_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The PreviousTxnLgrSeq field (Required).
    fn previous_txn_lgr_seq(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
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

    /// The ContractJson field (Required).
    /// Raw bytes; JSON is not yet typed in Rust.
    fn contract_json(&self) -> Result<[u8; RAW_UNMAPPED_FIELD_SIZE]> {
        let mut buffer = [0u8; RAW_UNMAPPED_FIELD_SIZE];
        let result_code = unsafe {
            get_ledger_obj_field(
                self.get_slot_num(),
                sfield::ContractJson.into(),
                buffer.as_mut_ptr(),
                buffer.len(),
            )
        };
        match_result_code(result_code, || buffer)
    }
}

/// Trait providing access to fields specific to the current ContractData object.
pub trait CurrentContractDataFields: CurrentLedgerObjectCommonFields {
    /// The PreviousTxnID field (Required).
    fn previous_txn_id(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::PreviousTxnID)
    }

    /// The PreviousTxnLgrSeq field (Required).
    fn previous_txn_lgr_seq(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::PreviousTxnLgrSeq)
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

    /// The ContractJson field (Required).
    /// Raw bytes; JSON is not yet typed in Rust.
    fn contract_json(&self) -> Result<[u8; RAW_UNMAPPED_FIELD_SIZE]> {
        let mut buffer = [0u8; RAW_UNMAPPED_FIELD_SIZE];
        let result_code = unsafe {
            get_current_ledger_obj_field(
                sfield::ContractJson.into(),
                buffer.as_mut_ptr(),
                buffer.len(),
            )
        };
        match_result_code(result_code, || buffer)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ContractData {
    pub(crate) slot_num: i32,
}

impl ContractData {
    /// Binds this handle to a host-managed slot holding a ContractData ledger object.
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }
}

impl LedgerObjectCommonFields for ContractData {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl ContractDataFields for ContractData {}

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

        let obj = ContractData::new(0);

        assert!(obj.previous_txn_id().is_ok());
        assert!(obj.previous_txn_lgr_seq().is_ok());
        assert!(obj.owner_node().is_ok());
        assert!(obj.owner().is_ok());
        assert!(obj.contract_account().is_ok());
        assert!(obj.contract_json().is_ok());
    }
}
