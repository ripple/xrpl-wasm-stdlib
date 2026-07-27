// GENERATED -- do not hand-edit. Run scripts/generate-ledger-objects.sh to regenerate.

/// Placeholder buffer size for fields whose XRPL wire type has no genuine Rust
/// mapping yet (VECTOR256, XCHAIN_BRIDGE, NUMBER, INT32, ...). Such getters return
/// raw, unparsed bytes; see the summary at the top of `generated/mod.rs`.
const RAW_UNMAPPED_FIELD_SIZE: usize = 512;

use crate::host::Result;
use crate::host::error_codes::match_result_code;
use crate::host::get_current_ledger_obj_field;
use crate::host::get_ledger_obj_field;
use crate::objects::array_object::Array;
use crate::objects::traits::CurrentLedgerObjectCommonFields;
use crate::objects::traits::LedgerObjectCommonFields;
use crate::objects::{current_ledger_object, ledger_object};
use crate::sfield;
use crate::types::account_id::AccountID;
use crate::types::uint::Hash256;

/// Trait providing access to fields specific to XChainOwnedCreateAccountClaimID objects in any ledger.
pub trait XChainOwnedCreateAccountClaimIDFields: LedgerObjectCommonFields {
    /// The account that owns this object.
    fn get_account(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Account)
    }

    /// The door accounts and assets of the bridge this object correlates to.
    /// Raw bytes; XCHAIN_BRIDGE is not yet typed in Rust.
    fn get_xchain_bridge(&self) -> Result<[u8; RAW_UNMAPPED_FIELD_SIZE]> {
        let mut buffer = [0u8; RAW_UNMAPPED_FIELD_SIZE];
        let result_code = unsafe {
            get_ledger_obj_field(
                self.get_slot_num(),
                sfield::XChainBridge.into(),
                buffer.as_mut_ptr(),
                buffer.len(),
            )
        };
        match_result_code(result_code, || buffer)
    }

    /// An integer that determines the order that accounts created through cross-chain transfers must be performed. Smaller numbers must execute before larger numbers.
    fn get_xchain_account_create_count(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::XChainAccountCreateCount)
    }

    /// Attestations collected from the witness servers. This includes the parameters needed to recreate the message that was signed, including the amount, destination, signature reward amount, and reward account for that signature. With the exception of the reward account, all signatures must sign the message created with common parameters.
    fn get_xchain_create_account_attestations(&self) -> Result<Array> {
        ledger_object::get_field(self.get_slot_num(), sfield::XChainCreateAccountAttestations)
    }

    /// The OwnerNode field (Required).
    fn get_owner_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::OwnerNode)
    }

    /// The PreviousTxnID field (Required).
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The PreviousTxnLgrSeq field (Required).
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }
}

/// Trait providing access to fields specific to the current XChainOwnedCreateAccountClaimID object.
pub trait CurrentXChainOwnedCreateAccountClaimIDFields: CurrentLedgerObjectCommonFields {
    /// The account that owns this object.
    fn get_account(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Account)
    }

    /// The door accounts and assets of the bridge this object correlates to.
    /// Raw bytes; XCHAIN_BRIDGE is not yet typed in Rust.
    fn get_xchain_bridge(&self) -> Result<[u8; RAW_UNMAPPED_FIELD_SIZE]> {
        let mut buffer = [0u8; RAW_UNMAPPED_FIELD_SIZE];
        let result_code = unsafe {
            get_current_ledger_obj_field(
                sfield::XChainBridge.into(),
                buffer.as_mut_ptr(),
                buffer.len(),
            )
        };
        match_result_code(result_code, || buffer)
    }

    /// An integer that determines the order that accounts created through cross-chain transfers must be performed. Smaller numbers must execute before larger numbers.
    fn get_xchain_account_create_count(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::XChainAccountCreateCount)
    }

    /// Attestations collected from the witness servers. This includes the parameters needed to recreate the message that was signed, including the amount, destination, signature reward amount, and reward account for that signature. With the exception of the reward account, all signatures must sign the message created with common parameters.
    fn get_xchain_create_account_attestations(&self) -> Result<Array> {
        current_ledger_object::get_field(sfield::XChainCreateAccountAttestations)
    }

    /// The OwnerNode field (Required).
    fn get_owner_node(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::OwnerNode)
    }

    /// The PreviousTxnID field (Required).
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::PreviousTxnID)
    }

    /// The PreviousTxnLgrSeq field (Required).
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::PreviousTxnLgrSeq)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct XChainOwnedCreateAccountClaimID {
    pub(crate) slot_num: i32,
}

impl XChainOwnedCreateAccountClaimID {
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }
}

impl LedgerObjectCommonFields for XChainOwnedCreateAccountClaimID {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl XChainOwnedCreateAccountClaimIDFields for XChainOwnedCreateAccountClaimID {}

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

        let obj = XChainOwnedCreateAccountClaimID::new(0);

        assert!(obj.get_account().is_ok());
        assert!(obj.get_xchain_bridge().is_ok());
        assert!(obj.get_xchain_account_create_count().is_ok());
        assert!(obj.get_owner_node().is_ok());
        assert!(obj.get_previous_txn_id().is_ok());
        assert!(obj.get_previous_txn_lgr_seq().is_ok());
    }
}
