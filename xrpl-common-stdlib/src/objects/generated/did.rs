// GENERATED -- do not hand-edit. Run scripts/generate-ledger-objects.sh to regenerate.

use crate::host::Result;
use crate::objects::traits::CurrentLedgerObjectCommonFields;
use crate::objects::traits::LedgerObjectCommonFields;
use crate::objects::{current_ledger_object, ledger_object};
use crate::sfield;
use crate::types::account_id::AccountID;
use crate::types::blob::{StandardBlob, UriBlob};
use crate::types::uint::Hash256;

/// Trait providing access to fields specific to DID objects in any ledger.
pub trait DIDFields: LedgerObjectCommonFields {
    /// The account that controls the DID.
    fn account(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Account)
    }

    /// The W3C standard DID document associated with the DID. The `DIDDocument` field isn't checked
    /// for validity and is limited to a maximum length of 256 bytes.
    fn did_document(&self) -> Result<Option<StandardBlob>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::DIDDocument)
    }

    /// The Universal Resource Identifier that points to the corresponding DID document or the data
    /// associated with the DID. This field can be an HTTP(S) URL or IPFS URI. This field isn't
    /// checked for validity and is limited to a maximum length of 256 bytes.
    fn uri(&self) -> Result<Option<UriBlob>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::URI)
    }

    /// The public attestations of identity credentials associated with the DID. The `Data` field
    /// isn't checked for validity and is limited to a maximum length of 256 bytes.
    fn data(&self) -> Result<Option<StandardBlob>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::Data)
    }

    /// A hint indicating which page of the sender's owner directory links to this entry, in case
    /// the directory consists of multiple pages.
    fn owner_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::OwnerNode)
    }

    /// The identifying hash of the transaction that most recently modified this object.
    fn previous_txn_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// object.
    fn previous_txn_lgr_seq(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }
}

/// Trait providing access to fields specific to the current DID object.
pub trait CurrentDIDFields: CurrentLedgerObjectCommonFields {
    /// The account that controls the DID.
    fn account(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Account)
    }

    /// The W3C standard DID document associated with the DID. The `DIDDocument` field isn't checked
    /// for validity and is limited to a maximum length of 256 bytes.
    fn did_document(&self) -> Result<Option<StandardBlob>> {
        current_ledger_object::get_field_optional(sfield::DIDDocument)
    }

    /// The Universal Resource Identifier that points to the corresponding DID document or the data
    /// associated with the DID. This field can be an HTTP(S) URL or IPFS URI. This field isn't
    /// checked for validity and is limited to a maximum length of 256 bytes.
    fn uri(&self) -> Result<Option<UriBlob>> {
        current_ledger_object::get_field_optional(sfield::URI)
    }

    /// The public attestations of identity credentials associated with the DID. The `Data` field
    /// isn't checked for validity and is limited to a maximum length of 256 bytes.
    fn data(&self) -> Result<Option<StandardBlob>> {
        current_ledger_object::get_field_optional(sfield::Data)
    }

    /// A hint indicating which page of the sender's owner directory links to this entry, in case
    /// the directory consists of multiple pages.
    fn owner_node(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::OwnerNode)
    }

    /// The identifying hash of the transaction that most recently modified this object.
    fn previous_txn_id(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// object.
    fn previous_txn_lgr_seq(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::PreviousTxnLgrSeq)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct DID {
    pub(crate) slot_num: i32,
}

impl DID {
    /// Binds this handle to a host-managed slot holding a DID ledger object.
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }
}

impl LedgerObjectCommonFields for DID {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl DIDFields for DID {}

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

        let obj = DID::new(0);

        assert!(obj.account().is_ok());
        assert!(obj.owner_node().is_ok());
        assert!(obj.previous_txn_id().is_ok());
        assert!(obj.previous_txn_lgr_seq().is_ok());
        assert!(obj.did_document().is_ok());
        assert!(obj.uri().is_ok());
        assert!(obj.data().is_ok());
    }
}
