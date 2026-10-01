//! `Signer` — one entry of a multi-signed transaction's `Signers` array.
//!
//! Read it through
//! [`TransactionCommonFields::get_signer`](crate::current_tx::traits::TransactionCommonFields::get_signer).
//! The host path is `Signers -> i -> Account|TxnSignature|SigningPubKey`: as with `Memos`, the
//! `Signer` wrapper object seen in JSON is not a path segment.

use crate::types::account_id::AccountID;
use crate::types::blob::SignatureBlob;
use crate::types::public_key::PublicKey;

/// One entry of a multi-signed transaction's `Signers` array.
///
/// rippled requires all three fields on every signer, so none is optional here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signer {
    /// `Account`: the address that produced this signature.
    pub account: AccountID,
    /// `TxnSignature`: this signer's signature over the transaction.
    pub txn_signature: SignatureBlob,
    /// `SigningPubKey`: the 33-byte public key that verifies `txn_signature`.
    pub signing_pub_key: PublicKey,
}
