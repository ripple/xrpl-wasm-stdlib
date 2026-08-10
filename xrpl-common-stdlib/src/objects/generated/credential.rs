// GENERATED -- do not hand-edit. Run scripts/generate-ledger-objects.sh to regenerate.

use crate::host::Result;
use crate::objects::traits::CurrentLedgerObjectCommonFields;
use crate::objects::traits::LedgerObjectCommonFields;
use crate::objects::{current_ledger_object, ledger_object};
use crate::sfield;
use crate::types::account_id::AccountID;
use crate::types::blob::{StandardBlob, UriBlob};
use crate::types::uint::Hash256;

/// Trait providing access to fields specific to Credential objects in any ledger.
pub trait CredentialFields: LedgerObjectCommonFields {
    /// The account that this credential is for.
    fn subject(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Subject)
    }

    /// The account that issued this credential.
    fn issuer(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Issuer)
    }

    /// Arbitrary data defining the type of credential this entry represents. The minimum length is
    /// 1 byte and the maximum length is 64 bytes.
    fn credential_type(&self) -> Result<StandardBlob> {
        ledger_object::get_field(self.get_slot_num(), sfield::CredentialType)
    }

    /// The Expiration field (Optional).
    fn expiration(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::Expiration)
    }

    /// Arbitrary additional data about the credential, for example a URL where a W3C-formatted
    /// Verifiable Credential can be retrieved.
    fn uri(&self) -> Result<Option<UriBlob>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::URI)
    }

    /// A hint indicating which page of the issuer's directory links to this entry, in case the
    /// directory consists of multiple pages.
    fn issuer_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::IssuerNode)
    }

    /// A hint indicating which page of the subject's owner directory links to this entry, in case
    /// the directory consists of multiple pages.
    fn subject_node(&self) -> Result<Option<u64>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::SubjectNode)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn previous_txn_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn previous_txn_lgr_seq(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }
}

/// Trait providing access to fields specific to the current Credential object.
pub trait CurrentCredentialFields: CurrentLedgerObjectCommonFields {
    /// The account that this credential is for.
    fn subject(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Subject)
    }

    /// The account that issued this credential.
    fn issuer(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Issuer)
    }

    /// Arbitrary data defining the type of credential this entry represents. The minimum length is
    /// 1 byte and the maximum length is 64 bytes.
    fn credential_type(&self) -> Result<StandardBlob> {
        current_ledger_object::get_field(sfield::CredentialType)
    }

    /// The Expiration field (Optional).
    fn expiration(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::Expiration)
    }

    /// Arbitrary additional data about the credential, for example a URL where a W3C-formatted
    /// Verifiable Credential can be retrieved.
    fn uri(&self) -> Result<Option<UriBlob>> {
        current_ledger_object::get_field_optional(sfield::URI)
    }

    /// A hint indicating which page of the issuer's directory links to this entry, in case the
    /// directory consists of multiple pages.
    fn issuer_node(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::IssuerNode)
    }

    /// A hint indicating which page of the subject's owner directory links to this entry, in case
    /// the directory consists of multiple pages.
    fn subject_node(&self) -> Result<Option<u64>> {
        current_ledger_object::get_field_optional(sfield::SubjectNode)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn previous_txn_id(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn previous_txn_lgr_seq(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::PreviousTxnLgrSeq)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Credential {
    pub(crate) slot_num: i32,
}

impl Credential {
    /// Binds this handle to a host-managed slot holding a Credential ledger object.
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }
}

impl LedgerObjectCommonFields for Credential {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl CredentialFields for Credential {}

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

        let obj = Credential::new(0);

        assert!(obj.subject().is_ok());
        assert!(obj.issuer().is_ok());
        assert!(obj.credential_type().is_ok());
        assert!(obj.issuer_node().is_ok());
        assert!(obj.previous_txn_id().is_ok());
        assert!(obj.previous_txn_lgr_seq().is_ok());
        assert!(obj.expiration().is_ok());
        assert!(obj.uri().is_ok());
        assert!(obj.subject_node().is_ok());
    }

    #[test]
    fn optional_fields_none() {
        let mut mock = MockHostBindings::new();
        mock_all_fields_not_found(&mut mock);
        let _guard = setup_mock(mock);

        let obj = Credential::new(0);

        assert!(obj.expiration().unwrap().is_none());
        assert!(obj.subject_node().unwrap().is_none());
    }
}
