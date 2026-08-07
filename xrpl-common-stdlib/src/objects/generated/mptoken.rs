// GENERATED -- do not hand-edit. Run scripts/generate-ledger-objects.sh to regenerate.

use crate::host::Result;
use crate::objects::traits::CurrentLedgerObjectCommonFields;
use crate::objects::traits::LedgerObjectCommonFields;
use crate::objects::{current_ledger_object, ledger_object};
use crate::sfield;
use crate::types::account_id::AccountID;
use crate::types::blob::StandardBlob;
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

    /// Encrypted inbox balance that receives incoming confidential transfers. Before it can be
    /// spent, the holder must merge it into their spending balance using the
    /// ConfidentialMPTMergeInbox transaction. Present when the holder has a confidential balance.
    fn confidential_balance_inbox(&self) -> Result<Option<StandardBlob>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::ConfidentialBalanceInbox)
    }

    /// Encrypted spending balance used to generate proofs for outgoing transactions. Present when
    /// the holder has a confidential balance.
    fn confidential_balance_spending(&self) -> Result<Option<StandardBlob>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::ConfidentialBalanceSpending)
    }

    /// Version number that increments each time the spending balance changes. This version is
    /// cryptographically bound to ZKPs in outgoing transactions to prevent replay attacks and
    /// ensure proof validity. If the version changes between proof generation and submission, the
    /// transaction will fail.
    fn confidential_balance_version(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::ConfidentialBalanceVersion)
    }

    /// Copy of the holder's total confidential balance encrypted for the issuer to audit supply.
    /// Present when the holder has a confidential balance.
    fn issuer_encrypted_balance(&self) -> Result<Option<StandardBlob>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::IssuerEncryptedBalance)
    }

    /// The holder's total confidential balance encrypted under the auditor's key for independent
    /// auditing. Only present if an auditor is configured.
    fn auditor_encrypted_balance(&self) -> Result<Option<StandardBlob>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::AuditorEncryptedBalance)
    }

    /// The holder's ElGamal public key for confidential balances. Present when the holder has a
    /// confidential balance.
    fn holder_encryption_key(&self) -> Result<Option<StandardBlob>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::HolderEncryptionKey)
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

    /// Encrypted inbox balance that receives incoming confidential transfers. Before it can be
    /// spent, the holder must merge it into their spending balance using the
    /// ConfidentialMPTMergeInbox transaction. Present when the holder has a confidential balance.
    fn confidential_balance_inbox(&self) -> Result<Option<StandardBlob>> {
        current_ledger_object::get_field_optional(sfield::ConfidentialBalanceInbox)
    }

    /// Encrypted spending balance used to generate proofs for outgoing transactions. Present when
    /// the holder has a confidential balance.
    fn confidential_balance_spending(&self) -> Result<Option<StandardBlob>> {
        current_ledger_object::get_field_optional(sfield::ConfidentialBalanceSpending)
    }

    /// Version number that increments each time the spending balance changes. This version is
    /// cryptographically bound to ZKPs in outgoing transactions to prevent replay attacks and
    /// ensure proof validity. If the version changes between proof generation and submission, the
    /// transaction will fail.
    fn confidential_balance_version(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::ConfidentialBalanceVersion)
    }

    /// Copy of the holder's total confidential balance encrypted for the issuer to audit supply.
    /// Present when the holder has a confidential balance.
    fn issuer_encrypted_balance(&self) -> Result<Option<StandardBlob>> {
        current_ledger_object::get_field_optional(sfield::IssuerEncryptedBalance)
    }

    /// The holder's total confidential balance encrypted under the auditor's key for independent
    /// auditing. Only present if an auditor is configured.
    fn auditor_encrypted_balance(&self) -> Result<Option<StandardBlob>> {
        current_ledger_object::get_field_optional(sfield::AuditorEncryptedBalance)
    }

    /// The holder's ElGamal public key for confidential balances. Present when the holder has a
    /// confidential balance.
    fn holder_encryption_key(&self) -> Result<Option<StandardBlob>> {
        current_ledger_object::get_field_optional(sfield::HolderEncryptionKey)
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
        assert!(obj.confidential_balance_inbox().is_ok());
        assert!(obj.confidential_balance_spending().is_ok());
        assert!(obj.confidential_balance_version().is_ok());
        assert!(obj.issuer_encrypted_balance().is_ok());
        assert!(obj.auditor_encrypted_balance().is_ok());
        assert!(obj.holder_encryption_key().is_ok());
    }

    #[test]
    fn optional_fields_none() {
        let mut mock = MockHostBindings::new();
        mock_all_fields_not_found(&mut mock);
        let _guard = setup_mock(mock);

        let obj = MPToken::new(0);

        assert!(obj.mpt_amount().unwrap().is_none());
        assert!(obj.locked_amount().unwrap().is_none());
        assert!(obj.confidential_balance_version().unwrap().is_none());
    }
}
