use crate::core::keylets::account_keylet;
use crate::core::ledger_objects::traits::{AccountFields, LedgerObjectCommonFields};
use crate::core::types::account_id::AccountID;
use crate::core::types::amount::Amount;
use crate::host;
use host::Error;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub struct AccountRoot {
    pub slot_num: i32,
}

impl LedgerObjectCommonFields for AccountRoot {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl AccountFields for AccountRoot {}

pub fn get_account_balance(account_id: &AccountID) -> host::Result<Option<Amount>> {
    // Construct the account keylet. This calls a host function, so propagate the error via `?`
    let account_keylet = match account_keylet(account_id) {
        host::Result::Ok(keylet) => keylet,
        host::Result::Err(e) => return host::Result::Err(e),
    };

    // Try to cache the ledger object inside rippled
    let slot = unsafe { host::cache_ledger_obj(account_keylet.as_ptr(), account_keylet.len(), 0) };
    if slot < 0 {
        return host::Result::Err(Error::from_code(slot));
    }

    // Get the balance.
    // We use the trait-bound implementation so as not to duplicate accessor logic.
    let account = AccountRoot { slot_num: slot };
    account.balance()
}
