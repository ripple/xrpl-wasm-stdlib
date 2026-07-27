//! The `Escrow` struct and its `EscrowFields` impl are generated — see
//! `crate::ledger_objects::generated`.

use crate::ledger_objects::traits::EscrowContractData;

pub use crate::ledger_objects::generated::Escrow;

impl EscrowContractData for Escrow {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let escrow = Escrow::new(42);
        assert_eq!(escrow.slot_num, 42);
    }
}
