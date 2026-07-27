// GENERATED -- do not hand-edit. Run scripts/generate-ledger-objects.sh to regenerate.

use crate::host::Result;
use crate::objects::array_object::Array;
use crate::objects::traits::CurrentLedgerObjectCommonFields;
use crate::objects::traits::LedgerObjectCommonFields;
use crate::objects::{current_ledger_object, ledger_object};
use crate::sfield;
use crate::types::blob::StandardBlob;
use crate::types::uint::Hash256;

/// Trait providing access to fields specific to NegativeUNL objects in any ledger.
pub trait NegativeUNLFields: LedgerObjectCommonFields {
    /// A list of `DisabledValidator` objects (see below), each representing a trusted validator that is currently disabled.
    fn get_disabled_validators(&self) -> Result<Option<Array>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::DisabledValidators)
    }

    /// The public key of a trusted validator that is scheduled to be disabled in the next flag ledger.
    fn get_validator_to_disable(&self) -> Result<Option<StandardBlob>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::ValidatorToDisable)
    }

    /// The public key of a trusted validator in the Negative UNL that is scheduled to be re-enabled in the next flag ledger.
    fn get_validator_to_re_enable(&self) -> Result<Option<StandardBlob>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::ValidatorToReEnable)
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

/// Trait providing access to fields specific to the current NegativeUNL object.
pub trait CurrentNegativeUNLFields: CurrentLedgerObjectCommonFields {
    /// A list of `DisabledValidator` objects (see below), each representing a trusted validator that is currently disabled.
    fn get_disabled_validators(&self) -> Result<Option<Array>> {
        current_ledger_object::get_field_optional(sfield::DisabledValidators)
    }

    /// The public key of a trusted validator that is scheduled to be disabled in the next flag ledger.
    fn get_validator_to_disable(&self) -> Result<Option<StandardBlob>> {
        current_ledger_object::get_field_optional(sfield::ValidatorToDisable)
    }

    /// The public key of a trusted validator in the Negative UNL that is scheduled to be re-enabled in the next flag ledger.
    fn get_validator_to_re_enable(&self) -> Result<Option<StandardBlob>> {
        current_ledger_object::get_field_optional(sfield::ValidatorToReEnable)
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
pub struct NegativeUNL {
    pub(crate) slot_num: i32,
}

impl NegativeUNL {
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }
}

impl LedgerObjectCommonFields for NegativeUNL {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl NegativeUNLFields for NegativeUNL {}

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

        let obj = NegativeUNL::new(0);

        assert!(obj.get_validator_to_disable().is_ok());
        assert!(obj.get_validator_to_re_enable().is_ok());
        assert!(obj.get_previous_txn_id().is_ok());
        assert!(obj.get_previous_txn_lgr_seq().is_ok());
    }

    #[test]
    fn optional_fields_none() {
        let mut mock = MockHostBindings::new();
        mock_all_fields_not_found(&mut mock);
        let _guard = setup_mock(mock);

        let obj = NegativeUNL::new(0);

        assert!(obj.get_previous_txn_id().unwrap().is_none());
        assert!(obj.get_previous_txn_lgr_seq().unwrap().is_none());
    }
}
