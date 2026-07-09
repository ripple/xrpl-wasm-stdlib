//! `TxOnly` only implements `FromCurrentTx`, so passing it to a function that
//! requires `FromLedger` must fail to compile.

use xrpl_common_stdlib::fields::decoder::{FieldDecoder, FromCurrentTx, FromLedger};
use xrpl_common_stdlib::types::decode_error::DecodeError;

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

fn requires_from_ledger<T: FromLedger>() {}

fn main() {
    requires_from_ledger::<TxOnly>();
}
