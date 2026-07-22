use crate::sfield;
use xrpl_wasm_stdlib::core::current_tx::get_field;
use xrpl_wasm_stdlib::core::current_tx::traits::TransactionCommonFields;
use xrpl_wasm_stdlib::core::types::account_id::AccountID;
use xrpl_wasm_stdlib::core::types::uint::Hash256;
use xrpl_wasm_stdlib::host::Result;

pub trait ContractCallFields: TransactionCommonFields {
    fn get_contract_account(&self) -> Result<AccountID> {
        get_field(sfield::ContractAccount)
    }

    fn get_id(&self) -> Result<Hash256> {
        get_field(sfield::ContractID)
    }
}
