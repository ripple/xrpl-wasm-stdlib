use xrpl_common_stdlib::objects::traits::LedgerObjectCommonFields;

use crate::ledger_objects::traits::EscrowFields;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Escrow {
    pub(crate) slot_num: i32,
}

impl LedgerObjectCommonFields for Escrow {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl EscrowFields for Escrow {}

impl Escrow {
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }
}
