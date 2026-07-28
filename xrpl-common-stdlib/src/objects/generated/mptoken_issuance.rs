// GENERATED -- do not hand-edit. Run scripts/generate-ledger-objects.sh to regenerate.

use crate::host::Result;
use crate::objects::traits::CurrentLedgerObjectCommonFields;
use crate::objects::traits::LedgerObjectCommonFields;
use crate::objects::{current_ledger_object, ledger_object};
use crate::sfield;
use crate::types::account_id::AccountID;
use crate::types::blob::StandardBlob;
use crate::types::uint::Hash256;

/// Trait providing access to fields specific to MPTokenIssuance objects in any ledger.
pub trait MPTokenIssuanceFields: LedgerObjectCommonFields {
    /// The address of the account that controls both the issuance amounts and characteristics of a particular fungible token.
    fn get_issuer(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Issuer)
    }

    /// The `Sequence` (or `Ticket`) number of the transaction that created this issuance. This helps to uniquely identify the issuance and distinguish it from any other later MPT issuances created by this account.
    fn get_sequence(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::Sequence)
    }

    /// This value specifies the fee, in tenths of a basis point, charged by the issuer for secondary sales of the token, if such sales are allowed at all. Valid values for this field are between 0 and 50,000 inclusive. A value of 1 is equivalent to 1/10 of a basis point or 0.001%, allowing transfer rates between 0% and 50%. A `TransferFee` of 50,000 corresponds to 50%. The default value for this field is 0. Any decimals in the transfer fee are rounded down. The fee can be rounded down to zero if the payment is small. Issuers should make sure that their MPT's `AssetScale` is large enough.
    fn get_transfer_fee(&self) -> Result<Option<u16>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::TransferFee)
    }

    /// A hint indicating which page of the owner directory links to this entry, in case the directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::OwnerNode)
    }

    /// Where to put the decimal place when displaying amounts of this MPT. More formally, the asset scale is a non-negative integer (0, 1, 2, …) such that one standard unit equals 10^(-scale) of a corresponding fractional unit. For example, if a US Dollar Stablecoin has an asset scale of _2_, then 1 unit of that MPT would equal 0.01 US Dollars. This indicates to how many decimal places the MPT can be subdivided. The default is `0`, meaning that the MPT cannot be divided into smaller than 1 unit.
    fn get_asset_scale(&self) -> Result<Option<u8>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::AssetScale)
    }

    /// The maximum number of MPTs that can exist at one time. If omitted, the maximum is currently limited to 2<sup>63</sup>-1.
    fn get_maximum_amount(&self) -> Result<Option<u64>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::MaximumAmount)
    }

    /// The total amount of MPTs of this issuance currently in circulation. This value increases when the issuer sends MPTs to a non-issuer, and decreases whenever the issuer receives MPTs.
    fn get_outstanding_amount(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::OutstandingAmount)
    }

    /// The amount of tokens currently locked up (for example, in escrow). This amount is already included in the `OutstandingAmount`.
    fn get_locked_amount(&self) -> Result<Option<u64>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::LockedAmount)
    }

    /// Arbitrary metadata about this issuance, in hex format. The limit for this field is 1024 bytes.
    fn get_mptoken_metadata(&self) -> Result<Option<StandardBlob>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::MPTokenMetadata)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The [index of the ledger][Ledger Index] that contains the transaction that most recently modified this object.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }

    /// The DomainID field (Optional).
    fn get_domain_id(&self) -> Result<Option<Hash256>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::DomainID)
    }

    /// The MutableFlags field (Optional).
    fn get_mutable_flags(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::MutableFlags)
    }
}

/// Trait providing access to fields specific to the current MPTokenIssuance object.
pub trait CurrentMPTokenIssuanceFields: CurrentLedgerObjectCommonFields {
    /// The address of the account that controls both the issuance amounts and characteristics of a particular fungible token.
    fn get_issuer(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Issuer)
    }

    /// The `Sequence` (or `Ticket`) number of the transaction that created this issuance. This helps to uniquely identify the issuance and distinguish it from any other later MPT issuances created by this account.
    fn get_sequence(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::Sequence)
    }

    /// This value specifies the fee, in tenths of a basis point, charged by the issuer for secondary sales of the token, if such sales are allowed at all. Valid values for this field are between 0 and 50,000 inclusive. A value of 1 is equivalent to 1/10 of a basis point or 0.001%, allowing transfer rates between 0% and 50%. A `TransferFee` of 50,000 corresponds to 50%. The default value for this field is 0. Any decimals in the transfer fee are rounded down. The fee can be rounded down to zero if the payment is small. Issuers should make sure that their MPT's `AssetScale` is large enough.
    fn get_transfer_fee(&self) -> Result<Option<u16>> {
        current_ledger_object::get_field_optional(sfield::TransferFee)
    }

    /// A hint indicating which page of the owner directory links to this entry, in case the directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::OwnerNode)
    }

    /// Where to put the decimal place when displaying amounts of this MPT. More formally, the asset scale is a non-negative integer (0, 1, 2, …) such that one standard unit equals 10^(-scale) of a corresponding fractional unit. For example, if a US Dollar Stablecoin has an asset scale of _2_, then 1 unit of that MPT would equal 0.01 US Dollars. This indicates to how many decimal places the MPT can be subdivided. The default is `0`, meaning that the MPT cannot be divided into smaller than 1 unit.
    fn get_asset_scale(&self) -> Result<Option<u8>> {
        current_ledger_object::get_field_optional(sfield::AssetScale)
    }

    /// The maximum number of MPTs that can exist at one time. If omitted, the maximum is currently limited to 2<sup>63</sup>-1.
    fn get_maximum_amount(&self) -> Result<Option<u64>> {
        current_ledger_object::get_field_optional(sfield::MaximumAmount)
    }

    /// The total amount of MPTs of this issuance currently in circulation. This value increases when the issuer sends MPTs to a non-issuer, and decreases whenever the issuer receives MPTs.
    fn get_outstanding_amount(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::OutstandingAmount)
    }

    /// The amount of tokens currently locked up (for example, in escrow). This amount is already included in the `OutstandingAmount`.
    fn get_locked_amount(&self) -> Result<Option<u64>> {
        current_ledger_object::get_field_optional(sfield::LockedAmount)
    }

    /// Arbitrary metadata about this issuance, in hex format. The limit for this field is 1024 bytes.
    fn get_mptoken_metadata(&self) -> Result<Option<StandardBlob>> {
        current_ledger_object::get_field_optional(sfield::MPTokenMetadata)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::PreviousTxnID)
    }

    /// The [index of the ledger][Ledger Index] that contains the transaction that most recently modified this object.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::PreviousTxnLgrSeq)
    }

    /// The DomainID field (Optional).
    fn get_domain_id(&self) -> Result<Option<Hash256>> {
        current_ledger_object::get_field_optional(sfield::DomainID)
    }

    /// The MutableFlags field (Optional).
    fn get_mutable_flags(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::MutableFlags)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct MPTokenIssuance {
    pub(crate) slot_num: i32,
}

impl MPTokenIssuance {
    /// Binds this handle to a host-managed slot holding a MPTokenIssuance ledger object.
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }
}

impl LedgerObjectCommonFields for MPTokenIssuance {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl MPTokenIssuanceFields for MPTokenIssuance {}

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

        let obj = MPTokenIssuance::new(0);

        assert!(obj.get_issuer().is_ok());
        assert!(obj.get_sequence().is_ok());
        assert!(obj.get_owner_node().is_ok());
        assert!(obj.get_outstanding_amount().is_ok());
        assert!(obj.get_previous_txn_id().is_ok());
        assert!(obj.get_previous_txn_lgr_seq().is_ok());
        assert!(obj.get_transfer_fee().is_ok());
        assert!(obj.get_asset_scale().is_ok());
        assert!(obj.get_maximum_amount().is_ok());
        assert!(obj.get_locked_amount().is_ok());
        assert!(obj.get_mptoken_metadata().is_ok());
        assert!(obj.get_domain_id().is_ok());
        assert!(obj.get_mutable_flags().is_ok());
    }

    #[test]
    fn optional_fields_none() {
        let mut mock = MockHostBindings::new();
        mock_all_fields_not_found(&mut mock);
        let _guard = setup_mock(mock);

        let obj = MPTokenIssuance::new(0);

        assert!(obj.get_transfer_fee().unwrap().is_none());
        assert!(obj.get_asset_scale().unwrap().is_none());
        assert!(obj.get_maximum_amount().unwrap().is_none());
        assert!(obj.get_locked_amount().unwrap().is_none());
        assert!(obj.get_domain_id().unwrap().is_none());
        assert!(obj.get_mutable_flags().unwrap().is_none());
    }
}
