//! Escrow-specific ledger-object field accessor traits.

use xrpl_wasm_stdlib::core::ledger_objects::traits::{
    CurrentLedgerObjectCommonFields, LedgerObjectCommonFields,
};
use xrpl_wasm_stdlib::core::ledger_objects::{current_ledger_object, ledger_object};
use xrpl_wasm_stdlib::core::types::account_id::AccountID;
use xrpl_wasm_stdlib::core::types::amount::Amount;
use xrpl_wasm_stdlib::core::types::blob::{CONDITION_BLOB_SIZE, ConditionBlob, WasmBlob};
use xrpl_wasm_stdlib::core::types::contract_data::{ContractData, XRPL_CONTRACT_DATA_SIZE};
use xrpl_wasm_stdlib::core::types::uint::Hash256;
use xrpl_wasm_stdlib::host::error_codes::{match_result_code, match_result_code_optional};
use xrpl_wasm_stdlib::host::{
    Error, get_current_ledger_obj_field, get_ledger_obj_field, update_data,
};
use xrpl_wasm_stdlib::host::{Result, Result::Err, Result::Ok};
use xrpl_wasm_stdlib::sfield;

/// Trait providing access to fields specific to Escrow objects in the current ledger.
pub trait CurrentEscrowFields: CurrentLedgerObjectCommonFields {
    fn get_account(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Account)
    }

    fn get_amount(&self) -> Result<Amount> {
        current_ledger_object::get_field(sfield::Amount)
    }

    fn get_cancel_after(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::CancelAfter)
    }

    fn get_condition(&self) -> Result<Option<ConditionBlob>> {
        let mut buffer = ConditionBlob::new();
        let result_code = unsafe {
            get_current_ledger_obj_field(
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

    fn get_destination(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Destination)
    }

    fn get_destination_node(&self) -> Result<Option<u64>> {
        current_ledger_object::get_field_optional(sfield::DestinationNode)
    }

    fn get_destination_tag(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::DestinationTag)
    }

    fn get_finish_after(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::FinishAfter)
    }

    fn get_owner_node(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::OwnerNode)
    }

    fn get_previous_txn_id(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::PreviousTxnID)
    }

    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::PreviousTxnLgrSeq)
    }

    fn get_source_tag(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::SourceTag)
    }

    fn get_finish_function(&self) -> Result<Option<WasmBlob>> {
        current_ledger_object::get_field_optional(sfield::FinishFunction)
    }

    fn get_data(&self) -> Result<ContractData> {
        let mut data: [u8; XRPL_CONTRACT_DATA_SIZE] = [0; XRPL_CONTRACT_DATA_SIZE];

        let result_code = unsafe {
            get_current_ledger_obj_field(sfield::Data.into(), data.as_mut_ptr(), data.len())
        };

        match result_code {
            code if code >= 0 => Ok(ContractData {
                data,
                len: code as usize,
            }),
            code => Err(Error::from_code(code)),
        }
    }

    fn update_current_escrow_data(data: ContractData) -> Result<()> {
        let result_code = unsafe { update_data(data.data.as_ptr(), data.len) };
        match_result_code(result_code, || ())
    }
}

/// Trait providing access to fields specific to Escrow objects in any ledger.
pub trait EscrowFields: LedgerObjectCommonFields {
    fn get_account(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Account)
    }

    fn get_amount(&self) -> Result<Amount> {
        const BUFFER_SIZE: usize = 48usize;
        let mut buffer = [0u8; BUFFER_SIZE];

        let result_code = unsafe {
            get_ledger_obj_field(
                self.get_slot_num(),
                sfield::Amount.into(),
                buffer.as_mut_ptr(),
                buffer.len(),
            )
        };

        match_result_code(result_code, || Amount::from(buffer))
    }

    fn get_cancel_after(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::CancelAfter)
    }

    fn get_condition(&self) -> Result<Option<ConditionBlob>> {
        let mut buffer = [0u8; CONDITION_BLOB_SIZE];

        let result_code = unsafe {
            get_ledger_obj_field(
                self.get_slot_num(),
                sfield::Condition.into(),
                buffer.as_mut_ptr(),
                buffer.len(),
            )
        };

        match_result_code_optional(result_code, || {
            if result_code > 0 {
                let blob = ConditionBlob {
                    data: buffer,
                    len: result_code as usize,
                };
                Some(blob)
            } else {
                None
            }
        })
    }

    fn get_destination(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Destination)
    }

    fn get_destination_node(&self) -> Result<Option<u64>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::DestinationNode)
    }

    fn get_destination_tag(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::DestinationTag)
    }

    fn get_finish_after(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::FinishAfter)
    }

    fn get_owner_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::OwnerNode)
    }

    fn get_previous_txn_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnID)
    }

    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }

    fn get_source_tag(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::SourceTag)
    }

    fn get_finish_function(&self) -> Result<Option<WasmBlob>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::FinishFunction)
    }

    fn get_data(&self) -> Result<ContractData> {
        let mut data: [u8; XRPL_CONTRACT_DATA_SIZE] = [0; XRPL_CONTRACT_DATA_SIZE];

        let result_code = unsafe {
            get_ledger_obj_field(
                self.get_slot_num(),
                sfield::Data.into(),
                data.as_mut_ptr(),
                data.len(),
            )
        };

        match result_code {
            code if code >= 0 => Ok(ContractData {
                data,
                len: code as usize,
            }),
            code => Err(Error::from_code(code)),
        }
    }
}
