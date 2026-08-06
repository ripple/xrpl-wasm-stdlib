// GENERATED -- do not hand-edit. Run scripts/generate-ledger-objects.sh to regenerate.

use crate::host::Result;
use crate::objects::traits::CurrentLedgerObjectCommonFields;
use crate::objects::traits::LedgerObjectCommonFields;
use crate::objects::{current_ledger_object, ledger_object};
use crate::sfield;
use crate::types::account_id::AccountID;
use crate::types::amount::Amount;
use crate::types::uint::Hash256;

/// Trait providing access to fields specific to Sponsorship objects in any ledger.
pub trait SponsorshipFields: LedgerObjectCommonFields {
    /// The PreviousTxnID field (Required).
    fn previous_txn_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The PreviousTxnLgrSeq field (Required).
    fn previous_txn_lgr_seq(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }

    /// The Owner field (Required).
    fn owner(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Owner)
    }

    /// The Sponsee field (Required).
    fn sponsee(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Sponsee)
    }

    /// The FeeAmount field (Optional).
    fn fee_amount(&self) -> Result<Option<Amount>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::FeeAmount)
    }

    /// The MaxFee field (Optional).
    fn max_fee(&self) -> Result<Option<Amount>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::MaxFee)
    }

    /// The RemainingOwnerCount field (Optional).
    fn remaining_owner_count(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::RemainingOwnerCount)
    }

    /// The OwnerNode field (Required).
    fn owner_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::OwnerNode)
    }

    /// The SponseeNode field (Required).
    fn sponsee_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::SponseeNode)
    }
}

/// Trait providing access to fields specific to the current Sponsorship object.
pub trait CurrentSponsorshipFields: CurrentLedgerObjectCommonFields {
    /// The PreviousTxnID field (Required).
    fn previous_txn_id(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::PreviousTxnID)
    }

    /// The PreviousTxnLgrSeq field (Required).
    fn previous_txn_lgr_seq(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::PreviousTxnLgrSeq)
    }

    /// The Owner field (Required).
    fn owner(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Owner)
    }

    /// The Sponsee field (Required).
    fn sponsee(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Sponsee)
    }

    /// The FeeAmount field (Optional).
    fn fee_amount(&self) -> Result<Option<Amount>> {
        current_ledger_object::get_field_optional(sfield::FeeAmount)
    }

    /// The MaxFee field (Optional).
    fn max_fee(&self) -> Result<Option<Amount>> {
        current_ledger_object::get_field_optional(sfield::MaxFee)
    }

    /// The RemainingOwnerCount field (Optional).
    fn remaining_owner_count(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::RemainingOwnerCount)
    }

    /// The OwnerNode field (Required).
    fn owner_node(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::OwnerNode)
    }

    /// The SponseeNode field (Required).
    fn sponsee_node(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::SponseeNode)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Sponsorship {
    pub(crate) slot_num: i32,
}

impl Sponsorship {
    /// Binds this handle to a host-managed slot holding a Sponsorship ledger object.
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }
}

impl LedgerObjectCommonFields for Sponsorship {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl SponsorshipFields for Sponsorship {}

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

        let obj = Sponsorship::new(0);

        assert!(obj.previous_txn_id().is_ok());
        assert!(obj.previous_txn_lgr_seq().is_ok());
        assert!(obj.owner().is_ok());
        assert!(obj.sponsee().is_ok());
        assert!(obj.owner_node().is_ok());
        assert!(obj.sponsee_node().is_ok());
        assert!(obj.fee_amount().is_ok());
        assert!(obj.max_fee().is_ok());
        assert!(obj.remaining_owner_count().is_ok());
    }

    #[test]
    fn optional_fields_none() {
        let mut mock = MockHostBindings::new();
        mock_all_fields_not_found(&mut mock);
        let _guard = setup_mock(mock);

        let obj = Sponsorship::new(0);

        assert!(obj.remaining_owner_count().unwrap().is_none());
    }
}
