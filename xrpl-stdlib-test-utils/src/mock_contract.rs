//! Smart Contract-specific scenario builder on top of [`crate::mock_common`].
//!
//! Translates domain facts (the calling account, function parameters, stored contract data,
//! ...) into `MockHostBindings` expectations, so tests read in terms of the contract scenario
//! instead of raw host-function wiring. Same shape as [`crate::mock_escrow::EscrowScenario`].

use crate::mock_common::{
    MockGuard, MockHostBindings, apply_default_expectations, setup_mock, write_bytes,
};
use xrpl_wasm_stdlib::core::type_codes::{
    STI_ACCOUNT, STI_UINT8, STI_UINT16, STI_UINT32, STI_UINT64, STI_UINT128,
};
use xrpl_wasm_stdlib::core::types::account_id::AccountID;
use xrpl_wasm_stdlib::sfield;

/// Pre-wires common Smart Contract test setups onto a [`MockHostBindings`].
///
/// ```ignore
/// let _guard = ContractScenario::builder()
///     .with_tx_account(some_account)
///     .with_function_param(0, 42u32)
///     .with_get_data_returns("balance", 1_000u64)
///     .install();
/// ```
pub struct ContractScenario;

impl ContractScenario {
    pub fn builder() -> ContractScenarioBuilder {
        ContractScenarioBuilder::default()
    }
}

#[derive(Default)]
pub struct ContractScenarioBuilder {
    tx_account: Option<AccountID>,
    function_params: Vec<(i32, i32, Vec<u8>)>,
    data_entries: Vec<(Vec<u8>, Vec<u8>)>,
}

impl ContractScenarioBuilder {
    /// Configures the `Account` field (the transaction sender) read via
    /// `TransactionCommonFields`/`ContractCallFields`.
    pub fn with_tx_account(mut self, account: AccountID) -> Self {
        self.tx_account = Some(account);
        self
    }

    /// Configures the value returned for the function parameter at `index`.
    ///
    /// Covers the common primitive types (`u8`/`u16`/`u32`/`u64`/`u128`/[`AccountID`]); for
    /// anything else, fall back to a raw `mock.expect_function_param()` via [`build`](Self::build)
    /// or [`build_onto`](Self::build_onto).
    pub fn with_function_param(mut self, index: i32, value: impl FunctionParamValue) -> Self {
        self.function_params
            .push((index, value.type_code(), value.to_param_bytes()));
        self
    }

    /// Configures the value returned by `ContractStorage::get` for `key`, regardless of which
    /// account's storage is being read.
    ///
    /// Covers the common primitive types (`u8`/`u16`/`u32`/`u64`/`u128`/[`AccountID`]); for
    /// anything else, fall back to a raw `mock.expect_get_data_object_field()` via
    /// [`build`](Self::build) or [`build_onto`](Self::build_onto).
    pub fn with_get_data_returns(
        mut self,
        key: impl AsRef<[u8]>,
        value: impl StorageValue,
    ) -> Self {
        self.data_entries
            .push((key.as_ref().to_vec(), value.to_data_bytes()));
        self
    }

    /// Builds a mock with this scenario's expectations, falling back to
    /// [`apply_default_expectations`] for anything the scenario doesn't configure.
    pub fn build(self) -> MockHostBindings {
        let mut mock = MockHostBindings::new();
        self.apply(&mut mock);
        apply_default_expectations(&mut mock);
        mock
    }

    /// Layers this scenario's expectations onto an existing mock. mockall matches
    /// expectations in the order they were registered, so anything already set on `mock`
    /// takes precedence over what the scenario adds here.
    pub fn build_onto(self, mut mock: MockHostBindings) -> MockHostBindings {
        self.apply(&mut mock);
        mock
    }

    /// Builds the scenario and installs it as the thread-local mock. The returned guard
    /// clears the mock on drop.
    pub fn install(self) -> MockGuard {
        setup_mock(self.build())
    }

    fn apply(&self, mock: &mut MockHostBindings) {
        if let Some(account) = self.tx_account {
            let account_code = i32::from(sfield::Account);
            mock.expect_get_tx_field()
                .returning(move |field, out_buff_ptr, out_buff_len| {
                    if field == account_code {
                        return unsafe { write_bytes(&account.0, out_buff_ptr, out_buff_len) };
                    }
                    out_buff_len as i32
                });
        }

        if !self.function_params.is_empty() {
            let params = self.function_params.clone();
            mock.expect_function_param().returning(
                move |index, st_type_id, out_buff_ptr, out_buff_len| {
                    for (param_index, param_type_code, bytes) in &params {
                        if *param_index == index && *param_type_code == st_type_id {
                            return unsafe { write_bytes(bytes, out_buff_ptr, out_buff_len) };
                        }
                    }
                    out_buff_len as i32
                },
            );
        }

        if !self.data_entries.is_empty() {
            let entries = self.data_entries.clone();
            mock.expect_get_data_object_field().returning(
                move |_account_ptr, _account_len, key_ptr, key_len, out_buff_ptr, out_buff_len| {
                    let key = unsafe { std::slice::from_raw_parts(key_ptr, key_len) };
                    for (entry_key, bytes) in &entries {
                        if entry_key.as_slice() == key {
                            return unsafe {
                                write_bytes(bytes, out_buff_ptr as *mut u8, out_buff_len)
                            };
                        }
                    }
                    out_buff_len as i32
                },
            );
        }
    }
}

/// Types that [`ContractScenarioBuilder::with_function_param`] can encode into the raw bytes
/// `function_param` hands back, mirroring `xrpl-contract-stdlib`'s `FuncParamBytes` decoders
/// (little-endian for integers, raw bytes for [`AccountID`]).
pub trait FunctionParamValue {
    fn type_code(&self) -> i32;
    fn to_param_bytes(&self) -> Vec<u8>;
}

impl FunctionParamValue for u8 {
    fn type_code(&self) -> i32 {
        i32::from(STI_UINT8)
    }
    fn to_param_bytes(&self) -> Vec<u8> {
        vec![*self]
    }
}

impl FunctionParamValue for u16 {
    fn type_code(&self) -> i32 {
        i32::from(STI_UINT16)
    }
    fn to_param_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl FunctionParamValue for u32 {
    fn type_code(&self) -> i32 {
        i32::from(STI_UINT32)
    }
    fn to_param_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl FunctionParamValue for u64 {
    fn type_code(&self) -> i32 {
        i32::from(STI_UINT64)
    }
    fn to_param_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl FunctionParamValue for u128 {
    fn type_code(&self) -> i32 {
        i32::from(STI_UINT128)
    }
    fn to_param_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl FunctionParamValue for AccountID {
    fn type_code(&self) -> i32 {
        i32::from(STI_ACCOUNT)
    }
    fn to_param_bytes(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

/// Types that [`ContractScenarioBuilder::with_get_data_returns`] can encode into the raw bytes
/// `get_data_object_field` hands back, mirroring `xrpl-contract-stdlib`'s `FromDataBytes`
/// decoders (big-endian for integers, a leading `0x14` length byte for [`AccountID`]).
pub trait StorageValue {
    fn to_data_bytes(&self) -> Vec<u8>;
}

impl StorageValue for u8 {
    fn to_data_bytes(&self) -> Vec<u8> {
        vec![*self]
    }
}

impl StorageValue for u16 {
    fn to_data_bytes(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}

impl StorageValue for u32 {
    fn to_data_bytes(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}

impl StorageValue for u64 {
    fn to_data_bytes(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}

impl StorageValue for u128 {
    fn to_data_bytes(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}

impl StorageValue for AccountID {
    fn to_data_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(21);
        bytes.push(0x14);
        bytes.extend_from_slice(&self.0);
        bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use xrpl_wasm_stdlib::core::current_tx::get_field;

    fn test_account() -> AccountID {
        AccountID::from([0xCD; 20])
    }

    #[test]
    fn with_tx_account_is_readable_back_through_the_real_getter() {
        let _guard = ContractScenario::builder()
            .with_tx_account(test_account())
            .install();

        let account: AccountID = get_field(sfield::Account).unwrap();
        assert_eq!(account, test_account());
    }

    #[test]
    fn with_function_param_is_readable_back_through_the_real_host_call() {
        let _guard = ContractScenario::builder()
            .with_function_param(0, 42u32)
            .install();

        let mut buf = [0u8; 4];
        let result = unsafe {
            xrpl_wasm_stdlib::host::function_param(
                0,
                i32::from(STI_UINT32),
                buf.as_mut_ptr(),
                buf.len(),
            )
        };
        assert_eq!(result, 4);
        assert_eq!(u32::from_le_bytes(buf), 42);
    }

    #[test]
    fn with_function_param_falls_back_to_default_for_unconfigured_indices() {
        let _guard = ContractScenario::builder()
            .with_function_param(0, 42u32)
            .install();

        let mut buf = [0u8; 4];
        let result = unsafe {
            xrpl_wasm_stdlib::host::function_param(
                1,
                i32::from(STI_UINT32),
                buf.as_mut_ptr(),
                buf.len(),
            )
        };
        assert_eq!(result, buf.len() as i32);
    }

    #[test]
    fn with_get_data_returns_is_readable_back_through_the_real_host_call() {
        let _guard = ContractScenario::builder()
            .with_get_data_returns("balance", 1_000u64)
            .install();

        let account = test_account();
        let mut buf = [0u8; 8];
        let result = unsafe {
            xrpl_wasm_stdlib::host::get_data_object_field(
                account.0.as_ptr(),
                account.0.len(),
                b"balance".as_ptr(),
                b"balance".len(),
                buf.as_mut_ptr(),
                buf.len(),
            )
        };
        assert_eq!(result, 8);
        assert_eq!(u64::from_be_bytes(buf), 1_000);
    }

    #[test]
    fn build_onto_lets_the_caller_override_the_scenario() {
        let overridden_account = AccountID::from([0u8; 20]);
        let mut mock = MockHostBindings::new();
        let expected_code: i32 = i32::from(sfield::Account);
        mock.expect_get_tx_field()
            .withf(move |field, _, _| *field == expected_code)
            .returning(move |_, out_buff_ptr, out_buff_len| unsafe {
                write_bytes(&overridden_account.0, out_buff_ptr, out_buff_len)
            });

        // The caller's own expectation above was registered first, so it takes precedence
        // over the scenario's account expectation added by `build_onto`.
        let mock = ContractScenario::builder()
            .with_tx_account(test_account())
            .build_onto(mock);

        let _guard = setup_mock(mock);
        let account: AccountID = get_field(sfield::Account).unwrap();
        assert_eq!(account, overridden_account);
        assert_ne!(account, test_account());
    }
}
