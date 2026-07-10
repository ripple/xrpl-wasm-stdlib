//! An object-typed SField (e.g. `sfield::Memo`, typed `SField<Object, _>`) cannot be passed to a
//! ledger-object getter: `Object` is a placeholder with no `FromLedger` impl, so reading it whole
//! is a compile error. Aggregate fields must be navigated with a locator instead.

use xrpl_common_stdlib::fields::ledger_obj;
use xrpl_common_stdlib::sfield;

fn main() {
    let _ = ledger_obj::get_field(0, sfield::Memo);
}
