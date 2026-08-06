//! The slot-based `Escrow` handle.
//!
//! `Escrow` and the generic `EscrowFields` trait are generated in
//! `xrpl_common_stdlib::objects` (re-exported here for a stable path). Every field — including
//! `Data`, which reads as a `StandardBlob` via `EscrowFields::data()` — is available through the
//! generated trait. Only the escrow-only, host-mutable *write* of `Data` (`set_data`) is
//! hand-written; see `ctx::EscrowFinishContext` / `ledger_objects::traits`.

pub use xrpl_common_stdlib::objects::Escrow;

#[cfg(test)]
mod tests {
    use super::*;
    use xrpl_common_stdlib::objects::traits::LedgerObjectCommonFields;

    #[test]
    fn test_new() {
        let escrow = Escrow::new(42);
        assert_eq!(escrow.get_slot_num(), 42);
    }
}
