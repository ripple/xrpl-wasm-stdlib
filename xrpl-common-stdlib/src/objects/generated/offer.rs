// GENERATED -- do not hand-edit. Run scripts/generate-ledger-objects.sh to regenerate.

use crate::host::Result;
use crate::objects::traits::CurrentLedgerObjectCommonFields;
use crate::objects::traits::LedgerObjectCommonFields;
use crate::objects::{current_ledger_object, ledger_object};
use crate::sfield;
use crate::types::account_id::AccountID;
use crate::types::amount::Amount;
use crate::types::uint::Hash256;

/// Trait providing access to fields specific to Offer objects in any ledger.
pub trait OfferFields: LedgerObjectCommonFields {
    /// The account that owns this offer.
    fn account(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Account)
    }

    /// The `Sequence` value of the OfferCreate transaction that created this offer. Used in
    /// combination with the `Account` to identify this offer.
    fn sequence(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::Sequence)
    }

    /// The remaining amount and type of currency requested by the offer creator.
    fn taker_pays(&self) -> Result<Amount> {
        ledger_object::get_field(self.get_slot_num(), sfield::TakerPays)
    }

    /// The remaining amount and type of currency being provided by the offer creator.
    fn taker_gets(&self) -> Result<Amount> {
        ledger_object::get_field(self.get_slot_num(), sfield::TakerGets)
    }

    /// The ID of the offer directory that links to this offer.
    fn book_directory(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::BookDirectory)
    }

    /// A hint indicating which page of the offer directory links to this entry, in case the
    /// directory consists of multiple pages.
    fn book_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::BookNode)
    }

    /// A hint indicating which page of the owner directory links to this entry, in case the
    /// directory consists of multiple pages.
    fn owner_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::OwnerNode)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn previous_txn_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// object.
    fn previous_txn_lgr_seq(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }

    /// Indicates the time after which this offer is considered unfunded. See Specifying Time for
    /// details.
    fn expiration(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::Expiration)
    }

    /// The ledger entry ID of a permissioned domain. If present, this offer belongs to the
    /// corresponding Permissioned DEX.
    fn domain_id(&self) -> Result<Option<Hash256>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::DomainID)
    }
}

/// Trait providing access to fields specific to the current Offer object.
pub trait CurrentOfferFields: CurrentLedgerObjectCommonFields {
    /// The account that owns this offer.
    fn account(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Account)
    }

    /// The `Sequence` value of the OfferCreate transaction that created this offer. Used in
    /// combination with the `Account` to identify this offer.
    fn sequence(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::Sequence)
    }

    /// The remaining amount and type of currency requested by the offer creator.
    fn taker_pays(&self) -> Result<Amount> {
        current_ledger_object::get_field(sfield::TakerPays)
    }

    /// The remaining amount and type of currency being provided by the offer creator.
    fn taker_gets(&self) -> Result<Amount> {
        current_ledger_object::get_field(sfield::TakerGets)
    }

    /// The ID of the offer directory that links to this offer.
    fn book_directory(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::BookDirectory)
    }

    /// A hint indicating which page of the offer directory links to this entry, in case the
    /// directory consists of multiple pages.
    fn book_node(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::BookNode)
    }

    /// A hint indicating which page of the owner directory links to this entry, in case the
    /// directory consists of multiple pages.
    fn owner_node(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::OwnerNode)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn previous_txn_id(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// object.
    fn previous_txn_lgr_seq(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::PreviousTxnLgrSeq)
    }

    /// Indicates the time after which this offer is considered unfunded. See Specifying Time for
    /// details.
    fn expiration(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::Expiration)
    }

    /// The ledger entry ID of a permissioned domain. If present, this offer belongs to the
    /// corresponding Permissioned DEX.
    fn domain_id(&self) -> Result<Option<Hash256>> {
        current_ledger_object::get_field_optional(sfield::DomainID)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Offer {
    pub(crate) slot_num: i32,
}

impl Offer {
    /// Binds this handle to a host-managed slot holding an Offer ledger object.
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }
}

impl LedgerObjectCommonFields for Offer {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl OfferFields for Offer {}

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

        let obj = Offer::new(0);

        assert!(obj.account().is_ok());
        assert!(obj.sequence().is_ok());
        assert!(obj.taker_pays().is_ok());
        assert!(obj.taker_gets().is_ok());
        assert!(obj.book_directory().is_ok());
        assert!(obj.book_node().is_ok());
        assert!(obj.owner_node().is_ok());
        assert!(obj.previous_txn_id().is_ok());
        assert!(obj.previous_txn_lgr_seq().is_ok());
        assert!(obj.expiration().is_ok());
        assert!(obj.domain_id().is_ok());
    }

    #[test]
    fn optional_fields_none() {
        let mut mock = MockHostBindings::new();
        mock_all_fields_not_found(&mut mock);
        let _guard = setup_mock(mock);

        let obj = Offer::new(0);

        assert!(obj.expiration().unwrap().is_none());
        assert!(obj.domain_id().unwrap().is_none());
    }
}
