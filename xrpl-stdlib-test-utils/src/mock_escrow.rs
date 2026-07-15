//! Escrow-specific scenario builder on top of [`crate::mock_common`].
//!
//! Translates domain facts (account, amount, ...) into `MockHostBindings` expectations, so
//! tests read in terms of the escrow scenario instead of raw host-function wiring.

use crate::mock_common::{MockGuard, MockHostBindings, apply_default_expectations, setup_mock};
use xrpl_wasm_stdlib::core::types::account_id::AccountID;
use xrpl_wasm_stdlib::core::types::amount::Amount;
use xrpl_wasm_stdlib::host::Error;
use xrpl_wasm_stdlib::host::error_codes::BUFFER_TOO_SMALL;
use xrpl_wasm_stdlib::sfield;

/// Pre-wires common Smart Escrow test setups onto a [`MockHostBindings`].
///
/// ```ignore
/// let _guard = EscrowScenario::builder()
///     .with_account(some_account)
///     .with_amount(Amount::XRP { num_drops: 1000 })
///     .install();
/// ```
pub struct EscrowScenario;

impl EscrowScenario {
    pub fn builder() -> EscrowScenarioBuilder {
        EscrowScenarioBuilder::default()
    }
}

#[derive(Default)]
pub struct EscrowScenarioBuilder {
    account: Option<AccountID>,
    amount: Option<Amount>,
    // Stored pre-converted to a host status code (0 == success) rather than `Result<(), Error>`
    // so the builder doesn't need `Result`/`Error` to be `Copy` to stash it in a field.
    update_data_status: Option<i32>,
}

impl EscrowScenarioBuilder {
    pub fn with_account(mut self, account: AccountID) -> Self {
        self.account = Some(account);
        self
    }

    pub fn with_amount(mut self, amount: Amount) -> Self {
        self.amount = Some(amount);
        self
    }

    pub fn with_update_data_returns(mut self, result: Result<(), Error>) -> Self {
        self.update_data_status = Some(match result {
            Ok(()) => 0,
            Err(error) => error.code(),
        });
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
        if self.account.is_some() || self.amount.is_some() {
            let account = self.account;
            let amount = self.amount.clone();
            let account_code = i32::from(sfield::Account);
            let amount_code = i32::from(sfield::Amount);

            mock.expect_get_tx_field()
                .returning(move |field, out_buff_ptr, out_buff_len| {
                    if field == account_code
                        && let Some(account) = account
                    {
                        return write_bytes(&account.0, out_buff_ptr, out_buff_len);
                    }
                    if field == amount_code
                        && let Some(amount) = &amount
                    {
                        let (bytes, len) = amount.to_stamount_bytes();
                        return write_bytes(&bytes[..len], out_buff_ptr, out_buff_len);
                    }
                    out_buff_len as i32
                });
        }

        if let Some(status) = self.update_data_status {
            mock.expect_update_data().returning(
                move |_data_ptr, data_len| {
                    if status == 0 { data_len as i32 } else { status }
                },
            );
        }
    }
}

/// Writes `bytes` into the raw output buffer, mirroring how the real host functions report
/// back the number of bytes written (or `BUFFER_TOO_SMALL` if the caller's buffer is too small).
fn write_bytes(bytes: &[u8], out_buff_ptr: *mut u8, out_buff_len: usize) -> i32 {
    if out_buff_len < bytes.len() {
        return BUFFER_TOO_SMALL;
    }
    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), out_buff_ptr, bytes.len());
    }
    bytes.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use xrpl_wasm_stdlib::core::current_tx::get_field;

    fn test_account() -> AccountID {
        AccountID::from([0xAB; 20])
    }

    #[test]
    fn write_bytes_returns_buffer_too_small_when_the_caller_buffer_is_undersized() {
        let mut undersized = [0u8; 4];
        let result = write_bytes(&[1, 2, 3, 4, 5], undersized.as_mut_ptr(), undersized.len());
        assert_eq!(result, BUFFER_TOO_SMALL);
    }

    #[test]
    fn with_account_is_readable_back_through_the_real_getter() {
        let _guard = EscrowScenario::builder()
            .with_account(test_account())
            .install();

        let account: AccountID = get_field(sfield::Account).unwrap();
        assert_eq!(account, test_account());
    }

    #[test]
    fn with_amount_is_readable_back_through_the_real_getter() {
        let configured = Amount::XRP { num_drops: 1_000 };
        let _guard = EscrowScenario::builder()
            .with_amount(configured.clone())
            .install();

        let amount: Amount = get_field(sfield::Amount).unwrap();
        assert_eq!(amount, configured);
    }

    #[test]
    fn unconfigured_fields_fall_back_to_defaults() {
        let _guard = EscrowScenario::builder()
            .with_account(test_account())
            .install();

        // OfferSequence wasn't configured by the scenario; the default fallback (declared
        // after the scenario's expectation) still handles it instead of panicking.
        let result: xrpl_wasm_stdlib::host::Result<u32> = get_field(sfield::OfferSequence);
        assert!(result.is_ok());
    }

    #[test]
    fn build_onto_lets_the_caller_override_the_scenario() {
        let overridden_account = AccountID::from([0u8; 20]);
        let mut mock = MockHostBindings::new();
        let expected_code: i32 = i32::from(sfield::Account);
        mock.expect_get_tx_field()
            .withf(move |field, _, _| *field == expected_code)
            .returning(move |_, out_buff_ptr, out_buff_len| {
                write_bytes(&overridden_account.0, out_buff_ptr, out_buff_len)
            });

        // The caller's own expectation above was registered first, so it takes precedence
        // over the scenario's account expectation added by `build_onto`.
        let mock = EscrowScenario::builder()
            .with_account(test_account())
            .build_onto(mock);

        let _guard = setup_mock(mock);
        let account: AccountID = get_field(sfield::Account).unwrap();
        assert_eq!(account, overridden_account);
        assert_ne!(account, test_account());
    }
}
