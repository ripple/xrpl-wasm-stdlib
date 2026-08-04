// GENERATED -- do not hand-edit. Run scripts/generate-ledger-objects.sh to regenerate.

/// Placeholder buffer size for fields whose XRPL wire type has no genuine Rust
/// mapping yet (VECTOR256, XCHAIN_BRIDGE, NUMBER, INT32, ...). Such getters return
/// raw, unparsed bytes; see the summary at the top of `generated/mod.rs`.
const RAW_UNMAPPED_FIELD_SIZE: usize = 512;

use crate::host::Result;
use crate::host::error_codes::match_result_code_optional;
use crate::host::get_current_ledger_obj_field;
use crate::host::get_ledger_obj_field;
use crate::objects::traits::CurrentLedgerObjectCommonFields;
use crate::objects::traits::LedgerObjectCommonFields;
use crate::objects::{current_ledger_object, ledger_object};
use crate::sfield;
use crate::types::account_id::AccountID;
use crate::types::blob::StandardBlob;
use crate::types::issue::Issue;
use crate::types::uint::{Hash192, Hash256};

/// Trait providing access to fields specific to Vault objects in any ledger.
pub trait VaultFields: LedgerObjectCommonFields {
    /// Identifies the transaction ID that most recently modified this object.
    fn previous_txn_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The sequence of the ledger that contains the transaction that most recently modified this object.
    fn previous_txn_lgr_seq(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }

    /// The transaction sequence number that created the vault.
    fn sequence(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::Sequence)
    }

    /// Identifies the page where this item is referenced in the owner's directory.
    fn owner_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::OwnerNode)
    }

    /// The account address of the Vault Owner.
    fn owner(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Owner)
    }

    /// The address of the vault's pseudo-account.
    fn account(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Account)
    }

    /// Arbitrary metadata, in hex format, about the vault. Limited to 256 bytes. See Data Field Format for more information.
    fn data(&self) -> Result<Option<StandardBlob>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::Data)
    }

    /// The asset of the vault. The vault supports XRP, trust line tokens, and MPTs.
    fn asset(&self) -> Result<Issue> {
        ledger_object::get_field(self.get_slot_num(), sfield::Asset)
    }

    /// The total value of the vault.
    /// Raw bytes; NUMBER is not yet typed in Rust.
    fn assets_total(&self) -> Result<Option<[u8; RAW_UNMAPPED_FIELD_SIZE]>> {
        let mut buffer = [0u8; RAW_UNMAPPED_FIELD_SIZE];
        let result_code = unsafe {
            get_ledger_obj_field(
                self.get_slot_num(),
                sfield::AssetsTotal.into(),
                buffer.as_mut_ptr(),
                buffer.len(),
            )
        };
        match_result_code_optional(result_code, || (result_code > 0).then_some(buffer))
    }

    /// The asset amount that is available in the vault.
    /// Raw bytes; NUMBER is not yet typed in Rust.
    fn assets_available(&self) -> Result<Option<[u8; RAW_UNMAPPED_FIELD_SIZE]>> {
        let mut buffer = [0u8; RAW_UNMAPPED_FIELD_SIZE];
        let result_code = unsafe {
            get_ledger_obj_field(
                self.get_slot_num(),
                sfield::AssetsAvailable.into(),
                buffer.as_mut_ptr(),
                buffer.len(),
            )
        };
        match_result_code_optional(result_code, || (result_code > 0).then_some(buffer))
    }

    /// The maximum asset amount that can be held in the vault. If set to 0, this indicates there is no cap.
    /// Raw bytes; NUMBER is not yet typed in Rust.
    fn assets_maximum(&self) -> Result<Option<[u8; RAW_UNMAPPED_FIELD_SIZE]>> {
        let mut buffer = [0u8; RAW_UNMAPPED_FIELD_SIZE];
        let result_code = unsafe {
            get_ledger_obj_field(
                self.get_slot_num(),
                sfield::AssetsMaximum.into(),
                buffer.as_mut_ptr(),
                buffer.len(),
            )
        };
        match_result_code_optional(result_code, || (result_code > 0).then_some(buffer))
    }

    /// The potential loss amount that is not yet realized, expressed as the vault's asset. Only a protocol connected to the vault can modify this attribute.
    /// Raw bytes; NUMBER is not yet typed in Rust.
    fn loss_unrealized(&self) -> Result<Option<[u8; RAW_UNMAPPED_FIELD_SIZE]>> {
        let mut buffer = [0u8; RAW_UNMAPPED_FIELD_SIZE];
        let result_code = unsafe {
            get_ledger_obj_field(
                self.get_slot_num(),
                sfield::LossUnrealized.into(),
                buffer.as_mut_ptr(),
                buffer.len(),
            )
        };
        match_result_code_optional(result_code, || (result_code > 0).then_some(buffer))
    }

    /// The identifier of the share `MPTokenIssuance` object.
    fn share_mpt_id(&self) -> Result<Hash192> {
        ledger_object::get_field(self.get_slot_num(), sfield::ShareMPTID)
    }

    /// Indicates the withdrawal strategy used by the vault.
    fn withdrawal_policy(&self) -> Result<u8> {
        ledger_object::get_field(self.get_slot_num(), sfield::WithdrawalPolicy)
    }

    /// Specifies decimal precision for share calculations. Assets are multiplied by 10<sup>Scale</sup > to convert fractional amounts into whole number shares. For example, with a `Scale` of `6`, depositing 20.3 units creates 20,300,000 shares (20.3 × 10<sup>Scale</sup >). For **trust line tokens** this can be configured at vault creation, and valid values are between 0-18, with the default being `6`. For **XRP** and **MPTs**, this is fixed at `0`. See Scaling Factor for more information.
    fn scale(&self) -> Result<Option<u8>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::Scale)
    }
}

/// Trait providing access to fields specific to the current Vault object.
pub trait CurrentVaultFields: CurrentLedgerObjectCommonFields {
    /// Identifies the transaction ID that most recently modified this object.
    fn previous_txn_id(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::PreviousTxnID)
    }

    /// The sequence of the ledger that contains the transaction that most recently modified this object.
    fn previous_txn_lgr_seq(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::PreviousTxnLgrSeq)
    }

    /// The transaction sequence number that created the vault.
    fn sequence(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::Sequence)
    }

    /// Identifies the page where this item is referenced in the owner's directory.
    fn owner_node(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::OwnerNode)
    }

    /// The account address of the Vault Owner.
    fn owner(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Owner)
    }

    /// The address of the vault's pseudo-account.
    fn account(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Account)
    }

    /// Arbitrary metadata, in hex format, about the vault. Limited to 256 bytes. See Data Field Format for more information.
    fn data(&self) -> Result<Option<StandardBlob>> {
        current_ledger_object::get_field_optional(sfield::Data)
    }

    /// The asset of the vault. The vault supports XRP, trust line tokens, and MPTs.
    fn asset(&self) -> Result<Issue> {
        current_ledger_object::get_field(sfield::Asset)
    }

    /// The total value of the vault.
    /// Raw bytes; NUMBER is not yet typed in Rust.
    fn assets_total(&self) -> Result<Option<[u8; RAW_UNMAPPED_FIELD_SIZE]>> {
        let mut buffer = [0u8; RAW_UNMAPPED_FIELD_SIZE];
        let result_code = unsafe {
            get_current_ledger_obj_field(
                sfield::AssetsTotal.into(),
                buffer.as_mut_ptr(),
                buffer.len(),
            )
        };
        match_result_code_optional(result_code, || (result_code > 0).then_some(buffer))
    }

    /// The asset amount that is available in the vault.
    /// Raw bytes; NUMBER is not yet typed in Rust.
    fn assets_available(&self) -> Result<Option<[u8; RAW_UNMAPPED_FIELD_SIZE]>> {
        let mut buffer = [0u8; RAW_UNMAPPED_FIELD_SIZE];
        let result_code = unsafe {
            get_current_ledger_obj_field(
                sfield::AssetsAvailable.into(),
                buffer.as_mut_ptr(),
                buffer.len(),
            )
        };
        match_result_code_optional(result_code, || (result_code > 0).then_some(buffer))
    }

    /// The maximum asset amount that can be held in the vault. If set to 0, this indicates there is no cap.
    /// Raw bytes; NUMBER is not yet typed in Rust.
    fn assets_maximum(&self) -> Result<Option<[u8; RAW_UNMAPPED_FIELD_SIZE]>> {
        let mut buffer = [0u8; RAW_UNMAPPED_FIELD_SIZE];
        let result_code = unsafe {
            get_current_ledger_obj_field(
                sfield::AssetsMaximum.into(),
                buffer.as_mut_ptr(),
                buffer.len(),
            )
        };
        match_result_code_optional(result_code, || (result_code > 0).then_some(buffer))
    }

    /// The potential loss amount that is not yet realized, expressed as the vault's asset. Only a protocol connected to the vault can modify this attribute.
    /// Raw bytes; NUMBER is not yet typed in Rust.
    fn loss_unrealized(&self) -> Result<Option<[u8; RAW_UNMAPPED_FIELD_SIZE]>> {
        let mut buffer = [0u8; RAW_UNMAPPED_FIELD_SIZE];
        let result_code = unsafe {
            get_current_ledger_obj_field(
                sfield::LossUnrealized.into(),
                buffer.as_mut_ptr(),
                buffer.len(),
            )
        };
        match_result_code_optional(result_code, || (result_code > 0).then_some(buffer))
    }

    /// The identifier of the share `MPTokenIssuance` object.
    fn share_mpt_id(&self) -> Result<Hash192> {
        current_ledger_object::get_field(sfield::ShareMPTID)
    }

    /// Indicates the withdrawal strategy used by the vault.
    fn withdrawal_policy(&self) -> Result<u8> {
        current_ledger_object::get_field(sfield::WithdrawalPolicy)
    }

    /// Specifies decimal precision for share calculations. Assets are multiplied by 10<sup>Scale</sup > to convert fractional amounts into whole number shares. For example, with a `Scale` of `6`, depositing 20.3 units creates 20,300,000 shares (20.3 × 10<sup>Scale</sup >). For **trust line tokens** this can be configured at vault creation, and valid values are between 0-18, with the default being `6`. For **XRP** and **MPTs**, this is fixed at `0`. See Scaling Factor for more information.
    fn scale(&self) -> Result<Option<u8>> {
        current_ledger_object::get_field_optional(sfield::Scale)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Vault {
    pub(crate) slot_num: i32,
}

impl Vault {
    /// Binds this handle to a host-managed slot holding a Vault ledger object.
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }
}

impl LedgerObjectCommonFields for Vault {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl VaultFields for Vault {}

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

        let obj = Vault::new(0);

        assert!(obj.previous_txn_id().is_ok());
        assert!(obj.previous_txn_lgr_seq().is_ok());
        assert!(obj.sequence().is_ok());
        assert!(obj.owner_node().is_ok());
        assert!(obj.owner().is_ok());
        assert!(obj.account().is_ok());
        assert!(obj.asset().is_ok());
        assert!(obj.share_mpt_id().is_ok());
        assert!(obj.withdrawal_policy().is_ok());
        assert!(obj.data().is_ok());
        assert!(obj.assets_total().is_ok());
        assert!(obj.assets_available().is_ok());
        assert!(obj.assets_maximum().is_ok());
        assert!(obj.loss_unrealized().is_ok());
        assert!(obj.scale().is_ok());
    }

    #[test]
    fn optional_fields_none() {
        let mut mock = MockHostBindings::new();
        mock_all_fields_not_found(&mut mock);
        let _guard = setup_mock(mock);

        let obj = Vault::new(0);

        assert!(obj.scale().unwrap().is_none());
    }
}
