use xrpl_common_stdlib::core::current_tx::traits::TransactionCommonFields;

use crate::current_tx::traits::EscrowFinishFields;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct EscrowFinish;

impl TransactionCommonFields for EscrowFinish {}

impl EscrowFinishFields for EscrowFinish {}

#[inline]
pub fn get_current_escrow_finish() -> EscrowFinish {
    EscrowFinish
}
