use xrpl_common_stdlib::ctx::SmartFeatureContext;
use xrpl_common_stdlib::host;

use crate::current_tx::escrow_finish::EscrowFinish;
use crate::ledger_objects::current_escrow::CurrentEscrow;

/// Entry-point context for a Smart Escrow finish operation.
///
/// Provides access to the current [`EscrowFinish`] transaction via
/// [`SmartFeatureContext::tx`] and to the escrow ledger object via
/// [`escrow`](EscrowFinishContext::escrow). Escrow-unique host functions
/// (e.g., [`update_data`](EscrowFinishContext::update_data)) are exposed as
/// safe inherent methods; no `unsafe` code is needed in user crates.
///
/// The `#[smart_escrow]` macro constructs this via `Default::default()` and
/// passes it to the user function.
pub struct EscrowFinishContext {
    tx: EscrowFinish,
    escrow: CurrentEscrow,
}

impl Default for EscrowFinishContext {
    fn default() -> Self {
        Self {
            tx: EscrowFinish,
            escrow: CurrentEscrow,
        }
    }
}

impl SmartFeatureContext for EscrowFinishContext {
    type Tx = EscrowFinish;
    fn tx(&self) -> &Self::Tx {
        &self.tx
    }
}

impl EscrowFinishContext {
    /// Returns a reference to the current escrow ledger object.
    pub fn escrow(&self) -> &CurrentEscrow {
        &self.escrow
    }

    /// **[host fn]** Write new data to the Smart Escrow object.
    pub fn update_data(&self, data: &[u8]) -> host::Result<()> {
        let n = unsafe { host::update_data(data.as_ptr(), data.len()) };
        if n < 0 {
            return host::Result::Err(host::Error::from_code(n));
        }
        host::Result::Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use xrpl_common_stdlib::host::Error;
    use xrpl_stdlib_test_utils::EscrowScenario;

    #[test]
    fn default_constructs() {
        let _ctx = EscrowFinishContext::default();
    }

    #[test]
    fn tx_and_escrow_accessors() {
        let ctx = EscrowFinishContext::default();
        let _tx: &EscrowFinish = ctx.tx();
        let _escrow: &CurrentEscrow = ctx.escrow();
    }

    #[test]
    fn update_data_returns_ok_on_success() {
        let _guard = EscrowScenario::builder()
            .with_update_data_returns(Ok(()))
            .install();

        let ctx = EscrowFinishContext::default();
        assert!(ctx.update_data(b"payload").is_ok());
    }

    #[test]
    fn update_data_returns_err_on_negative_code() {
        let _guard = EscrowScenario::builder()
            .with_update_data_returns(Err(Error::InternalError))
            .install();

        let ctx = EscrowFinishContext::default();
        assert!(ctx.update_data(b"payload").is_err());
    }
}
