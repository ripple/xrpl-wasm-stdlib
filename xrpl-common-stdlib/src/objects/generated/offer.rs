// GENERATED -- do not hand-edit. Run scripts/generate-ledger-objects.sh to regenerate.

use crate::host::Result;
use crate::objects::array_object::Array;
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
    fn get_account(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Account)
    }

    /// The `Sequence` value of the [OfferCreate][] transaction that created this offer. Used in combination with the `Account` to identify this offer.
    fn get_sequence(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::Sequence)
    }

    /// The remaining amount and type of currency requested by the offer creator.
    fn get_taker_pays(&self) -> Result<Amount> {
        ledger_object::get_field(self.get_slot_num(), sfield::TakerPays)
    }

    /// The remaining amount and type of currency being provided by the offer creator.
    fn get_taker_gets(&self) -> Result<Amount> {
        ledger_object::get_field(self.get_slot_num(), sfield::TakerGets)
    }

    /// The ID of the offer directory that links to this offer.
    fn get_book_directory(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::BookDirectory)
    }

    /// A hint indicating which page of the offer directory links to this entry, in case the directory consists of multiple pages.
    fn get_book_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::BookNode)
    }

    /// A hint indicating which page of the owner directory links to this entry, in case the directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::OwnerNode)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The [index of the ledger][Ledger Index] that contains the transaction that most recently modified this object.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }

    /// Indicates the time after which this offer is considered unfunded. See [Specifying Time][] for details.
    fn get_expiration(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::Expiration)
    }

    /// The ledger entry ID of a permissioned domain. If present, this offer belongs to the corresponding Permissioned DEX.
    fn get_domain_id(&self) -> Result<Option<Hash256>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::DomainID)
    }

    /// A list of additional offer directories that link to this offer. This field is only present if this is a hybrid offer in a permissioned DEX. The array always contains exactly 1 entry.
    fn get_additional_books(&self) -> Result<Option<Array>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::AdditionalBooks)
    }
}

/// Trait providing access to fields specific to the current Offer object.
pub trait CurrentOfferFields: CurrentLedgerObjectCommonFields {
    /// The account that owns this offer.
    fn get_account(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Account)
    }

    /// The `Sequence` value of the [OfferCreate][] transaction that created this offer. Used in combination with the `Account` to identify this offer.
    fn get_sequence(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::Sequence)
    }

    /// The remaining amount and type of currency requested by the offer creator.
    fn get_taker_pays(&self) -> Result<Amount> {
        current_ledger_object::get_field(sfield::TakerPays)
    }

    /// The remaining amount and type of currency being provided by the offer creator.
    fn get_taker_gets(&self) -> Result<Amount> {
        current_ledger_object::get_field(sfield::TakerGets)
    }

    /// The ID of the offer directory that links to this offer.
    fn get_book_directory(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::BookDirectory)
    }

    /// A hint indicating which page of the offer directory links to this entry, in case the directory consists of multiple pages.
    fn get_book_node(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::BookNode)
    }

    /// A hint indicating which page of the owner directory links to this entry, in case the directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::OwnerNode)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::PreviousTxnID)
    }

    /// The [index of the ledger][Ledger Index] that contains the transaction that most recently modified this object.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::PreviousTxnLgrSeq)
    }

    /// Indicates the time after which this offer is considered unfunded. See [Specifying Time][] for details.
    fn get_expiration(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::Expiration)
    }

    /// The ledger entry ID of a permissioned domain. If present, this offer belongs to the corresponding Permissioned DEX.
    fn get_domain_id(&self) -> Result<Option<Hash256>> {
        current_ledger_object::get_field_optional(sfield::DomainID)
    }

    /// A list of additional offer directories that link to this offer. This field is only present if this is a hybrid offer in a permissioned DEX. The array always contains exactly 1 entry.
    fn get_additional_books(&self) -> Result<Option<Array>> {
        current_ledger_object::get_field_optional(sfield::AdditionalBooks)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Offer {
    pub(crate) slot_num: i32,
}

impl Offer {
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }

    /// Loads the Offer ledger object identified by the given keylet arguments,
    /// caching it in a host-managed slot.
    pub fn load(owner: &AccountID, seq: u32) -> Result<Self> {
        let keylet = match crate::keylets::offer_keylet(owner, seq) {
            Result::Ok(k) => k,
            Result::Err(e) => return Result::Err(e),
        };
        let slot = unsafe { crate::host::cache_ledger_obj(keylet.as_ptr(), keylet.len(), 0) };
        if slot < 0 {
            return Result::Err(crate::host::Error::from_code(slot));
        }
        Result::Ok(Self { slot_num: slot })
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
    use crate::objects::test_support::*;

    #[test]
    fn read_all_fields() {
        let mut mock = MockHostBindings::new();
        mock_all_fields_present(&mut mock);
        let _guard = setup_mock(mock);

        let obj = Offer::new(0);

        assert!(obj.get_account().is_ok());
        assert!(obj.get_sequence().is_ok());
        assert!(obj.get_taker_pays().is_ok());
        assert!(obj.get_taker_gets().is_ok());
        assert!(obj.get_book_directory().is_ok());
        assert!(obj.get_book_node().is_ok());
        assert!(obj.get_owner_node().is_ok());
        assert!(obj.get_previous_txn_id().is_ok());
        assert!(obj.get_previous_txn_lgr_seq().is_ok());
        assert!(obj.get_expiration().is_ok());
        assert!(obj.get_domain_id().is_ok());
    }

    #[test]
    fn optional_fields_none() {
        let mut mock = MockHostBindings::new();
        mock_all_fields_not_found(&mut mock);
        let _guard = setup_mock(mock);

        let obj = Offer::new(0);

        assert!(obj.get_expiration().unwrap().is_none());
        assert!(obj.get_domain_id().unwrap().is_none());
    }

    #[test]
    fn load_success() {
        let mut mock = MockHostBindings::new();
        mock_offer_keylet_success(&mut mock);
        mock_cache_ledger_obj_success(&mut mock, 7);
        let _guard = setup_mock(mock);

        let result = Offer::load(&sample::account_id(), sample::seq());
        assert!(result.is_ok());
    }

    #[test]
    fn load_cache_error() {
        use crate::host::error_codes::INTERNAL_ERROR;

        let mut mock = MockHostBindings::new();
        mock_offer_keylet_success(&mut mock);
        mock_cache_ledger_obj_error(&mut mock, INTERNAL_ERROR);
        let _guard = setup_mock(mock);

        let result = Offer::load(&sample::account_id(), sample::seq());
        assert!(result.is_err());
    }
}
