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
///     fn accountroot_id(_account_ptr: *const u8, _account_len: usize,
///                       _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;
/// }
/// ```
///
/// Generates:
/// ```ignore
/// pub unsafe fn accountroot_id(_account_ptr: *const u8, _account_len: usize,
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
    fn ldgr_index(_out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;
    fn parent_ldgr_time(_out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;
    fn parent_ldgr_hash(_out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;
    fn base_fee(_out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;
    fn amendment_enabled(_amendment_ptr: *const u8, _amendment_len: usize) -> i32;
    fn cache_le(_keylet_ptr: *const u8, _keylet_len: usize, _cache_num: i32) -> i32;
    fn tx_field(_field: i32, _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;
    fn home_le_field(_field: i32, _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;
    fn le_field(_cache_num: i32, _field: i32, _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;
    fn tx_inner(_locator_ptr: *const u8, _locator_len: usize, _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;
    fn home_le_inner(_locator_ptr: *const u8, _locator_len: usize, _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;
    fn le_inner(_cache_num: i32, _locator_ptr: *const u8, _locator_len: usize, _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;
    fn tx_arr_len(_field: i32) -> i32;
    fn home_le_arr_len(_field: i32) -> i32;
    fn le_arr_len(_cache_num: i32, _field: i32) -> i32;
    fn tx_inner_arr_len(_locator_ptr: *const u8, _locator_len: usize) -> i32;
    fn home_le_inner_arr_len(_locator_ptr: *const u8, _locator_len: usize) -> i32;
    fn le_inner_arr_len(_cache_num: i32, _locator_ptr: *const u8, _locator_len: usize) -> i32;

    // Host Function Category: update current ledger entry
    fn set_data(_data_ptr: *const u8, _data_len: usize) -> i32;

    // Host Function Category: hash and keylet computation
    fn sha512_half(_data_ptr: *const u8, _data_len: usize, _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;
    fn check_sig(_message_ptr: *const u8, _message_len: usize, _signature_ptr: *const u8, _signature_len: usize, _pubkey_ptr: *const u8, _pubkey_len: usize) -> i32;
    fn accountroot_id(_account_ptr: *const u8, _account_len: usize, _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;
    fn amm_id(_issue1_ptr: *const u8, _issue1_len: usize, _issue2_ptr: *const u8, _issue2_len: usize, _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;
    fn check_id(_account_ptr: *const u8, _account_len: usize, _sequence_ptr: *const u8, _sequence_len: usize, _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;
    fn credential_id(_subject_ptr: *const u8, _subject_len: usize, _issuer_ptr: *const u8, _issuer_len: usize, _cred_type_ptr: *const u8, _cred_type_len: usize, _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;
    fn delegate_id(_account_ptr: *const u8, _account_len: usize, _authorize_ptr: *const u8, _authorize_len: usize, _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;
    fn deposit_preauth_id(_account_ptr: *const u8, _account_len: usize, _authorize_ptr: *const u8, _authorize_len: usize, _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;
    fn did_id(_account_ptr: *const u8, _account_len: usize, _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;
    fn escrow_id(_account_ptr: *const u8, _account_len: usize, _sequence_ptr: *const u8, _sequence_len: usize, _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;
    fn trustline_id(_account1_ptr: *const u8, _account1_len: usize, _account2_ptr: *const u8, _account2_len: usize, _currency_ptr: *const u8, _currency_len: usize, _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;
    fn mpt_issuance_id(_issuer_ptr: *const u8, _issuer_len: usize, _sequence_ptr: *const u8, _sequence_len: usize, _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;
    fn mptoken_id(_mptid_ptr: *const u8, _mptid_len: usize, _holder_ptr: *const u8, _holder_len: usize, _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;
    fn nft_offer_id(_account_ptr: *const u8, _account_len: usize, _sequence_ptr: *const u8, _sequence_len: usize, _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;
    fn offer_id(_account_ptr: *const u8, _account_len: usize, _sequence_ptr: *const u8, _sequence_len: usize, _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;
    fn oracle_id(_account_ptr: *const u8, _account_len: usize, _document_id_ptr: *const u8, _document_id_len: usize, _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;
    fn paychan_id(_account_ptr: *const u8, _account_len: usize, _destination_ptr: *const u8, _destination_len: usize, _sequence_ptr: *const u8, _sequence_len: usize, _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;
    fn permissioned_domain_id(_account_ptr: *const u8, _account_len: usize, _sequence_ptr: *const u8, _sequence_len: usize, _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;
    fn signers_id(_account_ptr: *const u8, _account_len: usize, _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;
    fn ticket_id(_account_ptr: *const u8, _account_len: usize, _sequence_ptr: *const u8, _sequence_len: usize, _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;
    fn vault_id(_account_ptr: *const u8, _account_len: usize, _sequence_ptr: *const u8, _sequence_len: usize, _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;

    // Host Function Category: NFT
    fn nft_uri(_account_ptr: *const u8, _account_len: usize, _nft_id_ptr: *const u8, _nft_id_len: usize, _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;
    fn nft_issuer(_nft_id_ptr: *const u8, _nft_id_len: usize, _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;
    fn nft_taxon(_nft_id_ptr: *const u8, _nft_id_len: usize, _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;
    fn nft_flags(_nft_id_ptr: *const u8, _nft_id_len: usize) -> i32;
    fn nft_xfer_fee(_nft_id_ptr: *const u8, _nft_id_len: usize) -> i32;
    fn nft_serial(_nft_id_ptr: *const u8, _nft_id_len: usize, _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32;

    // Host Function Category: FLOAT
    fn float_from_int(_in_int: i64, _out_buff: *mut u8, _out_buff_len: usize, _rounding_mode: i32) -> i32;
    fn float_from_uint(_in_uint_ptr: *const u8, _in_uint_len: usize, _out_buff: *mut u8, _out_buff_len: usize, _rounding_mode: i32) -> i32;
    fn float_set(_exponent: i32, _mantissa: i64, _out_buff: *mut u8, _out_buff_len: usize, _rounding_mode: i32) -> i32;
    fn float_cmp(_in_buff1: *const u8, _in_buff1_len: usize, _in_buff2: *const u8, _in_buff2_len: usize) -> i32;
    fn float_add(_in_buff1: *const u8, _in_buff1_len: usize, _in_buff2: *const u8, _in_buff2_len: usize, _out_buff: *mut u8, _out_buff_len: usize, _rounding_mode: i32) -> i32;
    fn float_sub(_in_buff1: *const u8, _in_buff1_len: usize, _in_buff2: *const u8, _in_buff2_len: usize, _out_buff: *mut u8, _out_buff_len: usize, _rounding_mode: i32) -> i32;
    fn float_mult(_in_buff1: *const u8, _in_buff1_len: usize, _in_buff2: *const u8, _in_buff2_len: usize, _out_buff: *mut u8, _out_buff_len: usize, _rounding_mode: i32) -> i32;
    fn float_div(_in_buff1: *const u8, _in_buff1_len: usize, _in_buff2: *const u8, _in_buff2_len: usize, _out_buff: *mut u8, _out_buff_len: usize, _rounding_mode: i32) -> i32;
    fn float_pow(_in_buff: *const u8, _in_buff_len: usize, _pow: i32, _out_buff: *mut u8, _out_buff_len: usize, _rounding_mode: i32) -> i32;
    fn float_root(_in_buff: *const u8, _in_buff_len: usize, _root: i32, _out_buff: *mut u8, _out_buff_len: usize, _rounding_mode: i32) -> i32;
    fn float_log(_in_buff: *const u8, _in_buff_len: usize, _out_buff: *mut u8, _out_buff_len: usize, _rounding_mode: i32) -> i32;

    // Host Function Category: TRACE
    fn trace(_msg_read_ptr: *const u8, _msg_read_len: usize, _data_read_ptr: *const u8, _data_read_len: usize, _as_hex: i32) -> i32;
    fn trace_num(_msg_read_ptr: *const u8, _msg_read_len: usize, _number: i64) -> i32;
    fn trace_acct(_msg_read_ptr: *const u8, _msg_read_len: usize, _account_ptr: *const u8, _account_len: usize) -> i32;
    fn trace_xfloat(_msg_read_ptr: *const u8, _msg_read_len: usize, _opaque_float_ptr: *const u8, _opaque_float_len: usize) -> i32;
    fn trace_amt(_msg_read_ptr: *const u8, _msg_read_len: usize, _amount_ptr: *const u8, _amount_len: usize) -> i32;

}
