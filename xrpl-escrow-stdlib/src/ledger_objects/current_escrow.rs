use xrpl_common_stdlib::objects::traits::CurrentLedgerObjectCommonFields;

use crate::ledger_objects::traits::CurrentEscrowFields;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct CurrentEscrow;

impl CurrentLedgerObjectCommonFields for CurrentEscrow {}

impl CurrentEscrowFields for CurrentEscrow {}

#[inline]
pub fn get_current_escrow() -> CurrentEscrow {
    CurrentEscrow
}
