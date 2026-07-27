// GENERATED -- do not hand-edit. Run scripts/generate-ledger-objects.sh to regenerate.

use xrpl_common_stdlib::host::Result;
use xrpl_common_stdlib::objects::current_ledger_object;
use xrpl_common_stdlib::objects::traits::EscrowFields;
use xrpl_common_stdlib::objects::traits::{
    CurrentLedgerObjectCommonFields, LedgerObjectCommonFields,
};
use xrpl_common_stdlib::sfield;
use xrpl_common_stdlib::types::account_id::AccountID;
use xrpl_common_stdlib::types::amount::Amount;
use xrpl_common_stdlib::types::blob::{ConditionBlob, WasmBlob};
use xrpl_common_stdlib::types::uint::Hash256;

/// Trait providing access to fields specific to the current Escrow object.
pub trait CurrentEscrowFields: CurrentLedgerObjectCommonFields {
    /// The address of the owner (sender) of this escrow. This is the account that provided the XRP
    /// and gets it back if the escrow is canceled.
    fn get_account(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Account)
    }

    /// The Sequence field (Optional).
    fn get_sequence(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::Sequence)
    }

    /// The destination address where the XRP is paid if the escrow is successful.
    fn get_destination(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Destination)
    }

    /// The amount of XRP, in drops, currently held in the escrow.
    fn get_amount(&self) -> Result<Amount> {
        current_ledger_object::get_field(sfield::Amount)
    }

    /// A PREIMAGE-SHA-256 crypto-condition in full crypto-condition format. If present, the EscrowFinish
    /// transaction must contain a fulfillment that satisfies this condition.
    fn get_condition(&self) -> Result<Option<ConditionBlob>> {
        current_ledger_object::get_field_optional(sfield::Condition)
    }

    /// The escrow can be canceled if and only if this field is present and the time it specifies
    /// has passed. Specifically, this is specified as seconds since the Ripple Epoch and it
    /// "has passed" if it's earlier than the close time of the previous validated ledger.
    fn get_cancel_after(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::CancelAfter)
    }

    /// The time, in seconds since the Ripple Epoch, after which this escrow can be finished. Any
    /// EscrowFinish transaction before this time fails. (Specifically, this is compared with the
    /// close time of the previous validated ledger.)
    fn get_finish_after(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::FinishAfter)
    }

    /// The WASM code that is executing.
    fn get_finish_function(&self) -> Result<Option<WasmBlob>> {
        current_ledger_object::get_field_optional(sfield::FinishFunction)
    }

    // SKIPPED get_data: hand-written (ContractData semantics)

    /// An arbitrary tag to further specify the source for this escrow, such as a hosted recipient
    /// at the owner's address.
    fn get_source_tag(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::SourceTag)
    }

    /// An arbitrary tag to further specify the destination for this escrow, such as a hosted
    /// recipient at the destination address.
    fn get_destination_tag(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::DestinationTag)
    }

    /// A hint indicating which page of the sender's owner directory links to this entry, in case
    /// the directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::OwnerNode)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::PreviousTxnLgrSeq)
    }

    /// A hint indicating which page of the destination's owner directory links to this object, in
    /// case the directory consists of multiple pages. Omitted on escrows created before enabling the fix1523 amendment.
    fn get_destination_node(&self) -> Result<Option<u64>> {
        current_ledger_object::get_field_optional(sfield::DestinationNode)
    }

    /// The TransferRate field (Optional).
    fn get_transfer_rate(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::TransferRate)
    }

    /// The IssuerNode field (Optional).
    fn get_issuer_node(&self) -> Result<Option<u64>> {
        current_ledger_object::get_field_optional(sfield::IssuerNode)
    }
}

/// `lsf*` flag constants for Escrow objects.
#[allow(non_upper_case_globals, dead_code)]
pub mod escrow_flags {
    // No lsf* flags are defined for Escrow in LedgerFormats.h.
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Escrow {
    pub(crate) slot_num: i32,
}

impl Escrow {
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
