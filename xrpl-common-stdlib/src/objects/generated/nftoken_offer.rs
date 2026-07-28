// GENERATED -- do not hand-edit. Run scripts/generate-ledger-objects.sh to regenerate.

use crate::host::Result;
use crate::objects::traits::CurrentLedgerObjectCommonFields;
use crate::objects::traits::LedgerObjectCommonFields;
use crate::objects::{current_ledger_object, ledger_object};
use crate::sfield;
use crate::types::account_id::AccountID;
use crate::types::amount::Amount;
use crate::types::uint::Hash256;

/// Trait providing access to fields specific to NFTokenOffer objects in any ledger.
pub trait NFTokenOfferFields: LedgerObjectCommonFields {
    /// The Owner field (Required).
    fn get_owner(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Owner)
    }

    /// The NFTokenID field (Required).
    fn get_nftoken_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::NFTokenID)
    }

    /// The Amount field (Required).
    fn get_amount(&self) -> Result<Amount> {
        ledger_object::get_field(self.get_slot_num(), sfield::Amount)
    }

    /// The OwnerNode field (Required).
    fn get_owner_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::OwnerNode)
    }

    /// The NFTokenOfferNode field (Required).
    fn get_nftoken_offer_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::NFTokenOfferNode)
    }

    /// The Destination field (Optional).
    fn get_destination(&self) -> Result<Option<AccountID>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::Destination)
    }

    /// The Expiration field (Optional).
    fn get_expiration(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::Expiration)
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

/// Trait providing access to fields specific to the current NFTokenOffer object.
pub trait CurrentNFTokenOfferFields: CurrentLedgerObjectCommonFields {
    /// The Owner field (Required).
    fn get_owner(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Owner)
    }

    /// The NFTokenID field (Required).
    fn get_nftoken_id(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::NFTokenID)
    }

    /// The Amount field (Required).
    fn get_amount(&self) -> Result<Amount> {
        current_ledger_object::get_field(sfield::Amount)
    }

    /// The OwnerNode field (Required).
    fn get_owner_node(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::OwnerNode)
    }

    /// The NFTokenOfferNode field (Required).
    fn get_nftoken_offer_node(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::NFTokenOfferNode)
    }

    /// The Destination field (Optional).
    fn get_destination(&self) -> Result<Option<AccountID>> {
        current_ledger_object::get_field_optional(sfield::Destination)
    }

    /// The Expiration field (Optional).
    fn get_expiration(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::Expiration)
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
pub struct NFTokenOffer {
    pub(crate) slot_num: i32,
}

impl NFTokenOffer {
    /// Binds this handle to a host-managed slot holding a NFTokenOffer ledger object.
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }
}

impl LedgerObjectCommonFields for NFTokenOffer {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl NFTokenOfferFields for NFTokenOffer {}

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

        let obj = NFTokenOffer::new(0);

        assert!(obj.get_owner().is_ok());
        assert!(obj.get_nftoken_id().is_ok());
        assert!(obj.get_amount().is_ok());
        assert!(obj.get_owner_node().is_ok());
        assert!(obj.get_nftoken_offer_node().is_ok());
        assert!(obj.get_previous_txn_id().is_ok());
        assert!(obj.get_previous_txn_lgr_seq().is_ok());
        assert!(obj.get_destination().is_ok());
        assert!(obj.get_expiration().is_ok());
    }

    #[test]
    fn optional_fields_none() {
        let mut mock = MockHostBindings::new();
        mock_all_fields_not_found(&mut mock);
        let _guard = setup_mock(mock);

        let obj = NFTokenOffer::new(0);

        assert!(obj.get_destination().unwrap().is_none());
        assert!(obj.get_expiration().unwrap().is_none());
    }
}
