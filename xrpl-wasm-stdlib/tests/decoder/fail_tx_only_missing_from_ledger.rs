//! `TxOnly` only implements `FromCurrentTx`, so passing it to a function that
//! requires `FromLedger` must fail to compile.

use xrpl_wasm_stdlib::fields::decoder::{FieldDecoder, FromCurrentTx, FromLedger};
use xrpl_wasm_stdlib::host::Error;

struct TxOnly;

impl FieldDecoder for TxOnly {
    type Buffer = [u8; 1];

    fn decode(_bytes: &[u8]) -> Result<Self, Error> {
        Ok(TxOnly)
    }
}

impl FromCurrentTx for TxOnly {}

fn requires_from_ledger<T: FromLedger>() {}

fn main() {
    requires_from_ledger::<TxOnly>();
}
