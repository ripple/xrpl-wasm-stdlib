// GENERATED -- do not hand-edit. Run scripts/generate-ledger-objects.sh to regenerate.

use crate::host::Result;
use crate::objects::traits::CurrentLedgerObjectCommonFields;
use crate::objects::traits::LedgerObjectCommonFields;
use crate::objects::{current_ledger_object, ledger_object};
use crate::sfield;
use crate::types::account_id::AccountID;
use crate::types::amount::Amount;
use crate::types::uint::Hash256;

/// Trait providing access to fields specific to RippleState objects in any ledger.
pub trait RippleStateFields: LedgerObjectCommonFields {
    /// The balance of the trust line, from the perspective of the low account. A negative balance
    /// indicates that the high account holds tokens issued by the low account. The issuer in this
    /// is always set to the neutral value ACCOUNT_ONE.
    fn balance(&self) -> Result<Amount> {
        ledger_object::get_field(self.get_slot_num(), sfield::Balance)
    }

    /// The limit that the low account has set on the trust line. The `issuer` is the address of the
    /// low account that set this limit.
    fn low_limit(&self) -> Result<Amount> {
        ledger_object::get_field(self.get_slot_num(), sfield::LowLimit)
    }

    /// The limit that the high account has set on the trust line. The `issuer` is the address of
    /// the high account that set this limit.
    fn high_limit(&self) -> Result<Amount> {
        ledger_object::get_field(self.get_slot_num(), sfield::HighLimit)
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

    /// (Omitted in some historical ledgers) A hint indicating which page of the low account's owner
    /// directory links to this entry, in case the directory consists of multiple pages.
    fn low_node(&self) -> Result<Option<u64>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::LowNode)
    }

    /// The inbound quality set by the low account, as an integer in the implied ratio
    /// `LowQualityIn`:1,000,000,000. As a special case, the value 0 is equivalent to 1 billion, or
    /// face value.
    fn low_quality_in(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::LowQualityIn)
    }

    /// The outbound quality set by the low account, as an integer in the implied ratio
    /// `LowQualityOut`:1,000,000,000. As a special case, the value 0 is equivalent to 1 billion, or
    /// face value.
    fn low_quality_out(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::LowQualityOut)
    }

    /// (Omitted in some historical ledgers) A hint indicating which page of the high account's
    /// owner directory links to this entry, in case the directory consists of multiple pages.
    fn high_node(&self) -> Result<Option<u64>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::HighNode)
    }

    /// The inbound quality set by the high account, as an integer in the implied ratio
    /// `HighQualityIn`:1,000,000,000. As a special case, the value 0 is equivalent to 1 billion, or
    /// face value.
    fn high_quality_in(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::HighQualityIn)
    }

    /// The outbound quality set by the high account, as an integer in the implied ratio
    /// `HighQualityOut`:1,000,000,000. As a special case, the value 0 is equivalent to 1 billion,
    /// or face value.
    fn high_quality_out(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::HighQualityOut)
    }

    /// The HighSponsor field (Optional).
    fn high_sponsor(&self) -> Result<Option<AccountID>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::HighSponsor)
    }

    /// The LowSponsor field (Optional).
    fn low_sponsor(&self) -> Result<Option<AccountID>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::LowSponsor)
    }
}

/// Trait providing access to fields specific to the current RippleState object.
pub trait CurrentRippleStateFields: CurrentLedgerObjectCommonFields {
    /// The balance of the trust line, from the perspective of the low account. A negative balance
    /// indicates that the high account holds tokens issued by the low account. The issuer in this
    /// is always set to the neutral value ACCOUNT_ONE.
    fn balance(&self) -> Result<Amount> {
        current_ledger_object::get_field(sfield::Balance)
    }

    /// The limit that the low account has set on the trust line. The `issuer` is the address of the
    /// low account that set this limit.
    fn low_limit(&self) -> Result<Amount> {
        current_ledger_object::get_field(sfield::LowLimit)
    }

    /// The limit that the high account has set on the trust line. The `issuer` is the address of
    /// the high account that set this limit.
    fn high_limit(&self) -> Result<Amount> {
        current_ledger_object::get_field(sfield::HighLimit)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn previous_txn_id(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn previous_txn_lgr_seq(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::PreviousTxnLgrSeq)
    }

    /// (Omitted in some historical ledgers) A hint indicating which page of the low account's owner
    /// directory links to this entry, in case the directory consists of multiple pages.
    fn low_node(&self) -> Result<Option<u64>> {
        current_ledger_object::get_field_optional(sfield::LowNode)
    }

    /// The inbound quality set by the low account, as an integer in the implied ratio
    /// `LowQualityIn`:1,000,000,000. As a special case, the value 0 is equivalent to 1 billion, or
    /// face value.
    fn low_quality_in(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::LowQualityIn)
    }

    /// The outbound quality set by the low account, as an integer in the implied ratio
    /// `LowQualityOut`:1,000,000,000. As a special case, the value 0 is equivalent to 1 billion, or
    /// face value.
    fn low_quality_out(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::LowQualityOut)
    }

    /// (Omitted in some historical ledgers) A hint indicating which page of the high account's
    /// owner directory links to this entry, in case the directory consists of multiple pages.
    fn high_node(&self) -> Result<Option<u64>> {
        current_ledger_object::get_field_optional(sfield::HighNode)
    }

    /// The inbound quality set by the high account, as an integer in the implied ratio
    /// `HighQualityIn`:1,000,000,000. As a special case, the value 0 is equivalent to 1 billion, or
    /// face value.
    fn high_quality_in(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::HighQualityIn)
    }

    /// The outbound quality set by the high account, as an integer in the implied ratio
    /// `HighQualityOut`:1,000,000,000. As a special case, the value 0 is equivalent to 1 billion,
    /// or face value.
    fn high_quality_out(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::HighQualityOut)
    }

    /// The HighSponsor field (Optional).
    fn high_sponsor(&self) -> Result<Option<AccountID>> {
        current_ledger_object::get_field_optional(sfield::HighSponsor)
    }

    /// The LowSponsor field (Optional).
    fn low_sponsor(&self) -> Result<Option<AccountID>> {
        current_ledger_object::get_field_optional(sfield::LowSponsor)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct RippleState {
    pub(crate) slot_num: i32,
}

impl RippleState {
    /// Binds this handle to a host-managed slot holding a RippleState ledger object.
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }
}

impl LedgerObjectCommonFields for RippleState {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl RippleStateFields for RippleState {}

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

        let obj = RippleState::new(0);

        assert!(obj.balance().is_ok());
        assert!(obj.low_limit().is_ok());
        assert!(obj.high_limit().is_ok());
        assert!(obj.previous_txn_id().is_ok());
        assert!(obj.previous_txn_lgr_seq().is_ok());
        assert!(obj.low_node().is_ok());
        assert!(obj.low_quality_in().is_ok());
        assert!(obj.low_quality_out().is_ok());
        assert!(obj.high_node().is_ok());
        assert!(obj.high_quality_in().is_ok());
        assert!(obj.high_quality_out().is_ok());
        assert!(obj.high_sponsor().is_ok());
        assert!(obj.low_sponsor().is_ok());
    }

    #[test]
    fn optional_fields_none() {
        let mut mock = MockHostBindings::new();
        mock_all_fields_not_found(&mut mock);
        let _guard = setup_mock(mock);

        let obj = RippleState::new(0);

        assert!(obj.low_node().unwrap().is_none());
        assert!(obj.low_quality_in().unwrap().is_none());
        assert!(obj.low_quality_out().unwrap().is_none());
        assert!(obj.high_node().unwrap().is_none());
        assert!(obj.high_quality_in().unwrap().is_none());
        assert!(obj.high_quality_out().unwrap().is_none());
        assert!(obj.high_sponsor().unwrap().is_none());
        assert!(obj.low_sponsor().unwrap().is_none());
    }
}
