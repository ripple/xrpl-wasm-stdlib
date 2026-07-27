// GENERATED -- do not hand-edit. Run scripts/generate-ledger-objects.sh to regenerate.

/// Placeholder buffer size for fields whose XRPL wire type has no genuine Rust
/// mapping yet (VECTOR256, XCHAIN_BRIDGE, NUMBER, INT32, ...). Such getters return
/// raw, unparsed bytes; see the summary at the top of `generated/mod.rs`.
const RAW_UNMAPPED_FIELD_SIZE: usize = 512;

use crate::host::Result;
use crate::host::error_codes::{match_result_code, match_result_code_optional};
use crate::host::get_current_ledger_obj_field;
use crate::host::get_ledger_obj_field;
use crate::objects::traits::CurrentLedgerObjectCommonFields;
use crate::objects::traits::LedgerObjectCommonFields;
use crate::objects::{current_ledger_object, ledger_object};
use crate::sfield;
use crate::types::account_id::AccountID;
use crate::types::uint::Hash256;

/// Trait providing access to fields specific to Loan objects in any ledger.
pub trait LoanFields: LedgerObjectCommonFields {
    /// Identifies the transaction ID that most recently modified this object.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The sequence of the ledger that contains the transaction that most recently modified this object.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }

    /// Identifies the page where this item is referenced in the owner's directory.
    fn get_owner_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::OwnerNode)
    }

    /// Identifies the page where this item is referenced in the `LoanBroker` owner directory.
    fn get_loan_broker_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::LoanBrokerNode)
    }

    /// The ID of the _Loan Broker_ associated with this loan.
    fn get_loan_broker_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::LoanBrokerID)
    }

    /// The sequence number of the loan.
    fn get_loan_sequence(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::LoanSequence)
    }

    /// The account address of the _Borrower_.
    fn get_borrower(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Borrower)
    }

    /// The amount paid to the _Loan Broker_, taken from the principal loan at creation.
    /// Raw bytes; NUMBER is not yet typed in Rust.
    fn get_loan_origination_fee(&self) -> Result<Option<[u8; RAW_UNMAPPED_FIELD_SIZE]>> {
        let mut buffer = [0u8; RAW_UNMAPPED_FIELD_SIZE];
        let result_code = unsafe {
            get_ledger_obj_field(
                self.get_slot_num(),
                sfield::LoanOriginationFee.into(),
                buffer.as_mut_ptr(),
                buffer.len(),
            )
        };
        match_result_code_optional(result_code, || (result_code > 0).then_some(buffer))
    }

    /// The amount paid to the _Loan Broker_ with each loan payment.
    /// Raw bytes; NUMBER is not yet typed in Rust.
    fn get_loan_service_fee(&self) -> Result<Option<[u8; RAW_UNMAPPED_FIELD_SIZE]>> {
        let mut buffer = [0u8; RAW_UNMAPPED_FIELD_SIZE];
        let result_code = unsafe {
            get_ledger_obj_field(
                self.get_slot_num(),
                sfield::LoanServiceFee.into(),
                buffer.as_mut_ptr(),
                buffer.len(),
            )
        };
        match_result_code_optional(result_code, || (result_code > 0).then_some(buffer))
    }

    /// The amount paid to the _Loan Broker_ for each late payment.
    /// Raw bytes; NUMBER is not yet typed in Rust.
    fn get_late_payment_fee(&self) -> Result<Option<[u8; RAW_UNMAPPED_FIELD_SIZE]>> {
        let mut buffer = [0u8; RAW_UNMAPPED_FIELD_SIZE];
        let result_code = unsafe {
            get_ledger_obj_field(
                self.get_slot_num(),
                sfield::LatePaymentFee.into(),
                buffer.as_mut_ptr(),
                buffer.len(),
            )
        };
        match_result_code_optional(result_code, || (result_code > 0).then_some(buffer))
    }

    /// The amount paid to the _Loan Broker_ when a full early payment is made.
    /// Raw bytes; NUMBER is not yet typed in Rust.
    fn get_close_payment_fee(&self) -> Result<Option<[u8; RAW_UNMAPPED_FIELD_SIZE]>> {
        let mut buffer = [0u8; RAW_UNMAPPED_FIELD_SIZE];
        let result_code = unsafe {
            get_ledger_obj_field(
                self.get_slot_num(),
                sfield::ClosePaymentFee.into(),
                buffer.as_mut_ptr(),
                buffer.len(),
            )
        };
        match_result_code_optional(result_code, || (result_code > 0).then_some(buffer))
    }

    /// The fee charged on overpayments, in units of 1/10th basis points. Valid values are 0 to 100000 (inclusive), representing 0% to 100%.
    fn get_overpayment_fee(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::OverpaymentFee)
    }

    /// The annualized interest rate of the loan, in 1/10th basis points.
    fn get_interest_rate(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::InterestRate)
    }

    /// The premium added to the interest rate for late payments, in units of 1/10th basis points. Valid values are 0 to 100000 (inclusive), representing 0% to 100%.
    fn get_late_interest_rate(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::LateInterestRate)
    }

    /// The interest rate charged for repaying the loan early, in units of 1/10th basis points. Valid values are 0 to 100000 (inclusive), representing 0% to 100%.
    fn get_close_interest_rate(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::CloseInterestRate)
    }

    /// The interest rate charged on overpayments, in units of 1/10th basis points. Valid values are 0 to 100000 (inclusive), representing 0% to 100%.
    fn get_overpayment_interest_rate(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::OverpaymentInterestRate)
    }

    /// The timestamp of when the loan started, in [seconds since the Ripple Epoch][].
    fn get_start_date(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::StartDate)
    }

    /// The number of seconds between loan payments.
    fn get_payment_interval(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::PaymentInterval)
    }

    /// The number of seconds after a loan payment is due before the loan defaults.
    fn get_grace_period(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::GracePeriod)
    }

    /// The timestamp of when the previous payment was made, in [seconds since the Ripple Epoch][].
    fn get_previous_payment_due_date(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::PreviousPaymentDueDate)
    }

    /// The timestamp of when the next payment is due, in [seconds since the Ripple Epoch][].
    fn get_next_payment_due_date(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::NextPaymentDueDate)
    }

    /// The number of payments remaining on the loan.
    fn get_payment_remaining(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::PaymentRemaining)
    }

    /// The amount due for each payment interval.
    /// Raw bytes; NUMBER is not yet typed in Rust.
    fn get_periodic_payment(&self) -> Result<[u8; RAW_UNMAPPED_FIELD_SIZE]> {
        let mut buffer = [0u8; RAW_UNMAPPED_FIELD_SIZE];
        let result_code = unsafe {
            get_ledger_obj_field(
                self.get_slot_num(),
                sfield::PeriodicPayment.into(),
                buffer.as_mut_ptr(),
                buffer.len(),
            )
        };
        match_result_code(result_code, || buffer)
    }

    /// The principal amount still owed on the loan.
    /// Raw bytes; NUMBER is not yet typed in Rust.
    fn get_principal_outstanding(&self) -> Result<Option<[u8; RAW_UNMAPPED_FIELD_SIZE]>> {
        let mut buffer = [0u8; RAW_UNMAPPED_FIELD_SIZE];
        let result_code = unsafe {
            get_ledger_obj_field(
                self.get_slot_num(),
                sfield::PrincipalOutstanding.into(),
                buffer.as_mut_ptr(),
                buffer.len(),
            )
        };
        match_result_code_optional(result_code, || (result_code > 0).then_some(buffer))
    }

    /// The total amount owed on the loan, including remaining principal and fees.
    /// Raw bytes; NUMBER is not yet typed in Rust.
    fn get_total_value_outstanding(&self) -> Result<Option<[u8; RAW_UNMAPPED_FIELD_SIZE]>> {
        let mut buffer = [0u8; RAW_UNMAPPED_FIELD_SIZE];
        let result_code = unsafe {
            get_ledger_obj_field(
                self.get_slot_num(),
                sfield::TotalValueOutstanding.into(),
                buffer.as_mut_ptr(),
                buffer.len(),
            )
        };
        match_result_code_optional(result_code, || (result_code > 0).then_some(buffer))
    }

    /// The remaining management fee owed to the loan broker.
    /// Raw bytes; NUMBER is not yet typed in Rust.
    fn get_management_fee_outstanding(&self) -> Result<Option<[u8; RAW_UNMAPPED_FIELD_SIZE]>> {
        let mut buffer = [0u8; RAW_UNMAPPED_FIELD_SIZE];
        let result_code = unsafe {
            get_ledger_obj_field(
                self.get_slot_num(),
                sfield::ManagementFeeOutstanding.into(),
                buffer.as_mut_ptr(),
                buffer.len(),
            )
        };
        match_result_code_optional(result_code, || (result_code > 0).then_some(buffer))
    }

    /// The scale factor that ensures all computed amounts are rounded to the same number of decimal places. It is based on the total loan value at creation time.
    /// Raw bytes; INT32 is not yet typed in Rust.
    fn get_loan_scale(&self) -> Result<Option<[u8; RAW_UNMAPPED_FIELD_SIZE]>> {
        let mut buffer = [0u8; RAW_UNMAPPED_FIELD_SIZE];
        let result_code = unsafe {
            get_ledger_obj_field(
                self.get_slot_num(),
                sfield::LoanScale.into(),
                buffer.as_mut_ptr(),
                buffer.len(),
            )
        };
        match_result_code_optional(result_code, || (result_code > 0).then_some(buffer))
    }
}

/// Trait providing access to fields specific to the current Loan object.
pub trait CurrentLoanFields: CurrentLedgerObjectCommonFields {
    /// Identifies the transaction ID that most recently modified this object.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::PreviousTxnID)
    }

    /// The sequence of the ledger that contains the transaction that most recently modified this object.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::PreviousTxnLgrSeq)
    }

    /// Identifies the page where this item is referenced in the owner's directory.
    fn get_owner_node(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::OwnerNode)
    }

    /// Identifies the page where this item is referenced in the `LoanBroker` owner directory.
    fn get_loan_broker_node(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::LoanBrokerNode)
    }

    /// The ID of the _Loan Broker_ associated with this loan.
    fn get_loan_broker_id(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::LoanBrokerID)
    }

    /// The sequence number of the loan.
    fn get_loan_sequence(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::LoanSequence)
    }

    /// The account address of the _Borrower_.
    fn get_borrower(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Borrower)
    }

    /// The amount paid to the _Loan Broker_, taken from the principal loan at creation.
    /// Raw bytes; NUMBER is not yet typed in Rust.
    fn get_loan_origination_fee(&self) -> Result<Option<[u8; RAW_UNMAPPED_FIELD_SIZE]>> {
        let mut buffer = [0u8; RAW_UNMAPPED_FIELD_SIZE];
        let result_code = unsafe {
            get_current_ledger_obj_field(
                sfield::LoanOriginationFee.into(),
                buffer.as_mut_ptr(),
                buffer.len(),
            )
        };
        match_result_code_optional(result_code, || (result_code > 0).then_some(buffer))
    }

    /// The amount paid to the _Loan Broker_ with each loan payment.
    /// Raw bytes; NUMBER is not yet typed in Rust.
    fn get_loan_service_fee(&self) -> Result<Option<[u8; RAW_UNMAPPED_FIELD_SIZE]>> {
        let mut buffer = [0u8; RAW_UNMAPPED_FIELD_SIZE];
        let result_code = unsafe {
            get_current_ledger_obj_field(
                sfield::LoanServiceFee.into(),
                buffer.as_mut_ptr(),
                buffer.len(),
            )
        };
        match_result_code_optional(result_code, || (result_code > 0).then_some(buffer))
    }

    /// The amount paid to the _Loan Broker_ for each late payment.
    /// Raw bytes; NUMBER is not yet typed in Rust.
    fn get_late_payment_fee(&self) -> Result<Option<[u8; RAW_UNMAPPED_FIELD_SIZE]>> {
        let mut buffer = [0u8; RAW_UNMAPPED_FIELD_SIZE];
        let result_code = unsafe {
            get_current_ledger_obj_field(
                sfield::LatePaymentFee.into(),
                buffer.as_mut_ptr(),
                buffer.len(),
            )
        };
        match_result_code_optional(result_code, || (result_code > 0).then_some(buffer))
    }

    /// The amount paid to the _Loan Broker_ when a full early payment is made.
    /// Raw bytes; NUMBER is not yet typed in Rust.
    fn get_close_payment_fee(&self) -> Result<Option<[u8; RAW_UNMAPPED_FIELD_SIZE]>> {
        let mut buffer = [0u8; RAW_UNMAPPED_FIELD_SIZE];
        let result_code = unsafe {
            get_current_ledger_obj_field(
                sfield::ClosePaymentFee.into(),
                buffer.as_mut_ptr(),
                buffer.len(),
            )
        };
        match_result_code_optional(result_code, || (result_code > 0).then_some(buffer))
    }

    /// The fee charged on overpayments, in units of 1/10th basis points. Valid values are 0 to 100000 (inclusive), representing 0% to 100%.
    fn get_overpayment_fee(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::OverpaymentFee)
    }

    /// The annualized interest rate of the loan, in 1/10th basis points.
    fn get_interest_rate(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::InterestRate)
    }

    /// The premium added to the interest rate for late payments, in units of 1/10th basis points. Valid values are 0 to 100000 (inclusive), representing 0% to 100%.
    fn get_late_interest_rate(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::LateInterestRate)
    }

    /// The interest rate charged for repaying the loan early, in units of 1/10th basis points. Valid values are 0 to 100000 (inclusive), representing 0% to 100%.
    fn get_close_interest_rate(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::CloseInterestRate)
    }

    /// The interest rate charged on overpayments, in units of 1/10th basis points. Valid values are 0 to 100000 (inclusive), representing 0% to 100%.
    fn get_overpayment_interest_rate(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::OverpaymentInterestRate)
    }

    /// The timestamp of when the loan started, in [seconds since the Ripple Epoch][].
    fn get_start_date(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::StartDate)
    }

    /// The number of seconds between loan payments.
    fn get_payment_interval(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::PaymentInterval)
    }

    /// The number of seconds after a loan payment is due before the loan defaults.
    fn get_grace_period(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::GracePeriod)
    }

    /// The timestamp of when the previous payment was made, in [seconds since the Ripple Epoch][].
    fn get_previous_payment_due_date(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::PreviousPaymentDueDate)
    }

    /// The timestamp of when the next payment is due, in [seconds since the Ripple Epoch][].
    fn get_next_payment_due_date(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::NextPaymentDueDate)
    }

    /// The number of payments remaining on the loan.
    fn get_payment_remaining(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::PaymentRemaining)
    }

    /// The amount due for each payment interval.
    /// Raw bytes; NUMBER is not yet typed in Rust.
    fn get_periodic_payment(&self) -> Result<[u8; RAW_UNMAPPED_FIELD_SIZE]> {
        let mut buffer = [0u8; RAW_UNMAPPED_FIELD_SIZE];
        let result_code = unsafe {
            get_current_ledger_obj_field(
                sfield::PeriodicPayment.into(),
                buffer.as_mut_ptr(),
                buffer.len(),
            )
        };
        match_result_code(result_code, || buffer)
    }

    /// The principal amount still owed on the loan.
    /// Raw bytes; NUMBER is not yet typed in Rust.
    fn get_principal_outstanding(&self) -> Result<Option<[u8; RAW_UNMAPPED_FIELD_SIZE]>> {
        let mut buffer = [0u8; RAW_UNMAPPED_FIELD_SIZE];
        let result_code = unsafe {
            get_current_ledger_obj_field(
                sfield::PrincipalOutstanding.into(),
                buffer.as_mut_ptr(),
                buffer.len(),
            )
        };
        match_result_code_optional(result_code, || (result_code > 0).then_some(buffer))
    }

    /// The total amount owed on the loan, including remaining principal and fees.
    /// Raw bytes; NUMBER is not yet typed in Rust.
    fn get_total_value_outstanding(&self) -> Result<Option<[u8; RAW_UNMAPPED_FIELD_SIZE]>> {
        let mut buffer = [0u8; RAW_UNMAPPED_FIELD_SIZE];
        let result_code = unsafe {
            get_current_ledger_obj_field(
                sfield::TotalValueOutstanding.into(),
                buffer.as_mut_ptr(),
                buffer.len(),
            )
        };
        match_result_code_optional(result_code, || (result_code > 0).then_some(buffer))
    }

    /// The remaining management fee owed to the loan broker.
    /// Raw bytes; NUMBER is not yet typed in Rust.
    fn get_management_fee_outstanding(&self) -> Result<Option<[u8; RAW_UNMAPPED_FIELD_SIZE]>> {
        let mut buffer = [0u8; RAW_UNMAPPED_FIELD_SIZE];
        let result_code = unsafe {
            get_current_ledger_obj_field(
                sfield::ManagementFeeOutstanding.into(),
                buffer.as_mut_ptr(),
                buffer.len(),
            )
        };
        match_result_code_optional(result_code, || (result_code > 0).then_some(buffer))
    }

    /// The scale factor that ensures all computed amounts are rounded to the same number of decimal places. It is based on the total loan value at creation time.
    /// Raw bytes; INT32 is not yet typed in Rust.
    fn get_loan_scale(&self) -> Result<Option<[u8; RAW_UNMAPPED_FIELD_SIZE]>> {
        let mut buffer = [0u8; RAW_UNMAPPED_FIELD_SIZE];
        let result_code = unsafe {
            get_current_ledger_obj_field(
                sfield::LoanScale.into(),
                buffer.as_mut_ptr(),
                buffer.len(),
            )
        };
        match_result_code_optional(result_code, || (result_code > 0).then_some(buffer))
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Loan {
    pub(crate) slot_num: i32,
}

impl Loan {
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }
}

impl LedgerObjectCommonFields for Loan {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl LoanFields for Loan {}

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

        let obj = Loan::new(0);

        assert!(obj.get_previous_txn_id().is_ok());
        assert!(obj.get_previous_txn_lgr_seq().is_ok());
        assert!(obj.get_owner_node().is_ok());
        assert!(obj.get_loan_broker_node().is_ok());
        assert!(obj.get_loan_broker_id().is_ok());
        assert!(obj.get_loan_sequence().is_ok());
        assert!(obj.get_borrower().is_ok());
        assert!(obj.get_start_date().is_ok());
        assert!(obj.get_payment_interval().is_ok());
        assert!(obj.get_periodic_payment().is_ok());
        assert!(obj.get_loan_origination_fee().is_ok());
        assert!(obj.get_loan_service_fee().is_ok());
        assert!(obj.get_late_payment_fee().is_ok());
        assert!(obj.get_close_payment_fee().is_ok());
        assert!(obj.get_overpayment_fee().is_ok());
        assert!(obj.get_interest_rate().is_ok());
        assert!(obj.get_late_interest_rate().is_ok());
        assert!(obj.get_close_interest_rate().is_ok());
        assert!(obj.get_overpayment_interest_rate().is_ok());
        assert!(obj.get_grace_period().is_ok());
        assert!(obj.get_previous_payment_due_date().is_ok());
        assert!(obj.get_next_payment_due_date().is_ok());
        assert!(obj.get_payment_remaining().is_ok());
        assert!(obj.get_principal_outstanding().is_ok());
        assert!(obj.get_total_value_outstanding().is_ok());
        assert!(obj.get_management_fee_outstanding().is_ok());
        assert!(obj.get_loan_scale().is_ok());
    }

    #[test]
    fn optional_fields_none() {
        let mut mock = MockHostBindings::new();
        mock_all_fields_not_found(&mut mock);
        let _guard = setup_mock(mock);

        let obj = Loan::new(0);

        assert!(obj.get_overpayment_fee().unwrap().is_none());
        assert!(obj.get_interest_rate().unwrap().is_none());
        assert!(obj.get_late_interest_rate().unwrap().is_none());
        assert!(obj.get_close_interest_rate().unwrap().is_none());
        assert!(obj.get_overpayment_interest_rate().unwrap().is_none());
        assert!(obj.get_grace_period().unwrap().is_none());
        assert!(obj.get_previous_payment_due_date().unwrap().is_none());
        assert!(obj.get_next_payment_due_date().unwrap().is_none());
        assert!(obj.get_payment_remaining().unwrap().is_none());
    }
}
