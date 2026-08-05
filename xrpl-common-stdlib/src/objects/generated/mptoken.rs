// GENERATED -- do not hand-edit. Run scripts/generate-ledger-objects.sh to regenerate.

use crate::host::Result;
use crate::objects::traits::CurrentLedgerObjectCommonFields;
use crate::objects::traits::LedgerObjectCommonFields;
use crate::objects::{current_ledger_object, ledger_object};
use crate::sfield;
use crate::types::account_id::AccountID;
use crate::types::uint::{Hash192, Hash256};

/// Trait providing access to fields specific to MPToken objects in any ledger.
pub trait MPTokenFields: LedgerObjectCommonFields {
    /// The owner (holder) of these MPTs.
    fn account(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Account)
    }

    /// The `MPTokenIssuance` identifier.
    fn mptoken_issuance_id(&self) -> Result<Hash192> {
        ledger_object::get_field(self.get_slot_num(), sfield::MPTokenIssuanceID)
    }

    /// The amount of tokens currently held by the owner. The minimum is 0 and the maximum is
    /// 2^63-1.
    fn mpt_amount(&self) -> Result<Option<u64>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::MPTAmount)
    }

    /// The amount of tokens currently locked up (for example, in escrow).
    fn locked_amount(&self) -> Result<Option<u64>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::LockedAmount)
    }

    /// A hint indicating which page of the owner directory links to this entry, in case the
    /// directory consists of multiple pages.
    fn owner_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::OwnerNode)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn previous_txn_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The sequence of the ledger that contains the transaction that most recently modified this
    /// object.
    fn previous_txn_lgr_seq(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }
}

/// Trait providing access to fields specific to the current MPToken object.
pub trait CurrentMPTokenFields: CurrentLedgerObjectCommonFields {
    /// The owner (holder) of these MPTs.
    fn account(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Account)
    }

    /// The `MPTokenIssuance` identifier.
    fn mptoken_issuance_id(&self) -> Result<Hash192> {
        current_ledger_object::get_field(sfield::MPTokenIssuanceID)
    }

    /// The amount of tokens currently held by the owner. The minimum is 0 and the maximum is
    /// 2^63-1.
    fn mpt_amount(&self) -> Result<Option<u64>> {
        current_ledger_object::get_field_optional(sfield::MPTAmount)
    }

    /// The amount of tokens currently locked up (for example, in escrow).
    fn locked_amount(&self) -> Result<Option<u64>> {
        current_ledger_object::get_field_optional(sfield::LockedAmount)
    }

    /// A hint indicating which page of the owner directory links to this entry, in case the
    /// directory consists of multiple pages.
    fn owner_node(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::OwnerNode)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn previous_txn_id(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::PreviousTxnID)
    }

    /// The sequence of the ledger that contains the transaction that most recently modified this
    /// object.
    fn previous_txn_lgr_seq(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::PreviousTxnLgrSeq)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct MPToken {
    pub(crate) slot_num: i32,
}

impl MPToken {
    /// Binds this handle to a host-managed slot holding a MPToken ledger object.
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }
}

impl LedgerObjectCommonFields for MPToken {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl MPTokenFields for MPToken {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::host::host_bindings_trait::MockHostBindings;
    use crate::host::setup_mock;
    use crate::objects::test_utils::*;

    #[test]
    fn read_all_fields() {
        let mut mock = MockHostBindings::new();
        mock_all_fields_present(&mut mock);
        let _guard = setup_mock(mock);

        let obj = MPToken::new(0);

        assert!(obj.account().is_ok());
        assert!(obj.mptoken_issuance_id().is_ok());
        assert!(obj.owner_node().is_ok());
        assert!(obj.previous_txn_id().is_ok());
        assert!(obj.previous_txn_lgr_seq().is_ok());
        assert!(obj.mpt_amount().is_ok());
        assert!(obj.locked_amount().is_ok());
    }

    #[test]
    fn optional_fields_none() {
        let mut mock = MockHostBindings::new();
        mock_all_fields_not_found(&mut mock);
        let _guard = setup_mock(mock);

        let obj = MPToken::new(0);

        assert!(obj.mpt_amount().unwrap().is_none());
        assert!(obj.locked_amount().unwrap().is_none());
    }
}
