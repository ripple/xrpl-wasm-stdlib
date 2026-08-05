// GENERATED -- do not hand-edit. Run scripts/generate-ledger-objects.sh to regenerate.

use crate::host::Result;
use crate::objects::traits::CurrentLedgerObjectCommonFields;
use crate::objects::traits::LedgerObjectCommonFields;
use crate::objects::{current_ledger_object, ledger_object};
use crate::sfield;
use crate::types::account_id::AccountID;
use crate::types::blob::{StandardBlob, UriBlob};
use crate::types::uint::Hash256;

/// Trait providing access to fields specific to Oracle objects in any ledger.
pub trait OracleFields: LedgerObjectCommonFields {
    /// The account with update and delete privileges for the oracle. It's recommended to set up
    /// multi-signing on this account.
    fn owner(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Owner)
    }

    /// The OracleDocumentID field (Optional).
    fn oracle_document_id(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::OracleDocumentID)
    }

    /// An arbitrary value that identifies an oracle provider, such as Chainlink, Band, or DIA. This
    /// field is a string, up to 256 ASCII hex encoded characters (`0x20`-`0x7E`).
    fn provider(&self) -> Result<StandardBlob> {
        ledger_object::get_field(self.get_slot_num(), sfield::Provider)
    }

    /// Arbitrary string to describe the type of asset, such as _currency_, _commodity_, or _index_.
    /// Must be formatted as hexadecimal representing ASCII characters (`0x20`-`0x7E`), maximum 16
    /// bytes.
    fn asset_class(&self) -> Result<StandardBlob> {
        ledger_object::get_field(self.get_slot_num(), sfield::AssetClass)
    }

    /// The time the data was last updated, represented in Unix time. (Note: Unlike many other time
    /// values on the XRP Ledger, this value does not use the Ripple Epoch.)
    fn last_update_time(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::LastUpdateTime)
    }

    /// An optional Universal Resource Identifier to reference price data off-chain. This field is
    /// limited to 256 bytes.
    fn uri(&self) -> Result<Option<UriBlob>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::URI)
    }

    /// A hint indicating which page of the oracle owner's owner directory links to this entry, in
    /// case the directory consists of multiple pages.
    fn owner_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::OwnerNode)
    }

    /// The hash of the previous transaction that modified this entry.
    fn previous_txn_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The ledger index that this object was most recently modified or created in.
    fn previous_txn_lgr_seq(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }
}

/// Trait providing access to fields specific to the current Oracle object.
pub trait CurrentOracleFields: CurrentLedgerObjectCommonFields {
    /// The account with update and delete privileges for the oracle. It's recommended to set up
    /// multi-signing on this account.
    fn owner(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Owner)
    }

    /// The OracleDocumentID field (Optional).
    fn oracle_document_id(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::OracleDocumentID)
    }

    /// An arbitrary value that identifies an oracle provider, such as Chainlink, Band, or DIA. This
    /// field is a string, up to 256 ASCII hex encoded characters (`0x20`-`0x7E`).
    fn provider(&self) -> Result<StandardBlob> {
        current_ledger_object::get_field(sfield::Provider)
    }

    /// Arbitrary string to describe the type of asset, such as _currency_, _commodity_, or _index_.
    /// Must be formatted as hexadecimal representing ASCII characters (`0x20`-`0x7E`), maximum 16
    /// bytes.
    fn asset_class(&self) -> Result<StandardBlob> {
        current_ledger_object::get_field(sfield::AssetClass)
    }

    /// The time the data was last updated, represented in Unix time. (Note: Unlike many other time
    /// values on the XRP Ledger, this value does not use the Ripple Epoch.)
    fn last_update_time(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::LastUpdateTime)
    }

    /// An optional Universal Resource Identifier to reference price data off-chain. This field is
    /// limited to 256 bytes.
    fn uri(&self) -> Result<Option<UriBlob>> {
        current_ledger_object::get_field_optional(sfield::URI)
    }

    /// A hint indicating which page of the oracle owner's owner directory links to this entry, in
    /// case the directory consists of multiple pages.
    fn owner_node(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::OwnerNode)
    }

    /// The hash of the previous transaction that modified this entry.
    fn previous_txn_id(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::PreviousTxnID)
    }

    /// The ledger index that this object was most recently modified or created in.
    fn previous_txn_lgr_seq(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::PreviousTxnLgrSeq)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Oracle {
    pub(crate) slot_num: i32,
}

impl Oracle {
    /// Binds this handle to a host-managed slot holding an Oracle ledger object.
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }
}

impl LedgerObjectCommonFields for Oracle {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl OracleFields for Oracle {}

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

        let obj = Oracle::new(0);

        assert!(obj.owner().is_ok());
        assert!(obj.provider().is_ok());
        assert!(obj.asset_class().is_ok());
        assert!(obj.last_update_time().is_ok());
        assert!(obj.owner_node().is_ok());
        assert!(obj.previous_txn_id().is_ok());
        assert!(obj.previous_txn_lgr_seq().is_ok());
        assert!(obj.oracle_document_id().is_ok());
        assert!(obj.uri().is_ok());
    }

    #[test]
    fn optional_fields_none() {
        let mut mock = MockHostBindings::new();
        mock_all_fields_not_found(&mut mock);
        let _guard = setup_mock(mock);

        let obj = Oracle::new(0);

        assert!(obj.oracle_document_id().unwrap().is_none());
    }
}
