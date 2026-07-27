// GENERATED -- do not hand-edit. Run scripts/generate-ledger-objects.sh to regenerate.

use crate::host::Result;
use crate::objects::array_object::{Array, Object};
use crate::objects::traits::{CurrentLedgerObjectCommonFields, LedgerObjectCommonFields};
use crate::objects::{current_ledger_object, ledger_object};
use crate::sfield;
use crate::types::account_id::AccountID;
use crate::types::amount::Amount;
use crate::types::blob::{ConditionBlob, PublicKeyBlob, StandardBlob, UriBlob, WasmBlob};
use crate::types::issue::Issue;
use crate::types::uint::{Hash128, Hash160, Hash192, Hash256};

/// Trait providing access to fields specific to NFTokenOffer objects in any ledger.
pub trait NFTokenOfferFields: LedgerObjectCommonFields {
    /// The Owner field (Required).
    fn get_owner(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Owner)
    }

    /// The NFTokenID field (Required).
    fn get_nf_token_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::NFTokenID)
    }

    /// The Amount field (Required).
    fn get_amount(&self) -> Result<Amount> {
        ledger_object::get_field(self.get_slot_num(), sfield::Amount)
    }

    /// A hint indicating which page of the owner's directory links to this entry, in case the
    /// directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::OwnerNode)
    }

    /// The NFTokenOfferNode field (Required).
    fn get_nf_token_offer_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::NFTokenOfferNode)
    }

    /// The Destination field (Optional).
    fn get_destination(&self) -> Result<Option<AccountID>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::Destination)
    }

    /// The Expiration field (Optional).
    fn get_expiration(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::Expiration)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }
}

/// Trait providing access to fields specific to the current NFTokenOffer object.
pub trait CurrentNFTokenOfferFields: CurrentLedgerObjectCommonFields {
    /// The Owner field (Required).
    fn get_owner(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Owner)
    }

    /// The NFTokenID field (Required).
    fn get_nf_token_id(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::NFTokenID)
    }

    /// The Amount field (Required).
    fn get_amount(&self) -> Result<Amount> {
        current_ledger_object::get_field(sfield::Amount)
    }

    /// A hint indicating which page of the owner's directory links to this entry, in case the
    /// directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::OwnerNode)
    }

    /// The NFTokenOfferNode field (Required).
    fn get_nf_token_offer_node(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::NFTokenOfferNode)
    }

    /// The Destination field (Optional).
    fn get_destination(&self) -> Result<Option<AccountID>> {
        current_ledger_object::get_field_optional(sfield::Destination)
    }

    /// The Expiration field (Optional).
    fn get_expiration(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::Expiration)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::PreviousTxnLgrSeq)
    }
}

/// `lsf*` flag constants for NFTokenOffer objects.
#[allow(non_upper_case_globals, dead_code)]
pub mod nf_token_offer_flags {
    pub const lsfSellNFToken: u32 = 0x00000001;
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct NFTokenOffer {
    pub(crate) slot_num: i32,
}

impl NFTokenOffer {
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }
}

impl LedgerObjectCommonFields for NFTokenOffer {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl NFTokenOfferFields for NFTokenOffer {}

/// Trait providing access to fields specific to Check objects in any ledger.
pub trait CheckFields: LedgerObjectCommonFields {
    /// The Account field (Required).
    fn get_account(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Account)
    }

    /// The Destination field (Required).
    fn get_destination(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Destination)
    }

    /// The SendMax field (Required).
    fn get_send_max(&self) -> Result<Amount> {
        ledger_object::get_field(self.get_slot_num(), sfield::SendMax)
    }

    /// The Sequence field (Required).
    fn get_sequence(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::Sequence)
    }

    /// A hint indicating which page of the owner's directory links to this entry, in case the
    /// directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::OwnerNode)
    }

    /// The DestinationNode field (Required).
    fn get_destination_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::DestinationNode)
    }

    /// The Expiration field (Optional).
    fn get_expiration(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::Expiration)
    }

    /// The InvoiceID field (Optional).
    fn get_invoice_id(&self) -> Result<Option<Hash256>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::InvoiceID)
    }

    /// The SourceTag field (Optional).
    fn get_source_tag(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::SourceTag)
    }

    /// The DestinationTag field (Optional).
    fn get_destination_tag(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::DestinationTag)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }
}

/// Trait providing access to fields specific to the current Check object.
pub trait CurrentCheckFields: CurrentLedgerObjectCommonFields {
    /// The Account field (Required).
    fn get_account(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Account)
    }

    /// The Destination field (Required).
    fn get_destination(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Destination)
    }

    /// The SendMax field (Required).
    fn get_send_max(&self) -> Result<Amount> {
        current_ledger_object::get_field(sfield::SendMax)
    }

    /// The Sequence field (Required).
    fn get_sequence(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::Sequence)
    }

    /// A hint indicating which page of the owner's directory links to this entry, in case the
    /// directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::OwnerNode)
    }

    /// The DestinationNode field (Required).
    fn get_destination_node(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::DestinationNode)
    }

    /// The Expiration field (Optional).
    fn get_expiration(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::Expiration)
    }

    /// The InvoiceID field (Optional).
    fn get_invoice_id(&self) -> Result<Option<Hash256>> {
        current_ledger_object::get_field_optional(sfield::InvoiceID)
    }

    /// The SourceTag field (Optional).
    fn get_source_tag(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::SourceTag)
    }

    /// The DestinationTag field (Optional).
    fn get_destination_tag(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::DestinationTag)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::PreviousTxnLgrSeq)
    }
}

/// `lsf*` flag constants for Check objects.
#[allow(non_upper_case_globals, dead_code)]
pub mod check_flags {
    // No lsf* flags are defined for Check in LedgerFormats.h.
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Check {
    pub(crate) slot_num: i32,
}

impl Check {
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }
}

impl LedgerObjectCommonFields for Check {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl CheckFields for Check {}

/// Trait providing access to fields specific to DID objects in any ledger.
pub trait DIDFields: LedgerObjectCommonFields {
    /// The Account field (Required).
    fn get_account(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Account)
    }

    /// The DIDDocument field (Optional).
    fn get_did_document(&self) -> Result<Option<StandardBlob>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::DIDDocument)
    }

    /// The URI field (Optional).
    fn get_uri(&self) -> Result<Option<UriBlob>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::URI)
    }

    /// The Data field (Optional).
    fn get_data(&self) -> Result<Option<StandardBlob>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::Data)
    }

    /// A hint indicating which page of the owner's directory links to this entry, in case the
    /// directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::OwnerNode)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }
}

/// Trait providing access to fields specific to the current DID object.
pub trait CurrentDIDFields: CurrentLedgerObjectCommonFields {
    /// The Account field (Required).
    fn get_account(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Account)
    }

    /// The DIDDocument field (Optional).
    fn get_did_document(&self) -> Result<Option<StandardBlob>> {
        current_ledger_object::get_field_optional(sfield::DIDDocument)
    }

    /// The URI field (Optional).
    fn get_uri(&self) -> Result<Option<UriBlob>> {
        current_ledger_object::get_field_optional(sfield::URI)
    }

    /// The Data field (Optional).
    fn get_data(&self) -> Result<Option<StandardBlob>> {
        current_ledger_object::get_field_optional(sfield::Data)
    }

    /// A hint indicating which page of the owner's directory links to this entry, in case the
    /// directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::OwnerNode)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::PreviousTxnLgrSeq)
    }
}

/// `lsf*` flag constants for DID objects.
#[allow(non_upper_case_globals, dead_code)]
pub mod did_flags {
    // No lsf* flags are defined for DID in LedgerFormats.h.
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct DID {
    pub(crate) slot_num: i32,
}

impl DID {
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }
}

impl LedgerObjectCommonFields for DID {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl DIDFields for DID {}

/// Trait providing access to fields specific to NegativeUNL objects in any ledger.
pub trait NegativeUNLFields: LedgerObjectCommonFields {
    /// The DisabledValidators field (Optional).
    fn get_disabled_validators(&self) -> Result<Option<Array>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::DisabledValidators)
    }

    /// The ValidatorToDisable field (Optional).
    fn get_validator_to_disable(&self) -> Result<Option<StandardBlob>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::ValidatorToDisable)
    }

    /// The ValidatorToReEnable field (Optional).
    fn get_validator_to_re_enable(&self) -> Result<Option<StandardBlob>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::ValidatorToReEnable)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Option<Hash256>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }
}

/// Trait providing access to fields specific to the current NegativeUNL object.
pub trait CurrentNegativeUNLFields: CurrentLedgerObjectCommonFields {
    /// The DisabledValidators field (Optional).
    fn get_disabled_validators(&self) -> Result<Option<Array>> {
        current_ledger_object::get_field_optional(sfield::DisabledValidators)
    }

    /// The ValidatorToDisable field (Optional).
    fn get_validator_to_disable(&self) -> Result<Option<StandardBlob>> {
        current_ledger_object::get_field_optional(sfield::ValidatorToDisable)
    }

    /// The ValidatorToReEnable field (Optional).
    fn get_validator_to_re_enable(&self) -> Result<Option<StandardBlob>> {
        current_ledger_object::get_field_optional(sfield::ValidatorToReEnable)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Option<Hash256>> {
        current_ledger_object::get_field_optional(sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::PreviousTxnLgrSeq)
    }
}

/// `lsf*` flag constants for NegativeUNL objects.
#[allow(non_upper_case_globals, dead_code)]
pub mod negative_unl_flags {
    // No lsf* flags are defined for NegativeUNL in LedgerFormats.h.
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct NegativeUNL {
    pub(crate) slot_num: i32,
}

impl NegativeUNL {
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }
}

impl LedgerObjectCommonFields for NegativeUNL {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl NegativeUNLFields for NegativeUNL {}

/// Trait providing access to fields specific to NFTokenPage objects in any ledger.
pub trait NFTokenPageFields: LedgerObjectCommonFields {
    /// The PreviousPageMin field (Optional).
    fn get_previous_page_min(&self) -> Result<Option<Hash256>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::PreviousPageMin)
    }

    /// The NextPageMin field (Optional).
    fn get_next_page_min(&self) -> Result<Option<Hash256>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::NextPageMin)
    }

    /// The NFTokens field (Required).
    fn get_nf_tokens(&self) -> Result<Array> {
        ledger_object::get_field(self.get_slot_num(), sfield::NFTokens)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }
}

/// Trait providing access to fields specific to the current NFTokenPage object.
pub trait CurrentNFTokenPageFields: CurrentLedgerObjectCommonFields {
    /// The PreviousPageMin field (Optional).
    fn get_previous_page_min(&self) -> Result<Option<Hash256>> {
        current_ledger_object::get_field_optional(sfield::PreviousPageMin)
    }

    /// The NextPageMin field (Optional).
    fn get_next_page_min(&self) -> Result<Option<Hash256>> {
        current_ledger_object::get_field_optional(sfield::NextPageMin)
    }

    /// The NFTokens field (Required).
    fn get_nf_tokens(&self) -> Result<Array> {
        current_ledger_object::get_field(sfield::NFTokens)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::PreviousTxnLgrSeq)
    }
}

/// `lsf*` flag constants for NFTokenPage objects.
#[allow(non_upper_case_globals, dead_code)]
pub mod nf_token_page_flags {
    // No lsf* flags are defined for NFTokenPage in LedgerFormats.h.
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct NFTokenPage {
    pub(crate) slot_num: i32,
}

impl NFTokenPage {
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }
}

impl LedgerObjectCommonFields for NFTokenPage {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl NFTokenPageFields for NFTokenPage {}

/// Trait providing access to fields specific to SignerList objects in any ledger.
pub trait SignerListFields: LedgerObjectCommonFields {
    /// The Owner field (Optional).
    fn get_owner(&self) -> Result<Option<AccountID>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::Owner)
    }

    /// A hint indicating which page of the owner's directory links to this entry, in case the
    /// directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::OwnerNode)
    }

    /// The SignerQuorum field (Required).
    fn get_signer_quorum(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::SignerQuorum)
    }

    /// The SignerEntries field (Required).
    fn get_signer_entries(&self) -> Result<Array> {
        ledger_object::get_field(self.get_slot_num(), sfield::SignerEntries)
    }

    /// The SignerListID field (Required).
    fn get_signer_list_id(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::SignerListID)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
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

    /// A hint indicating which page of the owner's directory links to this entry, in case the
    /// directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::OwnerNode)
    }

    /// The SignerQuorum field (Required).
    fn get_signer_quorum(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::SignerQuorum)
    }

    /// The SignerEntries field (Required).
    fn get_signer_entries(&self) -> Result<Array> {
        current_ledger_object::get_field(sfield::SignerEntries)
    }

    /// The SignerListID field (Required).
    fn get_signer_list_id(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::SignerListID)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::PreviousTxnLgrSeq)
    }
}

/// `lsf*` flag constants for SignerList objects.
#[allow(non_upper_case_globals, dead_code)]
pub mod signer_list_flags {
    pub const lsfOneOwnerCount: u32 = 0x00010000;
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct SignerList {
    pub(crate) slot_num: i32,
}

impl SignerList {
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }
}

impl LedgerObjectCommonFields for SignerList {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl SignerListFields for SignerList {}

/// Trait providing access to fields specific to Ticket objects in any ledger.
pub trait TicketFields: LedgerObjectCommonFields {
    /// The Account field (Required).
    fn get_account(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Account)
    }

    /// A hint indicating which page of the owner's directory links to this entry, in case the
    /// directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::OwnerNode)
    }

    /// The TicketSequence field (Required).
    fn get_ticket_sequence(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::TicketSequence)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }
}

/// Trait providing access to fields specific to the current Ticket object.
pub trait CurrentTicketFields: CurrentLedgerObjectCommonFields {
    /// The Account field (Required).
    fn get_account(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Account)
    }

    /// A hint indicating which page of the owner's directory links to this entry, in case the
    /// directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::OwnerNode)
    }

    /// The TicketSequence field (Required).
    fn get_ticket_sequence(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::TicketSequence)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::PreviousTxnLgrSeq)
    }
}

/// `lsf*` flag constants for Ticket objects.
#[allow(non_upper_case_globals, dead_code)]
pub mod ticket_flags {
    // No lsf* flags are defined for Ticket in LedgerFormats.h.
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Ticket {
    pub(crate) slot_num: i32,
}

impl Ticket {
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }
}

impl LedgerObjectCommonFields for Ticket {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl TicketFields for Ticket {}

/// Trait providing access to fields specific to AccountRoot objects in any ledger.
pub trait AccountRootFields: LedgerObjectCommonFields {
    /// The identifying address of the account.
    fn get_account(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Account)
    }

    /// The sequence number of the next valid transaction for this account.
    fn get_sequence(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::Sequence)
    }

    /// The account's current XRP balance in drops.
    fn get_balance(&self) -> Result<Amount> {
        ledger_object::get_field(self.get_slot_num(), sfield::Balance)
    }

    /// The number of objects this account owns in the ledger, which contributes to its owner reserve.
    fn get_owner_count(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::OwnerCount)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }

    /// AccountTxnID field for the account.
    fn get_account_txn_id(&self) -> Result<Option<Hash256>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::AccountTxnID)
    }

    /// The address of a key pair that can be used to sign transactions for this account instead of the master key.
    /// Use a SetRegularKey transaction to change this value.
    fn get_regular_key(&self) -> Result<Option<AccountID>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::RegularKey)
    }

    /// The MD5 hash of an email address. Clients can use this to look up an avatar through services such as Gravatar.
    fn get_email_hash(&self) -> Result<Option<Hash128>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::EmailHash)
    }

    /// An arbitrary 256-bit value that users can set.
    fn get_wallet_locator(&self) -> Result<Option<Hash256>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::WalletLocator)
    }

    /// The WalletSize field (Optional).
    fn get_wallet_size(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::WalletSize)
    }

    /// The MessageKey field (Optional).
    fn get_message_key(&self) -> Result<Option<PublicKeyBlob>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::MessageKey)
    }

    /// A transfer fee to charge other users for sending currency issued by this account to each other.
    fn get_transfer_rate(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::TransferRate)
    }

    /// A domain associated with this account. In JSON, this is the hexadecimal for the ASCII representation of the
    /// domain. Cannot be more than 256 bytes in length.
    fn get_domain(&self) -> Result<Option<UriBlob>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::Domain)
    }

    /// How many significant digits to use for exchange rates of Offers involving currencies issued by this address.
    /// Valid values are 3 to 15, inclusive. (Added by the TickSize amendment.)
    fn get_tick_size(&self) -> Result<Option<u8>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::TickSize)
    }

    /// How many Tickets this account owns in the ledger. This is updated automatically to ensure that
    /// the account stays within the hard limit of 250 Tickets at a time. This field is omitted if the account has zero
    /// Tickets. (Added by the TicketBatch amendment.)
    fn get_ticket_count(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::TicketCount)
    }

    /// Another account that can mint non-fungible tokens on behalf of this account.
    /// (Added by the NonFungibleTokensV1_1 amendment)
    fn get_nf_token_minter(&self) -> Result<Option<AccountID>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::NFTokenMinter)
    }

    /// How many total non-fungible tokens have been minted by and on behalf of this account.
    /// (Added by the NonFungibleTokensV1_1 amendment)
    fn get_minted_nf_tokens(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::MintedNFTokens)
    }

    /// How many total of this account's issued non-fungible tokens have been burned.
    /// This number is always equal or less than MintedNFTokens.
    fn get_burned_nf_tokens(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::BurnedNFTokens)
    }

    /// The account's Sequence Number at the time it minted its first non-fungible-token.
    /// (Added by the fixNFTokenRemint amendment)
    fn get_first_nf_token_sequence(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::FirstNFTokenSequence)
    }

    /// The ledger entry ID of the corresponding AMM ledger entry. Set during account creation; cannot be modified.
    /// If present, indicates that this is a special AMM AccountRoot; always omitted on non-AMM accounts.
    /// (Added by the AMM amendment)
    fn get_ammid(&self) -> Result<Option<Hash256>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::AMMID)
    }

    /// The VaultID field (Optional).
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
    /// The identifying address of the account.
    fn get_account(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Account)
    }

    /// The sequence number of the next valid transaction for this account.
    fn get_sequence(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::Sequence)
    }

    /// The account's current XRP balance in drops.
    fn get_balance(&self) -> Result<Amount> {
        current_ledger_object::get_field(sfield::Balance)
    }

    /// The number of objects this account owns in the ledger, which contributes to its owner reserve.
    fn get_owner_count(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::OwnerCount)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::PreviousTxnLgrSeq)
    }

    /// AccountTxnID field for the account.
    fn get_account_txn_id(&self) -> Result<Option<Hash256>> {
        current_ledger_object::get_field_optional(sfield::AccountTxnID)
    }

    /// The address of a key pair that can be used to sign transactions for this account instead of the master key.
    /// Use a SetRegularKey transaction to change this value.
    fn get_regular_key(&self) -> Result<Option<AccountID>> {
        current_ledger_object::get_field_optional(sfield::RegularKey)
    }

    /// The MD5 hash of an email address. Clients can use this to look up an avatar through services such as Gravatar.
    fn get_email_hash(&self) -> Result<Option<Hash128>> {
        current_ledger_object::get_field_optional(sfield::EmailHash)
    }

    /// An arbitrary 256-bit value that users can set.
    fn get_wallet_locator(&self) -> Result<Option<Hash256>> {
        current_ledger_object::get_field_optional(sfield::WalletLocator)
    }

    /// The WalletSize field (Optional).
    fn get_wallet_size(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::WalletSize)
    }

    /// The MessageKey field (Optional).
    fn get_message_key(&self) -> Result<Option<PublicKeyBlob>> {
        current_ledger_object::get_field_optional(sfield::MessageKey)
    }

    /// A transfer fee to charge other users for sending currency issued by this account to each other.
    fn get_transfer_rate(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::TransferRate)
    }

    /// A domain associated with this account. In JSON, this is the hexadecimal for the ASCII representation of the
    /// domain. Cannot be more than 256 bytes in length.
    fn get_domain(&self) -> Result<Option<UriBlob>> {
        current_ledger_object::get_field_optional(sfield::Domain)
    }

    /// How many significant digits to use for exchange rates of Offers involving currencies issued by this address.
    /// Valid values are 3 to 15, inclusive. (Added by the TickSize amendment.)
    fn get_tick_size(&self) -> Result<Option<u8>> {
        current_ledger_object::get_field_optional(sfield::TickSize)
    }

    /// How many Tickets this account owns in the ledger. This is updated automatically to ensure that
    /// the account stays within the hard limit of 250 Tickets at a time. This field is omitted if the account has zero
    /// Tickets. (Added by the TicketBatch amendment.)
    fn get_ticket_count(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::TicketCount)
    }

    /// Another account that can mint non-fungible tokens on behalf of this account.
    /// (Added by the NonFungibleTokensV1_1 amendment)
    fn get_nf_token_minter(&self) -> Result<Option<AccountID>> {
        current_ledger_object::get_field_optional(sfield::NFTokenMinter)
    }

    /// How many total non-fungible tokens have been minted by and on behalf of this account.
    /// (Added by the NonFungibleTokensV1_1 amendment)
    fn get_minted_nf_tokens(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::MintedNFTokens)
    }

    /// How many total of this account's issued non-fungible tokens have been burned.
    /// This number is always equal or less than MintedNFTokens.
    fn get_burned_nf_tokens(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::BurnedNFTokens)
    }

    /// The account's Sequence Number at the time it minted its first non-fungible-token.
    /// (Added by the fixNFTokenRemint amendment)
    fn get_first_nf_token_sequence(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::FirstNFTokenSequence)
    }

    /// The ledger entry ID of the corresponding AMM ledger entry. Set during account creation; cannot be modified.
    /// If present, indicates that this is a special AMM AccountRoot; always omitted on non-AMM accounts.
    /// (Added by the AMM amendment)
    fn get_ammid(&self) -> Result<Option<Hash256>> {
        current_ledger_object::get_field_optional(sfield::AMMID)
    }

    /// The VaultID field (Optional).
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

/// `lsf*` flag constants for AccountRoot objects.
#[allow(non_upper_case_globals, dead_code)]
pub mod account_root_flags {
    pub const lsfPasswordSpent: u32 = 0x00010000;
    pub const lsfRequireDestTag: u32 = 0x00020000;
    pub const lsfRequireAuth: u32 = 0x00040000;
    pub const lsfDisallowXRP: u32 = 0x00080000;
    pub const lsfDisableMaster: u32 = 0x00100000;
    pub const lsfNoFreeze: u32 = 0x00200000;
    pub const lsfGlobalFreeze: u32 = 0x00400000;
    pub const lsfDefaultRipple: u32 = 0x00800000;
    pub const lsfDepositAuth: u32 = 0x01000000;
    pub const lsfDisallowIncomingNFTokenOffer: u32 = 0x04000000;
    pub const lsfDisallowIncomingCheck: u32 = 0x08000000;
    pub const lsfDisallowIncomingPayChan: u32 = 0x10000000;
    pub const lsfDisallowIncomingTrustline: u32 = 0x20000000;
    pub const lsfAllowTrustLineLocking: u32 = 0x40000000;
    pub const lsfAllowTrustLineClawback: u32 = 0x80000000;
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct AccountRoot {
    pub(crate) slot_num: i32,
}

impl AccountRoot {
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }
}

impl LedgerObjectCommonFields for AccountRoot {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl AccountRootFields for AccountRoot {}

/// Trait providing access to fields specific to DirectoryNode objects in any ledger.
pub trait DirectoryNodeFields: LedgerObjectCommonFields {
    /// The Owner field (Optional).
    fn get_owner(&self) -> Result<Option<AccountID>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::Owner)
    }

    /// The TakerPaysCurrency field (Optional).
    fn get_taker_pays_currency(&self) -> Result<Option<Hash160>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::TakerPaysCurrency)
    }

    /// The TakerPaysIssuer field (Optional).
    fn get_taker_pays_issuer(&self) -> Result<Option<Hash160>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::TakerPaysIssuer)
    }

    /// The TakerGetsCurrency field (Optional).
    fn get_taker_gets_currency(&self) -> Result<Option<Hash160>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::TakerGetsCurrency)
    }

    /// The TakerGetsIssuer field (Optional).
    fn get_taker_gets_issuer(&self) -> Result<Option<Hash160>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::TakerGetsIssuer)
    }

    /// The ExchangeRate field (Optional).
    fn get_exchange_rate(&self) -> Result<Option<u64>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::ExchangeRate)
    }

    // SKIPPED get_indexes: VECTOR256 is not yet representable in Rust

    /// The RootIndex field (Required).
    fn get_root_index(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::RootIndex)
    }

    /// The IndexNext field (Optional).
    fn get_index_next(&self) -> Result<Option<u64>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::IndexNext)
    }

    /// The IndexPrevious field (Optional).
    fn get_index_previous(&self) -> Result<Option<u64>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::IndexPrevious)
    }

    /// The NFTokenID field (Optional).
    fn get_nf_token_id(&self) -> Result<Option<Hash256>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::NFTokenID)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Option<Hash256>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }

    /// The DomainID field (Optional).
    fn get_domain_id(&self) -> Result<Option<Hash256>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::DomainID)
    }
}

/// Trait providing access to fields specific to the current DirectoryNode object.
pub trait CurrentDirectoryNodeFields: CurrentLedgerObjectCommonFields {
    /// The Owner field (Optional).
    fn get_owner(&self) -> Result<Option<AccountID>> {
        current_ledger_object::get_field_optional(sfield::Owner)
    }

    /// The TakerPaysCurrency field (Optional).
    fn get_taker_pays_currency(&self) -> Result<Option<Hash160>> {
        current_ledger_object::get_field_optional(sfield::TakerPaysCurrency)
    }

    /// The TakerPaysIssuer field (Optional).
    fn get_taker_pays_issuer(&self) -> Result<Option<Hash160>> {
        current_ledger_object::get_field_optional(sfield::TakerPaysIssuer)
    }

    /// The TakerGetsCurrency field (Optional).
    fn get_taker_gets_currency(&self) -> Result<Option<Hash160>> {
        current_ledger_object::get_field_optional(sfield::TakerGetsCurrency)
    }

    /// The TakerGetsIssuer field (Optional).
    fn get_taker_gets_issuer(&self) -> Result<Option<Hash160>> {
        current_ledger_object::get_field_optional(sfield::TakerGetsIssuer)
    }

    /// The ExchangeRate field (Optional).
    fn get_exchange_rate(&self) -> Result<Option<u64>> {
        current_ledger_object::get_field_optional(sfield::ExchangeRate)
    }

    // SKIPPED get_indexes: VECTOR256 is not yet representable in Rust

    /// The RootIndex field (Required).
    fn get_root_index(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::RootIndex)
    }

    /// The IndexNext field (Optional).
    fn get_index_next(&self) -> Result<Option<u64>> {
        current_ledger_object::get_field_optional(sfield::IndexNext)
    }

    /// The IndexPrevious field (Optional).
    fn get_index_previous(&self) -> Result<Option<u64>> {
        current_ledger_object::get_field_optional(sfield::IndexPrevious)
    }

    /// The NFTokenID field (Optional).
    fn get_nf_token_id(&self) -> Result<Option<Hash256>> {
        current_ledger_object::get_field_optional(sfield::NFTokenID)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Option<Hash256>> {
        current_ledger_object::get_field_optional(sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::PreviousTxnLgrSeq)
    }

    /// The DomainID field (Optional).
    fn get_domain_id(&self) -> Result<Option<Hash256>> {
        current_ledger_object::get_field_optional(sfield::DomainID)
    }
}

/// `lsf*` flag constants for DirectoryNode objects.
#[allow(non_upper_case_globals, dead_code)]
pub mod directory_node_flags {
    pub const lsfNFTokenBuyOffers: u32 = 0x00000001;
    pub const lsfNFTokenSellOffers: u32 = 0x00000002;
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct DirectoryNode {
    pub(crate) slot_num: i32,
}

impl DirectoryNode {
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

/// Trait providing access to fields specific to Amendments objects in any ledger.
pub trait AmendmentsFields: LedgerObjectCommonFields {
    // SKIPPED get_amendments: VECTOR256 is not yet representable in Rust

    /// The Majorities field (Optional).
    fn get_majorities(&self) -> Result<Option<Array>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::Majorities)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Option<Hash256>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }
}

/// Trait providing access to fields specific to the current Amendments object.
pub trait CurrentAmendmentsFields: CurrentLedgerObjectCommonFields {
    // SKIPPED get_amendments: VECTOR256 is not yet representable in Rust

    /// The Majorities field (Optional).
    fn get_majorities(&self) -> Result<Option<Array>> {
        current_ledger_object::get_field_optional(sfield::Majorities)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Option<Hash256>> {
        current_ledger_object::get_field_optional(sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::PreviousTxnLgrSeq)
    }
}

/// `lsf*` flag constants for Amendments objects.
#[allow(non_upper_case_globals, dead_code)]
pub mod amendments_flags {
    // No lsf* flags are defined for Amendments in LedgerFormats.h.
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Amendments {
    pub(crate) slot_num: i32,
}

impl Amendments {
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }
}

impl LedgerObjectCommonFields for Amendments {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl AmendmentsFields for Amendments {}

/// Trait providing access to fields specific to LedgerHashes objects in any ledger.
pub trait LedgerHashesFields: LedgerObjectCommonFields {
    /// The FirstLedgerSequence field (Optional).
    fn get_first_ledger_sequence(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::FirstLedgerSequence)
    }

    /// The LastLedgerSequence field (Optional).
    fn get_last_ledger_sequence(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::LastLedgerSequence)
    }

    // SKIPPED get_hashes: VECTOR256 is not yet representable in Rust
}

/// Trait providing access to fields specific to the current LedgerHashes object.
pub trait CurrentLedgerHashesFields: CurrentLedgerObjectCommonFields {
    /// The FirstLedgerSequence field (Optional).
    fn get_first_ledger_sequence(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::FirstLedgerSequence)
    }

    /// The LastLedgerSequence field (Optional).
    fn get_last_ledger_sequence(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::LastLedgerSequence)
    }

    // SKIPPED get_hashes: VECTOR256 is not yet representable in Rust
}

/// `lsf*` flag constants for LedgerHashes objects.
#[allow(non_upper_case_globals, dead_code)]
pub mod ledger_hashes_flags {
    // No lsf* flags are defined for LedgerHashes in LedgerFormats.h.
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct LedgerHashes {
    pub(crate) slot_num: i32,
}

impl LedgerHashes {
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }
}

impl LedgerObjectCommonFields for LedgerHashes {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl LedgerHashesFields for LedgerHashes {}

/// Trait providing access to fields specific to Bridge objects in any ledger.
pub trait BridgeFields: LedgerObjectCommonFields {
    /// The Account field (Required).
    fn get_account(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Account)
    }

    /// The SignatureReward field (Required).
    fn get_signature_reward(&self) -> Result<Amount> {
        ledger_object::get_field(self.get_slot_num(), sfield::SignatureReward)
    }

    /// The MinAccountCreateAmount field (Optional).
    fn get_min_account_create_amount(&self) -> Result<Option<Amount>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::MinAccountCreateAmount)
    }

    // SKIPPED get_x_chain_bridge: XCHAIN_BRIDGE is not yet representable in Rust

    /// The XChainClaimID field (Required).
    fn get_x_chain_claim_id(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::XChainClaimID)
    }

    /// The XChainAccountCreateCount field (Required).
    fn get_x_chain_account_create_count(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::XChainAccountCreateCount)
    }

    /// The XChainAccountClaimCount field (Required).
    fn get_x_chain_account_claim_count(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::XChainAccountClaimCount)
    }

    /// A hint indicating which page of the owner's directory links to this entry, in case the
    /// directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::OwnerNode)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }
}

/// Trait providing access to fields specific to the current Bridge object.
pub trait CurrentBridgeFields: CurrentLedgerObjectCommonFields {
    /// The Account field (Required).
    fn get_account(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Account)
    }

    /// The SignatureReward field (Required).
    fn get_signature_reward(&self) -> Result<Amount> {
        current_ledger_object::get_field(sfield::SignatureReward)
    }

    /// The MinAccountCreateAmount field (Optional).
    fn get_min_account_create_amount(&self) -> Result<Option<Amount>> {
        current_ledger_object::get_field_optional(sfield::MinAccountCreateAmount)
    }

    // SKIPPED get_x_chain_bridge: XCHAIN_BRIDGE is not yet representable in Rust

    /// The XChainClaimID field (Required).
    fn get_x_chain_claim_id(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::XChainClaimID)
    }

    /// The XChainAccountCreateCount field (Required).
    fn get_x_chain_account_create_count(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::XChainAccountCreateCount)
    }

    /// The XChainAccountClaimCount field (Required).
    fn get_x_chain_account_claim_count(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::XChainAccountClaimCount)
    }

    /// A hint indicating which page of the owner's directory links to this entry, in case the
    /// directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::OwnerNode)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::PreviousTxnLgrSeq)
    }
}

/// `lsf*` flag constants for Bridge objects.
#[allow(non_upper_case_globals, dead_code)]
pub mod bridge_flags {
    // No lsf* flags are defined for Bridge in LedgerFormats.h.
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Bridge {
    pub(crate) slot_num: i32,
}

impl Bridge {
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }
}

impl LedgerObjectCommonFields for Bridge {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl BridgeFields for Bridge {}

/// Trait providing access to fields specific to Offer objects in any ledger.
pub trait OfferFields: LedgerObjectCommonFields {
    /// The Account field (Required).
    fn get_account(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Account)
    }

    /// The Sequence field (Required).
    fn get_sequence(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::Sequence)
    }

    /// The TakerPays field (Required).
    fn get_taker_pays(&self) -> Result<Amount> {
        ledger_object::get_field(self.get_slot_num(), sfield::TakerPays)
    }

    /// The TakerGets field (Required).
    fn get_taker_gets(&self) -> Result<Amount> {
        ledger_object::get_field(self.get_slot_num(), sfield::TakerGets)
    }

    /// The BookDirectory field (Required).
    fn get_book_directory(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::BookDirectory)
    }

    /// The BookNode field (Required).
    fn get_book_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::BookNode)
    }

    /// A hint indicating which page of the owner's directory links to this entry, in case the
    /// directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::OwnerNode)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }

    /// The Expiration field (Optional).
    fn get_expiration(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::Expiration)
    }

    /// The DomainID field (Optional).
    fn get_domain_id(&self) -> Result<Option<Hash256>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::DomainID)
    }

    /// The AdditionalBooks field (Optional).
    fn get_additional_books(&self) -> Result<Option<Array>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::AdditionalBooks)
    }
}

/// Trait providing access to fields specific to the current Offer object.
pub trait CurrentOfferFields: CurrentLedgerObjectCommonFields {
    /// The Account field (Required).
    fn get_account(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Account)
    }

    /// The Sequence field (Required).
    fn get_sequence(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::Sequence)
    }

    /// The TakerPays field (Required).
    fn get_taker_pays(&self) -> Result<Amount> {
        current_ledger_object::get_field(sfield::TakerPays)
    }

    /// The TakerGets field (Required).
    fn get_taker_gets(&self) -> Result<Amount> {
        current_ledger_object::get_field(sfield::TakerGets)
    }

    /// The BookDirectory field (Required).
    fn get_book_directory(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::BookDirectory)
    }

    /// The BookNode field (Required).
    fn get_book_node(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::BookNode)
    }

    /// A hint indicating which page of the owner's directory links to this entry, in case the
    /// directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::OwnerNode)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::PreviousTxnLgrSeq)
    }

    /// The Expiration field (Optional).
    fn get_expiration(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::Expiration)
    }

    /// The DomainID field (Optional).
    fn get_domain_id(&self) -> Result<Option<Hash256>> {
        current_ledger_object::get_field_optional(sfield::DomainID)
    }

    /// The AdditionalBooks field (Optional).
    fn get_additional_books(&self) -> Result<Option<Array>> {
        current_ledger_object::get_field_optional(sfield::AdditionalBooks)
    }
}

/// `lsf*` flag constants for Offer objects.
#[allow(non_upper_case_globals, dead_code)]
pub mod offer_flags {
    pub const lsfPassive: u32 = 0x00010000;
    pub const lsfSell: u32 = 0x00020000;
    pub const lsfHybrid: u32 = 0x00040000;
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Offer {
    pub(crate) slot_num: i32,
}

impl Offer {
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }
}

impl LedgerObjectCommonFields for Offer {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl OfferFields for Offer {}

/// Trait providing access to fields specific to DepositPreauth objects in any ledger.
pub trait DepositPreauthFields: LedgerObjectCommonFields {
    /// The Account field (Required).
    fn get_account(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Account)
    }

    /// The Authorize field (Optional).
    fn get_authorize(&self) -> Result<Option<AccountID>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::Authorize)
    }

    /// A hint indicating which page of the owner's directory links to this entry, in case the
    /// directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::OwnerNode)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }

    /// The AuthorizeCredentials field (Optional).
    fn get_authorize_credentials(&self) -> Result<Option<Array>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::AuthorizeCredentials)
    }
}

/// Trait providing access to fields specific to the current DepositPreauth object.
pub trait CurrentDepositPreauthFields: CurrentLedgerObjectCommonFields {
    /// The Account field (Required).
    fn get_account(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Account)
    }

    /// The Authorize field (Optional).
    fn get_authorize(&self) -> Result<Option<AccountID>> {
        current_ledger_object::get_field_optional(sfield::Authorize)
    }

    /// A hint indicating which page of the owner's directory links to this entry, in case the
    /// directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::OwnerNode)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::PreviousTxnLgrSeq)
    }

    /// The AuthorizeCredentials field (Optional).
    fn get_authorize_credentials(&self) -> Result<Option<Array>> {
        current_ledger_object::get_field_optional(sfield::AuthorizeCredentials)
    }
}

/// `lsf*` flag constants for DepositPreauth objects.
#[allow(non_upper_case_globals, dead_code)]
pub mod deposit_preauth_flags {
    // No lsf* flags are defined for DepositPreauth in LedgerFormats.h.
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct DepositPreauth {
    pub(crate) slot_num: i32,
}

impl DepositPreauth {
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }
}

impl LedgerObjectCommonFields for DepositPreauth {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl DepositPreauthFields for DepositPreauth {}

/// Trait providing access to fields specific to XChainOwnedClaimID objects in any ledger.
pub trait XChainOwnedClaimIDFields: LedgerObjectCommonFields {
    /// The Account field (Required).
    fn get_account(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Account)
    }

    // SKIPPED get_x_chain_bridge: XCHAIN_BRIDGE is not yet representable in Rust

    /// The XChainClaimID field (Required).
    fn get_x_chain_claim_id(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::XChainClaimID)
    }

    /// The OtherChainSource field (Required).
    fn get_other_chain_source(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::OtherChainSource)
    }

    /// The XChainClaimAttestations field (Required).
    fn get_x_chain_claim_attestations(&self) -> Result<Array> {
        ledger_object::get_field(self.get_slot_num(), sfield::XChainClaimAttestations)
    }

    /// The SignatureReward field (Required).
    fn get_signature_reward(&self) -> Result<Amount> {
        ledger_object::get_field(self.get_slot_num(), sfield::SignatureReward)
    }

    /// A hint indicating which page of the owner's directory links to this entry, in case the
    /// directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::OwnerNode)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }
}

/// Trait providing access to fields specific to the current XChainOwnedClaimID object.
pub trait CurrentXChainOwnedClaimIDFields: CurrentLedgerObjectCommonFields {
    /// The Account field (Required).
    fn get_account(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Account)
    }

    // SKIPPED get_x_chain_bridge: XCHAIN_BRIDGE is not yet representable in Rust

    /// The XChainClaimID field (Required).
    fn get_x_chain_claim_id(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::XChainClaimID)
    }

    /// The OtherChainSource field (Required).
    fn get_other_chain_source(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::OtherChainSource)
    }

    /// The XChainClaimAttestations field (Required).
    fn get_x_chain_claim_attestations(&self) -> Result<Array> {
        current_ledger_object::get_field(sfield::XChainClaimAttestations)
    }

    /// The SignatureReward field (Required).
    fn get_signature_reward(&self) -> Result<Amount> {
        current_ledger_object::get_field(sfield::SignatureReward)
    }

    /// A hint indicating which page of the owner's directory links to this entry, in case the
    /// directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::OwnerNode)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::PreviousTxnLgrSeq)
    }
}

/// `lsf*` flag constants for XChainOwnedClaimID objects.
#[allow(non_upper_case_globals, dead_code)]
pub mod x_chain_owned_claim_id_flags {
    // No lsf* flags are defined for XChainOwnedClaimID in LedgerFormats.h.
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct XChainOwnedClaimID {
    pub(crate) slot_num: i32,
}

impl XChainOwnedClaimID {
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }
}

impl LedgerObjectCommonFields for XChainOwnedClaimID {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl XChainOwnedClaimIDFields for XChainOwnedClaimID {}

/// Trait providing access to fields specific to RippleState objects in any ledger.
pub trait RippleStateFields: LedgerObjectCommonFields {
    /// The Balance field (Required).
    fn get_balance(&self) -> Result<Amount> {
        ledger_object::get_field(self.get_slot_num(), sfield::Balance)
    }

    /// The LowLimit field (Required).
    fn get_low_limit(&self) -> Result<Amount> {
        ledger_object::get_field(self.get_slot_num(), sfield::LowLimit)
    }

    /// The HighLimit field (Required).
    fn get_high_limit(&self) -> Result<Amount> {
        ledger_object::get_field(self.get_slot_num(), sfield::HighLimit)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }

    /// The LowNode field (Optional).
    fn get_low_node(&self) -> Result<Option<u64>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::LowNode)
    }

    /// The LowQualityIn field (Optional).
    fn get_low_quality_in(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::LowQualityIn)
    }

    /// The LowQualityOut field (Optional).
    fn get_low_quality_out(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::LowQualityOut)
    }

    /// The HighNode field (Optional).
    fn get_high_node(&self) -> Result<Option<u64>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::HighNode)
    }

    /// The HighQualityIn field (Optional).
    fn get_high_quality_in(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::HighQualityIn)
    }

    /// The HighQualityOut field (Optional).
    fn get_high_quality_out(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::HighQualityOut)
    }
}

/// Trait providing access to fields specific to the current RippleState object.
pub trait CurrentRippleStateFields: CurrentLedgerObjectCommonFields {
    /// The Balance field (Required).
    fn get_balance(&self) -> Result<Amount> {
        current_ledger_object::get_field(sfield::Balance)
    }

    /// The LowLimit field (Required).
    fn get_low_limit(&self) -> Result<Amount> {
        current_ledger_object::get_field(sfield::LowLimit)
    }

    /// The HighLimit field (Required).
    fn get_high_limit(&self) -> Result<Amount> {
        current_ledger_object::get_field(sfield::HighLimit)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::PreviousTxnLgrSeq)
    }

    /// The LowNode field (Optional).
    fn get_low_node(&self) -> Result<Option<u64>> {
        current_ledger_object::get_field_optional(sfield::LowNode)
    }

    /// The LowQualityIn field (Optional).
    fn get_low_quality_in(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::LowQualityIn)
    }

    /// The LowQualityOut field (Optional).
    fn get_low_quality_out(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::LowQualityOut)
    }

    /// The HighNode field (Optional).
    fn get_high_node(&self) -> Result<Option<u64>> {
        current_ledger_object::get_field_optional(sfield::HighNode)
    }

    /// The HighQualityIn field (Optional).
    fn get_high_quality_in(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::HighQualityIn)
    }

    /// The HighQualityOut field (Optional).
    fn get_high_quality_out(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::HighQualityOut)
    }
}

/// `lsf*` flag constants for RippleState objects.
#[allow(non_upper_case_globals, dead_code)]
pub mod ripple_state_flags {
    pub const lsfLowReserve: u32 = 0x00010000;
    pub const lsfHighReserve: u32 = 0x00020000;
    pub const lsfLowAuth: u32 = 0x00040000;
    pub const lsfHighAuth: u32 = 0x00080000;
    pub const lsfLowNoRipple: u32 = 0x00100000;
    pub const lsfHighNoRipple: u32 = 0x00200000;
    pub const lsfLowFreeze: u32 = 0x00400000;
    pub const lsfHighFreeze: u32 = 0x00800000;
    pub const lsfAMMNode: u32 = 0x01000000;
    pub const lsfLowDeepFreeze: u32 = 0x02000000;
    pub const lsfHighDeepFreeze: u32 = 0x04000000;
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct RippleState {
    pub(crate) slot_num: i32,
}

impl RippleState {
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }
}

impl LedgerObjectCommonFields for RippleState {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl RippleStateFields for RippleState {}

/// Trait providing access to fields specific to FeeSettings objects in any ledger.
pub trait FeeSettingsFields: LedgerObjectCommonFields {
    /// The BaseFee field (Optional).
    fn get_base_fee(&self) -> Result<Option<u64>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::BaseFee)
    }

    /// The ReferenceFeeUnits field (Optional).
    fn get_reference_fee_units(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::ReferenceFeeUnits)
    }

    /// The ReserveBase field (Optional).
    fn get_reserve_base(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::ReserveBase)
    }

    /// The ReserveIncrement field (Optional).
    fn get_reserve_increment(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::ReserveIncrement)
    }

    /// The BaseFeeDrops field (Optional).
    fn get_base_fee_drops(&self) -> Result<Option<Amount>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::BaseFeeDrops)
    }

    /// The ReserveBaseDrops field (Optional).
    fn get_reserve_base_drops(&self) -> Result<Option<Amount>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::ReserveBaseDrops)
    }

    /// The ReserveIncrementDrops field (Optional).
    fn get_reserve_increment_drops(&self) -> Result<Option<Amount>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::ReserveIncrementDrops)
    }

    /// The ExtensionComputeLimit field (Optional).
    fn get_extension_compute_limit(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::ExtensionComputeLimit)
    }

    /// The ExtensionSizeLimit field (Optional).
    fn get_extension_size_limit(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::ExtensionSizeLimit)
    }

    /// The GasPrice field (Optional).
    fn get_gas_price(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::GasPrice)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Option<Hash256>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }
}

/// Trait providing access to fields specific to the current FeeSettings object.
pub trait CurrentFeeSettingsFields: CurrentLedgerObjectCommonFields {
    /// The BaseFee field (Optional).
    fn get_base_fee(&self) -> Result<Option<u64>> {
        current_ledger_object::get_field_optional(sfield::BaseFee)
    }

    /// The ReferenceFeeUnits field (Optional).
    fn get_reference_fee_units(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::ReferenceFeeUnits)
    }

    /// The ReserveBase field (Optional).
    fn get_reserve_base(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::ReserveBase)
    }

    /// The ReserveIncrement field (Optional).
    fn get_reserve_increment(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::ReserveIncrement)
    }

    /// The BaseFeeDrops field (Optional).
    fn get_base_fee_drops(&self) -> Result<Option<Amount>> {
        current_ledger_object::get_field_optional(sfield::BaseFeeDrops)
    }

    /// The ReserveBaseDrops field (Optional).
    fn get_reserve_base_drops(&self) -> Result<Option<Amount>> {
        current_ledger_object::get_field_optional(sfield::ReserveBaseDrops)
    }

    /// The ReserveIncrementDrops field (Optional).
    fn get_reserve_increment_drops(&self) -> Result<Option<Amount>> {
        current_ledger_object::get_field_optional(sfield::ReserveIncrementDrops)
    }

    /// The ExtensionComputeLimit field (Optional).
    fn get_extension_compute_limit(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::ExtensionComputeLimit)
    }

    /// The ExtensionSizeLimit field (Optional).
    fn get_extension_size_limit(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::ExtensionSizeLimit)
    }

    /// The GasPrice field (Optional).
    fn get_gas_price(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::GasPrice)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Option<Hash256>> {
        current_ledger_object::get_field_optional(sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::PreviousTxnLgrSeq)
    }
}

/// `lsf*` flag constants for FeeSettings objects.
#[allow(non_upper_case_globals, dead_code)]
pub mod fee_settings_flags {
    // No lsf* flags are defined for FeeSettings in LedgerFormats.h.
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct FeeSettings {
    pub(crate) slot_num: i32,
}

impl FeeSettings {
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }
}

impl LedgerObjectCommonFields for FeeSettings {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl FeeSettingsFields for FeeSettings {}

/// Trait providing access to fields specific to XChainOwnedCreateAccountClaimID objects in any ledger.
pub trait XChainOwnedCreateAccountClaimIDFields: LedgerObjectCommonFields {
    /// The Account field (Required).
    fn get_account(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Account)
    }

    // SKIPPED get_x_chain_bridge: XCHAIN_BRIDGE is not yet representable in Rust

    /// The XChainAccountCreateCount field (Required).
    fn get_x_chain_account_create_count(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::XChainAccountCreateCount)
    }

    /// The XChainCreateAccountAttestations field (Required).
    fn get_x_chain_create_account_attestations(&self) -> Result<Array> {
        ledger_object::get_field(self.get_slot_num(), sfield::XChainCreateAccountAttestations)
    }

    /// A hint indicating which page of the owner's directory links to this entry, in case the
    /// directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::OwnerNode)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }
}

/// Trait providing access to fields specific to the current XChainOwnedCreateAccountClaimID object.
pub trait CurrentXChainOwnedCreateAccountClaimIDFields: CurrentLedgerObjectCommonFields {
    /// The Account field (Required).
    fn get_account(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Account)
    }

    // SKIPPED get_x_chain_bridge: XCHAIN_BRIDGE is not yet representable in Rust

    /// The XChainAccountCreateCount field (Required).
    fn get_x_chain_account_create_count(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::XChainAccountCreateCount)
    }

    /// The XChainCreateAccountAttestations field (Required).
    fn get_x_chain_create_account_attestations(&self) -> Result<Array> {
        current_ledger_object::get_field(sfield::XChainCreateAccountAttestations)
    }

    /// A hint indicating which page of the owner's directory links to this entry, in case the
    /// directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::OwnerNode)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::PreviousTxnLgrSeq)
    }
}

/// `lsf*` flag constants for XChainOwnedCreateAccountClaimID objects.
#[allow(non_upper_case_globals, dead_code)]
pub mod x_chain_owned_create_account_claim_id_flags {
    // No lsf* flags are defined for XChainOwnedCreateAccountClaimID in LedgerFormats.h.
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct XChainOwnedCreateAccountClaimID {
    pub(crate) slot_num: i32,
}

impl XChainOwnedCreateAccountClaimID {
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }
}

impl LedgerObjectCommonFields for XChainOwnedCreateAccountClaimID {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl XChainOwnedCreateAccountClaimIDFields for XChainOwnedCreateAccountClaimID {}

/// Trait providing access to fields specific to Escrow objects in any ledger.
pub trait EscrowFields: LedgerObjectCommonFields {
    /// The address of the owner (sender) of this escrow. This is the account that provided the XRP
    /// and gets it back if the escrow is canceled.
    fn get_account(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Account)
    }

    /// The Sequence field (Optional).
    fn get_sequence(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::Sequence)
    }

    /// The destination address where the XRP is paid if the escrow is successful.
    fn get_destination(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Destination)
    }

    /// The amount of XRP, in drops, currently held in the escrow.
    fn get_amount(&self) -> Result<Amount> {
        ledger_object::get_field(self.get_slot_num(), sfield::Amount)
    }

    /// A PREIMAGE-SHA-256 crypto-condition in full crypto-condition format. If present, the EscrowFinish
    /// transaction must contain a fulfillment that satisfies this condition.
    fn get_condition(&self) -> Result<Option<ConditionBlob>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::Condition)
    }

    /// The escrow can be canceled if and only if this field is present and the time it specifies
    /// has passed. Specifically, this is specified as seconds since the Ripple Epoch and it
    /// "has passed" if it's earlier than the close time of the previous validated ledger.
    fn get_cancel_after(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::CancelAfter)
    }

    /// The time, in seconds since the Ripple Epoch, after which this escrow can be finished. Any
    /// EscrowFinish transaction before this time fails. (Specifically, this is compared with the
    /// close time of the previous validated ledger.)
    fn get_finish_after(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::FinishAfter)
    }

    /// The WASM code that is executing.
    fn get_finish_function(&self) -> Result<Option<WasmBlob>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::FinishFunction)
    }

    // SKIPPED get_data: hand-written (ContractData semantics)

    /// An arbitrary tag to further specify the source for this escrow, such as a hosted recipient
    /// at the owner's address.
    fn get_source_tag(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::SourceTag)
    }

    /// An arbitrary tag to further specify the destination for this escrow, such as a hosted
    /// recipient at the destination address.
    fn get_destination_tag(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::DestinationTag)
    }

    /// A hint indicating which page of the sender's owner directory links to this entry, in case
    /// the directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::OwnerNode)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }

    /// A hint indicating which page of the destination's owner directory links to this object, in
    /// case the directory consists of multiple pages. Omitted on escrows created before enabling the fix1523 amendment.
    fn get_destination_node(&self) -> Result<Option<u64>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::DestinationNode)
    }

    /// The TransferRate field (Optional).
    fn get_transfer_rate(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::TransferRate)
    }

    /// The IssuerNode field (Optional).
    fn get_issuer_node(&self) -> Result<Option<u64>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::IssuerNode)
    }
}

/// Trait providing access to fields specific to PayChannel objects in any ledger.
pub trait PayChannelFields: LedgerObjectCommonFields {
    /// The Account field (Required).
    fn get_account(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Account)
    }

    /// The Destination field (Required).
    fn get_destination(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Destination)
    }

    /// The Sequence field (Optional).
    fn get_sequence(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::Sequence)
    }

    /// The Amount field (Required).
    fn get_amount(&self) -> Result<Amount> {
        ledger_object::get_field(self.get_slot_num(), sfield::Amount)
    }

    /// The Balance field (Required).
    fn get_balance(&self) -> Result<Amount> {
        ledger_object::get_field(self.get_slot_num(), sfield::Balance)
    }

    /// The PublicKey field (Required).
    fn get_public_key(&self) -> Result<PublicKeyBlob> {
        ledger_object::get_field(self.get_slot_num(), sfield::PublicKey)
    }

    /// The SettleDelay field (Required).
    fn get_settle_delay(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::SettleDelay)
    }

    /// The Expiration field (Optional).
    fn get_expiration(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::Expiration)
    }

    /// The CancelAfter field (Optional).
    fn get_cancel_after(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::CancelAfter)
    }

    /// The SourceTag field (Optional).
    fn get_source_tag(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::SourceTag)
    }

    /// The DestinationTag field (Optional).
    fn get_destination_tag(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::DestinationTag)
    }

    /// A hint indicating which page of the owner's directory links to this entry, in case the
    /// directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::OwnerNode)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }

    /// The DestinationNode field (Optional).
    fn get_destination_node(&self) -> Result<Option<u64>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::DestinationNode)
    }
}

/// Trait providing access to fields specific to the current PayChannel object.
pub trait CurrentPayChannelFields: CurrentLedgerObjectCommonFields {
    /// The Account field (Required).
    fn get_account(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Account)
    }

    /// The Destination field (Required).
    fn get_destination(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Destination)
    }

    /// The Sequence field (Optional).
    fn get_sequence(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::Sequence)
    }

    /// The Amount field (Required).
    fn get_amount(&self) -> Result<Amount> {
        current_ledger_object::get_field(sfield::Amount)
    }

    /// The Balance field (Required).
    fn get_balance(&self) -> Result<Amount> {
        current_ledger_object::get_field(sfield::Balance)
    }

    /// The PublicKey field (Required).
    fn get_public_key(&self) -> Result<PublicKeyBlob> {
        current_ledger_object::get_field(sfield::PublicKey)
    }

    /// The SettleDelay field (Required).
    fn get_settle_delay(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::SettleDelay)
    }

    /// The Expiration field (Optional).
    fn get_expiration(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::Expiration)
    }

    /// The CancelAfter field (Optional).
    fn get_cancel_after(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::CancelAfter)
    }

    /// The SourceTag field (Optional).
    fn get_source_tag(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::SourceTag)
    }

    /// The DestinationTag field (Optional).
    fn get_destination_tag(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::DestinationTag)
    }

    /// A hint indicating which page of the owner's directory links to this entry, in case the
    /// directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::OwnerNode)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::PreviousTxnLgrSeq)
    }

    /// The DestinationNode field (Optional).
    fn get_destination_node(&self) -> Result<Option<u64>> {
        current_ledger_object::get_field_optional(sfield::DestinationNode)
    }
}

/// `lsf*` flag constants for PayChannel objects.
#[allow(non_upper_case_globals, dead_code)]
pub mod pay_channel_flags {
    // No lsf* flags are defined for PayChannel in LedgerFormats.h.
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct PayChannel {
    pub(crate) slot_num: i32,
}

impl PayChannel {
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }
}

impl LedgerObjectCommonFields for PayChannel {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl PayChannelFields for PayChannel {}

/// Trait providing access to fields specific to AMM objects in any ledger.
pub trait AMMFields: LedgerObjectCommonFields {
    /// The Account field (Required).
    fn get_account(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Account)
    }

    /// The TradingFee field (Optional).
    fn get_trading_fee(&self) -> Result<Option<u16>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::TradingFee)
    }

    /// The VoteSlots field (Optional).
    fn get_vote_slots(&self) -> Result<Option<Array>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::VoteSlots)
    }

    /// The AuctionSlot field (Optional).
    fn get_auction_slot(&self) -> Result<Option<Object>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::AuctionSlot)
    }

    /// The LPTokenBalance field (Required).
    fn get_lp_token_balance(&self) -> Result<Amount> {
        ledger_object::get_field(self.get_slot_num(), sfield::LPTokenBalance)
    }

    /// The Asset field (Required).
    fn get_asset(&self) -> Result<Issue> {
        ledger_object::get_field(self.get_slot_num(), sfield::Asset)
    }

    /// The Asset2 field (Required).
    fn get_asset2(&self) -> Result<Issue> {
        ledger_object::get_field(self.get_slot_num(), sfield::Asset2)
    }

    /// A hint indicating which page of the owner's directory links to this entry, in case the
    /// directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::OwnerNode)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Option<Hash256>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }
}

/// Trait providing access to fields specific to the current AMM object.
pub trait CurrentAMMFields: CurrentLedgerObjectCommonFields {
    /// The Account field (Required).
    fn get_account(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Account)
    }

    /// The TradingFee field (Optional).
    fn get_trading_fee(&self) -> Result<Option<u16>> {
        current_ledger_object::get_field_optional(sfield::TradingFee)
    }

    /// The VoteSlots field (Optional).
    fn get_vote_slots(&self) -> Result<Option<Array>> {
        current_ledger_object::get_field_optional(sfield::VoteSlots)
    }

    /// The AuctionSlot field (Optional).
    fn get_auction_slot(&self) -> Result<Option<Object>> {
        current_ledger_object::get_field_optional(sfield::AuctionSlot)
    }

    /// The LPTokenBalance field (Required).
    fn get_lp_token_balance(&self) -> Result<Amount> {
        current_ledger_object::get_field(sfield::LPTokenBalance)
    }

    /// The Asset field (Required).
    fn get_asset(&self) -> Result<Issue> {
        current_ledger_object::get_field(sfield::Asset)
    }

    /// The Asset2 field (Required).
    fn get_asset2(&self) -> Result<Issue> {
        current_ledger_object::get_field(sfield::Asset2)
    }

    /// A hint indicating which page of the owner's directory links to this entry, in case the
    /// directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::OwnerNode)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Option<Hash256>> {
        current_ledger_object::get_field_optional(sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::PreviousTxnLgrSeq)
    }
}

/// `lsf*` flag constants for AMM objects.
#[allow(non_upper_case_globals, dead_code)]
pub mod amm_flags {
    // No lsf* flags are defined for AMM in LedgerFormats.h.
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct AMM {
    pub(crate) slot_num: i32,
}

impl AMM {
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }
}

impl LedgerObjectCommonFields for AMM {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl AMMFields for AMM {}

/// Trait providing access to fields specific to MPTokenIssuance objects in any ledger.
pub trait MPTokenIssuanceFields: LedgerObjectCommonFields {
    /// The Issuer field (Required).
    fn get_issuer(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Issuer)
    }

    /// The Sequence field (Required).
    fn get_sequence(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::Sequence)
    }

    /// The TransferFee field (Optional).
    fn get_transfer_fee(&self) -> Result<Option<u16>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::TransferFee)
    }

    /// A hint indicating which page of the owner's directory links to this entry, in case the
    /// directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::OwnerNode)
    }

    /// The AssetScale field (Optional).
    fn get_asset_scale(&self) -> Result<Option<u8>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::AssetScale)
    }

    /// The MaximumAmount field (Optional).
    fn get_maximum_amount(&self) -> Result<Option<u64>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::MaximumAmount)
    }

    /// The OutstandingAmount field (Required).
    fn get_outstanding_amount(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::OutstandingAmount)
    }

    /// The LockedAmount field (Optional).
    fn get_locked_amount(&self) -> Result<Option<u64>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::LockedAmount)
    }

    /// The MPTokenMetadata field (Optional).
    fn get_mp_token_metadata(&self) -> Result<Option<StandardBlob>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::MPTokenMetadata)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
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
    /// The Issuer field (Required).
    fn get_issuer(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Issuer)
    }

    /// The Sequence field (Required).
    fn get_sequence(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::Sequence)
    }

    /// The TransferFee field (Optional).
    fn get_transfer_fee(&self) -> Result<Option<u16>> {
        current_ledger_object::get_field_optional(sfield::TransferFee)
    }

    /// A hint indicating which page of the owner's directory links to this entry, in case the
    /// directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::OwnerNode)
    }

    /// The AssetScale field (Optional).
    fn get_asset_scale(&self) -> Result<Option<u8>> {
        current_ledger_object::get_field_optional(sfield::AssetScale)
    }

    /// The MaximumAmount field (Optional).
    fn get_maximum_amount(&self) -> Result<Option<u64>> {
        current_ledger_object::get_field_optional(sfield::MaximumAmount)
    }

    /// The OutstandingAmount field (Required).
    fn get_outstanding_amount(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::OutstandingAmount)
    }

    /// The LockedAmount field (Optional).
    fn get_locked_amount(&self) -> Result<Option<u64>> {
        current_ledger_object::get_field_optional(sfield::LockedAmount)
    }

    /// The MPTokenMetadata field (Optional).
    fn get_mp_token_metadata(&self) -> Result<Option<StandardBlob>> {
        current_ledger_object::get_field_optional(sfield::MPTokenMetadata)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
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

/// `lsf*` flag constants for MPTokenIssuance objects.
#[allow(non_upper_case_globals, dead_code)]
pub mod mp_token_issuance_flags {
    pub const lsfMPTLocked: u32 = 0x00000001;
    pub const lsfMPTCanLock: u32 = 0x00000002;
    pub const lsfMPTRequireAuth: u32 = 0x00000004;
    pub const lsfMPTCanEscrow: u32 = 0x00000008;
    pub const lsfMPTCanTrade: u32 = 0x00000010;
    pub const lsfMPTCanTransfer: u32 = 0x00000020;
    pub const lsfMPTCanClawback: u32 = 0x00000040;
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct MPTokenIssuance {
    pub(crate) slot_num: i32,
}

impl MPTokenIssuance {
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

/// Trait providing access to fields specific to MPToken objects in any ledger.
pub trait MPTokenFields: LedgerObjectCommonFields {
    /// The Account field (Required).
    fn get_account(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Account)
    }

    /// The MPTokenIssuanceID field (Required).
    fn get_mp_token_issuance_id(&self) -> Result<Hash192> {
        ledger_object::get_field(self.get_slot_num(), sfield::MPTokenIssuanceID)
    }

    /// The MPTAmount field (Optional).
    fn get_mpt_amount(&self) -> Result<Option<u64>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::MPTAmount)
    }

    /// The LockedAmount field (Optional).
    fn get_locked_amount(&self) -> Result<Option<u64>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::LockedAmount)
    }

    /// A hint indicating which page of the owner's directory links to this entry, in case the
    /// directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::OwnerNode)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }
}

/// Trait providing access to fields specific to the current MPToken object.
pub trait CurrentMPTokenFields: CurrentLedgerObjectCommonFields {
    /// The Account field (Required).
    fn get_account(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Account)
    }

    /// The MPTokenIssuanceID field (Required).
    fn get_mp_token_issuance_id(&self) -> Result<Hash192> {
        current_ledger_object::get_field(sfield::MPTokenIssuanceID)
    }

    /// The MPTAmount field (Optional).
    fn get_mpt_amount(&self) -> Result<Option<u64>> {
        current_ledger_object::get_field_optional(sfield::MPTAmount)
    }

    /// The LockedAmount field (Optional).
    fn get_locked_amount(&self) -> Result<Option<u64>> {
        current_ledger_object::get_field_optional(sfield::LockedAmount)
    }

    /// A hint indicating which page of the owner's directory links to this entry, in case the
    /// directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::OwnerNode)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::PreviousTxnLgrSeq)
    }
}

/// `lsf*` flag constants for MPToken objects.
#[allow(non_upper_case_globals, dead_code)]
pub mod mp_token_flags {
    pub const lsfMPTLocked: u32 = 0x00000001;
    pub const lsfMPTAuthorized: u32 = 0x00000002;
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct MPToken {
    pub(crate) slot_num: i32,
}

impl MPToken {
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

/// Trait providing access to fields specific to Oracle objects in any ledger.
pub trait OracleFields: LedgerObjectCommonFields {
    /// The Owner field (Required).
    fn get_owner(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Owner)
    }

    /// The OracleDocumentID field (Optional).
    fn get_oracle_document_id(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::OracleDocumentID)
    }

    /// The Provider field (Required).
    fn get_provider(&self) -> Result<StandardBlob> {
        ledger_object::get_field(self.get_slot_num(), sfield::Provider)
    }

    /// The PriceDataSeries field (Required).
    fn get_price_data_series(&self) -> Result<Array> {
        ledger_object::get_field(self.get_slot_num(), sfield::PriceDataSeries)
    }

    /// The AssetClass field (Required).
    fn get_asset_class(&self) -> Result<StandardBlob> {
        ledger_object::get_field(self.get_slot_num(), sfield::AssetClass)
    }

    /// The LastUpdateTime field (Required).
    fn get_last_update_time(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::LastUpdateTime)
    }

    /// The URI field (Optional).
    fn get_uri(&self) -> Result<Option<UriBlob>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::URI)
    }

    /// A hint indicating which page of the owner's directory links to this entry, in case the
    /// directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::OwnerNode)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }
}

/// Trait providing access to fields specific to the current Oracle object.
pub trait CurrentOracleFields: CurrentLedgerObjectCommonFields {
    /// The Owner field (Required).
    fn get_owner(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Owner)
    }

    /// The OracleDocumentID field (Optional).
    fn get_oracle_document_id(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::OracleDocumentID)
    }

    /// The Provider field (Required).
    fn get_provider(&self) -> Result<StandardBlob> {
        current_ledger_object::get_field(sfield::Provider)
    }

    /// The PriceDataSeries field (Required).
    fn get_price_data_series(&self) -> Result<Array> {
        current_ledger_object::get_field(sfield::PriceDataSeries)
    }

    /// The AssetClass field (Required).
    fn get_asset_class(&self) -> Result<StandardBlob> {
        current_ledger_object::get_field(sfield::AssetClass)
    }

    /// The LastUpdateTime field (Required).
    fn get_last_update_time(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::LastUpdateTime)
    }

    /// The URI field (Optional).
    fn get_uri(&self) -> Result<Option<UriBlob>> {
        current_ledger_object::get_field_optional(sfield::URI)
    }

    /// A hint indicating which page of the owner's directory links to this entry, in case the
    /// directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::OwnerNode)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::PreviousTxnLgrSeq)
    }
}

/// `lsf*` flag constants for Oracle objects.
#[allow(non_upper_case_globals, dead_code)]
pub mod oracle_flags {
    // No lsf* flags are defined for Oracle in LedgerFormats.h.
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Oracle {
    pub(crate) slot_num: i32,
}

impl Oracle {
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

/// Trait providing access to fields specific to Credential objects in any ledger.
pub trait CredentialFields: LedgerObjectCommonFields {
    /// The Subject field (Required).
    fn get_subject(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Subject)
    }

    /// The Issuer field (Required).
    fn get_issuer(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Issuer)
    }

    /// The CredentialType field (Required).
    fn get_credential_type(&self) -> Result<StandardBlob> {
        ledger_object::get_field(self.get_slot_num(), sfield::CredentialType)
    }

    /// The Expiration field (Optional).
    fn get_expiration(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::Expiration)
    }

    /// The URI field (Optional).
    fn get_uri(&self) -> Result<Option<UriBlob>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::URI)
    }

    /// The IssuerNode field (Required).
    fn get_issuer_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::IssuerNode)
    }

    /// The SubjectNode field (Optional).
    fn get_subject_node(&self) -> Result<Option<u64>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::SubjectNode)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }
}

/// Trait providing access to fields specific to the current Credential object.
pub trait CurrentCredentialFields: CurrentLedgerObjectCommonFields {
    /// The Subject field (Required).
    fn get_subject(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Subject)
    }

    /// The Issuer field (Required).
    fn get_issuer(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Issuer)
    }

    /// The CredentialType field (Required).
    fn get_credential_type(&self) -> Result<StandardBlob> {
        current_ledger_object::get_field(sfield::CredentialType)
    }

    /// The Expiration field (Optional).
    fn get_expiration(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::Expiration)
    }

    /// The URI field (Optional).
    fn get_uri(&self) -> Result<Option<UriBlob>> {
        current_ledger_object::get_field_optional(sfield::URI)
    }

    /// The IssuerNode field (Required).
    fn get_issuer_node(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::IssuerNode)
    }

    /// The SubjectNode field (Optional).
    fn get_subject_node(&self) -> Result<Option<u64>> {
        current_ledger_object::get_field_optional(sfield::SubjectNode)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::PreviousTxnLgrSeq)
    }
}

/// `lsf*` flag constants for Credential objects.
#[allow(non_upper_case_globals, dead_code)]
pub mod credential_flags {
    pub const lsfAccepted: u32 = 0x00010000;
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Credential {
    pub(crate) slot_num: i32,
}

impl Credential {
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

/// Trait providing access to fields specific to PermissionedDomain objects in any ledger.
pub trait PermissionedDomainFields: LedgerObjectCommonFields {
    /// The Owner field (Required).
    fn get_owner(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Owner)
    }

    /// The Sequence field (Required).
    fn get_sequence(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::Sequence)
    }

    /// The AcceptedCredentials field (Required).
    fn get_accepted_credentials(&self) -> Result<Array> {
        ledger_object::get_field(self.get_slot_num(), sfield::AcceptedCredentials)
    }

    /// A hint indicating which page of the owner's directory links to this entry, in case the
    /// directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::OwnerNode)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }
}

/// Trait providing access to fields specific to the current PermissionedDomain object.
pub trait CurrentPermissionedDomainFields: CurrentLedgerObjectCommonFields {
    /// The Owner field (Required).
    fn get_owner(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Owner)
    }

    /// The Sequence field (Required).
    fn get_sequence(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::Sequence)
    }

    /// The AcceptedCredentials field (Required).
    fn get_accepted_credentials(&self) -> Result<Array> {
        current_ledger_object::get_field(sfield::AcceptedCredentials)
    }

    /// A hint indicating which page of the owner's directory links to this entry, in case the
    /// directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::OwnerNode)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::PreviousTxnLgrSeq)
    }
}

/// `lsf*` flag constants for PermissionedDomain objects.
#[allow(non_upper_case_globals, dead_code)]
pub mod permissioned_domain_flags {
    // No lsf* flags are defined for PermissionedDomain in LedgerFormats.h.
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct PermissionedDomain {
    pub(crate) slot_num: i32,
}

impl PermissionedDomain {
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }
}

impl LedgerObjectCommonFields for PermissionedDomain {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl PermissionedDomainFields for PermissionedDomain {}

/// Trait providing access to fields specific to Delegate objects in any ledger.
pub trait DelegateFields: LedgerObjectCommonFields {
    /// The Account field (Required).
    fn get_account(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Account)
    }

    /// The Authorize field (Required).
    fn get_authorize(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Authorize)
    }

    /// The Permissions field (Required).
    fn get_permissions(&self) -> Result<Array> {
        ledger_object::get_field(self.get_slot_num(), sfield::Permissions)
    }

    /// A hint indicating which page of the owner's directory links to this entry, in case the
    /// directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::OwnerNode)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }
}

/// Trait providing access to fields specific to the current Delegate object.
pub trait CurrentDelegateFields: CurrentLedgerObjectCommonFields {
    /// The Account field (Required).
    fn get_account(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Account)
    }

    /// The Authorize field (Required).
    fn get_authorize(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Authorize)
    }

    /// The Permissions field (Required).
    fn get_permissions(&self) -> Result<Array> {
        current_ledger_object::get_field(sfield::Permissions)
    }

    /// A hint indicating which page of the owner's directory links to this entry, in case the
    /// directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::OwnerNode)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::PreviousTxnLgrSeq)
    }
}

/// `lsf*` flag constants for Delegate objects.
#[allow(non_upper_case_globals, dead_code)]
pub mod delegate_flags {
    // No lsf* flags are defined for Delegate in LedgerFormats.h.
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Delegate {
    pub(crate) slot_num: i32,
}

impl Delegate {
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }
}

impl LedgerObjectCommonFields for Delegate {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl DelegateFields for Delegate {}

/// Trait providing access to fields specific to Vault objects in any ledger.
pub trait VaultFields: LedgerObjectCommonFields {
    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }

    /// The Sequence field (Required).
    fn get_sequence(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::Sequence)
    }

    /// A hint indicating which page of the owner's directory links to this entry, in case the
    /// directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::OwnerNode)
    }

    /// The Owner field (Required).
    fn get_owner(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Owner)
    }

    /// The Account field (Required).
    fn get_account(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Account)
    }

    /// The Data field (Optional).
    fn get_data(&self) -> Result<Option<StandardBlob>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::Data)
    }

    /// The Asset field (Required).
    fn get_asset(&self) -> Result<Issue> {
        ledger_object::get_field(self.get_slot_num(), sfield::Asset)
    }

    // SKIPPED get_assets_total: NUMBER is not yet representable in Rust

    // SKIPPED get_assets_available: NUMBER is not yet representable in Rust

    // SKIPPED get_assets_maximum: NUMBER is not yet representable in Rust

    // SKIPPED get_loss_unrealized: NUMBER is not yet representable in Rust

    /// The ShareMPTID field (Required).
    fn get_share_mptid(&self) -> Result<Hash192> {
        ledger_object::get_field(self.get_slot_num(), sfield::ShareMPTID)
    }

    /// The WithdrawalPolicy field (Required).
    fn get_withdrawal_policy(&self) -> Result<u8> {
        ledger_object::get_field(self.get_slot_num(), sfield::WithdrawalPolicy)
    }

    /// The Scale field (Optional).
    fn get_scale(&self) -> Result<Option<u8>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::Scale)
    }
}

/// Trait providing access to fields specific to the current Vault object.
pub trait CurrentVaultFields: CurrentLedgerObjectCommonFields {
    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::PreviousTxnLgrSeq)
    }

    /// The Sequence field (Required).
    fn get_sequence(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::Sequence)
    }

    /// A hint indicating which page of the owner's directory links to this entry, in case the
    /// directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::OwnerNode)
    }

    /// The Owner field (Required).
    fn get_owner(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Owner)
    }

    /// The Account field (Required).
    fn get_account(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Account)
    }

    /// The Data field (Optional).
    fn get_data(&self) -> Result<Option<StandardBlob>> {
        current_ledger_object::get_field_optional(sfield::Data)
    }

    /// The Asset field (Required).
    fn get_asset(&self) -> Result<Issue> {
        current_ledger_object::get_field(sfield::Asset)
    }

    // SKIPPED get_assets_total: NUMBER is not yet representable in Rust

    // SKIPPED get_assets_available: NUMBER is not yet representable in Rust

    // SKIPPED get_assets_maximum: NUMBER is not yet representable in Rust

    // SKIPPED get_loss_unrealized: NUMBER is not yet representable in Rust

    /// The ShareMPTID field (Required).
    fn get_share_mptid(&self) -> Result<Hash192> {
        current_ledger_object::get_field(sfield::ShareMPTID)
    }

    /// The WithdrawalPolicy field (Required).
    fn get_withdrawal_policy(&self) -> Result<u8> {
        current_ledger_object::get_field(sfield::WithdrawalPolicy)
    }

    /// The Scale field (Optional).
    fn get_scale(&self) -> Result<Option<u8>> {
        current_ledger_object::get_field_optional(sfield::Scale)
    }
}

/// `lsf*` flag constants for Vault objects.
#[allow(non_upper_case_globals, dead_code)]
pub mod vault_flags {
    pub const lsfVaultPrivate: u32 = 0x00010000;
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Vault {
    pub(crate) slot_num: i32,
}

impl Vault {
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

/// Trait providing access to fields specific to LoanBroker objects in any ledger.
pub trait LoanBrokerFields: LedgerObjectCommonFields {
    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }

    /// The Sequence field (Required).
    fn get_sequence(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::Sequence)
    }

    /// A hint indicating which page of the owner's directory links to this entry, in case the
    /// directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::OwnerNode)
    }

    /// The VaultNode field (Required).
    fn get_vault_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::VaultNode)
    }

    /// The VaultID field (Required).
    fn get_vault_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::VaultID)
    }

    /// The Account field (Required).
    fn get_account(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Account)
    }

    /// The Owner field (Required).
    fn get_owner(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Owner)
    }

    /// The LoanSequence field (Required).
    fn get_loan_sequence(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::LoanSequence)
    }

    /// The Data field (Optional).
    fn get_data(&self) -> Result<Option<StandardBlob>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::Data)
    }

    /// The ManagementFeeRate field (Optional).
    fn get_management_fee_rate(&self) -> Result<Option<u16>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::ManagementFeeRate)
    }

    /// The OwnerCount field (Optional).
    fn get_owner_count(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::OwnerCount)
    }

    // SKIPPED get_debt_total: NUMBER is not yet representable in Rust

    // SKIPPED get_debt_maximum: NUMBER is not yet representable in Rust

    // SKIPPED get_cover_available: NUMBER is not yet representable in Rust

    /// The CoverRateMinimum field (Optional).
    fn get_cover_rate_minimum(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::CoverRateMinimum)
    }

    /// The CoverRateLiquidation field (Optional).
    fn get_cover_rate_liquidation(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::CoverRateLiquidation)
    }
}

/// Trait providing access to fields specific to the current LoanBroker object.
pub trait CurrentLoanBrokerFields: CurrentLedgerObjectCommonFields {
    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::PreviousTxnLgrSeq)
    }

    /// The Sequence field (Required).
    fn get_sequence(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::Sequence)
    }

    /// A hint indicating which page of the owner's directory links to this entry, in case the
    /// directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::OwnerNode)
    }

    /// The VaultNode field (Required).
    fn get_vault_node(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::VaultNode)
    }

    /// The VaultID field (Required).
    fn get_vault_id(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::VaultID)
    }

    /// The Account field (Required).
    fn get_account(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Account)
    }

    /// The Owner field (Required).
    fn get_owner(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Owner)
    }

    /// The LoanSequence field (Required).
    fn get_loan_sequence(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::LoanSequence)
    }

    /// The Data field (Optional).
    fn get_data(&self) -> Result<Option<StandardBlob>> {
        current_ledger_object::get_field_optional(sfield::Data)
    }

    /// The ManagementFeeRate field (Optional).
    fn get_management_fee_rate(&self) -> Result<Option<u16>> {
        current_ledger_object::get_field_optional(sfield::ManagementFeeRate)
    }

    /// The OwnerCount field (Optional).
    fn get_owner_count(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::OwnerCount)
    }

    // SKIPPED get_debt_total: NUMBER is not yet representable in Rust

    // SKIPPED get_debt_maximum: NUMBER is not yet representable in Rust

    // SKIPPED get_cover_available: NUMBER is not yet representable in Rust

    /// The CoverRateMinimum field (Optional).
    fn get_cover_rate_minimum(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::CoverRateMinimum)
    }

    /// The CoverRateLiquidation field (Optional).
    fn get_cover_rate_liquidation(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::CoverRateLiquidation)
    }
}

/// `lsf*` flag constants for LoanBroker objects.
#[allow(non_upper_case_globals, dead_code)]
pub mod loan_broker_flags {
    // No lsf* flags are defined for LoanBroker in LedgerFormats.h.
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct LoanBroker {
    pub(crate) slot_num: i32,
}

impl LoanBroker {
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }
}

impl LedgerObjectCommonFields for LoanBroker {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl LoanBrokerFields for LoanBroker {}

/// Trait providing access to fields specific to Loan objects in any ledger.
pub trait LoanFields: LedgerObjectCommonFields {
    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }

    /// A hint indicating which page of the owner's directory links to this entry, in case the
    /// directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::OwnerNode)
    }

    /// The LoanBrokerNode field (Required).
    fn get_loan_broker_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::LoanBrokerNode)
    }

    /// The LoanBrokerID field (Required).
    fn get_loan_broker_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::LoanBrokerID)
    }

    /// The LoanSequence field (Required).
    fn get_loan_sequence(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::LoanSequence)
    }

    /// The Borrower field (Required).
    fn get_borrower(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Borrower)
    }

    // SKIPPED get_loan_origination_fee: NUMBER is not yet representable in Rust

    // SKIPPED get_loan_service_fee: NUMBER is not yet representable in Rust

    // SKIPPED get_late_payment_fee: NUMBER is not yet representable in Rust

    // SKIPPED get_close_payment_fee: NUMBER is not yet representable in Rust

    /// The OverpaymentFee field (Optional).
    fn get_overpayment_fee(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::OverpaymentFee)
    }

    /// The InterestRate field (Optional).
    fn get_interest_rate(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::InterestRate)
    }

    /// The LateInterestRate field (Optional).
    fn get_late_interest_rate(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::LateInterestRate)
    }

    /// The CloseInterestRate field (Optional).
    fn get_close_interest_rate(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::CloseInterestRate)
    }

    /// The OverpaymentInterestRate field (Optional).
    fn get_overpayment_interest_rate(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::OverpaymentInterestRate)
    }

    /// The StartDate field (Required).
    fn get_start_date(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::StartDate)
    }

    /// The PaymentInterval field (Required).
    fn get_payment_interval(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::PaymentInterval)
    }

    /// The GracePeriod field (Optional).
    fn get_grace_period(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::GracePeriod)
    }

    /// The PreviousPaymentDueDate field (Optional).
    fn get_previous_payment_due_date(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::PreviousPaymentDueDate)
    }

    /// The NextPaymentDueDate field (Optional).
    fn get_next_payment_due_date(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::NextPaymentDueDate)
    }

    /// The PaymentRemaining field (Optional).
    fn get_payment_remaining(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::PaymentRemaining)
    }

    // SKIPPED get_periodic_payment: NUMBER is not yet representable in Rust

    // SKIPPED get_principal_outstanding: NUMBER is not yet representable in Rust

    // SKIPPED get_total_value_outstanding: NUMBER is not yet representable in Rust

    // SKIPPED get_management_fee_outstanding: NUMBER is not yet representable in Rust

    // SKIPPED get_loan_scale: INT32 is not yet representable in Rust
}

/// Trait providing access to fields specific to the current Loan object.
pub trait CurrentLoanFields: CurrentLedgerObjectCommonFields {
    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::PreviousTxnLgrSeq)
    }

    /// A hint indicating which page of the owner's directory links to this entry, in case the
    /// directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::OwnerNode)
    }

    /// The LoanBrokerNode field (Required).
    fn get_loan_broker_node(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::LoanBrokerNode)
    }

    /// The LoanBrokerID field (Required).
    fn get_loan_broker_id(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::LoanBrokerID)
    }

    /// The LoanSequence field (Required).
    fn get_loan_sequence(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::LoanSequence)
    }

    /// The Borrower field (Required).
    fn get_borrower(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Borrower)
    }

    // SKIPPED get_loan_origination_fee: NUMBER is not yet representable in Rust

    // SKIPPED get_loan_service_fee: NUMBER is not yet representable in Rust

    // SKIPPED get_late_payment_fee: NUMBER is not yet representable in Rust

    // SKIPPED get_close_payment_fee: NUMBER is not yet representable in Rust

    /// The OverpaymentFee field (Optional).
    fn get_overpayment_fee(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::OverpaymentFee)
    }

    /// The InterestRate field (Optional).
    fn get_interest_rate(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::InterestRate)
    }

    /// The LateInterestRate field (Optional).
    fn get_late_interest_rate(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::LateInterestRate)
    }

    /// The CloseInterestRate field (Optional).
    fn get_close_interest_rate(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::CloseInterestRate)
    }

    /// The OverpaymentInterestRate field (Optional).
    fn get_overpayment_interest_rate(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::OverpaymentInterestRate)
    }

    /// The StartDate field (Required).
    fn get_start_date(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::StartDate)
    }

    /// The PaymentInterval field (Required).
    fn get_payment_interval(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::PaymentInterval)
    }

    /// The GracePeriod field (Optional).
    fn get_grace_period(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::GracePeriod)
    }

    /// The PreviousPaymentDueDate field (Optional).
    fn get_previous_payment_due_date(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::PreviousPaymentDueDate)
    }

    /// The NextPaymentDueDate field (Optional).
    fn get_next_payment_due_date(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::NextPaymentDueDate)
    }

    /// The PaymentRemaining field (Optional).
    fn get_payment_remaining(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::PaymentRemaining)
    }

    // SKIPPED get_periodic_payment: NUMBER is not yet representable in Rust

    // SKIPPED get_principal_outstanding: NUMBER is not yet representable in Rust

    // SKIPPED get_total_value_outstanding: NUMBER is not yet representable in Rust

    // SKIPPED get_management_fee_outstanding: NUMBER is not yet representable in Rust

    // SKIPPED get_loan_scale: INT32 is not yet representable in Rust
}

/// `lsf*` flag constants for Loan objects.
#[allow(non_upper_case_globals, dead_code)]
pub mod loan_flags {
    pub const lsfLoanDefault: u32 = 0x00010000;
    pub const lsfLoanImpaired: u32 = 0x00020000;
    pub const lsfLoanOverpayment: u32 = 0x00040000;
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Loan {
    pub(crate) slot_num: i32,
}

impl Loan {
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }
}

impl LedgerObjectCommonFields for Loan {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl LoanFields for Loan {}

/// Trait providing access to fields specific to ContractSource objects in any ledger.
pub trait ContractSourceFields: LedgerObjectCommonFields {
    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }

    /// The ContractHash field (Required).
    fn get_contract_hash(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::ContractHash)
    }

    /// The ContractCode field (Required).
    fn get_contract_code(&self) -> Result<StandardBlob> {
        ledger_object::get_field(self.get_slot_num(), sfield::ContractCode)
    }

    /// The Functions field (Required).
    fn get_functions(&self) -> Result<Array> {
        ledger_object::get_field(self.get_slot_num(), sfield::Functions)
    }

    /// The InstanceParameters field (Optional).
    fn get_instance_parameters(&self) -> Result<Option<Array>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::InstanceParameters)
    }

    /// The ReferenceCount field (Required).
    fn get_reference_count(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::ReferenceCount)
    }
}

/// Trait providing access to fields specific to the current ContractSource object.
pub trait CurrentContractSourceFields: CurrentLedgerObjectCommonFields {
    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::PreviousTxnLgrSeq)
    }

    /// The ContractHash field (Required).
    fn get_contract_hash(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::ContractHash)
    }

    /// The ContractCode field (Required).
    fn get_contract_code(&self) -> Result<StandardBlob> {
        current_ledger_object::get_field(sfield::ContractCode)
    }

    /// The Functions field (Required).
    fn get_functions(&self) -> Result<Array> {
        current_ledger_object::get_field(sfield::Functions)
    }

    /// The InstanceParameters field (Optional).
    fn get_instance_parameters(&self) -> Result<Option<Array>> {
        current_ledger_object::get_field_optional(sfield::InstanceParameters)
    }

    /// The ReferenceCount field (Required).
    fn get_reference_count(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::ReferenceCount)
    }
}

/// `lsf*` flag constants for ContractSource objects.
#[allow(non_upper_case_globals, dead_code)]
pub mod contract_source_flags {
    // No lsf* flags are defined for ContractSource in LedgerFormats.h.
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ContractSource {
    pub(crate) slot_num: i32,
}

impl ContractSource {
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }
}

impl LedgerObjectCommonFields for ContractSource {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl ContractSourceFields for ContractSource {}

/// Trait providing access to fields specific to Contract objects in any ledger.
pub trait ContractFields: LedgerObjectCommonFields {
    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }

    /// The Sequence field (Required).
    fn get_sequence(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::Sequence)
    }

    /// A hint indicating which page of the owner's directory links to this entry, in case the
    /// directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::OwnerNode)
    }

    /// The Owner field (Required).
    fn get_owner(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Owner)
    }

    /// The ContractAccount field (Required).
    fn get_contract_account(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::ContractAccount)
    }

    /// The ContractHash field (Required).
    fn get_contract_hash(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::ContractHash)
    }

    /// The InstanceParameterValues field (Optional).
    fn get_instance_parameter_values(&self) -> Result<Option<Array>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::InstanceParameterValues)
    }

    /// The URI field (Optional).
    fn get_uri(&self) -> Result<Option<UriBlob>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::URI)
    }
}

/// Trait providing access to fields specific to the current Contract object.
pub trait CurrentContractFields: CurrentLedgerObjectCommonFields {
    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::PreviousTxnLgrSeq)
    }

    /// The Sequence field (Required).
    fn get_sequence(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::Sequence)
    }

    /// A hint indicating which page of the owner's directory links to this entry, in case the
    /// directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::OwnerNode)
    }

    /// The Owner field (Required).
    fn get_owner(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Owner)
    }

    /// The ContractAccount field (Required).
    fn get_contract_account(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::ContractAccount)
    }

    /// The ContractHash field (Required).
    fn get_contract_hash(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::ContractHash)
    }

    /// The InstanceParameterValues field (Optional).
    fn get_instance_parameter_values(&self) -> Result<Option<Array>> {
        current_ledger_object::get_field_optional(sfield::InstanceParameterValues)
    }

    /// The URI field (Optional).
    fn get_uri(&self) -> Result<Option<UriBlob>> {
        current_ledger_object::get_field_optional(sfield::URI)
    }
}

/// `lsf*` flag constants for Contract objects.
#[allow(non_upper_case_globals, dead_code)]
pub mod contract_flags {
    // No lsf* flags are defined for Contract in LedgerFormats.h.
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Contract {
    pub(crate) slot_num: i32,
}

impl Contract {
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }
}

impl LedgerObjectCommonFields for Contract {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl ContractFields for Contract {}

/// Trait providing access to fields specific to ContractData objects in any ledger.
pub trait ContractDataFields: LedgerObjectCommonFields {
    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }

    /// A hint indicating which page of the owner's directory links to this entry, in case the
    /// directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::OwnerNode)
    }

    /// The Owner field (Required).
    fn get_owner(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Owner)
    }

    /// The ContractAccount field (Required).
    fn get_contract_account(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::ContractAccount)
    }

    // SKIPPED get_contract_json: JSON is not yet representable in Rust
}

/// Trait providing access to fields specific to the current ContractData object.
pub trait CurrentContractDataFields: CurrentLedgerObjectCommonFields {
    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::PreviousTxnLgrSeq)
    }

    /// A hint indicating which page of the owner's directory links to this entry, in case the
    /// directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::OwnerNode)
    }

    /// The Owner field (Required).
    fn get_owner(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Owner)
    }

    /// The ContractAccount field (Required).
    fn get_contract_account(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::ContractAccount)
    }

    // SKIPPED get_contract_json: JSON is not yet representable in Rust
}

/// `lsf*` flag constants for ContractData objects.
#[allow(non_upper_case_globals, dead_code)]
pub mod contract_data_flags {
    // No lsf* flags are defined for ContractData in LedgerFormats.h.
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ContractData {
    pub(crate) slot_num: i32,
}

impl ContractData {
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }
}

impl LedgerObjectCommonFields for ContractData {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl ContractDataFields for ContractData {}
