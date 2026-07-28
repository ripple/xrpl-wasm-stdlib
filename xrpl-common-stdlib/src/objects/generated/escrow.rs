// GENERATED -- do not hand-edit. Run scripts/generate-ledger-objects.sh to regenerate.

use crate::host::Result;
use crate::objects::ledger_object;
use crate::objects::traits::LedgerObjectCommonFields;
use crate::sfield;
use crate::types::account_id::AccountID;
use crate::types::amount::Amount;
use crate::types::blob::{ConditionBlob, WasmBlob};
use crate::types::uint::Hash256;

/// Trait providing access to fields specific to Escrow objects in any ledger.
pub trait EscrowFields: LedgerObjectCommonFields {
    /// The address of the owner (sender) of this escrow. This is the account that provided the funds, and gets it back if the escrow is canceled.
    fn get_account(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Account)
    }

    /// The Sequence field (Optional).
    fn get_sequence(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::Sequence)
    }

    /// The destination address where the XRP is paid if the escrow is successful.
    fn get_destination(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Destination)
    }

    /// The amount to be delivered by the payment in escrow. The amount can be XRP, or with the TokenEscrow amendment, a fungible token.
    fn get_amount(&self) -> Result<Amount> {
        ledger_object::get_field(self.get_slot_num(), sfield::Amount)
    }

    /// A PREIMAGE-SHA-256 crypto-condition, as hexadecimal. If present, the [EscrowFinish transaction][] must contain a fulfillment that satisfies this condition.
    fn get_condition(&self) -> Result<Option<ConditionBlob>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::Condition)
    }

    /// The escrow can be canceled if and only if this field is present _and_ the time it specifies has passed. Specifically, this is specified as [seconds since the Ripple Epoch][] and it "has passed" if it's earlier than the close time of the previous validated ledger.
    fn get_cancel_after(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::CancelAfter)
    }

    /// The time, in [seconds since the Ripple Epoch][], after which this escrow can be finished. Any [EscrowFinish transaction][] before this time fails. (Specifically, this is compared with the close time of the previous validated ledger.)
    fn get_finish_after(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::FinishAfter)
    }

    /// The FinishFunction field (Optional).
    fn get_finish_function(&self) -> Result<Option<WasmBlob>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::FinishFunction)
    }

    /// An arbitrary tag to further specify the source for this escrow, such as a hosted recipient at the owner's address.
    fn get_source_tag(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::SourceTag)
    }

    /// An arbitrary tag to further specify the destination for this escrow, such as a hosted recipient at the destination address.
    fn get_destination_tag(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::DestinationTag)
    }

    /// A hint indicating which page of the sender's owner directory links to this entry, in case the directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::OwnerNode)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The [index of the ledger][Ledger Index] that contains the transaction that most recently modified this entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }

    /// A hint indicating which page of the destination's owner directory links to this object, in case the directory consists of multiple pages. Omitted on escrows created before enabling the [fix1523 amendment][].
    fn get_destination_node(&self) -> Result<Option<u64>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::DestinationNode)
    }

    /// The transfer rate or fee to charge when users finish an escrow, locked at the creation of an escrow contract and used during settlement. Applicable to Trust Line Tokens and MPTs only.
    fn get_transfer_rate(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::TransferRate)
    }

    /// The ledger index of the issuer's directory node associated with the `Escrow`. Used when the issuer is neither the source nor destination account.
    fn get_issuer_node(&self) -> Result<Option<u64>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::IssuerNode)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Escrow {
    pub(crate) slot_num: i32,
}

impl Escrow {
    /// Binds this handle to a host-managed slot holding a Escrow ledger object.
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }
}

impl LedgerObjectCommonFields for Escrow {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl EscrowFields for Escrow {}

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

        let obj = Escrow::new(0);

        assert!(obj.get_account().is_ok());
        assert!(obj.get_destination().is_ok());
        assert!(obj.get_amount().is_ok());
        assert!(obj.get_owner_node().is_ok());
        assert!(obj.get_previous_txn_id().is_ok());
        assert!(obj.get_previous_txn_lgr_seq().is_ok());
        assert!(obj.get_sequence().is_ok());
        assert!(obj.get_condition().is_ok());
        assert!(obj.get_cancel_after().is_ok());
        assert!(obj.get_finish_after().is_ok());
        assert!(obj.get_finish_function().is_ok());
        assert!(obj.get_source_tag().is_ok());
        assert!(obj.get_destination_tag().is_ok());
        assert!(obj.get_destination_node().is_ok());
        assert!(obj.get_transfer_rate().is_ok());
        assert!(obj.get_issuer_node().is_ok());
    }

    #[test]
    fn optional_fields_none() {
        let mut mock = MockHostBindings::new();
        mock_all_fields_not_found(&mut mock);
        let _guard = setup_mock(mock);

        let obj = Escrow::new(0);

        assert!(obj.get_sequence().unwrap().is_none());
        assert!(obj.get_cancel_after().unwrap().is_none());
        assert!(obj.get_finish_after().unwrap().is_none());
        assert!(obj.get_source_tag().unwrap().is_none());
        assert!(obj.get_destination_tag().unwrap().is_none());
        assert!(obj.get_destination_node().unwrap().is_none());
        assert!(obj.get_transfer_rate().unwrap().is_none());
        assert!(obj.get_issuer_node().unwrap().is_none());
    }
}
