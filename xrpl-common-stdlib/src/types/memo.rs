//! `Memo` — one entry of a transaction's `Memos` array.
//!
//! Read it through
//! [`TransactionCommonFields::get_memo`](crate::current_tx::traits::TransactionCommonFields::get_memo).
//! The host path is `Memos -> i -> MemoType|MemoData|MemoFormat`: the `Memo` wrapper object seen
//! in JSON is not a path segment. The accessor encodes this, so callers never spell the path out.

use crate::types::blob::StandardBlob;

/// One entry of a transaction's `Memos` array.
///
/// Every field is optional in the protocol, so each is an `Option`. A field that is present but
/// empty decodes as `Some(blob)` with `blob.len == 0` — only a missing field is `None`. Callers
/// decide whether an empty value is acceptable for their contract.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Memo {
    /// `MemoType`: conventionally an RFC 3986 URI-style hint about what `memo_data` holds.
    pub memo_type: Option<StandardBlob>,
    /// `MemoData`: the memo payload.
    pub memo_data: Option<StandardBlob>,
    /// `MemoFormat`: conventionally a MIME type describing how `memo_data` is encoded.
    pub memo_format: Option<StandardBlob>,
}
