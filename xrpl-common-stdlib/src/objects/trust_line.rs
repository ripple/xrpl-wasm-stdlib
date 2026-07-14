use crate::host;
use crate::host::Error;
use crate::keylets::line_keylet;
use crate::objects::traits::{LedgerObjectCommonFields, TrustLineFields};
use crate::types::account_id::AccountID;
use crate::types::currency::Currency;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct TrustLine {
    pub slot_num: i32,
}

impl LedgerObjectCommonFields for TrustLine {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl TrustLineFields for TrustLine {}

impl TrustLine {
    pub fn load(
        account1: &AccountID,
        account2: &AccountID,
        currency: &Currency,
    ) -> host::Result<Self> {
        // Construct the trust line keylet. This calls a host function, so propagate the error via `?`.
        let keylet = match line_keylet(account1, account2, currency) {
            host::Result::Ok(keylet) => keylet,
            host::Result::Err(e) => return host::Result::Err(e),
        };

        // Try to cache the ledger object inside rippled.
        let slot = unsafe { host::cache_ledger_obj(keylet.as_ptr(), keylet.len(), 0) };
        if slot < 0 {
            return host::Result::Err(Error::from_code(slot));
        }

        host::Result::Ok(TrustLine { slot_num: slot })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::host::error_codes::INTERNAL_ERROR;
    use crate::host::host_bindings_trait::MockHostBindings;
    use crate::host::setup_mock;
    use crate::keylets::XRPL_KEYLET_SIZE;

    /// Mock line_keylet to write 0xCC bytes and return success.
    /// The byte value is arbitrary — `cache_ledger_obj` is itself mocked and
    /// never reads the buffer; what matters is that the keylet's `MaybeUninit`
    /// storage is initialized before downstream code calls `assume_init`.
    fn mock_line_keylet_success(mock: &mut MockHostBindings) {
        mock.expect_line_keylet().times(1).returning(
            |_, _, _, _, _, _, out_buff_ptr, out_buff_len| {
                assert_eq!(out_buff_len, XRPL_KEYLET_SIZE);
                unsafe {
                    for i in 0..XRPL_KEYLET_SIZE {
                        *out_buff_ptr.add(i) = 0xCC;
                    }
                }
                XRPL_KEYLET_SIZE as i32
            },
        );
    }

    #[test]
    fn test_load_success() {
        let mut mock = MockHostBindings::new();
        let slot = 7;

        mock_line_keylet_success(&mut mock);

        mock.expect_cache_ledger_obj()
            .times(1)
            .returning(move |_, _, _| slot);

        let _guard = setup_mock(mock);

        let account1 = AccountID::from([0xAA; 20]);
        let account2 = AccountID::from([0xBB; 20]);
        let currency = Currency::from([0xCC; 20]);

        let result = TrustLine::load(&account1, &account2, &currency);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().slot_num, slot);
    }

    #[test]
    fn test_load_keylet_error() {
        let mut mock = MockHostBindings::new();

        mock.expect_line_keylet()
            .times(1)
            .returning(|_, _, _, _, _, _, _, _| INTERNAL_ERROR);

        let _guard = setup_mock(mock);

        let account1 = AccountID::from([0xAA; 20]);
        let account2 = AccountID::from([0xBB; 20]);
        let currency = Currency::from([0xCC; 20]);

        let result = TrustLine::load(&account1, &account2, &currency);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().code(), INTERNAL_ERROR);
    }

    #[test]
    fn test_load_cache_error() {
        let mut mock = MockHostBindings::new();

        mock_line_keylet_success(&mut mock);

        mock.expect_cache_ledger_obj()
            .times(1)
            .returning(|_, _, _| INTERNAL_ERROR);

        let _guard = setup_mock(mock);

        let account1 = AccountID::from([0xAA; 20]);
        let account2 = AccountID::from([0xBB; 20]);
        let currency = Currency::from([0xCC; 20]);

        let result = TrustLine::load(&account1, &account2, &currency);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().code(), INTERNAL_ERROR);
    }
}
