use crate::core::ledger_objects::traits::{EscrowFields, LedgerObjectCommonFields};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub struct Escrow {
    pub(crate) slot_num: i32,
}

impl LedgerObjectCommonFields for Escrow {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl EscrowFields for Escrow {}
