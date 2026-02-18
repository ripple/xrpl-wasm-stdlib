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

impl Escrow {
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let escrow = Escrow::new(42);
        assert_eq!(escrow.slot_num, 42);
    }
}
