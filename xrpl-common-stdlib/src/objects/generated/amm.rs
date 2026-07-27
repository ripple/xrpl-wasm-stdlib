// GENERATED -- do not hand-edit. Run scripts/generate-ledger-objects.sh to regenerate.

use crate::host::Result;
use crate::objects::array_object::{Array, Object};
use crate::objects::traits::CurrentLedgerObjectCommonFields;
use crate::objects::traits::LedgerObjectCommonFields;
use crate::objects::{current_ledger_object, ledger_object};
use crate::sfield;
use crate::types::account_id::AccountID;
use crate::types::amount::Amount;
use crate::types::issue::Issue;
use crate::types::uint::Hash256;

/// Trait providing access to fields specific to AMM objects in any ledger.
pub trait AMMFields: LedgerObjectCommonFields {
    /// The address of the special account that holds this AMM's assets.
    fn get_account(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Account)
    }

    /// The percentage fee to be charged for trades against this AMM instance, in units of 1/100,000. The maximum value is 1000, for a 1% fee.
    fn get_trading_fee(&self) -> Result<Option<u16>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::TradingFee)
    }

    /// A list of vote objects, representing votes on the pool's trading fee.
    fn get_vote_slots(&self) -> Result<Option<Array>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::VoteSlots)
    }

    /// Details of the current owner of the auction slot, as an Auction Slot object.
    fn get_auction_slot(&self) -> Result<Option<Object>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::AuctionSlot)
    }

    /// The total outstanding balance of liquidity provider tokens from this AMM instance. The holders of these tokens can vote on the AMM's trading fee in proportion to their holdings, or redeem the tokens for a share of the AMM's assets which grows with the trading fees collected.
    fn get_lp_token_balance(&self) -> Result<Amount> {
        ledger_object::get_field(self.get_slot_num(), sfield::LPTokenBalance)
    }

    /// The definition for one of the two assets this AMM holds. In JSON, this is an object with `currency` and `issuer` fields.
    fn get_asset(&self) -> Result<Issue> {
        ledger_object::get_field(self.get_slot_num(), sfield::Asset)
    }

    /// The definition for the other asset this AMM holds. In JSON, this is an object with `currency` and `issuer` fields.
    fn get_asset2(&self) -> Result<Issue> {
        ledger_object::get_field(self.get_slot_num(), sfield::Asset2)
    }

    /// The OwnerNode field (Required).
    fn get_owner_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::OwnerNode)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Option<Hash256>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The [index of the ledger][Ledger Index] that contains the transaction that most recently modified this entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }
}

/// Trait providing access to fields specific to the current AMM object.
pub trait CurrentAMMFields: CurrentLedgerObjectCommonFields {
    /// The address of the special account that holds this AMM's assets.
    fn get_account(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Account)
    }

    /// The percentage fee to be charged for trades against this AMM instance, in units of 1/100,000. The maximum value is 1000, for a 1% fee.
    fn get_trading_fee(&self) -> Result<Option<u16>> {
        current_ledger_object::get_field_optional(sfield::TradingFee)
    }

    /// A list of vote objects, representing votes on the pool's trading fee.
    fn get_vote_slots(&self) -> Result<Option<Array>> {
        current_ledger_object::get_field_optional(sfield::VoteSlots)
    }

    /// Details of the current owner of the auction slot, as an Auction Slot object.
    fn get_auction_slot(&self) -> Result<Option<Object>> {
        current_ledger_object::get_field_optional(sfield::AuctionSlot)
    }

    /// The total outstanding balance of liquidity provider tokens from this AMM instance. The holders of these tokens can vote on the AMM's trading fee in proportion to their holdings, or redeem the tokens for a share of the AMM's assets which grows with the trading fees collected.
    fn get_lp_token_balance(&self) -> Result<Amount> {
        current_ledger_object::get_field(sfield::LPTokenBalance)
    }

    /// The definition for one of the two assets this AMM holds. In JSON, this is an object with `currency` and `issuer` fields.
    fn get_asset(&self) -> Result<Issue> {
        current_ledger_object::get_field(sfield::Asset)
    }

    /// The definition for the other asset this AMM holds. In JSON, this is an object with `currency` and `issuer` fields.
    fn get_asset2(&self) -> Result<Issue> {
        current_ledger_object::get_field(sfield::Asset2)
    }

    /// The OwnerNode field (Required).
    fn get_owner_node(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::OwnerNode)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Option<Hash256>> {
        current_ledger_object::get_field_optional(sfield::PreviousTxnID)
    }

    /// The [index of the ledger][Ledger Index] that contains the transaction that most recently modified this entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::PreviousTxnLgrSeq)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct AMM {
    pub(crate) slot_num: i32,
}

impl AMM {
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }

    /// Loads the AMM ledger object identified by the given keylet arguments,
    /// caching it in a host-managed slot.
    pub fn load(issue1: &Issue, issue2: &Issue) -> Result<Self> {
        let keylet = match crate::keylets::amm_keylet(issue1, issue2) {
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

impl LedgerObjectCommonFields for AMM {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl AMMFields for AMM {}

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

        let obj = AMM::new(0);

        assert!(obj.get_account().is_ok());
        assert!(obj.get_lp_token_balance().is_ok());
        assert!(obj.get_asset().is_ok());
        assert!(obj.get_asset2().is_ok());
        assert!(obj.get_owner_node().is_ok());
        assert!(obj.get_trading_fee().is_ok());
        assert!(obj.get_previous_txn_id().is_ok());
        assert!(obj.get_previous_txn_lgr_seq().is_ok());
    }

    #[test]
    fn optional_fields_none() {
        let mut mock = MockHostBindings::new();
        mock_all_fields_not_found(&mut mock);
        let _guard = setup_mock(mock);

        let obj = AMM::new(0);

        assert!(obj.get_trading_fee().unwrap().is_none());
        assert!(obj.get_previous_txn_id().unwrap().is_none());
        assert!(obj.get_previous_txn_lgr_seq().unwrap().is_none());
    }

    #[test]
    fn load_success() {
        let mut mock = MockHostBindings::new();
        mock_amm_keylet_success(&mut mock);
        mock_cache_ledger_obj_success(&mut mock, 7);
        let _guard = setup_mock(mock);

        let result = AMM::load(&sample::issue(), &sample::issue());
        assert!(result.is_ok());
    }

    #[test]
    fn load_cache_error() {
        use crate::host::error_codes::INTERNAL_ERROR;

        let mut mock = MockHostBindings::new();
        mock_amm_keylet_success(&mut mock);
        mock_cache_ledger_obj_error(&mut mock, INTERNAL_ERROR);
        let _guard = setup_mock(mock);

        let result = AMM::load(&sample::issue(), &sample::issue());
        assert!(result.is_err());
    }
}
