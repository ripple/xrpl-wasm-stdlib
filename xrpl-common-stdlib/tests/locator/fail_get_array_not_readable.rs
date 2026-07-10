//! The terminal `.get::<T>()` of the transaction path builder requires `T: FromCurrentTx`. `Array`
//! is an aggregate placeholder that implements no decoder marker, so asking the builder to decode
//! one whole is a compile error — you navigate *into* an array with `.field(...).index(...)`, you
//! don't read it back as a value.

use xrpl_common_stdlib::fields::current_tx::traits::TransactionCommonFields;
use xrpl_common_stdlib::objects::array::Array;
use xrpl_common_stdlib::sfield;

struct Tx;
impl TransactionCommonFields for Tx {}

fn main() {
    let _ = Tx.path().field(sfield::Memos).get::<Array>();
}
