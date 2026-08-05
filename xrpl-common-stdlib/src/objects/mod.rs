//! Ledger-object field access.
//!
//! Reading a field from a ledger object is the same two steps as reading one from the current
//! transaction: call a host function into a buffer, then decode the bytes into a typed value.
//! That decode step is captured once per type by [`crate::fields::decoder::FieldDecoder`], and a
//! type opts into being *readable from a ledger object* via the
//! [`crate::fields::decoder::FromLedger`] marker.
//!
//! Two entry points, mirroring the transaction side:
//!
//! - [`ledger_object`] — read from a ledger object cached into a slot (via `cache_ledger_obj`).
//! - [`current_ledger_object`] — read from the current ledger object, without a slot.
//!
//! Both are the generic `FieldDecoder`-based accessors re-exported from [`crate::fields`]; see
//! that module for the decode machinery and the `Blob<N>` zero-copy accessors.
//!
//! ```rust,no_run
//! use xrpl_common_stdlib::objects::{ledger_object, current_ledger_object};
//! use xrpl_common_stdlib::sfield;
//!
//! fn example() {
//!   let slot = 0;
//!   // Get a required field from a specific (slot-cached) ledger object
//!   let balance = ledger_object::get_field(slot, sfield::Balance).unwrap();
//!   let account = ledger_object::get_field(slot, sfield::Account).unwrap();
//!
//!   // Get an optional field from the current ledger object
//!   let flags = current_ledger_object::get_field_optional(sfield::Flags).unwrap();
//! }
//! ```

pub mod any_object;
pub mod array_object;
// Crate-internal: the flat `pub use generated::{...}` below is the public path
// to each type (`objects::AccountRoot`), not `objects::generated::...`.
pub(crate) mod generated;
#[cfg(all(any(test, feature = "test-host-bindings"), not(target_arch = "wasm32")))]
pub mod test_utils;
pub mod traits;

/// Typed field accessors for the current ledger object (no slot). See
/// [`crate::fields::current_ledger_obj`].
pub use crate::fields::current_ledger_obj as current_ledger_object;
/// Typed field accessors for a ledger object cached into a slot. See [`crate::fields::ledger_obj`].
pub use crate::fields::ledger_obj as ledger_object;
/// Untyped handle to a slot-cached ledger object, for object types with no typed wrapper.
pub use any_object::LedgerObject;

pub use generated::{
    AMM, AMMFields, AccountRoot, AccountRootFields, Amendments, AmendmentsFields, Bridge,
    BridgeFields, Check, CheckFields, Credential, CredentialFields, CurrentAMMFields,
    CurrentAccountRootFields, CurrentAmendmentsFields, CurrentBridgeFields, CurrentCheckFields,
    CurrentCredentialFields, CurrentDIDFields, CurrentDelegateFields, CurrentDepositPreauthFields,
    CurrentDirectoryNodeFields, CurrentFeeSettingsFields, CurrentLedgerHashesFields,
    CurrentLoanBrokerFields, CurrentLoanFields, CurrentMPTokenFields, CurrentMPTokenIssuanceFields,
    CurrentNFTokenOfferFields, CurrentNFTokenPageFields, CurrentNegativeUNLFields,
    CurrentOfferFields, CurrentOracleFields, CurrentPayChannelFields,
    CurrentPermissionedDomainFields, CurrentRippleStateFields, CurrentSignerListFields,
    CurrentTicketFields, CurrentVaultFields, CurrentXChainOwnedClaimIDFields,
    CurrentXChainOwnedCreateAccountClaimIDFields, DID, DIDFields, Delegate, DelegateFields,
    DepositPreauth, DepositPreauthFields, DirectoryNode, DirectoryNodeFields, Escrow, EscrowFields,
    FeeSettings, FeeSettingsFields, LedgerHashes, LedgerHashesFields, Loan, LoanBroker,
    LoanBrokerFields, LoanFields, MPToken, MPTokenFields, MPTokenIssuance, MPTokenIssuanceFields,
    NFTokenOffer, NFTokenOfferFields, NFTokenPage, NFTokenPageFields, NegativeUNL,
    NegativeUNLFields, Offer, OfferFields, Oracle, OracleFields, PayChannel, PayChannelFields,
    PermissionedDomain, PermissionedDomainFields, RippleState, RippleStateFields, SignerList,
    SignerListFields, Ticket, TicketFields, Vault, VaultFields, XChainOwnedClaimID,
    XChainOwnedClaimIDFields, XChainOwnedCreateAccountClaimID,
    XChainOwnedCreateAccountClaimIDFields,
};
