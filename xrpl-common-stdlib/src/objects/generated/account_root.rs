// GENERATED -- do not hand-edit. Run scripts/generate-ledger-objects.sh to regenerate.

use crate::host::Result;
use crate::objects::traits::CurrentLedgerObjectCommonFields;
use crate::objects::traits::LedgerObjectCommonFields;
use crate::objects::{current_ledger_object, ledger_object};
use crate::sfield;
use crate::types::account_id::AccountID;
use crate::types::amount::Amount;
use crate::types::blob::{PublicKeyBlob, UriBlob};
use crate::types::uint::{Hash128, Hash256};

/// Trait providing access to fields specific to AccountRoot objects in any ledger.
pub trait AccountRootFields: LedgerObjectCommonFields {
    /// The identifying (classic) address of this account.
    fn get_account(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Account)
    }

    /// The sequence number of the next valid transaction for this account.
    fn get_sequence(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::Sequence)
    }

    /// The account's current [XRP balance in drops][XRP, in drops], represented as a string.
    fn get_balance(&self) -> Result<Amount> {
        ledger_object::get_field(self.get_slot_num(), sfield::Balance)
    }

    /// The number of objects this account owns in the ledger, which contributes to its owner reserve.
    fn get_owner_count(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::OwnerCount)
    }

    /// The identifying hash of the transaction that most recently modified this object.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The [index of the ledger][Ledger Index] that contains the transaction that most recently modified this object.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }

    /// The identifying hash of the transaction most recently sent by this account. This field must be enabled to use the `AccountTxnID` transaction field. To enable it, send an AccountSet transaction with the `asfAccountTxnID` flag enabled.
    fn get_account_txn_id(&self) -> Result<Option<Hash256>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::AccountTxnID)
    }

    /// The address of a key pair that can be used to sign transactions for this account instead of the master key. Use a [SetRegularKey transaction][] to change this value.
    fn get_regular_key(&self) -> Result<Option<AccountID>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::RegularKey)
    }

    /// The md5 hash of an email address. Clients can use this to look up an avatar through services such as Gravatar.
    fn get_email_hash(&self) -> Result<Option<Hash128>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::EmailHash)
    }

    /// An arbitrary 256-bit value that users can set.
    fn get_wallet_locator(&self) -> Result<Option<Hash256>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::WalletLocator)
    }

    /// Unused. (The code supports this field but there is no way to set it.)
    fn get_wallet_size(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::WalletSize)
    }

    /// A public key that may be used to send encrypted messages to this account. In JSON, uses hexadecimal. Must be exactly 33 bytes, with the first byte indicating the key type: `0x02` or `0x03` for secp256k1 keys, `0xED` for Ed25519 keys.
    fn get_message_key(&self) -> Result<Option<PublicKeyBlob>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::MessageKey)
    }

    /// A transfer fee to charge other users for sending currency issued by this account to each other.
    fn get_transfer_rate(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::TransferRate)
    }

    /// A domain associated with this account. In JSON, this is the hexadecimal for the ASCII representation of the domain. Cannot be more than 256 bytes in length.
    fn get_domain(&self) -> Result<Option<UriBlob>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::Domain)
    }

    /// How many significant digits to use for exchange rates of Offers involving currencies issued by this address. Valid values are `3` to `15`, inclusive.
    fn get_tick_size(&self) -> Result<Option<u8>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::TickSize)
    }

    /// How many Tickets this account owns in the ledger. This is updated automatically to ensure that the account stays within the hard limit of 250 Tickets at a time. This field is omitted if the account has zero Tickets.
    fn get_ticket_count(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::TicketCount)
    }

    /// Another account that can mint non-fungible tokens on behalf of this account.
    fn get_nftoken_minter(&self) -> Result<Option<AccountID>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::NFTokenMinter)
    }

    /// How many total non-fungible tokens have been minted by and on behalf of this account.
    fn get_minted_nftokens(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::MintedNFTokens)
    }

    /// How many total of this account's issued non-fungible tokens have been burned. This number is always equal or less than `MintedNFTokens`.
    fn get_burned_nftokens(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::BurnedNFTokens)
    }

    /// The account's [Sequence Number][] at the time it minted its first non-fungible-token.
    fn get_first_nftoken_sequence(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::FirstNFTokenSequence)
    }

    /// The ledger entry ID of the corresponding AMM ledger entry. Set during account creation; cannot be modified. If present, indicates that this is a special AMM pseudo-account AccountRoot; always omitted on non-AMM accounts.
    fn get_amm_id(&self) -> Result<Option<Hash256>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::AMMID)
    }

    /// The ID of the `Vault` entry associated with this account. Set during account creation; cannot be modified. If present, indicates that this is a special Vault pseudo-account AccountRoot; always omitted on non-Vault accounts.
    fn get_vault_id(&self) -> Result<Option<Hash256>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::VaultID)
    }

    /// The LoanBrokerID field (Optional).
    fn get_loan_broker_id(&self) -> Result<Option<Hash256>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::LoanBrokerID)
    }

    /// The ContractID field (Optional).
    fn get_contract_id(&self) -> Result<Option<Hash256>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::ContractID)
    }
}

/// Trait providing access to fields specific to the current AccountRoot object.
pub trait CurrentAccountRootFields: CurrentLedgerObjectCommonFields {
    /// The identifying (classic) address of this account.
    fn get_account(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Account)
    }

    /// The sequence number of the next valid transaction for this account.
    fn get_sequence(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::Sequence)
    }

    /// The account's current [XRP balance in drops][XRP, in drops], represented as a string.
    fn get_balance(&self) -> Result<Amount> {
        current_ledger_object::get_field(sfield::Balance)
    }

    /// The number of objects this account owns in the ledger, which contributes to its owner reserve.
    fn get_owner_count(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::OwnerCount)
    }

    /// The identifying hash of the transaction that most recently modified this object.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::PreviousTxnID)
    }

    /// The [index of the ledger][Ledger Index] that contains the transaction that most recently modified this object.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::PreviousTxnLgrSeq)
    }

    /// The identifying hash of the transaction most recently sent by this account. This field must be enabled to use the `AccountTxnID` transaction field. To enable it, send an AccountSet transaction with the `asfAccountTxnID` flag enabled.
    fn get_account_txn_id(&self) -> Result<Option<Hash256>> {
        current_ledger_object::get_field_optional(sfield::AccountTxnID)
    }

    /// The address of a key pair that can be used to sign transactions for this account instead of the master key. Use a [SetRegularKey transaction][] to change this value.
    fn get_regular_key(&self) -> Result<Option<AccountID>> {
        current_ledger_object::get_field_optional(sfield::RegularKey)
    }

    /// The md5 hash of an email address. Clients can use this to look up an avatar through services such as Gravatar.
    fn get_email_hash(&self) -> Result<Option<Hash128>> {
        current_ledger_object::get_field_optional(sfield::EmailHash)
    }

    /// An arbitrary 256-bit value that users can set.
    fn get_wallet_locator(&self) -> Result<Option<Hash256>> {
        current_ledger_object::get_field_optional(sfield::WalletLocator)
    }

    /// Unused. (The code supports this field but there is no way to set it.)
    fn get_wallet_size(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::WalletSize)
    }

    /// A public key that may be used to send encrypted messages to this account. In JSON, uses hexadecimal. Must be exactly 33 bytes, with the first byte indicating the key type: `0x02` or `0x03` for secp256k1 keys, `0xED` for Ed25519 keys.
    fn get_message_key(&self) -> Result<Option<PublicKeyBlob>> {
        current_ledger_object::get_field_optional(sfield::MessageKey)
    }

    /// A transfer fee to charge other users for sending currency issued by this account to each other.
    fn get_transfer_rate(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::TransferRate)
    }

    /// A domain associated with this account. In JSON, this is the hexadecimal for the ASCII representation of the domain. Cannot be more than 256 bytes in length.
    fn get_domain(&self) -> Result<Option<UriBlob>> {
        current_ledger_object::get_field_optional(sfield::Domain)
    }

    /// How many significant digits to use for exchange rates of Offers involving currencies issued by this address. Valid values are `3` to `15`, inclusive.
    fn get_tick_size(&self) -> Result<Option<u8>> {
        current_ledger_object::get_field_optional(sfield::TickSize)
    }

    /// How many Tickets this account owns in the ledger. This is updated automatically to ensure that the account stays within the hard limit of 250 Tickets at a time. This field is omitted if the account has zero Tickets.
    fn get_ticket_count(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::TicketCount)
    }

    /// Another account that can mint non-fungible tokens on behalf of this account.
    fn get_nftoken_minter(&self) -> Result<Option<AccountID>> {
        current_ledger_object::get_field_optional(sfield::NFTokenMinter)
    }

    /// How many total non-fungible tokens have been minted by and on behalf of this account.
    fn get_minted_nftokens(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::MintedNFTokens)
    }

    /// How many total of this account's issued non-fungible tokens have been burned. This number is always equal or less than `MintedNFTokens`.
    fn get_burned_nftokens(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::BurnedNFTokens)
    }

    /// The account's [Sequence Number][] at the time it minted its first non-fungible-token.
    fn get_first_nftoken_sequence(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::FirstNFTokenSequence)
    }

    /// The ledger entry ID of the corresponding AMM ledger entry. Set during account creation; cannot be modified. If present, indicates that this is a special AMM pseudo-account AccountRoot; always omitted on non-AMM accounts.
    fn get_amm_id(&self) -> Result<Option<Hash256>> {
        current_ledger_object::get_field_optional(sfield::AMMID)
    }

    /// The ID of the `Vault` entry associated with this account. Set during account creation; cannot be modified. If present, indicates that this is a special Vault pseudo-account AccountRoot; always omitted on non-Vault accounts.
    fn get_vault_id(&self) -> Result<Option<Hash256>> {
        current_ledger_object::get_field_optional(sfield::VaultID)
    }

    /// The LoanBrokerID field (Optional).
    fn get_loan_broker_id(&self) -> Result<Option<Hash256>> {
        current_ledger_object::get_field_optional(sfield::LoanBrokerID)
    }

    /// The ContractID field (Optional).
    fn get_contract_id(&self) -> Result<Option<Hash256>> {
        current_ledger_object::get_field_optional(sfield::ContractID)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct AccountRoot {
    pub(crate) slot_num: i32,
}

impl AccountRoot {
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }

    /// Loads the AccountRoot ledger object identified by the given keylet arguments,
    /// caching it in a host-managed slot.
    pub fn load(account: &AccountID) -> Result<Self> {
        let keylet = match crate::keylets::account_keylet(account) {
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

impl LedgerObjectCommonFields for AccountRoot {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl AccountRootFields for AccountRoot {}

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

        let obj = AccountRoot::new(0);

        assert!(obj.get_account().is_ok());
        assert!(obj.get_sequence().is_ok());
        assert!(obj.get_balance().is_ok());
        assert!(obj.get_owner_count().is_ok());
        assert!(obj.get_previous_txn_id().is_ok());
        assert!(obj.get_previous_txn_lgr_seq().is_ok());
        assert!(obj.get_account_txn_id().is_ok());
        assert!(obj.get_regular_key().is_ok());
        assert!(obj.get_email_hash().is_ok());
        assert!(obj.get_wallet_locator().is_ok());
        assert!(obj.get_wallet_size().is_ok());
        assert!(obj.get_message_key().is_ok());
        assert!(obj.get_transfer_rate().is_ok());
        assert!(obj.get_domain().is_ok());
        assert!(obj.get_tick_size().is_ok());
        assert!(obj.get_ticket_count().is_ok());
        assert!(obj.get_nftoken_minter().is_ok());
        assert!(obj.get_minted_nftokens().is_ok());
        assert!(obj.get_burned_nftokens().is_ok());
        assert!(obj.get_first_nftoken_sequence().is_ok());
        assert!(obj.get_amm_id().is_ok());
        assert!(obj.get_vault_id().is_ok());
        assert!(obj.get_loan_broker_id().is_ok());
        assert!(obj.get_contract_id().is_ok());
    }

    #[test]
    fn optional_fields_none() {
        let mut mock = MockHostBindings::new();
        mock_all_fields_not_found(&mut mock);
        let _guard = setup_mock(mock);

        let obj = AccountRoot::new(0);

        assert!(obj.get_account_txn_id().unwrap().is_none());
        assert!(obj.get_regular_key().unwrap().is_none());
        assert!(obj.get_email_hash().unwrap().is_none());
        assert!(obj.get_wallet_locator().unwrap().is_none());
        assert!(obj.get_wallet_size().unwrap().is_none());
        assert!(obj.get_transfer_rate().unwrap().is_none());
        assert!(obj.get_tick_size().unwrap().is_none());
        assert!(obj.get_ticket_count().unwrap().is_none());
        assert!(obj.get_nftoken_minter().unwrap().is_none());
        assert!(obj.get_minted_nftokens().unwrap().is_none());
        assert!(obj.get_burned_nftokens().unwrap().is_none());
        assert!(obj.get_first_nftoken_sequence().unwrap().is_none());
        assert!(obj.get_amm_id().unwrap().is_none());
        assert!(obj.get_vault_id().unwrap().is_none());
        assert!(obj.get_loan_broker_id().unwrap().is_none());
        assert!(obj.get_contract_id().unwrap().is_none());
    }

    #[test]
    fn load_success() {
        let mut mock = MockHostBindings::new();
        mock_account_keylet_success(&mut mock);
        mock_cache_ledger_obj_success(&mut mock, 7);
        let _guard = setup_mock(mock);

        let result = AccountRoot::load(&sample::account_id());
        assert!(result.is_ok());
    }

    #[test]
    fn load_cache_error() {
        use crate::host::error_codes::INTERNAL_ERROR;

        let mut mock = MockHostBindings::new();
        mock_account_keylet_success(&mut mock);
        mock_cache_ledger_obj_error(&mut mock, INTERNAL_ERROR);
        let _guard = setup_mock(mock);

        let result = AccountRoot::load(&sample::account_id());
        assert!(result.is_err());
    }
}
