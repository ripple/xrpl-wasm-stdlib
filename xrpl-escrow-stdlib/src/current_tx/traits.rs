//! Escrow-finish-specific transaction field accessor trait.

use xrpl_wasm_stdlib::core::current_tx::get_field;
use xrpl_wasm_stdlib::core::current_tx::traits::TransactionCommonFields;
use xrpl_wasm_stdlib::core::types::account_id::AccountID;
use xrpl_wasm_stdlib::core::types::blob::{ConditionBlob, FulfillmentBlob};
use xrpl_wasm_stdlib::host::error_codes::match_result_code_optional;
use xrpl_wasm_stdlib::host::{Result, get_tx_field};
use xrpl_wasm_stdlib::sfield;

/// Trait providing access to fields specific to EscrowFinish transactions.
pub trait EscrowFinishFields: TransactionCommonFields {
    fn get_owner(&self) -> Result<AccountID> {
        get_field(sfield::Owner)
    }

    fn get_offer_sequence(&self) -> Result<u32> {
        get_field(sfield::OfferSequence)
    }

    fn get_condition(&self) -> Result<Option<ConditionBlob>> {
        let mut buffer = ConditionBlob::new();
        let result_code = unsafe {
            get_tx_field(
                sfield::Condition.into(),
                buffer.data.as_mut_ptr(),
                buffer.capacity(),
            )
        };
        match_result_code_optional(result_code, || {
            buffer.len = result_code as usize;
            (result_code > 0).then_some(buffer)
        })
    }

    fn get_fulfillment(&self) -> Result<Option<FulfillmentBlob>> {
        let mut buffer = FulfillmentBlob::new();
        let result_code = unsafe {
            get_tx_field(
                sfield::Fulfillment.into(),
                buffer.data.as_mut_ptr(),
                buffer.capacity(),
            )
        };
        match_result_code_optional(result_code, || {
            buffer.len = result_code as usize;
            (result_code > 0).then_some(buffer)
        })
    }
}
