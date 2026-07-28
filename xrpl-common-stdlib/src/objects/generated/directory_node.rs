// GENERATED -- do not hand-edit. Run scripts/generate-ledger-objects.sh to regenerate.

/// Placeholder buffer size for fields whose XRPL wire type has no genuine Rust
/// mapping yet (VECTOR256, XCHAIN_BRIDGE, NUMBER, INT32, ...). Such getters return
/// raw, unparsed bytes; see the summary at the top of `generated/mod.rs`.
const RAW_UNMAPPED_FIELD_SIZE: usize = 512;

use crate::host::Result;
use crate::host::error_codes::match_result_code;
use crate::host::get_current_ledger_obj_field;
use crate::host::get_ledger_obj_field;
use crate::objects::traits::CurrentLedgerObjectCommonFields;
use crate::objects::traits::LedgerObjectCommonFields;
use crate::objects::{current_ledger_object, ledger_object};
use crate::sfield;
use crate::types::account_id::AccountID;
use crate::types::uint::{Hash160, Hash256};

/// Trait providing access to fields specific to DirectoryNode objects in any ledger.
pub trait DirectoryNodeFields: LedgerObjectCommonFields {
    /// (Owner directories only) The address of the account that owns the objects in this directory.
    fn get_owner(&self) -> Result<Option<AccountID>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::Owner)
    }

    /// (Offer directories only) The currency code of the `TakerPays` amount from the offers in this directory.
    fn get_taker_pays_currency(&self) -> Result<Option<Hash160>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::TakerPaysCurrency)
    }

    /// (Offer directories only) The issuer of the `TakerPays` amount from the offers in this directory.
    fn get_taker_pays_issuer(&self) -> Result<Option<Hash160>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::TakerPaysIssuer)
    }

    /// (Offer directories only) The currency code of the `TakerGets` amount from the offers in this directory.
    fn get_taker_gets_currency(&self) -> Result<Option<Hash160>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::TakerGetsCurrency)
    }

    /// (Offer directories only) The issuer of the `TakerGets` amount from the offers in this directory.
    fn get_taker_gets_issuer(&self) -> Result<Option<Hash160>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::TakerGetsIssuer)
    }

    /// (Offer directories only) **DEPRECATED**. Do not use.
    fn get_exchange_rate(&self) -> Result<Option<u64>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::ExchangeRate)
    }

    /// The contents of this directory: an array of IDs of other objects.
    /// Raw bytes; VECTOR256 is not yet typed in Rust.
    fn get_indexes(&self) -> Result<[u8; RAW_UNMAPPED_FIELD_SIZE]> {
        let mut buffer = [0u8; RAW_UNMAPPED_FIELD_SIZE];
        let result_code = unsafe {
            get_ledger_obj_field(
                self.get_slot_num(),
                sfield::Indexes.into(),
                buffer.as_mut_ptr(),
                buffer.len(),
            )
        };
        match_result_code(result_code, || buffer)
    }

    /// The ID of root object for this directory.
    fn get_root_index(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::RootIndex)
    }

    /// If this directory consists of multiple pages, this ID links to the next object in the chain, wrapping around at the end.
    fn get_index_next(&self) -> Result<Option<u64>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::IndexNext)
    }

    /// If this directory consists of multiple pages, this ID links to the previous object in the chain, wrapping around at the beginning.
    fn get_index_previous(&self) -> Result<Option<u64>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::IndexPrevious)
    }

    /// (NFT offer directories only) ID of the NFT in a buy or sell offer.
    fn get_nftoken_id(&self) -> Result<Option<Hash256>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::NFTokenID)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Option<Hash256>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The [index of the ledger][Ledger Index] that contains the transaction that most recently modified this entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }

    /// (Offer directories only) The ledger entry ID of a permissioned domain. If present, this order book belongs to the corresponding Permissioned DEX. Otherwise, this order book is part of the open DEX.
    fn get_domain_id(&self) -> Result<Option<Hash256>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::DomainID)
    }
}

/// Trait providing access to fields specific to the current DirectoryNode object.
pub trait CurrentDirectoryNodeFields: CurrentLedgerObjectCommonFields {
    /// (Owner directories only) The address of the account that owns the objects in this directory.
    fn get_owner(&self) -> Result<Option<AccountID>> {
        current_ledger_object::get_field_optional(sfield::Owner)
    }

    /// (Offer directories only) The currency code of the `TakerPays` amount from the offers in this directory.
    fn get_taker_pays_currency(&self) -> Result<Option<Hash160>> {
        current_ledger_object::get_field_optional(sfield::TakerPaysCurrency)
    }

    /// (Offer directories only) The issuer of the `TakerPays` amount from the offers in this directory.
    fn get_taker_pays_issuer(&self) -> Result<Option<Hash160>> {
        current_ledger_object::get_field_optional(sfield::TakerPaysIssuer)
    }

    /// (Offer directories only) The currency code of the `TakerGets` amount from the offers in this directory.
    fn get_taker_gets_currency(&self) -> Result<Option<Hash160>> {
        current_ledger_object::get_field_optional(sfield::TakerGetsCurrency)
    }

    /// (Offer directories only) The issuer of the `TakerGets` amount from the offers in this directory.
    fn get_taker_gets_issuer(&self) -> Result<Option<Hash160>> {
        current_ledger_object::get_field_optional(sfield::TakerGetsIssuer)
    }

    /// (Offer directories only) **DEPRECATED**. Do not use.
    fn get_exchange_rate(&self) -> Result<Option<u64>> {
        current_ledger_object::get_field_optional(sfield::ExchangeRate)
    }

    /// The contents of this directory: an array of IDs of other objects.
    /// Raw bytes; VECTOR256 is not yet typed in Rust.
    fn get_indexes(&self) -> Result<[u8; RAW_UNMAPPED_FIELD_SIZE]> {
        let mut buffer = [0u8; RAW_UNMAPPED_FIELD_SIZE];
        let result_code = unsafe {
            get_current_ledger_obj_field(sfield::Indexes.into(), buffer.as_mut_ptr(), buffer.len())
        };
        match_result_code(result_code, || buffer)
    }

    /// The ID of root object for this directory.
    fn get_root_index(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::RootIndex)
    }

    /// If this directory consists of multiple pages, this ID links to the next object in the chain, wrapping around at the end.
    fn get_index_next(&self) -> Result<Option<u64>> {
        current_ledger_object::get_field_optional(sfield::IndexNext)
    }

    /// If this directory consists of multiple pages, this ID links to the previous object in the chain, wrapping around at the beginning.
    fn get_index_previous(&self) -> Result<Option<u64>> {
        current_ledger_object::get_field_optional(sfield::IndexPrevious)
    }

    /// (NFT offer directories only) ID of the NFT in a buy or sell offer.
    fn get_nftoken_id(&self) -> Result<Option<Hash256>> {
        current_ledger_object::get_field_optional(sfield::NFTokenID)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Option<Hash256>> {
        current_ledger_object::get_field_optional(sfield::PreviousTxnID)
    }

    /// The [index of the ledger][Ledger Index] that contains the transaction that most recently modified this entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::PreviousTxnLgrSeq)
    }

    /// (Offer directories only) The ledger entry ID of a permissioned domain. If present, this order book belongs to the corresponding Permissioned DEX. Otherwise, this order book is part of the open DEX.
    fn get_domain_id(&self) -> Result<Option<Hash256>> {
        current_ledger_object::get_field_optional(sfield::DomainID)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct DirectoryNode {
    pub(crate) slot_num: i32,
}

impl DirectoryNode {
    /// Binds this handle to a host-managed slot holding a DirectoryNode ledger object.
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }
}

impl LedgerObjectCommonFields for DirectoryNode {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl DirectoryNodeFields for DirectoryNode {}

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

        let obj = DirectoryNode::new(0);

        assert!(obj.get_indexes().is_ok());
        assert!(obj.get_root_index().is_ok());
        assert!(obj.get_owner().is_ok());
        assert!(obj.get_taker_pays_currency().is_ok());
        assert!(obj.get_taker_pays_issuer().is_ok());
        assert!(obj.get_taker_gets_currency().is_ok());
        assert!(obj.get_taker_gets_issuer().is_ok());
        assert!(obj.get_exchange_rate().is_ok());
        assert!(obj.get_index_next().is_ok());
        assert!(obj.get_index_previous().is_ok());
        assert!(obj.get_nftoken_id().is_ok());
        assert!(obj.get_previous_txn_id().is_ok());
        assert!(obj.get_previous_txn_lgr_seq().is_ok());
        assert!(obj.get_domain_id().is_ok());
    }

    #[test]
    fn optional_fields_none() {
        let mut mock = MockHostBindings::new();
        mock_all_fields_not_found(&mut mock);
        let _guard = setup_mock(mock);

        let obj = DirectoryNode::new(0);

        assert!(obj.get_owner().unwrap().is_none());
        assert!(obj.get_taker_pays_currency().unwrap().is_none());
        assert!(obj.get_taker_pays_issuer().unwrap().is_none());
        assert!(obj.get_taker_gets_currency().unwrap().is_none());
        assert!(obj.get_taker_gets_issuer().unwrap().is_none());
        assert!(obj.get_exchange_rate().unwrap().is_none());
        assert!(obj.get_index_next().unwrap().is_none());
        assert!(obj.get_index_previous().unwrap().is_none());
        assert!(obj.get_nftoken_id().unwrap().is_none());
        assert!(obj.get_previous_txn_id().unwrap().is_none());
        assert!(obj.get_previous_txn_lgr_seq().unwrap().is_none());
        assert!(obj.get_domain_id().unwrap().is_none());
    }
}
