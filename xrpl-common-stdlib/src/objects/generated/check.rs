// GENERATED -- do not hand-edit. Run scripts/generate-ledger-objects.sh to regenerate.

use crate::host::Result;
use crate::objects::traits::CurrentLedgerObjectCommonFields;
use crate::objects::traits::LedgerObjectCommonFields;
use crate::objects::{current_ledger_object, ledger_object};
use crate::sfield;
use crate::types::account_id::AccountID;
use crate::types::amount::Amount;
use crate::types::uint::Hash256;

/// Trait providing access to fields specific to Check objects in any ledger.
pub trait CheckFields: LedgerObjectCommonFields {
    /// The sender of the Check. Cashing the Check debits this address's balance.
    fn get_account(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Account)
    }

    /// The intended recipient of the Check. Only this address can cash the Check, using a [CheckCash transaction][].
    fn get_destination(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Destination)
    }

    /// The maximum amount of currency this Check can debit the sender. If the Check is successfully cashed, the destination is credited in the same currency for up to this amount.
    fn get_send_max(&self) -> Result<Amount> {
        ledger_object::get_field(self.get_slot_num(), sfield::SendMax)
    }

    /// The sequence number of the [CheckCreate transaction][] that created this check.
    fn get_sequence(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::Sequence)
    }

    /// A hint indicating which page of the sender's owner directory links to this object, in case the directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::OwnerNode)
    }

    /// A hint indicating which page of the destination's owner directory links to this object, in case the directory consists of multiple pages.
    fn get_destination_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::DestinationNode)
    }

    /// Indicates the time after which this Check is considered expired. See [Specifying Time][] for details.
    fn get_expiration(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::Expiration)
    }

    /// Arbitrary 256-bit hash provided by the sender as a specific reason or identifier for this Check.
    fn get_invoice_id(&self) -> Result<Option<Hash256>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::InvoiceID)
    }

    /// An arbitrary tag to further specify the source for this Check, such as a hosted recipient at the sender's address.
    fn get_source_tag(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::SourceTag)
    }

    /// An arbitrary tag to further specify the destination for this Check, such as a hosted recipient at the destination address.
    fn get_destination_tag(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::DestinationTag)
    }

    /// The identifying hash of the transaction that most recently modified this object.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The [index of the ledger][Ledger Index] that contains the transaction that most recently modified this object.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }
}

/// Trait providing access to fields specific to the current Check object.
pub trait CurrentCheckFields: CurrentLedgerObjectCommonFields {
    /// The sender of the Check. Cashing the Check debits this address's balance.
    fn get_account(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Account)
    }

    /// The intended recipient of the Check. Only this address can cash the Check, using a [CheckCash transaction][].
    fn get_destination(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Destination)
    }

    /// The maximum amount of currency this Check can debit the sender. If the Check is successfully cashed, the destination is credited in the same currency for up to this amount.
    fn get_send_max(&self) -> Result<Amount> {
        current_ledger_object::get_field(sfield::SendMax)
    }

    /// The sequence number of the [CheckCreate transaction][] that created this check.
    fn get_sequence(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::Sequence)
    }

    /// A hint indicating which page of the sender's owner directory links to this object, in case the directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::OwnerNode)
    }

    /// A hint indicating which page of the destination's owner directory links to this object, in case the directory consists of multiple pages.
    fn get_destination_node(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::DestinationNode)
    }

    /// Indicates the time after which this Check is considered expired. See [Specifying Time][] for details.
    fn get_expiration(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::Expiration)
    }

    /// Arbitrary 256-bit hash provided by the sender as a specific reason or identifier for this Check.
    fn get_invoice_id(&self) -> Result<Option<Hash256>> {
        current_ledger_object::get_field_optional(sfield::InvoiceID)
    }

    /// An arbitrary tag to further specify the source for this Check, such as a hosted recipient at the sender's address.
    fn get_source_tag(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::SourceTag)
    }

    /// An arbitrary tag to further specify the destination for this Check, such as a hosted recipient at the destination address.
    fn get_destination_tag(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::DestinationTag)
    }

    /// The identifying hash of the transaction that most recently modified this object.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::PreviousTxnID)
    }

    /// The [index of the ledger][Ledger Index] that contains the transaction that most recently modified this object.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::PreviousTxnLgrSeq)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Check {
    pub(crate) slot_num: i32,
}

impl Check {
    /// Binds this handle to a host-managed slot holding a Check ledger object.
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }
}

impl LedgerObjectCommonFields for Check {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl CheckFields for Check {}

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

        let obj = Check::new(0);

        assert!(obj.get_account().is_ok());
        assert!(obj.get_destination().is_ok());
        assert!(obj.get_send_max().is_ok());
        assert!(obj.get_sequence().is_ok());
        assert!(obj.get_owner_node().is_ok());
        assert!(obj.get_destination_node().is_ok());
        assert!(obj.get_previous_txn_id().is_ok());
        assert!(obj.get_previous_txn_lgr_seq().is_ok());
        assert!(obj.get_expiration().is_ok());
        assert!(obj.get_invoice_id().is_ok());
        assert!(obj.get_source_tag().is_ok());
        assert!(obj.get_destination_tag().is_ok());
    }

    #[test]
    fn optional_fields_none() {
        let mut mock = MockHostBindings::new();
        mock_all_fields_not_found(&mut mock);
        let _guard = setup_mock(mock);

        let obj = Check::new(0);

        assert!(obj.get_expiration().unwrap().is_none());
        assert!(obj.get_invoice_id().unwrap().is_none());
        assert!(obj.get_source_tag().unwrap().is_none());
        assert!(obj.get_destination_tag().unwrap().is_none());
    }
}
