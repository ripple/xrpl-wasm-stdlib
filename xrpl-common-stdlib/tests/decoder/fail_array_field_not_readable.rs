//! Array SFields (e.g. `Signers`) are marker-less placeholders: `Array` implements neither
//! `FromCurrentTx` nor `FromLedger`, so passing one to a ledger-object getter must fail to
//! compile rather than panic at runtime. Navigate arrays with `Locator` instead.

use xrpl_common_stdlib::objects::ledger_object;
use xrpl_common_stdlib::sfield;

fn main() {
    let _ = ledger_object::get_field(0, sfield::Signers);
}
