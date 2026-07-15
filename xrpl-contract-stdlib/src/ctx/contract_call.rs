use crate::current_tx::contract_call::ContractCall;
use crate::current_tx::traits::ContractCallFields;
use crate::data::codec::{
    AsKeyBytes, FromDataBytes, ToDataBytes, get_array_element, get_data, get_nested_array_element,
    get_nested_data, set_array_element, set_data, set_nested_array_element, set_nested_data,
};
use crate::submit::emit::{EmitError, EmittableTx};
use xrpl_wasm_stdlib::core::types::account_id::AccountID;
#[cfg(test)]
use xrpl_wasm_stdlib::core::types::transaction_type::TransactionType;
use xrpl_wasm_stdlib::ctx::SmartFeatureContext;
use xrpl_wasm_stdlib::host;
use xrpl_wasm_stdlib::host::{build_txn, emit_built_txn};

/// Entry-point context for a Smart Contract call.
///
/// Provides access to the current [`ContractCall`] transaction via
/// [`SmartFeatureContext::tx`], plus Smart-Contract-unique host functions
/// (persistent storage via [`storage`](ContractCallContext::storage) /
/// [`user_storage`](ContractCallContext::user_storage), and outbound
/// transaction emission via [`emit`](ContractCallContext::emit)) as safe
/// methods; no `unsafe` code is needed in user crates.
#[derive(Default)]
pub struct ContractCallContext {
    tx: ContractCall,
}

impl SmartFeatureContext for ContractCallContext {
    type Tx = ContractCall;
    fn tx(&self) -> &Self::Tx {
        &self.tx
    }
}

impl ContractCallContext {
    /// Returns a [`ContractStorage`] handle bound to this contract's own
    /// account -- i.e. contract-owned `ContractData` (`Owner` = the
    /// contract's pseudo-account).
    pub fn storage(&self) -> ContractStorage {
        let account = self.tx.get_contract_account().unwrap_or_panic();
        ContractStorage { account }
    }

    /// Returns a [`ContractStorage`] handle bound to `user`'s account --
    /// i.e. user-owned `ContractData` (`Owner` = `user`, `ContractAccount` =
    /// this contract).
    pub fn user_storage(&self, user: &AccountID) -> ContractStorage {
        ContractStorage { account: *user }
    }

    /// Builds and emits `txn` as a new outbound transaction.
    pub fn emit(&self, txn: impl EmittableTx) -> Result<(), EmitError> {
        let txn_index = unsafe { build_txn(txn.transaction_type() as i32) };
        if txn_index < 0 {
            return Err(EmitError::BuildFailed(txn_index));
        }

        txn.write_fields(txn_index)?;

        let n = unsafe { emit_built_txn(txn_index) };
        if n < 0 {
            return Err(EmitError::EmitFailed(n));
        }
        Ok(())
    }
}

/// A handle to one account's `ContractData`, scoped by [`ContractCallContext::storage`]
/// (the contract's own data) or [`ContractCallContext::user_storage`] (a
/// specific user's data under this contract).
///
/// Thin wrapper over the generic `get_data`/`set_data` family in
/// [`crate::data::codec`], which are already keyed by `&AccountID` -- this
/// type just binds that account once so call sites don't have to repeat it.
pub struct ContractStorage {
    account: AccountID,
}

impl ContractStorage {
    #[inline]
    pub fn get<T: FromDataBytes>(&self, key: &(impl AsKeyBytes + ?Sized)) -> Option<T> {
        get_data(&self.account, key)
    }

    #[inline]
    pub fn get_nested<T: FromDataBytes>(
        &self,
        nested: &(impl AsKeyBytes + ?Sized),
        key: &(impl AsKeyBytes + ?Sized),
    ) -> Option<T> {
        get_nested_data(&self.account, nested, key)
    }

    #[inline]
    pub fn get_array_element<T: FromDataBytes>(
        &self,
        key: &(impl AsKeyBytes + ?Sized),
        index: usize,
    ) -> Option<T> {
        get_array_element(&self.account, key, index)
    }

    #[inline]
    pub fn get_nested_array_element<T: FromDataBytes>(
        &self,
        key: &(impl AsKeyBytes + ?Sized),
        index: usize,
        field_key: &(impl AsKeyBytes + ?Sized),
    ) -> Option<T> {
        get_nested_array_element(&self.account, key, index, field_key)
    }

    #[inline]
    pub fn set<T: ToDataBytes>(
        &self,
        key: &(impl AsKeyBytes + ?Sized),
        value: T,
    ) -> host::Result<()> {
        wrap(set_data(&self.account, key, value))
    }

    #[inline]
    pub fn set_nested<T: ToDataBytes>(
        &self,
        nested: &(impl AsKeyBytes + ?Sized),
        key: &(impl AsKeyBytes + ?Sized),
        value: T,
    ) -> host::Result<()> {
        wrap(set_nested_data(&self.account, nested, key, value))
    }

    #[inline]
    pub fn set_array_element<T: ToDataBytes>(
        &self,
        key: &(impl AsKeyBytes + ?Sized),
        index: usize,
        value: T,
    ) -> host::Result<()> {
        wrap(set_array_element(&self.account, key, index, value))
    }

    #[inline]
    pub fn set_nested_array_element<T: ToDataBytes>(
        &self,
        key: &(impl AsKeyBytes + ?Sized),
        index: usize,
        field_key: &(impl AsKeyBytes + ?Sized),
        value: T,
    ) -> host::Result<()> {
        wrap(set_nested_array_element(
            &self.account,
            key,
            index,
            field_key,
            value,
        ))
    }
}

#[inline]
fn wrap(result: Result<(), i32>) -> host::Result<()> {
    match result {
        Ok(()) => host::Result::Ok(()),
        Err(code) => host::Result::Err(host::Error::from_code(code)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use xrpl_wasm_stdlib::host::host_bindings_trait::MockHostBindings;
    use xrpl_wasm_stdlib::host::setup_mock;

    #[test]
    fn default_constructs() {
        let _ctx = ContractCallContext::default();
    }

    #[test]
    fn tx_accessor() {
        let ctx = ContractCallContext::default();
        let _tx: &ContractCall = ctx.tx();
    }

    #[test]
    fn user_storage_binds_given_account_without_a_host_call() {
        // No mock expectations set: if this called into the host at all, the
        // unconfigured mock would panic, so this also proves user_storage
        // never resolves ContractAccount for its own account.
        let ctx = ContractCallContext::default();
        let user = AccountID([7u8; 20]);
        let storage = ctx.user_storage(&user);
        assert_eq!(storage.account, user);
    }

    struct StubTx {
        txn_type: TransactionType,
        field_result: Result<(), EmitError>,
    }

    impl EmittableTx for StubTx {
        fn transaction_type(&self) -> TransactionType {
            self.txn_type
        }

        fn write_fields(&self, _txn_index: i32) -> Result<(), EmitError> {
            self.field_result
        }
    }

    #[test]
    fn emit_returns_build_failed_on_negative_build_txn() {
        let mut mock = MockHostBindings::new();
        mock.expect_build_txn().times(1).returning(|_| -7);
        let _guard = setup_mock(mock);

        let ctx = ContractCallContext::default();
        let txn = StubTx {
            txn_type: TransactionType::Payment,
            field_result: Ok(()),
        };
        assert_eq!(ctx.emit(txn), Err(EmitError::BuildFailed(-7)));
    }

    #[test]
    fn emit_propagates_field_write_error_without_emitting() {
        let mut mock = MockHostBindings::new();
        mock.expect_build_txn().times(1).returning(|_| 0);
        // emit_built_txn must never be called if field writing failed.
        mock.expect_emit_built_txn().times(0);
        let _guard = setup_mock(mock);

        let ctx = ContractCallContext::default();
        let txn = StubTx {
            txn_type: TransactionType::Payment,
            field_result: Err(EmitError::FieldFailed(-9)),
        };
        assert_eq!(ctx.emit(txn), Err(EmitError::FieldFailed(-9)));
    }

    #[test]
    fn emit_succeeds_when_build_fields_and_emit_all_succeed() {
        let mut mock = MockHostBindings::new();
        mock.expect_build_txn().times(1).returning(|_| 0);
        mock.expect_emit_built_txn().times(1).returning(|_| 0);
        let _guard = setup_mock(mock);

        let ctx = ContractCallContext::default();
        let txn = StubTx {
            txn_type: TransactionType::Payment,
            field_result: Ok(()),
        };
        assert_eq!(ctx.emit(txn), Ok(()));
    }

    #[test]
    fn emit_returns_emit_failed_on_negative_emit_built_txn() {
        let mut mock = MockHostBindings::new();
        mock.expect_build_txn().times(1).returning(|_| 0);
        mock.expect_emit_built_txn().times(1).returning(|_| -12);
        let _guard = setup_mock(mock);

        let ctx = ContractCallContext::default();
        let txn = StubTx {
            txn_type: TransactionType::Payment,
            field_result: Ok(()),
        };
        assert_eq!(ctx.emit(txn), Err(EmitError::EmitFailed(-12)));
    }
}
