//! Object SFields (e.g. `Memo`) are marker-less placeholders: `Object` implements neither
//! `FromCurrentTx` nor `FromLedger`, so passing one to a current-ledger-object getter must fail
//! to compile rather than panic at runtime. Navigate objects with `Locator` instead.

use xrpl_common_stdlib::objects::current_ledger_object;
use xrpl_common_stdlib::sfield;

fn main() {
    let _ = current_ledger_object::get_field(sfield::Memo);
}
