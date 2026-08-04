// GENERATED -- do not hand-edit. Run scripts/generate-ledger-objects.sh to regenerate.

/// Placeholder buffer size for fields whose XRPL wire type has no genuine Rust
/// mapping yet (VECTOR256, XCHAIN_BRIDGE, NUMBER, INT32, ...). Such getters return
/// raw, unparsed bytes; see the summary at the top of `generated/mod.rs`.
const RAW_UNMAPPED_FIELD_SIZE: usize = 512;

use crate::host::Result;
use crate::host::error_codes::match_result_code_optional;
use crate::host::get_current_ledger_obj_field;
use crate::host::get_ledger_obj_field;
use crate::objects::traits::CurrentLedgerObjectCommonFields;
use crate::objects::traits::LedgerObjectCommonFields;
use crate::objects::{current_ledger_object, ledger_object};
use crate::sfield;
use crate::types::account_id::AccountID;
use crate::types::blob::StandardBlob;
use crate::types::uint::Hash256;

/// Trait providing access to fields specific to LoanBroker objects in any ledger.
pub trait LoanBrokerFields: LedgerObjectCommonFields {
    /// Identifies the transaction ID that most recently modified this object.
    fn previous_txn_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The sequence of the ledger that contains the transaction that most recently modified this object.
    fn previous_txn_lgr_seq(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }

    /// The transaction sequence number that created the LoanBroker.
    fn sequence(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::Sequence)
    }

    /// Identifies the page where this item is referenced in the owner's directory.
    fn owner_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::OwnerNode)
    }

    /// Identifies the page where this item is referenced in the `Vault` pseudo-account owner's directory.
    fn vault_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::VaultNode)
    }

    /// The ID of the vault that provides the loaned assets.
    fn vault_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::VaultID)
    }

    /// The address of the `LoanBroker` pseudo-account.
    fn account(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Account)
    }

    /// The account address of the vault owner.
    fn owner(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Owner)
    }

    /// A sequential identifier for `Loan` ledger entires, incremented each time a new loan is created by this `LoanBroker`.
    fn loan_sequence(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::LoanSequence)
    }

    /// Arbitrary metadata about the vault. Limited to 256 bytes.
    fn data(&self) -> Result<Option<StandardBlob>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::Data)
    }

    /// The fee charged by the lending protocol on any loan interest, in units of 1/10th basis points. Valid values are 0 to 10000 (inclusive), representing 0% to 10%.
    fn management_fee_rate(&self) -> Result<Option<u16>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::ManagementFeeRate)
    }

    /// The number of active loans issued by the LoanBroker.
    fn owner_count(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::OwnerCount)
    }

    /// The total asset amount the protocol owes the vault, including interest.
    /// Raw bytes; NUMBER is not yet typed in Rust.
    fn debt_total(&self) -> Result<Option<[u8; RAW_UNMAPPED_FIELD_SIZE]>> {
        let mut buffer = [0u8; RAW_UNMAPPED_FIELD_SIZE];
        let result_code = unsafe {
            get_ledger_obj_field(
                self.get_slot_num(),
                sfield::DebtTotal.into(),
                buffer.as_mut_ptr(),
                buffer.len(),
            )
        };
        match_result_code_optional(result_code, || (result_code > 0).then_some(buffer))
    }

    /// The maximum amount the protocol can owe the vault. The default value of `0` means there is no limit to the debt.
    /// Raw bytes; NUMBER is not yet typed in Rust.
    fn debt_maximum(&self) -> Result<Option<[u8; RAW_UNMAPPED_FIELD_SIZE]>> {
        let mut buffer = [0u8; RAW_UNMAPPED_FIELD_SIZE];
        let result_code = unsafe {
            get_ledger_obj_field(
                self.get_slot_num(),
                sfield::DebtMaximum.into(),
                buffer.as_mut_ptr(),
                buffer.len(),
            )
        };
        match_result_code_optional(result_code, || (result_code > 0).then_some(buffer))
    }

    /// The total amount of first-loss capital deposited into the lending protocol.
    /// Raw bytes; NUMBER is not yet typed in Rust.
    fn cover_available(&self) -> Result<Option<[u8; RAW_UNMAPPED_FIELD_SIZE]>> {
        let mut buffer = [0u8; RAW_UNMAPPED_FIELD_SIZE];
        let result_code = unsafe {
            get_ledger_obj_field(
                self.get_slot_num(),
                sfield::CoverAvailable.into(),
                buffer.as_mut_ptr(),
                buffer.len(),
            )
        };
        match_result_code_optional(result_code, || (result_code > 0).then_some(buffer))
    }

    /// The 1/10th basis point of the `DebtTotal` that the first-loss capital must cover. Valid values are 0 to 100000 (inclusive), representing 0% to 100%.
    fn cover_rate_minimum(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::CoverRateMinimum)
    }

    /// The 1/10th basis point of minimum required first-loss capital that is moved to an asset vault to cover a loan default. Valid values are 0 to 100000 (inclusive), representing 0% to 100%.
    fn cover_rate_liquidation(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::CoverRateLiquidation)
    }
}

/// Trait providing access to fields specific to the current LoanBroker object.
pub trait CurrentLoanBrokerFields: CurrentLedgerObjectCommonFields {
    /// Identifies the transaction ID that most recently modified this object.
    fn previous_txn_id(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::PreviousTxnID)
    }

    /// The sequence of the ledger that contains the transaction that most recently modified this object.
    fn previous_txn_lgr_seq(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::PreviousTxnLgrSeq)
    }

    /// The transaction sequence number that created the LoanBroker.
    fn sequence(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::Sequence)
    }

    /// Identifies the page where this item is referenced in the owner's directory.
    fn owner_node(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::OwnerNode)
    }

    /// Identifies the page where this item is referenced in the `Vault` pseudo-account owner's directory.
    fn vault_node(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::VaultNode)
    }

    /// The ID of the vault that provides the loaned assets.
    fn vault_id(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::VaultID)
    }

    /// The address of the `LoanBroker` pseudo-account.
    fn account(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Account)
    }

    /// The account address of the vault owner.
    fn owner(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Owner)
    }

    /// A sequential identifier for `Loan` ledger entires, incremented each time a new loan is created by this `LoanBroker`.
    fn loan_sequence(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::LoanSequence)
    }

    /// Arbitrary metadata about the vault. Limited to 256 bytes.
    fn data(&self) -> Result<Option<StandardBlob>> {
        current_ledger_object::get_field_optional(sfield::Data)
    }

    /// The fee charged by the lending protocol on any loan interest, in units of 1/10th basis points. Valid values are 0 to 10000 (inclusive), representing 0% to 10%.
    fn management_fee_rate(&self) -> Result<Option<u16>> {
        current_ledger_object::get_field_optional(sfield::ManagementFeeRate)
    }

    /// The number of active loans issued by the LoanBroker.
    fn owner_count(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::OwnerCount)
    }

    /// The total asset amount the protocol owes the vault, including interest.
    /// Raw bytes; NUMBER is not yet typed in Rust.
    fn debt_total(&self) -> Result<Option<[u8; RAW_UNMAPPED_FIELD_SIZE]>> {
        let mut buffer = [0u8; RAW_UNMAPPED_FIELD_SIZE];
        let result_code = unsafe {
            get_current_ledger_obj_field(
                sfield::DebtTotal.into(),
                buffer.as_mut_ptr(),
                buffer.len(),
            )
        };
        match_result_code_optional(result_code, || (result_code > 0).then_some(buffer))
    }

    /// The maximum amount the protocol can owe the vault. The default value of `0` means there is no limit to the debt.
    /// Raw bytes; NUMBER is not yet typed in Rust.
    fn debt_maximum(&self) -> Result<Option<[u8; RAW_UNMAPPED_FIELD_SIZE]>> {
        let mut buffer = [0u8; RAW_UNMAPPED_FIELD_SIZE];
        let result_code = unsafe {
            get_current_ledger_obj_field(
                sfield::DebtMaximum.into(),
                buffer.as_mut_ptr(),
                buffer.len(),
            )
        };
        match_result_code_optional(result_code, || (result_code > 0).then_some(buffer))
    }

    /// The total amount of first-loss capital deposited into the lending protocol.
    /// Raw bytes; NUMBER is not yet typed in Rust.
    fn cover_available(&self) -> Result<Option<[u8; RAW_UNMAPPED_FIELD_SIZE]>> {
        let mut buffer = [0u8; RAW_UNMAPPED_FIELD_SIZE];
        let result_code = unsafe {
            get_current_ledger_obj_field(
                sfield::CoverAvailable.into(),
                buffer.as_mut_ptr(),
                buffer.len(),
            )
        };
        match_result_code_optional(result_code, || (result_code > 0).then_some(buffer))
    }

    /// The 1/10th basis point of the `DebtTotal` that the first-loss capital must cover. Valid values are 0 to 100000 (inclusive), representing 0% to 100%.
    fn cover_rate_minimum(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::CoverRateMinimum)
    }

    /// The 1/10th basis point of minimum required first-loss capital that is moved to an asset vault to cover a loan default. Valid values are 0 to 100000 (inclusive), representing 0% to 100%.
    fn cover_rate_liquidation(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::CoverRateLiquidation)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct LoanBroker {
    pub(crate) slot_num: i32,
}

impl LoanBroker {
    /// Binds this handle to a host-managed slot holding a LoanBroker ledger object.
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }
}

impl LedgerObjectCommonFields for LoanBroker {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl LoanBrokerFields for LoanBroker {}

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

        let obj = LoanBroker::new(0);

        assert!(obj.previous_txn_id().is_ok());
        assert!(obj.previous_txn_lgr_seq().is_ok());
        assert!(obj.sequence().is_ok());
        assert!(obj.owner_node().is_ok());
        assert!(obj.vault_node().is_ok());
        assert!(obj.vault_id().is_ok());
        assert!(obj.account().is_ok());
        assert!(obj.owner().is_ok());
        assert!(obj.loan_sequence().is_ok());
        assert!(obj.data().is_ok());
        assert!(obj.management_fee_rate().is_ok());
        assert!(obj.owner_count().is_ok());
        assert!(obj.debt_total().is_ok());
        assert!(obj.debt_maximum().is_ok());
        assert!(obj.cover_available().is_ok());
        assert!(obj.cover_rate_minimum().is_ok());
        assert!(obj.cover_rate_liquidation().is_ok());
    }

    #[test]
    fn optional_fields_none() {
        let mut mock = MockHostBindings::new();
        mock_all_fields_not_found(&mut mock);
        let _guard = setup_mock(mock);

        let obj = LoanBroker::new(0);

        assert!(obj.management_fee_rate().unwrap().is_none());
        assert!(obj.owner_count().unwrap().is_none());
        assert!(obj.cover_rate_minimum().unwrap().is_none());
        assert!(obj.cover_rate_liquidation().unwrap().is_none());
    }
}
