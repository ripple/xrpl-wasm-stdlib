//! `TxPathBuilder::get` requires `FromCurrentTx` specifically — not merely a `FieldDecoder`, and
//! not `FromLedger`. A type that opts only into the ledger-object context must be rejected at a
//! transaction path builder's terminal, guarding against the bound being accidentally loosened.

use xrpl_common_stdlib::fields::current_tx::traits::TransactionCommonFields;
use xrpl_common_stdlib::fields::decoder::{FieldDecoder, FromLedger};
use xrpl_common_stdlib::sfield;
use xrpl_common_stdlib::types::decode_error::DecodeError;

struct Tx;
impl TransactionCommonFields for Tx {}

struct LedgerOnly;

impl FieldDecoder for LedgerOnly {
    type Buffer = [u8; 1];

    fn empty_buffer() -> Self::Buffer {
        [0u8; 1]
    }

    fn decode(_bytes: &[u8]) -> Result<Self, DecodeError> {
        Ok(LedgerOnly)
    }
}

impl FromLedger for LedgerOnly {}

fn main() {
    let _ = Tx.path().field(sfield::Memos).get::<LedgerOnly>();
}
