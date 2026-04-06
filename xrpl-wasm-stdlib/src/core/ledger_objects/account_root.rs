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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::keylets::XRPL_KEYLET_SIZE;
    use crate::core::types::amount::AMOUNT_SIZE;
    use crate::host::error_codes::INTERNAL_ERROR;
    use crate::host::host_bindings_trait::MockHostBindings;
    use crate::host::setup_mock;
    use crate::sfield;
    use mockall::predicate::{always, eq};

    /// Mock account_keylet to write 0xCC bytes and return success.
    fn mock_account_keylet_success(mock: &mut MockHostBindings) {
        mock.expect_account_keylet()
            .times(1)
            .returning(|_, _, out_buff_ptr, out_buff_len| {
                assert_eq!(out_buff_len, XRPL_KEYLET_SIZE);
                unsafe {
                    for i in 0..XRPL_KEYLET_SIZE {
                        *out_buff_ptr.add(i) = 0xCC;
                    }
                }
                XRPL_KEYLET_SIZE as i32
            });
    }

    #[test]
    fn test_get_account_balance_success() {
        let mut mock = MockHostBindings::new();
        let slot = 5;
        let balance_field_code: i32 = sfield::Balance.into();

        mock_account_keylet_success(&mut mock);

        // Mock cache_ledger_obj to return a valid slot
        mock.expect_cache_ledger_obj()
            .times(1)
            .returning(move |_, _, _| slot);

        // Mock get_ledger_obj_field for Balance
        mock.expect_get_ledger_obj_field()
            .with(eq(slot), eq(balance_field_code), always(), eq(AMOUNT_SIZE))
            .times(1)
            .returning(move |_, _, _, _| AMOUNT_SIZE as i32);

        let _guard = setup_mock(mock);

        let account_id = AccountID::from([0xBB; 20]);
        let result = get_account_balance(&account_id);
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[test]
    fn test_get_account_balance_keylet_error() {
        let mut mock = MockHostBindings::new();

        // Mock account_keylet to fail
        mock.expect_account_keylet()
            .times(1)
            .returning(|_, _, _, _| INTERNAL_ERROR);

        let _guard = setup_mock(mock);

        let account_id = AccountID::from([0xBB; 20]);
        let result = get_account_balance(&account_id);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().code(), INTERNAL_ERROR);
    }

    #[test]
    fn test_get_account_balance_cache_error() {
        let mut mock = MockHostBindings::new();

        mock_account_keylet_success(&mut mock);

        // Mock cache_ledger_obj to return error
        mock.expect_cache_ledger_obj()
            .times(1)
            .returning(|_, _, _| INTERNAL_ERROR);

        let _guard = setup_mock(mock);

        let account_id = AccountID::from([0xBB; 20]);
        let result = get_account_balance(&account_id);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().code(), INTERNAL_ERROR);
    }
}
