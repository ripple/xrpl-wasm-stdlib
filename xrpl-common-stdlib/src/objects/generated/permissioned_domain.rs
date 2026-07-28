// GENERATED -- do not hand-edit. Run scripts/generate-ledger-objects.sh to regenerate.

use crate::host::Result;
use crate::objects::array_object::Array;
use crate::objects::traits::CurrentLedgerObjectCommonFields;
use crate::objects::traits::LedgerObjectCommonFields;
use crate::objects::{current_ledger_object, ledger_object};
use crate::sfield;
use crate::types::account_id::AccountID;
use crate::types::uint::Hash256;

/// Trait providing access to fields specific to PermissionedDomain objects in any ledger.
pub trait PermissionedDomainFields: LedgerObjectCommonFields {
    /// The address of the account that owns this domain.
    fn get_owner(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Owner)
    }

    /// The `Sequence` value of the transaction that created this entry.
    fn get_sequence(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::Sequence)
    }

    /// A list of 1 to 10 Credential objects that grant access to this domain. The array is stored sorted by issuer.
    fn get_accepted_credentials(&self) -> Result<Array> {
        ledger_object::get_field(self.get_slot_num(), sfield::AcceptedCredentials)
    }

    /// A hint indicating which page of the owner directory links to this entry, in case the directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::OwnerNode)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The [index of the ledger][Ledger Index] that contains the transaction that most recently modified this object.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }
}

/// Trait providing access to fields specific to the current PermissionedDomain object.
pub trait CurrentPermissionedDomainFields: CurrentLedgerObjectCommonFields {
    /// The address of the account that owns this domain.
    fn get_owner(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Owner)
    }

    /// The `Sequence` value of the transaction that created this entry.
    fn get_sequence(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::Sequence)
    }

    /// A list of 1 to 10 Credential objects that grant access to this domain. The array is stored sorted by issuer.
    fn get_accepted_credentials(&self) -> Result<Array> {
        current_ledger_object::get_field(sfield::AcceptedCredentials)
    }

    /// A hint indicating which page of the owner directory links to this entry, in case the directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::OwnerNode)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::PreviousTxnID)
    }

    /// The [index of the ledger][Ledger Index] that contains the transaction that most recently modified this object.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::PreviousTxnLgrSeq)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct PermissionedDomain {
    pub(crate) slot_num: i32,
}

impl PermissionedDomain {
    /// Binds this handle to a host-managed slot holding a PermissionedDomain ledger object.
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }
}

impl LedgerObjectCommonFields for PermissionedDomain {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl PermissionedDomainFields for PermissionedDomain {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::host::host_bindings_trait::MockHostBindings;
    use crate::host::setup_mock;
    use crate::objects::test_support::*;

    #[test]
    fn read_all_fields() {
        let mut mock = MockHostBindings::new();
        mock_all_fields_present(&mut mock);
        let _guard = setup_mock(mock);

        let obj = PermissionedDomain::new(0);

        assert!(obj.get_owner().is_ok());
        assert!(obj.get_sequence().is_ok());
        assert!(obj.get_owner_node().is_ok());
        assert!(obj.get_previous_txn_id().is_ok());
        assert!(obj.get_previous_txn_lgr_seq().is_ok());
    }
}
