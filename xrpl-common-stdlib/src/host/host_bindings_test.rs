// The mock implementation of the host functions, used by `cargo test` and by crates that
// enable the `test-host-bindings` feature. `mod.rs` only `include!`s this file on native
// targets under that cfg, so nothing in here needs its own `#[cfg]`.
use crate::host::host_bindings_trait::{HostBindings, MockHostBindings};
use std::cell::RefCell;

pub struct MockGuard;

impl Drop for MockGuard {
    fn drop(&mut self) {
        clear_mock_host_bindings();
    }
}

pub fn setup_mock(mock: MockHostBindings) -> MockGuard {
    set_mock_host_bindings(mock);
    MockGuard
}

// Creates a mock with a permissive default expectation for every host function.
pub fn create_default_mock() -> MockHostBindings {
    let mut mock = MockHostBindings::new();
    apply_default_expectations(&mut mock);
    mock
}

/// Applies the same default `.returning(...)` wiring as [`create_default_mock`] onto an
/// existing mock instead of constructing a new one.
///
/// Exposed so callers (e.g. scenario builders in `xrpl-stdlib-test-utils`) can register their
/// own expectations on a fresh mock first, then layer these defaults on top as a fallback:
/// mockall checks expectations in the order they were registered, so registering
/// scenario-specific expectations before calling this function lets them take priority over
/// the unconditional defaults added here.
pub fn apply_default_expectations(mock: &mut MockHostBindings) {
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
        .returning(|_, id_len, _| id_len as i32);
    // A real host returns the number of bytes it actually wrote, which for an `Amount` is the
    // variant's wire length (8 XRP / 33 MPT / 48 IOU), not the full buffer. Amount-typed fields
    // are exactly those whose serialized type code (high 16 bits) is `STI_AMOUNT` (6); the zeroed
    // default buffer decodes as XRP, so report 8 for them. Every other (fixed-size) field still
    // reports the full buffer length, which equals its exact size. This applies uniformly to
    // reads from the current transaction, the current ledger object, and a slot-cached one.
    const STI_AMOUNT: i32 = 6;
    mock.expect_tx_field().returning(|field, _, out_buff_len| {
        if field >> 16 == STI_AMOUNT {
            8
        } else {
            out_buff_len as i32
        }
    });
    mock.expect_home_le_field()
        .returning(|field, _, out_buff_len| {
            if field >> 16 == STI_AMOUNT {
                8
            } else {
                out_buff_len as i32
            }
        });
    mock.expect_le_field()
        .returning(|_, field, _, out_buff_len| {
            if field >> 16 == STI_AMOUNT {
                8
            } else {
                out_buff_len as i32
            }
        });
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

    // Ledger entry ID functions - all return buffer length
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
    mock.expect_sponsorship_id()
        .returning(|_, _, _, _, _, out_buff_len| out_buff_len as i32);
    mock.expect_loan_broker_id()
        .returning(|_, _, _, _, _, out_buff_len| out_buff_len as i32);
    mock.expect_loan_id()
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
    mock.expect_float_from_mant_exp()
        .returning(|_, _, _, out_buff_len, _| out_buff_len as i32);
    mock.expect_float_from_stamount()
        .returning(|_, _, _, out_buff_len, _| out_buff_len as i32);
    mock.expect_float_from_stnumber()
        .returning(|_, _, _, out_buff_len, _| out_buff_len as i32);
    mock.expect_float_to_int()
        .returning(|_, _, _, out_buff_len, _| out_buff_len as i32);
    mock.expect_float_to_mant_exp()
        .returning(|_, _, _, _, _, _| 8);
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

    // Trace
    mock.expect_trace().returning(|_, _, _, _, _| ());
}

thread_local! {
    static MOCK_STATE: RefCell<Option<MockHostBindings>> = RefCell::new(Some(create_default_mock()));
}

// Helper functions to manage the mock state
pub fn set_mock_host_bindings(mock: MockHostBindings) {
    MOCK_STATE.with(|state| {
        *state.borrow_mut() = Some(mock);
    });
}

pub fn clear_mock_host_bindings() {
    MOCK_STATE.with(|state| {
        *state.borrow_mut() = None;
    });
}

// Defines a free function for every host function that forwards to the thread-local mock.
// Expanded from the generated `for_each_host_function!` list (host_bindings_list.rs), so it
// needs no changes when rippled adds a host function.
macro_rules! dispatch_to_mock {
    ($( fn $name:ident($($param:ident: $param_ty:ty),*) -> $ret:ty; )*) => {
        $(
            #[allow(clippy::too_many_arguments, clippy::missing_safety_doc, clippy::unused_unit)]
            pub unsafe fn $name($($param: $param_ty),*) -> $ret {
                MOCK_STATE.with(|state| {
                    // The thread-local starts out holding a default mock, so this is only `None`
                    // if a test called `clear_mock_host_bindings` and then kept calling host
                    // functions. Fail with a clear message in that case.
                    let mock = state.borrow();
                    let mock_ref = mock.as_ref().expect("MockHostBindings not initialized");
                    unsafe { mock_ref.$name($($param),*) }
                })
            }
        )*
    };
}
for_each_host_function!(dispatch_to_mock);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::host::trace::TraceDataType;

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

        mock.expect_trace()
            .times(2)
            .returning(|_msg_ptr, _msg_len, _data_type, _data_ptr, _data_len| ());

        let message = b"Test message";
        let data = b"Test data";
        let number = 42i64.to_le_bytes();

        unsafe {
            mock.trace(
                message.as_ptr(),
                message.len(),
                TraceDataType::AsText as i32,
                data.as_ptr(),
                data.len(),
            );

            mock.trace(
                message.as_ptr(),
                message.len(),
                TraceDataType::Int64 as i32,
                number.as_ptr(),
                number.len(),
            );
        }
    }

    #[test]
    fn test_id_functions_with_mock() {
        let mut mock = MockHostBindings::new();

        // Mock accountroot_id to return a test ledger entry ID
        mock.expect_accountroot_id().times(1).returning(
            |_account_ptr, _account_len, out_buff_ptr, out_buff_len| {
                if out_buff_len >= 32 {
                    unsafe {
                        // Write a test ledger entry ID (32 bytes of 0xAA)
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

        // Test ledger entry ID generation
        let account = [0u8; 20]; // Mock account ID
        let mut id_buffer = [0u8; 32];

        unsafe {
            let result = mock.accountroot_id(
                account.as_ptr(),
                account.len(),
                id_buffer.as_mut_ptr(),
                id_buffer.len(),
            );

            assert_eq!(result, 32);
            assert_eq!(id_buffer, [0xAA; 32]);
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

/// Checks that [`apply_default_expectations`] covers every function in the generated list, by
/// calling each one once against the default mock. When rippled adds a host function and the
/// list is regenerated, this test fails (mockall panics with "No matching expectation found")
/// until a default for the new function is added.
#[cfg(test)]
mod default_expectation_coverage {
    use super::*;

    trait ZeroArg {
        fn zero() -> Self;
    }
    impl ZeroArg for i32 {
        fn zero() -> Self {
            0
        }
    }
    impl ZeroArg for i64 {
        fn zero() -> Self {
            0
        }
    }
    impl ZeroArg for usize {
        fn zero() -> Self {
            0
        }
    }
    impl ZeroArg for *const u8 {
        fn zero() -> Self {
            core::ptr::null()
        }
    }
    impl ZeroArg for *mut u8 {
        fn zero() -> Self {
            core::ptr::null_mut()
        }
    }

    macro_rules! call_every_host_function {
        ($( fn $name:ident($($param:ident: $param_ty:ty),*) -> $ret:ty; )*) => {
            #[test]
            #[allow(clippy::let_unit_value)]
            fn every_host_function_has_a_default_expectation() {
                let _guard = setup_mock(create_default_mock());
                $(
                    let _: $ret = unsafe { super::$name($(<$param_ty as ZeroArg>::zero()),*) };
                )*
            }
        };
    }
    for_each_host_function!(call_every_host_function);
}
