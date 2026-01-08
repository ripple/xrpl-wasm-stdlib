#[cfg(not(target_arch = "wasm32"))]
/// Macro to generate stub implementations of host functions for non-WASM builds.
///
/// This macro creates empty stub functions that return an empty buffer with the length
/// of the final parameter value. This allows code to compile and run basic tests in
/// non-WASM environments without actual host bindings (in particular, doc code).
///
/// # How it works
///
/// For each function signature, the macro:
/// 1. Generates a public unsafe function with the same signature
/// 2. Calls the `@return_value` helper with just the parameter names (no types)
/// 3. The helper recursively finds the last parameter and returns its value as i32
///
/// # Example
///
/// Input:
/// ```ignore
/// export_host_functions! {
///     fn account_keylet(_account_ptr: *const u8, _account_len: usize,
///                       _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;
/// }
/// ```
///
/// Generates:
/// ```ignore
/// pub unsafe fn account_keylet(_account_ptr: *const u8, _account_len: usize,
///                              _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32 {
///     _out_buff_len as i32  // Returns 32 at runtime
/// }
/// ```
macro_rules! export_host_functions {
    // Main rule: Generate function definitions
    // Matches zero or more function signatures and generates stub implementations
    ($(
        $(#[$attr:meta])*
        fn $name:ident($($param:ident: $param_ty:ty),*) -> $ret:ty;
    )*) => {
        $(
            #[allow(clippy::too_many_arguments)]
            #[allow(clippy::missing_safety_doc)]
            $(#[$attr])*
            pub unsafe fn $name($($param: $param_ty),*) -> $ret {
                // Call helper rule with parameter names only (types stripped)
                // This will recursively find and return the last parameter value
                export_host_functions!(@return_value $($param),*)
            }
        )*
    };

    // Helper rule: Find and return the last parameter value
    //
    // This uses tail recursion to traverse the parameter list:
    // - Recursively strips off the first parameter
    // - Continues until only one parameter remains
    // - Returns that final parameter value as i32
    //
    // Example: @return_value a, b, c, d
    //   → @return_value b, c, d
    //   → @return_value c, d
    //   → @return_value d
    //   → d as i32

    // Base case 1: No parameters
    // If the function has no parameters, return 0
    (@return_value) => {
        0
    };

    // Base case 2: One parameter remaining
    // We've found the last parameter! Return its value cast to i32
    (@return_value $last_param:ident) => {
        $last_param as i32
    };

    // Recursive case: Multiple parameters
    // Strip off the first parameter ($param) and recurse with the rest ($rest)
    // This effectively "walks" through the parameter list to find the last one
    (@return_value $param:ident, $($rest:ident),+) => {
        export_host_functions!(@return_value $($rest),+)
    };
}

// Re-export all host functions as public functions for use by the rest of the codebase
// For non-WASM targets, _these are stub implementations that panic
// The actual test implementations using MockHostBindings are in the tests module below

// Generate all the stub functions
export_host_functions! {
    // Host Function Category: ledger and transaction info
    fn get_ledger_sqn() -> i32;
    fn get_parent_ledger_time() -> i32;
    fn get_parent_ledger_hash(_out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;
    fn get_base_fee() -> i32;
    fn amendment_enabled(_amendment_ptr: *const u8, _amendment_len: usize) -> i32;
    fn cache_ledger_obj(_keylet_ptr: *const u8, _keylet_len: usize, _cache_num: i32) -> i32;
    fn get_tx_field(_field: i32, _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;
    fn get_current_ledger_obj_field(_field: i32, _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;
    fn get_ledger_obj_field(_cache_num: i32, _field: i32, _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;
    fn get_tx_nested_field(_locator_ptr: *const u8, _locator_len: usize, _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;
    fn get_current_ledger_obj_nested_field(_locator_ptr: *const u8, _locator_len: usize, _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;
    fn get_ledger_obj_nested_field(_cache_num: i32, _locator_ptr: *const u8, _locator_len: usize, _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;
    fn get_tx_array_len(_field: i32) -> i32;
    fn get_current_ledger_obj_array_len(_field: i32) -> i32;
    fn get_ledger_obj_array_len(_cache_num: i32, _field: i32) -> i32;
    fn get_tx_nested_array_len(_locator_ptr: *const u8, _locator_len: usize) -> i32;
    fn get_current_ledger_obj_nested_array_len(_locator_ptr: *const u8, _locator_len: usize) -> i32;
    fn get_ledger_obj_nested_array_len(_cache_num: i32, _locator_ptr: *const u8, _locator_len: usize) -> i32;

    // // Host Function Category: update current ledger entry
    fn update_data(_data_ptr: *const u8, _data_len: usize) -> i32;

    // Host Function Category: hash and keylet computation
    fn compute_sha512_half(_data_ptr: *const u8, _data_len: usize, _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;
    fn check_sig(_message_ptr: *const u8, _message_len: usize, _signature_ptr: *const u8, _signature_len: usize, _pubkey_ptr: *const u8, _pubkey_len: usize) -> i32;
    fn account_keylet(_account_ptr: *const u8, _account_len: usize, _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;
    fn amm_keylet(_issue1_ptr: *const u8, _issue1_len: usize, _issue2_ptr: *const u8, _issue2_len: usize, _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;
    fn check_keylet(_account_ptr: *const u8, _account_len: usize, _sequence: i32, _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;
    fn credential_keylet(_subject_ptr: *const u8, _subject_len: usize, _issuer_ptr: *const u8, _issuer_len: usize, _cred_type_ptr: *const u8, _cred_type_len: usize, _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;
    fn delegate_keylet(_account_ptr: *const u8, _account_len: usize, _authorize_ptr: *const u8, _authorize_len: usize, _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;
    fn deposit_preauth_keylet(_account_ptr: *const u8, _account_len: usize, _authorize_ptr: *const u8, _authorize_len: usize, _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;
    fn did_keylet(_account_ptr: *const u8, _account_len: usize, _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;
    fn escrow_keylet(_account_ptr: *const u8, _account_len: usize, _sequence: i32, _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;
    fn line_keylet(_account1_ptr: *const u8, _account1_len: usize, _account2_ptr: *const u8, _account2_len: usize, _currency_ptr: *const u8, _currency_len: usize, _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;
    fn mpt_issuance_keylet(_issuer_ptr: *const u8, _issuer_len: usize, _sequence: i32, _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;
    fn mptoken_keylet(_mptid_ptr: *const u8, _mptid_len: usize, _holder_ptr: *const u8, _holder_len: usize, _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;
    fn nft_offer_keylet(_account_ptr: *const u8, _account_len: usize, _sequence: i32, _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;
    fn offer_keylet(_account_ptr: *const u8, _account_len: usize, _sequence: i32, _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;
    fn oracle_keylet(_account_ptr: *const u8, _account_len: usize, _document_id: i32, _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;
    fn paychan_keylet(_account_ptr: *const u8, _account_len: usize, _destination_ptr: *const u8, _destination_len: usize, _sequence: i32, _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;
    fn permissioned_domain_keylet(_account_ptr: *const u8, _account_len: usize, _sequence: i32, _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;
    fn signers_keylet(_account_ptr: *const u8, _account_len: usize, _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;
    fn ticket_keylet(_account_ptr: *const u8, _account_len: usize, _sequence: i32, _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;
    fn vault_keylet(_account_ptr: *const u8, _account_len: usize, _sequence: i32, _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;

    // Host Function Category: NFT
    fn get_nft(_account_ptr: *const u8, _account_len: usize, _nft_id_ptr: *const u8, _nft_id_len: usize, _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;
    fn get_nft_issuer(_nft_id_ptr: *const u8, _nft_id_len: usize, _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;
    fn get_nft_taxon(_nft_id_ptr: *const u8, _nft_id_len: usize, _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;
    fn get_nft_flags(_nft_id_ptr: *const u8, _nft_id_len: usize) -> i32;
    fn get_nft_transfer_fee(_nft_id_ptr: *const u8, _nft_id_len: usize) -> i32;
    fn get_nft_serial(_nft_id_ptr: *const u8, _nft_id_len: usize, _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;

    // Host Function Category: FLOAT
    fn float_from_int(_in_int: i64, _out_buff: *mut u8, _out_buff_len: usize, _rounding_mode: i32) -> i32;
    fn float_from_uint(_in_uint_ptr: *const u8, _in_uint_len: usize, _out_buff: *mut u8, _out_buff_len: usize, _rounding_mode: i32) -> i32;
    fn float_set(_exponent: i32, _mantissa: i64, _out_buff: *mut u8, _out_buff_len: usize, _rounding_mode: i32) -> i32;
    fn float_compare(_in_buff1: *const u8, _in_buff1_len: usize, _in_buff2: *const u8, _in_buff2_len: usize) -> i32;
    fn float_add(_in_buff1: *const u8, _in_buff1_len: usize, _in_buff2: *const u8, _in_buff2_len: usize, _out_buff: *mut u8, _out_buff_len: usize, _rounding_mode: i32) -> i32;
    fn float_subtract(_in_buff1: *const u8, _in_buff1_len: usize, _in_buff2: *const u8, _in_buff2_len: usize, _out_buff: *mut u8, _out_buff_len: usize, _rounding_mode: i32) -> i32;
    fn float_multiply(_in_buff1: *const u8, _in_buff1_len: usize, _in_buff2: *const u8, _in_buff2_len: usize, _out_buff: *mut u8, _out_buff_len: usize, _rounding_mode: i32) -> i32;
    fn float_divide(_in_buff1: *const u8, _in_buff1_len: usize, _in_buff2: *const u8, _in_buff2_len: usize, _out_buff: *mut u8, _out_buff_len: usize, _rounding_mode: i32) -> i32;
    fn float_pow(_in_buff: *const u8, _in_buff_len: usize, _in_int: i32, _out_buff: *mut u8, _out_buff_len: usize, _rounding_mode: i32) -> i32;
    fn float_root(_in_buff: *const u8, _in_buff_len: usize, _n: i32, _out_buff: *mut u8, _out_buff_len: usize, _rounding_mode: i32) -> i32;
    fn float_log(_in_buff: *const u8, _in_buff_len: usize, _out_buff: *mut u8, _out_buff_len: usize, _rounding_mode: i32) -> i32;

    // Host Function Category: TRACE
    fn trace(_msg_read_ptr: *const u8, _msg_read_len: usize, _data_read_ptr: *const u8, _data_read_len: usize, _as_hex: i32) -> i32;
    fn trace_num(_msg_read_ptr: *const u8, _msg_read_len: usize, _number: i64) -> i32;
    fn trace_account(_msg_read_ptr: *const u8, _msg_read_len: usize, _account_ptr: *const u8, _account_len: usize) -> i32;
    fn trace_opaque_float(_msg_read_ptr: *const u8, _msg_read_len: usize, _opaque_float_ptr: *const u8, _opaque_float_len: usize) -> i32;
    fn trace_amount(_msg_read_ptr: *const u8, _msg_read_len: usize, _amount_ptr: *const u8, _amount_len: usize) -> i32;
}
