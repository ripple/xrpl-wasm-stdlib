//! The terminal `.get::<T>()` of the ledger path builder requires `T: FromLedger`. `Array` is an
//! aggregate placeholder that implements no decoder marker, so asking the builder to decode one
//! whole is a compile error — you navigate *into* an array with `.field(...).index(...)`, you don't
//! read it back as a value.

use xrpl_common_stdlib::objects::array_object::Array;
use xrpl_common_stdlib::objects::traits::LedgerObjectCommonFields;
use xrpl_common_stdlib::sfield;

struct Obj;
impl LedgerObjectCommonFields for Obj {
    fn get_slot_num(&self) -> i32 {
        0
    }
}

fn main() {
    let _ = Obj.path().field(sfield::SignerEntries).get::<Array>();
}
