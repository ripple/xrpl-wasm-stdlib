#[cfg(not(target_arch = "wasm32"))]
use crate::host::host_bindings_trait::{HostBindings, MockHostBindings};
use std::cell::RefCell;

#[cfg(all(any(test, feature = "test-host-bindings"), not(target_arch = "wasm32")))]
pub struct MockGuard;

#[cfg(all(any(test, feature = "test-host-bindings"), not(target_arch = "wasm32")))]
impl Drop for MockGuard {
    fn drop(&mut self) {
        clear_mock_host_bindings();
    }
}

#[cfg(all(any(test, feature = "test-host-bindings"), not(target_arch = "wasm32")))]
pub fn setup_mock(mock: MockHostBindings) -> MockGuard {
    set_mock_host_bindings(mock);
    MockGuard
}

// Create a default mock with stub return values matching the old host_bindings_for_testing.rs
#[cfg(all(any(test, feature = "test-host-bindings"), not(target_arch = "wasm32")))]
fn create_default_mock() -> MockHostBindings {
    let mut mock = MockHostBindings::new();

    // Ledger info functions - return small positive values
    mock.expect_ldgr_index()
        .returning(|_, out_buff_len| out_buff_len as i32);
    mock.expect_parent_ldgr_time()
        .returning(|_, out_buff_len| out_buff_len as i32);
    mock.expect_base_fee()
        .returning(|_, out_buff_len| out_buff_len as i32);

    // Functions that return buffer length
    mock.expect_parent_ldgr_hash()
        .returning(|_, out_buff_len| out_buff_len as i32);
    mock.expect_amendment_enabled()
        .returning(|_, amendment_len| amendment_len as i32);
    mock.expect_cache_le()
        .returning(|_, keylet_len, _| keylet_len as i32);
    mock.expect_tx_field()
        .returning(|_, _, out_buff_len| out_buff_len as i32);
    mock.expect_home_le_field()
        .returning(|_, _, out_buff_len| out_buff_len as i32);
    mock.expect_le_field()
        .returning(|_, _, _, out_buff_len| out_buff_len as i32);
    mock.expect_tx_inner()
        .returning(|_, _, _, out_buff_len| out_buff_len as i32);
    mock.expect_home_le_inner()
        .returning(|_, _, _, out_buff_len| out_buff_len as i32);
    mock.expect_le_inner()
        .returning(|_, _, _, _, out_buff_len| out_buff_len as i32);

    // Array length functions
    mock.expect_tx_arr_len().returning(|_| 0);
    mock.expect_home_le_arr_len().returning(|_| 0);
    mock.expect_le_arr_len().returning(|_, _| 0);
    mock.expect_tx_inner_arr_len().returning(|_, _| 0);
    // Note: These two return locator_len, not 0
    mock.expect_home_le_inner_arr_len()
        .returning(|_, locator_len| locator_len as i32);
    mock.expect_le_inner_arr_len()
        .returning(|_, _, locator_len| locator_len as i32);

    // Update and crypto functions
    mock.expect_set_data()
        .returning(|_, data_len| data_len as i32);
    mock.expect_sha512_half()
        .returning(|_, _, _, out_buff_len| out_buff_len as i32);
    mock.expect_check_sig().returning(|_, _, _, _, _, _| 0);

    // Keylet functions - all return buffer length
    mock.expect_accountroot_id()
        .returning(|_, _, _, out_buff_len| out_buff_len as i32);
    mock.expect_amm_id()
        .returning(|_, _, _, _, _, out_buff_len| out_buff_len as i32);
    mock.expect_check_id()
        .returning(|_, _, _, _, _, out_buff_len| out_buff_len as i32);
    mock.expect_credential_id()
        .returning(|_, _, _, _, _, _, _, out_buff_len| out_buff_len as i32);
    mock.expect_delegate_id()
        .returning(|_, _, _, _, _, out_buff_len| out_buff_len as i32);
    mock.expect_deposit_preauth_id()
        .returning(|_, _, _, _, _, out_buff_len| out_buff_len as i32);
    mock.expect_did_id()
        .returning(|_, _, _, out_buff_len| out_buff_len as i32);
    mock.expect_escrow_id()
        .returning(|_, _, _, _, _, out_buff_len| out_buff_len as i32);
    mock.expect_trustline_id()
        .returning(|_, _, _, _, _, _, _, out_buff_len| out_buff_len as i32);
    mock.expect_mpt_issuance_id()
        .returning(|_, _, _, _, _, out_buff_len| out_buff_len as i32);
    mock.expect_mptoken_id()
        .returning(|_, _, _, _, _, out_buff_len| out_buff_len as i32);
    mock.expect_nft_offer_id()
        .returning(|_, _, _, _, _, out_buff_len| out_buff_len as i32);
    mock.expect_offer_id()
        .returning(|_, _, _, _, _, out_buff_len| out_buff_len as i32);
    mock.expect_oracle_id()
        .returning(|_, _, _, _, _, out_buff_len| out_buff_len as i32);
    mock.expect_paychan_id()
        .returning(|_, _, _, _, _, _, _, out_buff_len| out_buff_len as i32);
    mock.expect_permissioned_domain_id()
        .returning(|_, _, _, _, _, out_buff_len| out_buff_len as i32);
    mock.expect_signers_id()
        .returning(|_, _, _, out_buff_len| out_buff_len as i32);
    mock.expect_ticket_id()
        .returning(|_, _, _, _, _, out_buff_len| out_buff_len as i32);
    mock.expect_vault_id()
        .returning(|_, _, _, _, _, out_buff_len| out_buff_len as i32);

    // NFT functions
    mock.expect_nft_uri()
        .returning(|_, _, _, _, _, out_buff_len| out_buff_len as i32);
    mock.expect_nft_issuer()
        .returning(|_, _, _, out_buff_len| out_buff_len as i32);
    mock.expect_nft_taxon()
        .returning(|_, _, _, out_buff_len| out_buff_len as i32);
    mock.expect_nft_flags()
        .returning(|_, nft_id_len| nft_id_len as i32);
    mock.expect_nft_xfer_fee()
        .returning(|_, nft_id_len| nft_id_len as i32);
    mock.expect_nft_serial()
        .returning(|_, _, _, out_buff_len| out_buff_len as i32);

    // Float functions
    mock.expect_float_from_int()
        .returning(|_, _, out_buff_len, _| out_buff_len as i32);
    mock.expect_float_from_uint()
        .returning(|_, _, _, out_buff_len, _| out_buff_len as i32);
    mock.expect_float_set()
        .returning(|_, _, _, out_buff_len, _| out_buff_len as i32);
    mock.expect_float_cmp().returning(|_, _, _, _| 0);
    mock.expect_float_add()
        .returning(|_, _, _, _, _, out_buff_len, _| out_buff_len as i32);
    mock.expect_float_sub()
        .returning(|_, _, _, _, _, out_buff_len, _| out_buff_len as i32);
    mock.expect_float_mult()
        .returning(|_, _, _, _, _, out_buff_len, _| out_buff_len as i32);
    mock.expect_float_div()
        .returning(|_, _, _, _, _, out_buff_len, _| out_buff_len as i32);
    mock.expect_float_pow()
        .returning(|_, _, _, _, out_buff_len, _| out_buff_len as i32);
    mock.expect_float_root()
        .returning(|_, _, _, _, out_buff_len, _| out_buff_len as i32);
    mock.expect_float_log()
        .returning(|_, _, _, out_buff_len, _| out_buff_len as i32);

    // Helper to calculate sum of two lengths, clamping to i32::MAX
    let sum_lengths = |len1: usize, len2: usize| -> i32 {
        len1.saturating_add(len2).min(i32::MAX as usize) as i32
    };

    // Trace functions - return sum of lengths (matching old host_bindings_for_testing.rs)
    mock.expect_trace()
        .returning(move |_, msg_len, _, data_len, _| sum_lengths(msg_len, data_len));
    mock.expect_trace_num()
        .returning(move |_, msg_len, _| sum_lengths(msg_len, 8));
    mock.expect_trace_acct()
        .returning(move |_, msg_len, _, acc_len| sum_lengths(msg_len, acc_len));
    mock.expect_trace_xfloat()
        .returning(move |_, msg_len, _, float_len| sum_lengths(msg_len, float_len));
    mock.expect_trace_amt()
        .returning(move |_, msg_len, _, amt_len| sum_lengths(msg_len, amt_len));

    mock
}

// #[cfg(test)]
#[cfg(all(any(test, feature = "test-host-bindings"), not(target_arch = "wasm32")))]
thread_local! {
    static MOCK_STATE: RefCell<Option<MockHostBindings>> = RefCell::new(Some(create_default_mock()));
}

// Helper functions to manage the mock state
#[cfg(all(any(test, feature = "test-host-bindings"), not(target_arch = "wasm32")))]
pub fn set_mock_host_bindings(mock: MockHostBindings) {
    MOCK_STATE.with(|state| {
        *state.borrow_mut() = Some(mock);
    });
}

#[cfg(all(any(test, feature = "test-host-bindings"), not(target_arch = "wasm32")))]
pub fn clear_mock_host_bindings() {
    MOCK_STATE.with(|state| {
        *state.borrow_mut() = None;
    });
}

#[cfg(all(any(test, feature = "test-host-bindings"), not(target_arch = "wasm32")))]
// Macro to generate stub functions for non-WASM targets
// These functions delegate to the MockHostBindings in MOCK_STATE
macro_rules! export_host_functions {
    ($(
        $(#[$attr:meta])*
        fn $name:ident($($param:ident: $param_ty:ty),*) -> $ret:ty;
    )*) => {
        $(
            #[allow(clippy::too_many_arguments)]
            #[allow(clippy::missing_safety_doc)]
            $(#[$attr])*
            pub unsafe fn $name($($param: $param_ty),*) -> $ret {
                MOCK_STATE.with(|state|  {
                    // The mock should always be present due to default initialization
                    // If it's not, panic with a clear error message
                    let mock = state.borrow();
                    let mock_ref = mock.as_ref().expect("MockHostBindings not initialized");
                    unsafe { mock_ref.$name($($param),*) }
                })
            }
        )*
    };
}

// Re-export all host functions as public functions for use by the rest of the codebase
// For non-WASM targets, these are stub implementations that panic
// The actual test implementations using MockHostBindings are in the tests module below

// Generate all the stub functions
export_host_functions! {
    // Host Function Category: ledger and transaction info
    fn ldgr_index(out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn parent_ldgr_time(out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn parent_ldgr_hash(out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn base_fee(out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn amendment_enabled(amendment_ptr: *const u8, amendment_len: usize) -> i32;
    fn cache_le(keylet_ptr: *const u8, keylet_len: usize, cache_num: i32) -> i32;
    fn tx_field(field: i32, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn home_le_field(field: i32, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn le_field(cache_num: i32, field: i32, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn tx_inner(locator_ptr: *const u8, locator_len: usize, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn home_le_inner(locator_ptr: *const u8, locator_len: usize, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn le_inner(cache_num: i32, locator_ptr: *const u8, locator_len: usize, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn tx_arr_len(field: i32) -> i32;
    fn home_le_arr_len(field: i32) -> i32;
    fn le_arr_len(cache_num: i32, field: i32) -> i32;
    fn tx_inner_arr_len(locator_ptr: *const u8, locator_len: usize) -> i32;
    fn home_le_inner_arr_len(locator_ptr: *const u8, locator_len: usize) -> i32;
    fn le_inner_arr_len(cache_num: i32, locator_ptr: *const u8, locator_len: usize) -> i32;

    // Host Function Category: update current ledger entry
    fn set_data(data_ptr: *const u8, data_len: usize) -> i32;

    // Host Function Category: hash and keylet computation
    fn sha512_half(data_ptr: *const u8, data_len: usize, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn check_sig(message_ptr: *const u8, message_len: usize, signature_ptr: *const u8, signature_len: usize, pubkey_ptr: *const u8, pubkey_len: usize) -> i32;
    fn accountroot_id(account_ptr: *const u8, account_len: usize, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn amm_id(issue1_ptr: *const u8, issue1_len: usize, issue2_ptr: *const u8, issue2_len: usize, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn check_id(account_ptr: *const u8, account_len: usize, sequence_ptr: *const u8, sequence_len: usize, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn credential_id(subject_ptr: *const u8, subject_len: usize, issuer_ptr: *const u8, issuer_len: usize, cred_type_ptr: *const u8, cred_type_len: usize, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn delegate_id(account_ptr: *const u8, account_len: usize, authorize_ptr: *const u8, authorize_len: usize, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn deposit_preauth_id(account_ptr: *const u8, account_len: usize, authorize_ptr: *const u8, authorize_len: usize, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn did_id(account_ptr: *const u8, account_len: usize, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn escrow_id(account_ptr: *const u8, account_len: usize, sequence_ptr: *const u8, sequence_len: usize, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn trustline_id(account1_ptr: *const u8, account1_len: usize, account2_ptr: *const u8, account2_len: usize, currency_ptr: *const u8, currency_len: usize, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn mpt_issuance_id(issuer_ptr: *const u8, issuer_len: usize, sequence_ptr: *const u8, sequence_len: usize, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn mptoken_id(mptid_ptr: *const u8, mptid_len: usize, holder_ptr: *const u8, holder_len: usize, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn nft_offer_id(account_ptr: *const u8, account_len: usize, sequence_ptr: *const u8, sequence_len: usize, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn offer_id(account_ptr: *const u8, account_len: usize, sequence_ptr: *const u8, sequence_len: usize, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn oracle_id(account_ptr: *const u8, account_len: usize, document_id_ptr: *const u8, document_id_len: usize, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn paychan_id(account_ptr: *const u8, account_len: usize, destination_ptr: *const u8, destination_len: usize, sequence_ptr: *const u8, sequence_len: usize, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn permissioned_domain_id(account_ptr: *const u8, account_len: usize, sequence_ptr: *const u8, sequence_len: usize, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn signers_id(account_ptr: *const u8, account_len: usize, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn ticket_id(account_ptr: *const u8, account_len: usize, sequence_ptr: *const u8, sequence_len: usize, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn vault_id(account_ptr: *const u8, account_len: usize, sequence_ptr: *const u8, sequence_len: usize, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;

    // Host Function Category: NFT
    fn nft_uri(account_ptr: *const u8, account_len: usize, nft_id_ptr: *const u8, nft_id_len: usize, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn nft_issuer(nft_id_ptr: *const u8, nft_id_len: usize, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn nft_taxon(nft_id_ptr: *const u8, nft_id_len: usize, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn nft_flags(nft_id_ptr: *const u8, nft_id_len: usize) -> i32;
    fn nft_xfer_fee(nft_id_ptr: *const u8, nft_id_len: usize) -> i32;
    fn nft_serial(nft_id_ptr: *const u8, nft_id_len: usize, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;

    // Host Function Category: FLOAT
    fn float_from_int(in_int: i64, out_buff: *mut u8, out_buff_len: usize, rounding_mode: i32) -> i32;
    fn float_from_uint(in_uint_ptr: *const u8, in_uint_len: usize, out_buff: *mut u8, out_buff_len: usize, rounding_mode: i32) -> i32;
    fn float_set(exponent: i32, mantissa: i64, out_buff: *mut u8, out_buff_len: usize, rounding_mode: i32) -> i32;
    fn float_cmp(in_buff1: *const u8, in_buff1_len: usize, in_buff2: *const u8, in_buff2_len: usize) -> i32;
    fn float_add(in_buff1: *const u8, in_buff1_len: usize, in_buff2: *const u8, in_buff2_len: usize, out_buff: *mut u8, out_buff_len: usize, rounding_mode: i32) -> i32;
    fn float_sub(in_buff1: *const u8, in_buff1_len: usize, in_buff2: *const u8, in_buff2_len: usize, out_buff: *mut u8, out_buff_len: usize, rounding_mode: i32) -> i32;
    fn float_mult(in_buff1: *const u8, in_buff1_len: usize, in_buff2: *const u8, in_buff2_len: usize, out_buff: *mut u8, out_buff_len: usize, rounding_mode: i32) -> i32;
    fn float_div(in_buff1: *const u8, in_buff1_len: usize, in_buff2: *const u8, in_buff2_len: usize, out_buff: *mut u8, out_buff_len: usize, rounding_mode: i32) -> i32;
    fn float_pow(in_buff: *const u8, in_buff_len: usize, pow: i32, out_buff: *mut u8, out_buff_len: usize, rounding_mode: i32) -> i32;
    fn float_root(in_buff: *const u8, in_buff_len: usize, root: i32, out_buff: *mut u8, out_buff_len: usize, rounding_mode: i32) -> i32;
    fn float_log(in_buff: *const u8, in_buff_len: usize, out_buff: *mut u8, out_buff_len: usize, rounding_mode: i32) -> i32;

    // Host Function Category: TRACE
    fn trace(msg_read_ptr: *const u8, msg_read_len: usize, data_read_ptr: *const u8, data_read_len: usize, as_hex: i32) -> i32;
    fn trace_num(msg_read_ptr: *const u8, msg_read_len: usize, number: i64) -> i32;
    fn trace_acct(msg_read_ptr: *const u8, msg_read_len: usize, account_ptr: *const u8, account_len: usize) -> i32;
    fn trace_xfloat(msg_read_ptr: *const u8, msg_read_len: usize, opaque_float_ptr: *const u8, opaque_float_len: usize) -> i32;
    fn trace_amt(msg_read_ptr: *const u8, msg_read_len: usize, amount_ptr: *const u8, amount_len: usize) -> i32;

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ledger_functions_with_mock() {
        let mut mock = MockHostBindings::new();

        // Set up expectations - these functions now take buffer parameters
        mock.expect_ldgr_index().times(1).returning(|_, _| 12345);
        mock.expect_parent_ldgr_time()
            .times(1)
            .returning(|_, _| 1234567890);
        mock.expect_base_fee().times(1).returning(|_, _| 10);

        // Set the mock in thread-local storage
        set_mock_host_bindings(mock);

        // Test the exported functions (they will use the mock)
        let mut buffer = [0u8; 32];
        unsafe {
            assert_eq!(ldgr_index(buffer.as_mut_ptr(), buffer.len()), 12345);
            assert_eq!(
                parent_ldgr_time(buffer.as_mut_ptr(), buffer.len()),
                1234567890
            );
            assert_eq!(base_fee(buffer.as_mut_ptr(), buffer.len()), 10);
        }

        // Clean up
        clear_mock_host_bindings();
    }

    #[test]
    fn test_buffer_operations_with_mock() {
        let mut mock = MockHostBindings::new();

        // Mock parent_ldgr_hash to write test data
        mock.expect_parent_ldgr_hash()
            .times(1)
            .returning(|out_buff_ptr, out_buff_len| {
                if out_buff_len >= 32 {
                    unsafe {
                        // Write test hash data
                        for i in 0..32 {
                            *out_buff_ptr.add(i) = (i * 2) as u8;
                        }
                    }
                    32 // Return bytes written
                } else {
                    -1 // Buffer too small error
                }
            });

        // Test it
        let mut buffer = [0u8; 32];
        unsafe {
            let result = mock.parent_ldgr_hash(buffer.as_mut_ptr(), buffer.len());
            assert_eq!(result, 32);

            // Verify the mock wrote the expected data
            for (i, _) in buffer.iter().enumerate() {
                assert_eq!(buffer[i], (i * 2) as u8);
            }
        }
    }

    #[test]
    fn test_trace_functions_with_mock() {
        let mut mock = MockHostBindings::new();

        // Mock trace function
        mock.expect_trace().times(1).returning(
            |_msg_ptr, msg_len, _data_ptr, data_len, _as_hex| (msg_len + data_len) as i32,
        );

        // Mock trace_num function
        mock.expect_trace_num()
            .times(1)
            .returning(|_msg_ptr, msg_len, _number| msg_len as i32);

        // Test trace functions
        let message = b"Test message";
        let data = b"Test data";

        unsafe {
            let result = mock.trace(
                message.as_ptr(),
                message.len(),
                data.as_ptr(),
                data.len(),
                0,
            );
            assert_eq!(result, (message.len() + data.len()) as i32);

            let result = mock.trace_num(message.as_ptr(), message.len(), 42);
            assert_eq!(result, message.len() as i32);
        }
    }

    #[test]
    fn test_keylet_functions_with_mock() {
        let mut mock = MockHostBindings::new();

        // Mock accountroot_id to return a test keylet
        mock.expect_accountroot_id().times(1).returning(
            |_account_ptr, _account_len, out_buff_ptr, out_buff_len| {
                if out_buff_len >= 32 {
                    unsafe {
                        // Write a test keylet (32 bytes of 0xAA)
                        for i in 0..32 {
                            *out_buff_ptr.add(i) = 0xAA;
                        }
                    }
                    32
                } else {
                    -1
                }
            },
        );

        // Test keylet generation
        let account = [0u8; 20]; // Mock account ID
        let mut keylet_buffer = [0u8; 32];

        unsafe {
            let result = mock.accountroot_id(
                account.as_ptr(),
                account.len(),
                keylet_buffer.as_mut_ptr(),
                keylet_buffer.len(),
            );

            assert_eq!(result, 32);
            assert_eq!(keylet_buffer, [0xAA; 32]);
        }
    }

    #[test]
    fn test_error_conditions_with_mock() {
        let mut mock = MockHostBindings::new();

        // Mock a function to return an error code
        mock.expect_ldgr_index().times(1).returning(|_, _| -1); // Return error

        mock.expect_parent_ldgr_hash()
            .times(1)
            .returning(|_out_buff_ptr, _out_buff_len| -2); // Buffer too small

        unsafe {
            // Test error conditions
            let mut buffer = [0u8; 32];
            assert_eq!(mock.ldgr_index(buffer.as_mut_ptr(), buffer.len()), -1);

            let mut small_buffer = [0u8; 16]; // Too small buffer
            let result = mock.parent_ldgr_hash(small_buffer.as_mut_ptr(), small_buffer.len());
            assert_eq!(result, -2);
        }
    }

    #[test]
    fn test_generic_function_with_mock() {
        // Example of testing a function that takes HostBindings as a parameter
        fn get_ledger_info<H: HostBindings>(host: &H) -> (i32, i32, i32) {
            let mut buffer = [0u8; 32];
            unsafe {
                let sqn = host.ldgr_index(buffer.as_mut_ptr(), buffer.len());
                let time = host.parent_ldgr_time(buffer.as_mut_ptr(), buffer.len());
                let fee = host.base_fee(buffer.as_mut_ptr(), buffer.len());
                (sqn, time, fee)
            }
        }

        let mut mock = MockHostBindings::new();

        mock.expect_ldgr_index().returning(|_, _| 999);
        mock.expect_parent_ldgr_time().returning(|_, _| 888);
        mock.expect_base_fee().returning(|_, _| 777);

        let (sqn, time, fee) = get_ledger_info(&mock);
        assert_eq!(sqn, 999);
        assert_eq!(time, 888);
        assert_eq!(fee, 777);
    }
}
