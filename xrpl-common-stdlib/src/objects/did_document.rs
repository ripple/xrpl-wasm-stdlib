use crate::host;
use crate::host::Error;
use crate::keylets::did_keylet;
use crate::objects::traits::{DidFields, LedgerObjectCommonFields};
use crate::types::account_id::AccountID;

/// A DID (Decentralized Identifier) ledger object.
///
/// Wraps the slot (register number) of a cached DID object so that its fields can be read through
/// the [`DidFields`] trait.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct DidDocument {
    pub slot_num: i32,
}

impl LedgerObjectCommonFields for DidDocument {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl DidFields for DidDocument {}

impl DidDocument {
    /// Loads the DID object owned by `account`.
    ///
    /// Computes the DID keylet for the account, caches the ledger object inside the host, and wraps
    /// the returned slot. Returns an error if the keylet cannot be computed or the object is not
    /// present in the ledger.
    pub fn load(account: &AccountID) -> host::Result<Self> {
        // Construct the DID keylet. This calls a host function, so propagate the error.
        let keylet = match did_keylet(account) {
            host::Result::Ok(keylet) => keylet,
            host::Result::Err(e) => return host::Result::Err(e),
        };

        // Try to cache the ledger object inside rippled.
        let slot = unsafe { host::cache_ledger_obj(keylet.as_ptr(), keylet.len(), 0) };
        if slot < 0 {
            return host::Result::Err(Error::from_code(slot));
        }

        host::Result::Ok(DidDocument { slot_num: slot })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::host::error_codes::INTERNAL_ERROR;
    use crate::host::host_bindings_trait::MockHostBindings;
    use crate::host::setup_mock;
    use crate::keylets::XRPL_KEYLET_SIZE;

    /// Mock did_keylet to write 0xCC bytes and return success.
    /// The byte value is arbitrary — `cache_ledger_obj` is itself mocked and
    /// never reads the buffer; what matters is that the keylet's `MaybeUninit`
    /// storage is initialized before downstream code calls `assume_init`.
    fn mock_did_keylet_success(mock: &mut MockHostBindings) {
        mock.expect_did_keylet()
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
    fn test_load_success() {
        let mut mock = MockHostBindings::new();
        let slot = 9;

        mock_did_keylet_success(&mut mock);

        mock.expect_cache_ledger_obj()
            .times(1)
            .returning(move |_, _, _| slot);

        let _guard = setup_mock(mock);

        let account = AccountID::from([0xBB; 20]);
        let result = DidDocument::load(&account);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().slot_num, slot);
    }

    #[test]
    fn test_load_keylet_error() {
        let mut mock = MockHostBindings::new();

        mock.expect_did_keylet()
            .times(1)
            .returning(|_, _, _, _| INTERNAL_ERROR);

        let _guard = setup_mock(mock);

        let account = AccountID::from([0xBB; 20]);
        let result = DidDocument::load(&account);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().code(), INTERNAL_ERROR);
    }

    #[test]
    fn test_load_cache_error() {
        let mut mock = MockHostBindings::new();

        mock_did_keylet_success(&mut mock);

        mock.expect_cache_ledger_obj()
            .times(1)
            .returning(|_, _, _| INTERNAL_ERROR);

        let _guard = setup_mock(mock);

        let account = AccountID::from([0xBB; 20]);
        let result = DidDocument::load(&account);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().code(), INTERNAL_ERROR);
    }
}
