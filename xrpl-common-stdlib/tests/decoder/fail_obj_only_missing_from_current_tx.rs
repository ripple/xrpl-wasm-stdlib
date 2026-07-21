//! `ObjOnly` only implements `FromLedger`, so passing it to a function that
//! requires `FromCurrentTx` must fail to compile.

use xrpl_common_stdlib::fields::decoder::{FieldDecoder, FromCurrentTx, FromLedger};
use xrpl_common_stdlib::host::Error;

struct ObjOnly;

impl FieldDecoder for ObjOnly {
    type Buffer = [u8; 1];

    fn decode(_bytes: &[u8]) -> Result<Self, Error> {
        Ok(ObjOnly)
    }
}

impl FromLedger for ObjOnly {}

fn requires_from_current_tx<T: FromCurrentTx>() {}

fn main() {
    requires_from_current_tx::<ObjOnly>();
}
