// GENERATED -- do not hand-edit. Run scripts/generate-ledger-objects.sh to regenerate.

use crate::host::Result;
use crate::objects::array_object::Array;
use crate::objects::traits::CurrentLedgerObjectCommonFields;
use crate::objects::traits::LedgerObjectCommonFields;
use crate::objects::{current_ledger_object, ledger_object};
use crate::sfield;
use crate::types::account_id::AccountID;
use crate::types::uint::Hash256;

/// Trait providing access to fields specific to SignerList objects in any ledger.
pub trait SignerListFields: LedgerObjectCommonFields {
    /// The Owner field (Optional).
    fn get_owner(&self) -> Result<Option<AccountID>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::Owner)
    }

    /// A hint indicating which page of the owner directory links to this object, in case the directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::OwnerNode)
    }

    /// A target number for signer weights. To produce a valid signature for the owner of this SignerList, the signers must provide valid signatures whose weights sum to this value or more.
    fn get_signer_quorum(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::SignerQuorum)
    }

    /// An array of Signer Entry objects representing the parties who are part of this signer list.
    fn get_signer_entries(&self) -> Result<Array> {
        ledger_object::get_field(self.get_slot_num(), sfield::SignerEntries)
    }

    /// An ID for this signer list. Currently always set to `0`. If a future amendment allows multiple signer lists for an account, this may change.
    fn get_signer_list_id(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::SignerListID)
    }

    /// The identifying hash of the transaction that most recently modified this object.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The [index of the ledger][Ledger Index] that contains the transaction that most recently modified this object.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }
}

/// Trait providing access to fields specific to the current SignerList object.
pub trait CurrentSignerListFields: CurrentLedgerObjectCommonFields {
    /// The Owner field (Optional).
    fn get_owner(&self) -> Result<Option<AccountID>> {
        current_ledger_object::get_field_optional(sfield::Owner)
    }

    /// A hint indicating which page of the owner directory links to this object, in case the directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::OwnerNode)
    }

    /// A target number for signer weights. To produce a valid signature for the owner of this SignerList, the signers must provide valid signatures whose weights sum to this value or more.
    fn get_signer_quorum(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::SignerQuorum)
    }

    /// An array of Signer Entry objects representing the parties who are part of this signer list.
    fn get_signer_entries(&self) -> Result<Array> {
        current_ledger_object::get_field(sfield::SignerEntries)
    }

    /// An ID for this signer list. Currently always set to `0`. If a future amendment allows multiple signer lists for an account, this may change.
    fn get_signer_list_id(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::SignerListID)
    }

    /// The identifying hash of the transaction that most recently modified this object.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::PreviousTxnID)
    }

    /// The [index of the ledger][Ledger Index] that contains the transaction that most recently modified this object.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::PreviousTxnLgrSeq)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct SignerList {
    pub(crate) slot_num: i32,
}

impl SignerList {
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }

    /// Loads the SignerList ledger object identified by the given keylet arguments,
    /// caching it in a host-managed slot.
    pub fn load(account: &AccountID) -> Result<Self> {
        let keylet = match crate::keylets::signers_keylet(account) {
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

impl LedgerObjectCommonFields for SignerList {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl SignerListFields for SignerList {}

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

        let obj = SignerList::new(0);

        assert!(obj.get_owner_node().is_ok());
        assert!(obj.get_signer_quorum().is_ok());
        assert!(obj.get_signer_list_id().is_ok());
        assert!(obj.get_previous_txn_id().is_ok());
        assert!(obj.get_previous_txn_lgr_seq().is_ok());
        assert!(obj.get_owner().is_ok());
    }

    #[test]
    fn optional_fields_none() {
        let mut mock = MockHostBindings::new();
        mock_all_fields_not_found(&mut mock);
        let _guard = setup_mock(mock);

        let obj = SignerList::new(0);

        assert!(obj.get_owner().unwrap().is_none());
    }

    #[test]
    fn load_success() {
        let mut mock = MockHostBindings::new();
        mock_signers_keylet_success(&mut mock);
        mock_cache_ledger_obj_success(&mut mock, 7);
        let _guard = setup_mock(mock);

        let result = SignerList::load(&sample::account_id());
        assert!(result.is_ok());
    }

    #[test]
    fn load_cache_error() {
        use crate::host::error_codes::INTERNAL_ERROR;

        let mut mock = MockHostBindings::new();
        mock_signers_keylet_success(&mut mock);
        mock_cache_ledger_obj_error(&mut mock, INTERNAL_ERROR);
        let _guard = setup_mock(mock);

        let result = SignerList::load(&sample::account_id());
        assert!(result.is_err());
    }
}
