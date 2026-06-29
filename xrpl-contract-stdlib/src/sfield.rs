#![allow(non_upper_case_globals)]

//! Contract-specific SField constants. Also re-exports the shared SFields from
//! `xrpl_wasm_stdlib::sfield` so contract code can access both via a single
//! `use crate::sfield;` import.

pub use xrpl_wasm_stdlib::sfield::*;

use xrpl_wasm_stdlib::core::ledger_objects::array_object::{Array, Object};
use xrpl_wasm_stdlib::core::types::account_id::AccountID;
use xrpl_wasm_stdlib::core::types::blob::StandardBlob;
use xrpl_wasm_stdlib::core::types::uint::Hash256;

pub const ParameterFlag: SField<u32, 131146> = SField::new();
pub const ContractHash: SField<Hash256, 327719> = SField::new();
pub const ContractID: SField<Hash256, 327720> = SField::new();
pub const ContractCode: SField<StandardBlob, 458785> = SField::new();
pub const FunctionName: SField<StandardBlob, 458786> = SField::new();
pub const ContractAccount: SField<AccountID, 524315> = SField::new();
pub const Function: SField<Object, 917542> = SField::new();
pub const InstanceParameter: SField<Object, 917543> = SField::new();
pub const InstanceParameterValue: SField<Object, 917544> = SField::new();
pub const Parameter: SField<Object, 917545> = SField::new();
pub const Functions: SField<Array, 983072> = SField::new();
pub const InstanceParameters: SField<Array, 983073> = SField::new();
pub const InstanceParameterValues: SField<Array, 983074> = SField::new();
pub const Parameters: SField<Array, 983075> = SField::new();
pub const ContractResult: SField<u8, 1048597> = SField::new();
pub const ParameterValue: SField<u8, 1769473> = SField::new();
pub const ParameterType: SField<u8, 1835009> = SField::new();
pub const ContractJson: SField<u8, 1900545> = SField::new();
