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
use crate::types::amount::Amount;
use crate::types::uint::Hash256;

/// Trait providing access to fields specific to Bridge objects in any ledger.
pub trait BridgeFields: LedgerObjectCommonFields {
    /// The account that submitted the `XChainCreateBridge` transaction on the blockchain.
    fn account(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Account)
    }

    /// The total amount, in XRP, to be rewarded for providing a signature for cross-chain transfer or for signing for the cross-chain reward. This amount will be split among the signers.
    fn signature_reward(&self) -> Result<Amount> {
        ledger_object::get_field(self.get_slot_num(), sfield::SignatureReward)
    }

    /// The minimum amount, in XRP, required for an `XChainAccountCreateCommit` transaction. If this isn't present, the `XChainAccountCreateCommit` transaction will fail. This field can only be present on XRP-XRP bridges.
    fn min_account_create_amount(&self) -> Result<Option<Amount>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::MinAccountCreateAmount)
    }

    /// The door accounts and assets of the bridge this object correlates to.
    /// Raw bytes; XCHAIN_BRIDGE is not yet typed in Rust.
    fn xchain_bridge(&self) -> Result<[u8; RAW_UNMAPPED_FIELD_SIZE]> {
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

    /// The value of the next `XChainClaimID` to be created.
    fn xchain_claim_id(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::XChainClaimID)
    }

    /// A counter used to order the execution of account create transactions. It is incremented every time a successful `XChainAccountCreateCommit` transaction is run for the source chain.
    fn xchain_account_create_count(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::XChainAccountCreateCount)
    }

    /// A counter used to order the execution of account create transactions. It is incremented every time a `XChainAccountCreateCommit` transaction is "claimed" on the destination chain. When the "claim" transaction is run on the destination chain, the `XChainAccountClaimCount` must match the value that the `XChainAccountCreateCount` had at the time the `XChainAccountClaimCount` was run on the source chain. This orders the claims so that they run in the same order that the `XChainAccountCreateCommit` transactions ran on the source chain, to prevent transaction replay.
    fn xchain_account_claim_count(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::XChainAccountClaimCount)
    }

    /// The OwnerNode field (Required).
    fn owner_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::OwnerNode)
    }

    /// The PreviousTxnID field (Required).
    fn previous_txn_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The PreviousTxnLgrSeq field (Required).
    fn previous_txn_lgr_seq(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }
}

/// Trait providing access to fields specific to the current Bridge object.
pub trait CurrentBridgeFields: CurrentLedgerObjectCommonFields {
    /// The account that submitted the `XChainCreateBridge` transaction on the blockchain.
    fn account(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Account)
    }

    /// The total amount, in XRP, to be rewarded for providing a signature for cross-chain transfer or for signing for the cross-chain reward. This amount will be split among the signers.
    fn signature_reward(&self) -> Result<Amount> {
        current_ledger_object::get_field(sfield::SignatureReward)
    }

    /// The minimum amount, in XRP, required for an `XChainAccountCreateCommit` transaction. If this isn't present, the `XChainAccountCreateCommit` transaction will fail. This field can only be present on XRP-XRP bridges.
    fn min_account_create_amount(&self) -> Result<Option<Amount>> {
        current_ledger_object::get_field_optional(sfield::MinAccountCreateAmount)
    }

    /// The door accounts and assets of the bridge this object correlates to.
    /// Raw bytes; XCHAIN_BRIDGE is not yet typed in Rust.
    fn xchain_bridge(&self) -> Result<[u8; RAW_UNMAPPED_FIELD_SIZE]> {
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

    /// The value of the next `XChainClaimID` to be created.
    fn xchain_claim_id(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::XChainClaimID)
    }

    /// A counter used to order the execution of account create transactions. It is incremented every time a successful `XChainAccountCreateCommit` transaction is run for the source chain.
    fn xchain_account_create_count(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::XChainAccountCreateCount)
    }

    /// A counter used to order the execution of account create transactions. It is incremented every time a `XChainAccountCreateCommit` transaction is "claimed" on the destination chain. When the "claim" transaction is run on the destination chain, the `XChainAccountClaimCount` must match the value that the `XChainAccountCreateCount` had at the time the `XChainAccountClaimCount` was run on the source chain. This orders the claims so that they run in the same order that the `XChainAccountCreateCommit` transactions ran on the source chain, to prevent transaction replay.
    fn xchain_account_claim_count(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::XChainAccountClaimCount)
    }

    /// The OwnerNode field (Required).
    fn owner_node(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::OwnerNode)
    }

    /// The PreviousTxnID field (Required).
    fn previous_txn_id(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::PreviousTxnID)
    }

    /// The PreviousTxnLgrSeq field (Required).
    fn previous_txn_lgr_seq(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::PreviousTxnLgrSeq)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Bridge {
    pub(crate) slot_num: i32,
}

impl Bridge {
    /// Binds this handle to a host-managed slot holding a Bridge ledger object.
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }
}

impl LedgerObjectCommonFields for Bridge {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl BridgeFields for Bridge {}

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

        let obj = Bridge::new(0);

        assert!(obj.account().is_ok());
        assert!(obj.signature_reward().is_ok());
        assert!(obj.xchain_bridge().is_ok());
        assert!(obj.xchain_claim_id().is_ok());
        assert!(obj.xchain_account_create_count().is_ok());
        assert!(obj.xchain_account_claim_count().is_ok());
        assert!(obj.owner_node().is_ok());
        assert!(obj.previous_txn_id().is_ok());
        assert!(obj.previous_txn_lgr_seq().is_ok());
        assert!(obj.min_account_create_amount().is_ok());
    }
}
