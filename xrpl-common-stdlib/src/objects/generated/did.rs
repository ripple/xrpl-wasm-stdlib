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
    fn get_account(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Account)
    }

    /// The W3C standard DID document associated with the DID. The `DIDDocument` field isn't checked for validity and is limited to a maximum length of 256 bytes.
    fn get_did_document(&self) -> Result<Option<StandardBlob>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::DIDDocument)
    }

    /// The Universal Resource Identifier that points to the corresponding DID document or the data associated with the DID. This field can be an HTTP(S) URL or IPFS URI. This field isn't checked for validity and is limited to a maximum length of 256 bytes.
    fn get_uri(&self) -> Result<Option<UriBlob>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::URI)
    }

    /// The public attestations of identity credentials associated with the DID. The `Data` field isn't checked for validity and is limited to a maximum length of 256 bytes.
    fn get_data(&self) -> Result<Option<StandardBlob>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::Data)
    }

    /// A hint indicating which page of the sender's owner directory links to this entry, in case the directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::OwnerNode)
    }

    /// The identifying hash of the transaction that most recently modified this object.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this object.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }
}

/// Trait providing access to fields specific to the current DID object.
pub trait CurrentDIDFields: CurrentLedgerObjectCommonFields {
    /// The account that controls the DID.
    fn get_account(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Account)
    }

    /// The W3C standard DID document associated with the DID. The `DIDDocument` field isn't checked for validity and is limited to a maximum length of 256 bytes.
    fn get_did_document(&self) -> Result<Option<StandardBlob>> {
        current_ledger_object::get_field_optional(sfield::DIDDocument)
    }

    /// The Universal Resource Identifier that points to the corresponding DID document or the data associated with the DID. This field can be an HTTP(S) URL or IPFS URI. This field isn't checked for validity and is limited to a maximum length of 256 bytes.
    fn get_uri(&self) -> Result<Option<UriBlob>> {
        current_ledger_object::get_field_optional(sfield::URI)
    }

    /// The public attestations of identity credentials associated with the DID. The `Data` field isn't checked for validity and is limited to a maximum length of 256 bytes.
    fn get_data(&self) -> Result<Option<StandardBlob>> {
        current_ledger_object::get_field_optional(sfield::Data)
    }

    /// A hint indicating which page of the sender's owner directory links to this entry, in case the directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::OwnerNode)
    }

    /// The identifying hash of the transaction that most recently modified this object.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this object.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::PreviousTxnLgrSeq)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct DID {
    pub(crate) slot_num: i32,
}

impl DID {
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }

    /// Loads the DID ledger object identified by the given keylet arguments,
    /// caching it in a host-managed slot.
    pub fn load(account: &AccountID) -> Result<Self> {
        let keylet = match crate::keylets::did_keylet(account) {
            Result::Ok(k) => k,
            Result::Err(e) => return Result::Err(e),
        };
        let slot = unsafe { crate::host::cache_ledger_obj(keylet.as_ptr(), keylet.len(), 0) };
        if slot < 0 {
            return Result::Err(crate::host::Error::from_code(slot));
        }
        Result::Ok(Self { slot_num: slot })
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
    use crate::objects::test_support::*;

    #[test]
    fn read_all_fields() {
        let mut mock = MockHostBindings::new();
        mock_all_fields_present(&mut mock);
        let _guard = setup_mock(mock);

        let obj = DID::new(0);

        assert!(obj.get_account().is_ok());
        assert!(obj.get_owner_node().is_ok());
        assert!(obj.get_previous_txn_id().is_ok());
        assert!(obj.get_previous_txn_lgr_seq().is_ok());
        assert!(obj.get_did_document().is_ok());
        assert!(obj.get_uri().is_ok());
        assert!(obj.get_data().is_ok());
    }

    #[test]
    fn load_success() {
        let mut mock = MockHostBindings::new();
        mock_did_keylet_success(&mut mock);
        mock_cache_ledger_obj_success(&mut mock, 7);
        let _guard = setup_mock(mock);

        let result = DID::load(&sample::account_id());
        assert!(result.is_ok());
    }

    #[test]
    fn load_cache_error() {
        use crate::host::error_codes::INTERNAL_ERROR;

        let mut mock = MockHostBindings::new();
        mock_did_keylet_success(&mut mock);
        mock_cache_ledger_obj_error(&mut mock, INTERNAL_ERROR);
        let _guard = setup_mock(mock);

        let result = DID::load(&sample::account_id());
        assert!(result.is_err());
    }
}
