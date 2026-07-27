// GENERATED -- do not hand-edit. Run scripts/generate-ledger-objects.sh to regenerate.

use crate::host::Result;
use crate::objects::traits::CurrentLedgerObjectCommonFields;
use crate::objects::traits::LedgerObjectCommonFields;
use crate::objects::{current_ledger_object, ledger_object};
use crate::sfield;
use crate::types::amount::Amount;
use crate::types::uint::Hash256;

/// Trait providing access to fields specific to FeeSettings objects in any ledger.
pub trait FeeSettingsFields: LedgerObjectCommonFields {
    /// The transaction cost of the "reference transaction" in drops of XRP as hexadecimal.
    fn get_base_fee(&self) -> Result<Option<u64>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::BaseFee)
    }

    /// The `BaseFee` translated into "fee units".
    fn get_reference_fee_units(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::ReferenceFeeUnits)
    }

    /// The base reserve for an account in the XRP Ledger, as drops of XRP.
    fn get_reserve_base(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::ReserveBase)
    }

    /// The incremental owner reserve for owning objects, as drops of XRP.
    fn get_reserve_increment(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::ReserveIncrement)
    }

    /// The transaction cost of the "reference transaction" in drops of XRP.
    fn get_base_fee_drops(&self) -> Result<Option<Amount>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::BaseFeeDrops)
    }

    /// The base reserve for an account in the XRP Ledger, as drops of XRP.
    fn get_reserve_base_drops(&self) -> Result<Option<Amount>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::ReserveBaseDrops)
    }

    /// The incremental owner reserve for owning objects, as drops of XRP.
    fn get_reserve_increment_drops(&self) -> Result<Option<Amount>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::ReserveIncrementDrops)
    }

    /// The ExtensionComputeLimit field (Optional).
    fn get_extension_compute_limit(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::ExtensionComputeLimit)
    }

    /// The ExtensionSizeLimit field (Optional).
    fn get_extension_size_limit(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::ExtensionSizeLimit)
    }

    /// The GasPrice field (Optional).
    fn get_gas_price(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::GasPrice)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Option<Hash256>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The [index of the ledger][Ledger Index] that contains the transaction that most recently modified this entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }
}

/// Trait providing access to fields specific to the current FeeSettings object.
pub trait CurrentFeeSettingsFields: CurrentLedgerObjectCommonFields {
    /// The transaction cost of the "reference transaction" in drops of XRP as hexadecimal.
    fn get_base_fee(&self) -> Result<Option<u64>> {
        current_ledger_object::get_field_optional(sfield::BaseFee)
    }

    /// The `BaseFee` translated into "fee units".
    fn get_reference_fee_units(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::ReferenceFeeUnits)
    }

    /// The base reserve for an account in the XRP Ledger, as drops of XRP.
    fn get_reserve_base(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::ReserveBase)
    }

    /// The incremental owner reserve for owning objects, as drops of XRP.
    fn get_reserve_increment(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::ReserveIncrement)
    }

    /// The transaction cost of the "reference transaction" in drops of XRP.
    fn get_base_fee_drops(&self) -> Result<Option<Amount>> {
        current_ledger_object::get_field_optional(sfield::BaseFeeDrops)
    }

    /// The base reserve for an account in the XRP Ledger, as drops of XRP.
    fn get_reserve_base_drops(&self) -> Result<Option<Amount>> {
        current_ledger_object::get_field_optional(sfield::ReserveBaseDrops)
    }

    /// The incremental owner reserve for owning objects, as drops of XRP.
    fn get_reserve_increment_drops(&self) -> Result<Option<Amount>> {
        current_ledger_object::get_field_optional(sfield::ReserveIncrementDrops)
    }

    /// The ExtensionComputeLimit field (Optional).
    fn get_extension_compute_limit(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::ExtensionComputeLimit)
    }

    /// The ExtensionSizeLimit field (Optional).
    fn get_extension_size_limit(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::ExtensionSizeLimit)
    }

    /// The GasPrice field (Optional).
    fn get_gas_price(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::GasPrice)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Option<Hash256>> {
        current_ledger_object::get_field_optional(sfield::PreviousTxnID)
    }

    /// The [index of the ledger][Ledger Index] that contains the transaction that most recently modified this entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::PreviousTxnLgrSeq)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct FeeSettings {
    pub(crate) slot_num: i32,
}

impl FeeSettings {
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }
}

impl LedgerObjectCommonFields for FeeSettings {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl FeeSettingsFields for FeeSettings {}

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

        let obj = FeeSettings::new(0);

        assert!(obj.get_base_fee().is_ok());
        assert!(obj.get_reference_fee_units().is_ok());
        assert!(obj.get_reserve_base().is_ok());
        assert!(obj.get_reserve_increment().is_ok());
        assert!(obj.get_base_fee_drops().is_ok());
        assert!(obj.get_reserve_base_drops().is_ok());
        assert!(obj.get_reserve_increment_drops().is_ok());
        assert!(obj.get_extension_compute_limit().is_ok());
        assert!(obj.get_extension_size_limit().is_ok());
        assert!(obj.get_gas_price().is_ok());
        assert!(obj.get_previous_txn_id().is_ok());
        assert!(obj.get_previous_txn_lgr_seq().is_ok());
    }

    #[test]
    fn optional_fields_none() {
        let mut mock = MockHostBindings::new();
        mock_all_fields_not_found(&mut mock);
        let _guard = setup_mock(mock);

        let obj = FeeSettings::new(0);

        assert!(obj.get_base_fee().unwrap().is_none());
        assert!(obj.get_reference_fee_units().unwrap().is_none());
        assert!(obj.get_reserve_base().unwrap().is_none());
        assert!(obj.get_reserve_increment().unwrap().is_none());
        assert!(obj.get_extension_compute_limit().unwrap().is_none());
        assert!(obj.get_extension_size_limit().unwrap().is_none());
        assert!(obj.get_gas_price().unwrap().is_none());
        assert!(obj.get_previous_txn_id().unwrap().is_none());
        assert!(obj.get_previous_txn_lgr_seq().unwrap().is_none());
    }
}
