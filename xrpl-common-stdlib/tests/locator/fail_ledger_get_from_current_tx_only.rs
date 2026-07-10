//! `LedgerPathBuilder::get` requires `FromLedger` specifically — not merely a `FieldDecoder`, and
//! not `FromCurrentTx`. A type that opts only into the transaction context must be rejected at a
//! ledger path builder's terminal, guarding against the bound being accidentally loosened.

use xrpl_common_stdlib::fields::decoder::{FieldDecoder, FromCurrentTx};
use xrpl_common_stdlib::objects::traits::LedgerObjectCommonFields;
use xrpl_common_stdlib::sfield;
use xrpl_common_stdlib::types::decode_error::DecodeError;

struct Obj;
impl LedgerObjectCommonFields for Obj {
    fn get_slot_num(&self) -> i32 {
        0
    }
}

struct TxOnly;

impl FieldDecoder for TxOnly {
    type Buffer = [u8; 1];

    fn empty_buffer() -> Self::Buffer {
        [0u8; 1]
    }

    fn decode(_bytes: &[u8]) -> Result<Self, DecodeError> {
        Ok(TxOnly)
    }
}

impl FromCurrentTx for TxOnly {}

fn main() {
    let _ = Obj.path().field(sfield::Flags).get::<TxOnly>();
}
