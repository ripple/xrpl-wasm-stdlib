// GENERATED -- do not hand-edit. Run scripts/generate-ledger-objects.sh to regenerate.

use crate::host::Result;
use crate::objects::ledger_object;
use crate::objects::traits::LedgerObjectCommonFields;
use crate::sfield;
use crate::types::account_id::AccountID;
use crate::types::amount::Amount;
use crate::types::blob::{ConditionBlob, StandardBlob, WasmBlob};
use crate::types::uint::Hash256;

/// Trait providing access to fields specific to Escrow objects in any ledger.
pub trait EscrowFields: LedgerObjectCommonFields {
    /// The address of the owner (sender) of this escrow. This is the account that provided the
    /// funds, and gets it back if the escrow is canceled.
    fn account(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Account)
    }

    /// The Sequence field (Optional).
    fn sequence(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::Sequence)
    }

    /// The destination address where the XRP is paid if the escrow is successful.
    fn destination(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Destination)
    }

    /// The amount to be delivered by the payment in escrow. The amount can be XRP, or with the
    /// TokenEscrow amendment, a fungible token.
    fn amount(&self) -> Result<Amount> {
        ledger_object::get_field(self.get_slot_num(), sfield::Amount)
    }

    /// A PREIMAGE-SHA-256 crypto-condition, as hexadecimal. If present, the EscrowFinish
    /// transaction must contain a fulfillment that satisfies this condition.
    fn condition(&self) -> Result<Option<ConditionBlob>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::Condition)
    }

    /// The escrow can be canceled if and only if this field is present _and_ the time it specifies
    /// has passed. Specifically, this is specified as seconds since the Ripple Epoch and it "has
    /// passed" if it's earlier than the close time of the previous validated ledger.
    fn cancel_after(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::CancelAfter)
    }

    /// The time, in seconds since the Ripple Epoch, after which this escrow can be finished. Any
    /// EscrowFinish transaction before this time fails. (Specifically, this is compared with the
    /// close time of the previous validated ledger.)
    fn finish_after(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::FinishAfter)
    }

    /// The Bytecode field (Optional).
    fn bytecode(&self) -> Result<Option<WasmBlob>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::Bytecode)
    }

    /// The Data field (Optional).
    fn data(&self) -> Result<Option<StandardBlob>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::Data)
    }

    /// An arbitrary tag to further specify the source for this escrow, such as a hosted recipient
    /// at the owner's address.
    fn source_tag(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::SourceTag)
    }

    /// An arbitrary tag to further specify the destination for this escrow, such as a hosted
    /// recipient at the destination address.
    fn destination_tag(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::DestinationTag)
    }

    /// A hint indicating which page of the sender's owner directory links to this entry, in case
    /// the directory consists of multiple pages.
    fn owner_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::OwnerNode)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn previous_txn_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn previous_txn_lgr_seq(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }

    /// A hint indicating which page of the destination's owner directory links to this object, in
    /// case the directory consists of multiple pages. Omitted on escrows created before enabling
    /// the fix1523 amendment.
    fn destination_node(&self) -> Result<Option<u64>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::DestinationNode)
    }

    /// The transfer rate or fee to charge when users finish an escrow, locked at the creation of an
    /// escrow contract and used during settlement. Applicable to Trust Line Tokens and MPTs only.
    fn transfer_rate(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::TransferRate)
    }

    /// The ledger index of the issuer's directory node associated with the `Escrow`. Used when the
    /// issuer is neither the source nor destination account.
    fn issuer_node(&self) -> Result<Option<u64>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::IssuerNode)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Escrow {
    pub(crate) slot_num: i32,
}

impl Escrow {
    /// Binds this handle to a host-managed slot holding an Escrow ledger object.
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
    use crate::objects::test_utils::*;

    #[test]
    fn read_all_fields() {
        let mut mock = MockHostBindings::new();
        mock_all_fields_present(&mut mock);
        let _guard = setup_mock(mock);

        let obj = Escrow::new(0);

        assert!(obj.account().is_ok());
        assert!(obj.destination().is_ok());
        assert!(obj.amount().is_ok());
        assert!(obj.owner_node().is_ok());
        assert!(obj.previous_txn_id().is_ok());
        assert!(obj.previous_txn_lgr_seq().is_ok());
        assert!(obj.sequence().is_ok());
        assert!(obj.condition().is_ok());
        assert!(obj.cancel_after().is_ok());
        assert!(obj.finish_after().is_ok());
        assert!(obj.bytecode().is_ok());
        assert!(obj.data().is_ok());
        assert!(obj.source_tag().is_ok());
        assert!(obj.destination_tag().is_ok());
        assert!(obj.destination_node().is_ok());
        assert!(obj.transfer_rate().is_ok());
        assert!(obj.issuer_node().is_ok());
    }

    #[test]
    fn optional_fields_none() {
        let mut mock = MockHostBindings::new();
        mock_all_fields_not_found(&mut mock);
        let _guard = setup_mock(mock);

        let obj = Escrow::new(0);

        assert!(obj.sequence().unwrap().is_none());
        assert!(obj.cancel_after().unwrap().is_none());
        assert!(obj.finish_after().unwrap().is_none());
        assert!(obj.source_tag().unwrap().is_none());
        assert!(obj.destination_tag().unwrap().is_none());
        assert!(obj.destination_node().unwrap().is_none());
        assert!(obj.transfer_rate().unwrap().is_none());
        assert!(obj.issuer_node().unwrap().is_none());
    }
}
