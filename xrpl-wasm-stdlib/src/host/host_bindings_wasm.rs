use crate::host::host_bindings_trait::HostBindings;

/// This module hides the actual host functions from outside callers so that the correct
/// implementations are called, regardless of target.
mod host_defined_functions {

    // Defines the `host_lib` functions that will be supplied by the host (i.e., `xrpld`). Note
    // that these functions are declared as `pub(super)` so that only the parent module can access
    // them. This allows the parent module to be the face for any callers of these functions,
    // which is important so that we can swap out this implementation for the non-WASM version
    // found in `host_bindings_test` (e.g., for unit testing purposes).

    #[link(wasm_import_module = "host_lib")]
    unsafe extern "C" {
        #[link_name = "ldgr_index"]
        pub(super) fn get_ledger_sqn(out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
        #[link_name = "parent_ldgr_time"]
        pub(super) fn get_parent_ledger_time(out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
        #[link_name = "parent_ldgr_hash"]
        pub(super) fn get_parent_ledger_hash(out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
        #[link_name = "base_fee"]
        pub(super) fn get_base_fee(out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
        pub(super) fn amendment_enabled(amendment_ptr: *const u8, amendment_len: usize) -> i32;
        #[link_name = "cache_le"]
        pub(super) fn cache_ledger_obj(
            keylet_ptr: *const u8,
            keylet_len: usize,
            cache_num: i32,
        ) -> i32;
        #[link_name = "tx_field"]
        pub(super) fn get_tx_field(field: i32, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
        #[link_name = "home_le_field"]
        pub(super) fn get_current_ledger_obj_field(
            field: i32,
            out_buff_ptr: *mut u8,
            out_buff_len: usize,
        ) -> i32;
        #[link_name = "le_field"]
        pub(super) fn get_ledger_obj_field(
            cache_num: i32,
            field: i32,
            out_buff_ptr: *mut u8,
            out_buff_len: usize,
        ) -> i32;
        #[link_name = "tx_inner"]
        pub(super) fn get_tx_nested_field(
            locator_ptr: *const u8,
            locator_len: usize,
            out_buff_ptr: *mut u8,
            out_buff_len: usize,
        ) -> i32;
        #[link_name = "home_le_inner"]
        pub(super) fn get_current_ledger_obj_nested_field(
            locator_ptr: *const u8,
            locator_len: usize,
            out_buff_ptr: *mut u8,
            out_buff_len: usize,
        ) -> i32;
        #[link_name = "le_inner"]
        pub(super) fn get_ledger_obj_nested_field(
            cache_num: i32,
            locator_ptr: *const u8,
            locator_len: usize,
            out_buff_ptr: *mut u8,
            out_buff_len: usize,
        ) -> i32;
        #[link_name = "tx_arr_len"]
        pub(super) fn get_tx_array_len(field: i32) -> i32;
        #[link_name = "home_le_arr_len"]
        pub(super) fn get_current_ledger_obj_array_len(field: i32) -> i32;
        #[link_name = "le_arr_len"]
        pub(super) fn get_ledger_obj_array_len(cache_num: i32, field: i32) -> i32;
        #[link_name = "tx_inner_arr_len"]
        pub(super) fn get_tx_nested_array_len(locator_ptr: *const u8, locator_len: usize) -> i32;
        #[link_name = "home_le_inner_arr_len"]
        pub(super) fn get_current_ledger_obj_nested_array_len(
            locator_ptr: *const u8,
            locator_len: usize,
        ) -> i32;
        #[link_name = "le_inner_arr_len"]
        pub(super) fn get_ledger_obj_nested_array_len(
            cache_num: i32,
            locator_ptr: *const u8,
            locator_len: usize,
        ) -> i32;
        #[link_name = "set_data"]
        pub(super) fn update_data(data_ptr: *const u8, data_len: usize) -> i32;
        #[link_name = "sha512_half"]
        pub(super) fn compute_sha512_half(
            data_ptr: *const u8,
            data_len: usize,
            out_buff_ptr: *mut u8,
            out_buff_len: usize,
        ) -> i32;
        pub(super) fn check_sig(
            message_ptr: *const u8,
            message_len: usize,
            signature_ptr: *const u8,
            signature_len: usize,
            pubkey_ptr: *const u8,
            pubkey_len: usize,
        ) -> i32;
        #[link_name = "accountroot_id"]
        pub(super) fn account_keylet(
            account_ptr: *const u8,
            account_len: usize,
            out_buff_ptr: *mut u8,
            out_buff_len: usize,
        ) -> i32;
        #[link_name = "amm_id"]
        pub(super) fn amm_keylet(
            issue1_ptr: *const u8,
            issue1_len: usize,
            issue2_ptr: *const u8,
            issue2_len: usize,
            out_buff_ptr: *mut u8,
            out_buff_len: usize,
        ) -> i32;
        #[link_name = "check_id"]
        pub(super) fn check_keylet(
            account_ptr: *const u8,
            account_len: usize,
            sequence_ptr: *const u8,
            sequence_len: usize,
            out_buff_ptr: *mut u8,
            out_buff_len: usize,
        ) -> i32;
        #[link_name = "credential_id"]
        pub(super) fn credential_keylet(
            subject_ptr: *const u8,
            subject_len: usize,
            issuer_ptr: *const u8,
            issuer_len: usize,
            cred_type_ptr: *const u8,
            cred_type_len: usize,
            out_buff_ptr: *mut u8,
            out_buff_len: usize,
        ) -> i32;
        #[link_name = "delegate_id"]
        pub(super) fn delegate_keylet(
            account_ptr: *const u8,
            account_len: usize,
            authorize_ptr: *const u8,
            authorize_len: usize,
            out_buff_ptr: *mut u8,
            out_buff_len: usize,
        ) -> i32;
        #[link_name = "deposit_preauth_id"]
        pub(super) fn deposit_preauth_keylet(
            account_ptr: *const u8,
            account_len: usize,
            authorize_ptr: *const u8,
            authorize_len: usize,
            out_buff_ptr: *mut u8,
            out_buff_len: usize,
        ) -> i32;
        #[link_name = "did_id"]
        pub(super) fn did_keylet(
            account_ptr: *const u8,
            account_len: usize,
            out_buff_ptr: *mut u8,
            out_buff_len: usize,
        ) -> i32;
        #[link_name = "escrow_id"]
        pub(super) fn escrow_keylet(
            account_ptr: *const u8,
            account_len: usize,
            sequence_ptr: *const u8,
            sequence_len: usize,
            out_buff_ptr: *mut u8,
            out_buff_len: usize,
        ) -> i32;
        #[link_name = "trustline_id"]
        pub(super) fn line_keylet(
            account1_ptr: *const u8,
            account1_len: usize,
            account2_ptr: *const u8,
            account2_len: usize,
            currency_ptr: *const u8,
            currency_len: usize,
            out_buff_ptr: *mut u8,
            out_buff_len: usize,
        ) -> i32;
        #[link_name = "mpt_issuance_id"]
        pub(super) fn mpt_issuance_keylet(
            issuer_ptr: *const u8,
            issuer_len: usize,
            sequence_ptr: *const u8,
            sequence_len: usize,
            out_buff_ptr: *mut u8,
            out_buff_len: usize,
        ) -> i32;
        #[link_name = "mptoken_id"]
        pub(super) fn mptoken_keylet(
            mptid_ptr: *const u8,
            mptid_len: usize,
            holder_ptr: *const u8,
            holder_len: usize,
            out_buff_ptr: *mut u8,
            out_buff_len: usize,
        ) -> i32;
        #[link_name = "nft_offer_id"]
        pub(super) fn nft_offer_keylet(
            account_ptr: *const u8,
            account_len: usize,
            sequence_ptr: *const u8,
            sequence_len: usize,
            out_buff_ptr: *mut u8,
            out_buff_len: usize,
        ) -> i32;
        #[link_name = "offer_id"]
        pub(super) fn offer_keylet(
            account_ptr: *const u8,
            account_len: usize,
            sequence_ptr: *const u8,
            sequence_len: usize,
            out_buff_ptr: *mut u8,
            out_buff_len: usize,
        ) -> i32;
        #[link_name = "oracle_id"]
        pub(super) fn oracle_keylet(
            account_ptr: *const u8,
            account_len: usize,
            document_id_ptr: *const u8,
            document_id_len: usize,
            out_buff_ptr: *mut u8,
            out_buff_len: usize,
        ) -> i32;
        #[link_name = "paychan_id"]
        pub(super) fn paychan_keylet(
            account_ptr: *const u8,
            account_len: usize,
            destination_ptr: *const u8,
            destination_len: usize,
            sequence_ptr: *const u8,
            sequence_len: usize,
            out_buff_ptr: *mut u8,
            out_buff_len: usize,
        ) -> i32;
        #[link_name = "permissioned_domain_id"]
        pub(super) fn permissioned_domain_keylet(
            account_ptr: *const u8,
            account_len: usize,
            sequence_ptr: *const u8,
            sequence_len: usize,
            out_buff_ptr: *mut u8,
            out_buff_len: usize,
        ) -> i32;
        #[link_name = "signers_id"]
        pub(super) fn signers_keylet(
            account_ptr: *const u8,
            account_len: usize,
            out_buff_ptr: *mut u8,
            out_buff_len: usize,
        ) -> i32;
        #[link_name = "ticket_id"]
        pub(super) fn ticket_keylet(
            account_ptr: *const u8,
            account_len: usize,
            sequence_ptr: *const u8,
            sequence_len: usize,
            out_buff_ptr: *mut u8,
            out_buff_len: usize,
        ) -> i32;
        #[link_name = "vault_id"]
        pub(super) fn vault_keylet(
            account_ptr: *const u8,
            account_len: usize,
            sequence_ptr: *const u8,
            sequence_len: usize,
            out_buff_ptr: *mut u8,
            out_buff_len: usize,
        ) -> i32;
        #[link_name = "nft_uri"]
        pub(super) fn get_nft(
            account_ptr: *const u8,
            account_len: usize,
            nft_id_ptr: *const u8,
            nft_id_len: usize,
            out_buff_ptr: *mut u8,
            out_buff_len: usize,
        ) -> i32;
        #[link_name = "nft_issuer"]
        pub(super) fn get_nft_issuer(
            nft_id_ptr: *const u8,
            nft_id_len: usize,
            out_buff_ptr: *mut u8,
            out_buff_len: usize,
        ) -> i32;
        #[link_name = "nft_taxon"]
        pub(super) fn get_nft_taxon(
            nft_id_ptr: *const u8,
            nft_id_len: usize,
            out_buff_ptr: *mut u8,
            out_buff_len: usize,
        ) -> i32;
        #[link_name = "nft_flags"]
        pub(super) fn get_nft_flags(nft_id_ptr: *const u8, nft_id_len: usize) -> i32;
        #[link_name = "nft_xfer_fee"]
        pub(super) fn get_nft_transfer_fee(nft_id_ptr: *const u8, nft_id_len: usize) -> i32;
        #[link_name = "nft_serial"]
        pub(super) fn get_nft_serial(
            nft_id_ptr: *const u8,
            nft_id_len: usize,
            out_buff_ptr: *mut u8,
            out_buff_len: usize,
        ) -> i32;
        pub(super) fn float_from_int(
            in_int: i64,
            out_buff: *mut u8,
            out_buff_len: usize,
            rounding_mode: i32,
        ) -> i32;
        pub(super) fn float_from_uint(
            in_uint_ptr: *const u8,
            in_uint_len: usize,
            out_buff: *mut u8,
            out_buff_len: usize,
            rounding_mode: i32,
        ) -> i32;
        #[link_name = "float_from_mant_exp"]
        pub(super) fn float_set(
            mantissa: i64,
            exponent: i32,
            out_buff: *mut u8,
            out_buff_len: usize,
            rounding_mode: i32,
        ) -> i32;
        #[link_name = "float_cmp"]
        pub(super) fn float_compare(
            in_buff1: *const u8,
            in_buff1_len: usize,
            in_buff2: *const u8,
            in_buff2_len: usize,
        ) -> i32;
        pub(super) fn float_add(
            in_buff1: *const u8,
            in_buff1_len: usize,
            in_buff2: *const u8,
            in_buff2_len: usize,
            out_buff: *mut u8,
            out_buff_len: usize,
            rounding_mode: i32,
        ) -> i32;
        #[link_name = "float_sub"]
        pub(super) fn float_subtract(
            in_buff1: *const u8,
            in_buff1_len: usize,
            in_buff2: *const u8,
            in_buff2_len: usize,
            out_buff: *mut u8,
            out_buff_len: usize,
            rounding_mode: i32,
        ) -> i32;
        #[link_name = "float_mult"]
        pub(super) fn float_multiply(
            in_buff1: *const u8,
            in_buff1_len: usize,
            in_buff2: *const u8,
            in_buff2_len: usize,
            out_buff: *mut u8,
            out_buff_len: usize,
            rounding_mode: i32,
        ) -> i32;
        #[link_name = "float_div"]
        pub(super) fn float_divide(
            in_buff1: *const u8,
            in_buff1_len: usize,
            in_buff2: *const u8,
            in_buff2_len: usize,
            out_buff: *mut u8,
            out_buff_len: usize,
            rounding_mode: i32,
        ) -> i32;
        pub(super) fn float_pow(
            in_buff: *const u8,
            in_buff_len: usize,
            pow: i32,
            out_buff: *mut u8,
            out_buff_len: usize,
            rounding_mode: i32,
        ) -> i32;
        pub(super) fn trace(
            msg_read_ptr: *const u8,
            msg_read_len: usize,
            data_type: i32,
            data_read_ptr: *const u8,
            data_read_len: usize,
        );
        pub(super) fn instance_param(
            index: i32,
            st_type_id: i32,
            out_buff_ptr: *mut u8,
            out_buff_len: usize,
        ) -> i32;
        pub(super) fn function_param(
            index: i32,
            st_type_id: i32,
            out_buff_ptr: *mut u8,
            out_buff_len: usize,
        ) -> i32;
        pub(super) fn get_data_object_field(
            account_ptr: *const u8,
            account_len: usize,
            key_ptr: *const u8,
            key_len: usize,
            out_buff_ptr: *const u8,
            out_buff_len: usize,
        ) -> i32;
        pub(super) fn get_data_nested_object_field(
            account_ptr: *const u8,
            account_len: usize,
            key_ptr: *const u8,
            key_len: usize,
            nst_ptr: *const u8,
            nst_len: usize,
            out_buff_ptr: *const u8,
            out_buff_len: usize,
        ) -> i32;
        pub(super) fn get_data_array_element_field(
            account_ptr: *const u8,
            account_len: usize,
            key_ptr: *const u8,
            key_len: usize,
            index: i32,
            out_buff_ptr: *const u8,
            out_buff_len: usize,
        ) -> i32;
        pub(super) fn get_data_nested_array_element_field(
            account_ptr: *const u8,
            account_len: usize,
            key_ptr: *const u8,
            key_len: usize,
            index: i32,
            nst_ptr: *const u8,
            nst_len: usize,
            out_buff_ptr: *const u8,
            out_buff_len: usize,
        ) -> i32;
        pub(super) fn set_data_object_field(
            account_ptr: *const u8,
            account_len: usize,
            key_ptr: *const u8,
            key_len: usize,
            data_ptr: *const u8,
            data_len: usize,
        ) -> i32;
        pub(super) fn set_data_nested_object_field(
            account_ptr: *const u8,
            account_len: usize,
            key_ptr: *const u8,
            key_len: usize,
            nst_ptr: *const u8,
            nst_len: usize,
            data_ptr: *const u8,
            data_len: usize,
        ) -> i32;
        pub(super) fn set_data_array_element_field(
            account_ptr: *const u8,
            account_len: usize,
            key_ptr: *const u8,
            key_len: usize,
            index: i32,
            data_ptr: *const u8,
            data_len: usize,
        ) -> i32;
        pub(super) fn set_data_nested_array_element_field(
            account_ptr: *const u8,
            account_len: usize,
            key_ptr: *const u8,
            key_len: usize,
            index: i32,
            nst_ptr: *const u8,
            nst_len: usize,
            data_ptr: *const u8,
            data_len: usize,
        ) -> i32;
        pub(super) fn build_txn(txn_type: i32) -> i32;
        pub(super) fn add_txn_field(
            index: i32,
            field: i32,
            write_ptr: *const u8,
            write_len: usize,
        ) -> i32;
        pub(super) fn emit_built_txn(index: i32, out_buff_ptr: *mut u8, out_buff_len: usize)
        -> i32;
        pub(super) fn emit_txn(
            txn_read_ptr: *const u8,
            txn_read_len: usize,
            out_buff_ptr: *mut u8,
            out_buff_len: usize,
        ) -> i32;
        pub(super) fn emit_event(
            name_ptr: *const u8,
            name_len: usize,
            data_ptr: *const u8,
            data_len: usize,
        ) -> i32;
    }
}

// `TraceDataType` codes from xrpld's host ABI.
const TRACE_INT64: i32 = 1;
const TRACE_XFLOAT: i32 = 3;
const TRACE_ACCOUNT: i32 = 4;
const TRACE_AMOUNT: i32 = 5;
const TRACE_AS_HEX: i32 = 6;
const TRACE_AS_TEXT: i32 = 7;

/// `HostError::Unimplemented`, for functions xrpld no longer provides.
const HOST_UNIMPLEMENTED: i32 = -1;

/// xrpld has one `trace` import that takes a data type and returns nothing.
unsafe fn trace_typed(
    msg_ptr: *const u8,
    msg_len: usize,
    data_type: i32,
    data_ptr: *const u8,
    data_len: usize,
) -> i32 {
    unsafe { host_defined_functions::trace(msg_ptr, msg_len, data_type, data_ptr, data_len) };
    0
}

/// Implementation of host bindings for WASM targets.
pub struct WasmHostBindings;

/// WASM implementation of HostBindings.
impl HostBindings for WasmHostBindings {
    unsafe fn get_ledger_sqn(&self, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32 {
        unsafe { host_defined_functions::get_ledger_sqn(out_buff_ptr, out_buff_len) }
    }

    unsafe fn get_parent_ledger_time(&self, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32 {
        unsafe { host_defined_functions::get_parent_ledger_time(out_buff_ptr, out_buff_len) }
    }

    unsafe fn get_parent_ledger_hash(&self, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32 {
        unsafe { host_defined_functions::get_parent_ledger_hash(out_buff_ptr, out_buff_len) }
    }

    unsafe fn get_base_fee(&self, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32 {
        unsafe { host_defined_functions::get_base_fee(out_buff_ptr, out_buff_len) }
    }

    unsafe fn amendment_enabled(&self, amendment_ptr: *const u8, amendment_len: usize) -> i32 {
        unsafe { host_defined_functions::amendment_enabled(amendment_ptr, amendment_len) }
    }

    unsafe fn cache_ledger_obj(
        &self,
        keylet_ptr: *const u8,
        keylet_len: usize,
        cache_num: i32,
    ) -> i32 {
        unsafe { host_defined_functions::cache_ledger_obj(keylet_ptr, keylet_len, cache_num) }
    }

    unsafe fn get_tx_field(&self, field: i32, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32 {
        unsafe { host_defined_functions::get_tx_field(field, out_buff_ptr, out_buff_len) }
    }

    unsafe fn get_current_ledger_obj_field(
        &self,
        field: i32,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::get_current_ledger_obj_field(field, out_buff_ptr, out_buff_len)
        }
    }

    unsafe fn get_ledger_obj_field(
        &self,
        cache_num: i32,
        field: i32,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::get_ledger_obj_field(
                cache_num,
                field,
                out_buff_ptr,
                out_buff_len,
            )
        }
    }

    unsafe fn get_tx_nested_field(
        &self,
        locator_ptr: *const u8,
        locator_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::get_tx_nested_field(
                locator_ptr,
                locator_len,
                out_buff_ptr,
                out_buff_len,
            )
        }
    }

    unsafe fn get_current_ledger_obj_nested_field(
        &self,
        locator_ptr: *const u8,
        locator_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::get_current_ledger_obj_nested_field(
                locator_ptr,
                locator_len,
                out_buff_ptr,
                out_buff_len,
            )
        }
    }

    unsafe fn get_ledger_obj_nested_field(
        &self,
        cache_num: i32,
        locator_ptr: *const u8,
        locator_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::get_ledger_obj_nested_field(
                cache_num,
                locator_ptr,
                locator_len,
                out_buff_ptr,
                out_buff_len,
            )
        }
    }

    unsafe fn get_tx_array_len(&self, field: i32) -> i32 {
        unsafe { host_defined_functions::get_tx_array_len(field) }
    }

    unsafe fn get_current_ledger_obj_array_len(&self, field: i32) -> i32 {
        unsafe { host_defined_functions::get_current_ledger_obj_array_len(field) }
    }

    unsafe fn get_ledger_obj_array_len(&self, cache_num: i32, field: i32) -> i32 {
        unsafe { host_defined_functions::get_ledger_obj_array_len(cache_num, field) }
    }

    unsafe fn get_tx_nested_array_len(&self, locator_ptr: *const u8, locator_len: usize) -> i32 {
        unsafe { host_defined_functions::get_tx_nested_array_len(locator_ptr, locator_len) }
    }

    unsafe fn get_current_ledger_obj_nested_array_len(
        &self,
        locator_ptr: *const u8,
        locator_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::get_current_ledger_obj_nested_array_len(
                locator_ptr,
                locator_len,
            )
        }
    }

    unsafe fn get_ledger_obj_nested_array_len(
        &self,
        cache_num: i32,
        locator_ptr: *const u8,
        locator_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::get_ledger_obj_nested_array_len(
                cache_num,
                locator_ptr,
                locator_len,
            )
        }
    }

    unsafe fn update_data(&self, data_ptr: *const u8, data_len: usize) -> i32 {
        unsafe { host_defined_functions::update_data(data_ptr, data_len) }
    }

    unsafe fn compute_sha512_half(
        &self,
        data_ptr: *const u8,
        data_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::compute_sha512_half(
                data_ptr,
                data_len,
                out_buff_ptr,
                out_buff_len,
            )
        }
    }

    unsafe fn check_sig(
        &self,
        message_ptr: *const u8,
        message_len: usize,
        signature_ptr: *const u8,
        signature_len: usize,
        pubkey_ptr: *const u8,
        pubkey_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::check_sig(
                message_ptr,
                message_len,
                signature_ptr,
                signature_len,
                pubkey_ptr,
                pubkey_len,
            )
        }
    }

    unsafe fn account_keylet(
        &self,
        account_ptr: *const u8,
        account_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::account_keylet(
                account_ptr,
                account_len,
                out_buff_ptr,
                out_buff_len,
            )
        }
    }

    unsafe fn amm_keylet(
        &self,
        issue1_ptr: *const u8,
        issue1_len: usize,
        issue2_ptr: *const u8,
        issue2_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::amm_keylet(
                issue1_ptr,
                issue1_len,
                issue2_ptr,
                issue2_len,
                out_buff_ptr,
                out_buff_len,
            )
        }
    }

    unsafe fn check_keylet(
        &self,
        account_ptr: *const u8,
        account_len: usize,
        sequence_ptr: *const u8,
        sequence_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::check_keylet(
                account_ptr,
                account_len,
                sequence_ptr,
                sequence_len,
                out_buff_ptr,
                out_buff_len,
            )
        }
    }

    unsafe fn credential_keylet(
        &self,
        subject_ptr: *const u8,
        subject_len: usize,
        issuer_ptr: *const u8,
        issuer_len: usize,
        cred_type_ptr: *const u8,
        cred_type_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::credential_keylet(
                subject_ptr,
                subject_len,
                issuer_ptr,
                issuer_len,
                cred_type_ptr,
                cred_type_len,
                out_buff_ptr,
                out_buff_len,
            )
        }
    }

    unsafe fn delegate_keylet(
        &self,
        account_ptr: *const u8,
        account_len: usize,
        authorize_ptr: *const u8,
        authorize_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::delegate_keylet(
                account_ptr,
                account_len,
                authorize_ptr,
                authorize_len,
                out_buff_ptr,
                out_buff_len,
            )
        }
    }

    unsafe fn deposit_preauth_keylet(
        &self,
        account_ptr: *const u8,
        account_len: usize,
        authorize_ptr: *const u8,
        authorize_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::deposit_preauth_keylet(
                account_ptr,
                account_len,
                authorize_ptr,
                authorize_len,
                out_buff_ptr,
                out_buff_len,
            )
        }
    }

    unsafe fn did_keylet(
        &self,
        account_ptr: *const u8,
        account_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::did_keylet(account_ptr, account_len, out_buff_ptr, out_buff_len)
        }
    }

    unsafe fn escrow_keylet(
        &self,
        account_ptr: *const u8,
        account_len: usize,
        sequence_ptr: *const u8,
        sequence_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::escrow_keylet(
                account_ptr,
                account_len,
                sequence_ptr,
                sequence_len,
                out_buff_ptr,
                out_buff_len,
            )
        }
    }

    unsafe fn line_keylet(
        &self,
        account1_ptr: *const u8,
        account1_len: usize,
        account2_ptr: *const u8,
        account2_len: usize,
        currency_ptr: *const u8,
        currency_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::line_keylet(
                account1_ptr,
                account1_len,
                account2_ptr,
                account2_len,
                currency_ptr,
                currency_len,
                out_buff_ptr,
                out_buff_len,
            )
        }
    }

    unsafe fn mpt_issuance_keylet(
        &self,
        issuer_ptr: *const u8,
        issuer_len: usize,
        sequence_ptr: *const u8,
        sequence_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::mpt_issuance_keylet(
                issuer_ptr,
                issuer_len,
                sequence_ptr,
                sequence_len,
                out_buff_ptr,
                out_buff_len,
            )
        }
    }

    unsafe fn mptoken_keylet(
        &self,
        mptid_ptr: *const u8,
        mptid_len: usize,
        holder_ptr: *const u8,
        holder_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::mptoken_keylet(
                mptid_ptr,
                mptid_len,
                holder_ptr,
                holder_len,
                out_buff_ptr,
                out_buff_len,
            )
        }
    }

    unsafe fn nft_offer_keylet(
        &self,
        account_ptr: *const u8,
        account_len: usize,
        sequence_ptr: *const u8,
        sequence_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::nft_offer_keylet(
                account_ptr,
                account_len,
                sequence_ptr,
                sequence_len,
                out_buff_ptr,
                out_buff_len,
            )
        }
    }

    unsafe fn offer_keylet(
        &self,
        account_ptr: *const u8,
        account_len: usize,
        sequence_ptr: *const u8,
        sequence_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::offer_keylet(
                account_ptr,
                account_len,
                sequence_ptr,
                sequence_len,
                out_buff_ptr,
                out_buff_len,
            )
        }
    }

    unsafe fn oracle_keylet(
        &self,
        account_ptr: *const u8,
        account_len: usize,
        document_id_ptr: *const u8,
        document_id_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::oracle_keylet(
                account_ptr,
                account_len,
                document_id_ptr,
                document_id_len,
                out_buff_ptr,
                out_buff_len,
            )
        }
    }

    unsafe fn paychan_keylet(
        &self,
        account_ptr: *const u8,
        account_len: usize,
        destination_ptr: *const u8,
        destination_len: usize,
        sequence_ptr: *const u8,
        sequence_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::paychan_keylet(
                account_ptr,
                account_len,
                destination_ptr,
                destination_len,
                sequence_ptr,
                sequence_len,
                out_buff_ptr,
                out_buff_len,
            )
        }
    }

    unsafe fn permissioned_domain_keylet(
        &self,
        account_ptr: *const u8,
        account_len: usize,
        sequence_ptr: *const u8,
        sequence_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::permissioned_domain_keylet(
                account_ptr,
                account_len,
                sequence_ptr,
                sequence_len,
                out_buff_ptr,
                out_buff_len,
            )
        }
    }

    unsafe fn signers_keylet(
        &self,
        account_ptr: *const u8,
        account_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::signers_keylet(
                account_ptr,
                account_len,
                out_buff_ptr,
                out_buff_len,
            )
        }
    }

    unsafe fn ticket_keylet(
        &self,
        account_ptr: *const u8,
        account_len: usize,
        sequence_ptr: *const u8,
        sequence_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::ticket_keylet(
                account_ptr,
                account_len,
                sequence_ptr,
                sequence_len,
                out_buff_ptr,
                out_buff_len,
            )
        }
    }

    unsafe fn vault_keylet(
        &self,
        account_ptr: *const u8,
        account_len: usize,
        sequence_ptr: *const u8,
        sequence_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::vault_keylet(
                account_ptr,
                account_len,
                sequence_ptr,
                sequence_len,
                out_buff_ptr,
                out_buff_len,
            )
        }
    }

    unsafe fn get_nft(
        &self,
        account_ptr: *const u8,
        account_len: usize,
        nft_id_ptr: *const u8,
        nft_id_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::get_nft(
                account_ptr,
                account_len,
                nft_id_ptr,
                nft_id_len,
                out_buff_ptr,
                out_buff_len,
            )
        }
    }

    unsafe fn get_nft_issuer(
        &self,
        nft_id_ptr: *const u8,
        nft_id_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::get_nft_issuer(
                nft_id_ptr,
                nft_id_len,
                out_buff_ptr,
                out_buff_len,
            )
        }
    }

    unsafe fn get_nft_taxon(
        &self,
        nft_id_ptr: *const u8,
        nft_id_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::get_nft_taxon(
                nft_id_ptr,
                nft_id_len,
                out_buff_ptr,
                out_buff_len,
            )
        }
    }

    unsafe fn get_nft_flags(&self, nft_id_ptr: *const u8, nft_id_len: usize) -> i32 {
        unsafe { host_defined_functions::get_nft_flags(nft_id_ptr, nft_id_len) }
    }

    unsafe fn get_nft_transfer_fee(&self, nft_id_ptr: *const u8, nft_id_len: usize) -> i32 {
        unsafe { host_defined_functions::get_nft_transfer_fee(nft_id_ptr, nft_id_len) }
    }

    unsafe fn get_nft_serial(
        &self,
        nft_id_ptr: *const u8,
        nft_id_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::get_nft_serial(
                nft_id_ptr,
                nft_id_len,
                out_buff_ptr,
                out_buff_len,
            )
        }
    }

    unsafe fn float_from_int(
        &self,
        in_int: i64,
        out_buff: *mut u8,
        out_buff_len: usize,
        rounding_mode: i32,
    ) -> i32 {
        unsafe {
            host_defined_functions::float_from_int(in_int, out_buff, out_buff_len, rounding_mode)
        }
    }

    unsafe fn float_from_uint(
        &self,
        in_uint_ptr: *const u8,
        in_uint_len: usize,
        out_buff: *mut u8,
        out_buff_len: usize,
        rounding_mode: i32,
    ) -> i32 {
        unsafe {
            host_defined_functions::float_from_uint(
                in_uint_ptr,
                in_uint_len,
                out_buff,
                out_buff_len,
                rounding_mode,
            )
        }
    }

    unsafe fn float_set(
        &self,
        exponent: i32,
        mantissa: i64,
        out_buff: *mut u8,
        out_buff_len: usize,
        rounding_mode: i32,
    ) -> i32 {
        unsafe {
            host_defined_functions::float_set(
                mantissa,
                exponent,
                out_buff,
                out_buff_len,
                rounding_mode,
            )
        }
    }

    unsafe fn float_compare(
        &self,
        in_buff1: *const u8,
        in_buff1_len: usize,
        in_buff2: *const u8,
        in_buff2_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::float_compare(in_buff1, in_buff1_len, in_buff2, in_buff2_len)
        }
    }

    unsafe fn float_add(
        &self,
        in_buff1: *const u8,
        in_buff1_len: usize,
        in_buff2: *const u8,
        in_buff2_len: usize,
        out_buff: *mut u8,
        out_buff_len: usize,
        rounding_mode: i32,
    ) -> i32 {
        unsafe {
            host_defined_functions::float_add(
                in_buff1,
                in_buff1_len,
                in_buff2,
                in_buff2_len,
                out_buff,
                out_buff_len,
                rounding_mode,
            )
        }
    }

    unsafe fn float_subtract(
        &self,
        in_buff1: *const u8,
        in_buff1_len: usize,
        in_buff2: *const u8,
        in_buff2_len: usize,
        out_buff: *mut u8,
        out_buff_len: usize,
        rounding_mode: i32,
    ) -> i32 {
        unsafe {
            host_defined_functions::float_subtract(
                in_buff1,
                in_buff1_len,
                in_buff2,
                in_buff2_len,
                out_buff,
                out_buff_len,
                rounding_mode,
            )
        }
    }

    unsafe fn float_multiply(
        &self,
        in_buff1: *const u8,
        in_buff1_len: usize,
        in_buff2: *const u8,
        in_buff2_len: usize,
        out_buff: *mut u8,
        out_buff_len: usize,
        rounding_mode: i32,
    ) -> i32 {
        unsafe {
            host_defined_functions::float_multiply(
                in_buff1,
                in_buff1_len,
                in_buff2,
                in_buff2_len,
                out_buff,
                out_buff_len,
                rounding_mode,
            )
        }
    }

    unsafe fn float_divide(
        &self,
        in_buff1: *const u8,
        in_buff1_len: usize,
        in_buff2: *const u8,
        in_buff2_len: usize,
        out_buff: *mut u8,
        out_buff_len: usize,
        rounding_mode: i32,
    ) -> i32 {
        unsafe {
            host_defined_functions::float_divide(
                in_buff1,
                in_buff1_len,
                in_buff2,
                in_buff2_len,
                out_buff,
                out_buff_len,
                rounding_mode,
            )
        }
    }

    unsafe fn float_pow(
        &self,
        in_buff: *const u8,
        in_buff_len: usize,
        pow: i32,
        out_buff: *mut u8,
        out_buff_len: usize,
        rounding_mode: i32,
    ) -> i32 {
        unsafe {
            host_defined_functions::float_pow(
                in_buff,
                in_buff_len,
                pow,
                out_buff,
                out_buff_len,
                rounding_mode,
            )
        }
    }

    unsafe fn float_root(
        &self,
        in_buff: *const u8,
        in_buff_len: usize,
        root: i32,
        out_buff: *mut u8,
        out_buff_len: usize,
        rounding_mode: i32,
    ) -> i32 {
        let _ = (
            in_buff,
            in_buff_len,
            root,
            out_buff,
            out_buff_len,
            rounding_mode,
        );
        HOST_UNIMPLEMENTED
    }

    unsafe fn float_log(
        &self,
        in_buff: *const u8,
        in_buff_len: usize,
        out_buff: *mut u8,
        out_buff_len: usize,
        rounding_mode: i32,
    ) -> i32 {
        let _ = (in_buff, in_buff_len, out_buff, out_buff_len, rounding_mode);
        HOST_UNIMPLEMENTED
    }

    unsafe fn trace(
        &self,
        msg_read_ptr: *const u8,
        msg_read_len: usize,
        data_read_ptr: *const u8,
        data_read_len: usize,
        as_hex: i32,
    ) -> i32 {
        let data_type = if as_hex != 0 {
            TRACE_AS_HEX
        } else {
            TRACE_AS_TEXT
        };
        unsafe {
            trace_typed(
                msg_read_ptr,
                msg_read_len,
                data_type,
                data_read_ptr,
                data_read_len,
            )
        }
    }

    unsafe fn trace_num(&self, msg_read_ptr: *const u8, msg_read_len: usize, number: i64) -> i32 {
        let bytes = number.to_le_bytes();
        unsafe {
            trace_typed(
                msg_read_ptr,
                msg_read_len,
                TRACE_INT64,
                bytes.as_ptr(),
                bytes.len(),
            )
        }
    }

    unsafe fn trace_account(
        &self,
        msg_read_ptr: *const u8,
        msg_read_len: usize,
        account_ptr: *const u8,
        account_len: usize,
    ) -> i32 {
        unsafe {
            trace_typed(
                msg_read_ptr,
                msg_read_len,
                TRACE_ACCOUNT,
                account_ptr,
                account_len,
            )
        }
    }

    unsafe fn trace_opaque_float(
        &self,
        msg_read_ptr: *const u8,
        msg_read_len: usize,
        opaque_float_ptr: *const u8,
        opaque_float_len: usize,
    ) -> i32 {
        unsafe {
            trace_typed(
                msg_read_ptr,
                msg_read_len,
                TRACE_XFLOAT,
                opaque_float_ptr,
                opaque_float_len,
            )
        }
    }

    unsafe fn trace_amount(
        &self,
        msg_read_ptr: *const u8,
        msg_read_len: usize,
        amount_ptr: *const u8,
        amount_len: usize,
    ) -> i32 {
        unsafe {
            trace_typed(
                msg_read_ptr,
                msg_read_len,
                TRACE_AMOUNT,
                amount_ptr,
                amount_len,
            )
        }
    }

    unsafe fn instance_param(
        &self,
        index: i32,
        st_type_id: i32,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::instance_param(index, st_type_id, out_buff_ptr, out_buff_len)
        }
    }

    unsafe fn function_param(
        &self,
        index: i32,
        st_type_id: i32,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::function_param(index, st_type_id, out_buff_ptr, out_buff_len)
        }
    }

    unsafe fn get_data_object_field(
        &self,
        account_ptr: *const u8,
        account_len: usize,
        key_ptr: *const u8,
        key_len: usize,
        out_buff_ptr: *const u8,
        out_buff_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::get_data_object_field(
                account_ptr,
                account_len,
                key_ptr,
                key_len,
                out_buff_ptr,
                out_buff_len,
            )
        }
    }

    unsafe fn get_data_nested_object_field(
        &self,
        account_ptr: *const u8,
        account_len: usize,
        key_ptr: *const u8,
        key_len: usize,
        nst_ptr: *const u8,
        nst_len: usize,
        out_buff_ptr: *const u8,
        out_buff_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::get_data_nested_object_field(
                account_ptr,
                account_len,
                key_ptr,
                key_len,
                nst_ptr,
                nst_len,
                out_buff_ptr,
                out_buff_len,
            )
        }
    }

    unsafe fn get_data_array_element_field(
        &self,
        account_ptr: *const u8,
        account_len: usize,
        key_ptr: *const u8,
        key_len: usize,
        index: i32,
        out_buff_ptr: *const u8,
        out_buff_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::get_data_array_element_field(
                account_ptr,
                account_len,
                key_ptr,
                key_len,
                index,
                out_buff_ptr,
                out_buff_len,
            )
        }
    }

    unsafe fn get_data_nested_array_element_field(
        &self,
        account_ptr: *const u8,
        account_len: usize,
        key_ptr: *const u8,
        key_len: usize,
        index: i32,
        nst_ptr: *const u8,
        nst_len: usize,
        out_buff_ptr: *const u8,
        out_buff_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::get_data_nested_array_element_field(
                account_ptr,
                account_len,
                key_ptr,
                key_len,
                index,
                nst_ptr,
                nst_len,
                out_buff_ptr,
                out_buff_len,
            )
        }
    }

    unsafe fn set_data_object_field(
        &self,
        account_ptr: *const u8,
        account_len: usize,
        key_ptr: *const u8,
        key_len: usize,
        data_ptr: *const u8,
        data_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::set_data_object_field(
                account_ptr,
                account_len,
                key_ptr,
                key_len,
                data_ptr,
                data_len,
            )
        }
    }

    unsafe fn set_data_nested_object_field(
        &self,
        account_ptr: *const u8,
        account_len: usize,
        key_ptr: *const u8,
        key_len: usize,
        nst_ptr: *const u8,
        nst_len: usize,
        data_ptr: *const u8,
        data_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::set_data_nested_object_field(
                account_ptr,
                account_len,
                key_ptr,
                key_len,
                nst_ptr,
                nst_len,
                data_ptr,
                data_len,
            )
        }
    }

    unsafe fn set_data_array_element_field(
        &self,
        account_ptr: *const u8,
        account_len: usize,
        key_ptr: *const u8,
        key_len: usize,
        index: i32,
        data_ptr: *const u8,
        data_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::set_data_array_element_field(
                account_ptr,
                account_len,
                key_ptr,
                key_len,
                index,
                data_ptr,
                data_len,
            )
        }
    }

    unsafe fn set_data_nested_array_element_field(
        &self,
        account_ptr: *const u8,
        account_len: usize,
        key_ptr: *const u8,
        key_len: usize,
        index: i32,
        nst_ptr: *const u8,
        nst_len: usize,
        data_ptr: *const u8,
        data_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::set_data_nested_array_element_field(
                account_ptr,
                account_len,
                key_ptr,
                key_len,
                index,
                nst_ptr,
                nst_len,
                data_ptr,
                data_len,
            )
        }
    }

    unsafe fn build_txn(&self, txn_type: i32) -> i32 {
        unsafe { host_defined_functions::build_txn(txn_type) }
    }

    unsafe fn add_txn_field(
        &self,
        index: i32,
        field: i32,
        write_ptr: *const u8,
        write_len: usize,
    ) -> i32 {
        unsafe { host_defined_functions::add_txn_field(index, field, write_ptr, write_len) }
    }

    unsafe fn emit_built_txn(&self, index: i32) -> i32 {
        let mut ter = [0u8; 4];
        let len =
            unsafe { host_defined_functions::emit_built_txn(index, ter.as_mut_ptr(), ter.len()) };
        if len < 0 {
            len
        } else {
            i32::from_le_bytes(ter)
        }
    }

    unsafe fn emit_txn(&self, txn_read_ptr: *const u8, txn_read_len: usize) -> i32 {
        let mut ter = [0u8; 4];
        let len = unsafe {
            host_defined_functions::emit_txn(
                txn_read_ptr,
                txn_read_len,
                ter.as_mut_ptr(),
                ter.len(),
            )
        };
        if len < 0 {
            len
        } else {
            i32::from_le_bytes(ter)
        }
    }

    unsafe fn emit_event(
        &self,
        name_ptr: *const u8,
        name_len: usize,
        data_ptr: *const u8,
        data_len: usize,
    ) -> i32 {
        unsafe { host_defined_functions::emit_event(name_ptr, name_len, data_ptr, data_len) }
    }
}

// Re-export all host functions as public functions for use by the rest of the codebase
// These create a WasmHostBindings instance and delegate to the trait methods

// Macro to generate re-export functions that delegate to WasmHostBindings
macro_rules! export_host_functions {
    ($(
        $(#[$attr:meta])*
        fn $name:ident($($param:ident: $param_ty:ty),*) -> $ret:ty;
    )*) => {
        $(
            $(#[$attr])*
            pub unsafe fn $name($($param: $param_ty),*) -> $ret {
                unsafe { WasmHostBindings.$name($($param),*) }
            }
        )*
    };
}

// Generate all the re-export functions
export_host_functions! {
    // Host Function Category: ledger and transaction info
    fn get_ledger_sqn(out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn get_parent_ledger_time(out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn get_parent_ledger_hash(out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn get_base_fee(out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn amendment_enabled(amendment_ptr: *const u8, amendment_len: usize) -> i32;
    fn cache_ledger_obj(keylet_ptr: *const u8, keylet_len: usize, cache_num: i32) -> i32;
    fn get_tx_field(field: i32, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn get_current_ledger_obj_field(field: i32, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn get_ledger_obj_field(cache_num: i32, field: i32, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn get_tx_nested_field(locator_ptr: *const u8, locator_len: usize, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn get_current_ledger_obj_nested_field(locator_ptr: *const u8, locator_len: usize, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn get_ledger_obj_nested_field(cache_num: i32, locator_ptr: *const u8, locator_len: usize, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn get_tx_array_len(field: i32) -> i32;
    fn get_current_ledger_obj_array_len(field: i32) -> i32;
    fn get_ledger_obj_array_len(cache_num: i32, field: i32) -> i32;
    fn get_tx_nested_array_len(locator_ptr: *const u8, locator_len: usize) -> i32;
    fn get_current_ledger_obj_nested_array_len(locator_ptr: *const u8, locator_len: usize) -> i32;
    fn get_ledger_obj_nested_array_len(cache_num: i32, locator_ptr: *const u8, locator_len: usize) -> i32;

    // Host Function Category: update current ledger entry
    fn update_data(data_ptr: *const u8, data_len: usize) -> i32;

    // Host Function Category: hash and keylet computation
    fn compute_sha512_half(data_ptr: *const u8, data_len: usize, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn check_sig(message_ptr: *const u8, message_len: usize, signature_ptr: *const u8, signature_len: usize, pubkey_ptr: *const u8, pubkey_len: usize) -> i32;
    fn account_keylet(account_ptr: *const u8, account_len: usize, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn amm_keylet(issue1_ptr: *const u8, issue1_len: usize, issue2_ptr: *const u8, issue2_len: usize, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn check_keylet(account_ptr: *const u8, account_len: usize, sequence_ptr: *const u8, sequence_len: usize, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn credential_keylet(subject_ptr: *const u8, subject_len: usize, issuer_ptr: *const u8, issuer_len: usize, cred_type_ptr: *const u8, cred_type_len: usize, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn delegate_keylet(account_ptr: *const u8, account_len: usize, authorize_ptr: *const u8, authorize_len: usize, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn deposit_preauth_keylet(account_ptr: *const u8, account_len: usize, authorize_ptr: *const u8, authorize_len: usize, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn did_keylet(account_ptr: *const u8, account_len: usize, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn escrow_keylet(account_ptr: *const u8, account_len: usize, sequence_ptr: *const u8, sequence_len: usize, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn line_keylet(account1_ptr: *const u8, account1_len: usize, account2_ptr: *const u8, account2_len: usize, currency_ptr: *const u8, currency_len: usize, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn mpt_issuance_keylet(issuer_ptr: *const u8, issuer_len: usize, sequence_ptr: *const u8, sequence_len: usize, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn mptoken_keylet(mptid_ptr: *const u8, mptid_len: usize, holder_ptr: *const u8, holder_len: usize, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn nft_offer_keylet(account_ptr: *const u8, account_len: usize, sequence_ptr: *const u8, sequence_len: usize, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn offer_keylet(account_ptr: *const u8, account_len: usize, sequence_ptr: *const u8, sequence_len: usize, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn oracle_keylet(account_ptr: *const u8, account_len: usize, document_id_ptr: *const u8, document_id_len: usize, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn paychan_keylet(account_ptr: *const u8, account_len: usize, destination_ptr: *const u8, destination_len: usize, sequence_ptr: *const u8, sequence_len: usize, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn permissioned_domain_keylet(account_ptr: *const u8, account_len: usize, sequence_ptr: *const u8, sequence_len: usize, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn signers_keylet(account_ptr: *const u8, account_len: usize, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn ticket_keylet(account_ptr: *const u8, account_len: usize, sequence_ptr: *const u8, sequence_len: usize, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn vault_keylet(account_ptr: *const u8, account_len: usize, sequence_ptr: *const u8, sequence_len: usize, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;

    // Host Function Category: NFT
    fn get_nft(account_ptr: *const u8, account_len: usize, nft_id_ptr: *const u8, nft_id_len: usize, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn get_nft_issuer(nft_id_ptr: *const u8, nft_id_len: usize, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn get_nft_taxon(nft_id_ptr: *const u8, nft_id_len: usize, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn get_nft_flags(nft_id_ptr: *const u8, nft_id_len: usize) -> i32;
    fn get_nft_transfer_fee(nft_id_ptr: *const u8, nft_id_len: usize) -> i32;
    fn get_nft_serial(nft_id_ptr: *const u8, nft_id_len: usize, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;

    // Host Function Category: FLOAT
    fn float_from_int(in_int: i64, out_buff: *mut u8, out_buff_len: usize, rounding_mode: i32) -> i32;
    fn float_from_uint(in_uint_ptr: *const u8, in_uint_len: usize, out_buff: *mut u8, out_buff_len: usize, rounding_mode: i32) -> i32;
    fn float_set(exponent: i32, mantissa: i64, out_buff: *mut u8, out_buff_len: usize, rounding_mode: i32) -> i32;
    fn float_compare(in_buff1: *const u8, in_buff1_len: usize, in_buff2: *const u8, in_buff2_len: usize) -> i32;
    fn float_add(in_buff1: *const u8, in_buff1_len: usize, in_buff2: *const u8, in_buff2_len: usize, out_buff: *mut u8, out_buff_len: usize, rounding_mode: i32) -> i32;
    fn float_subtract(in_buff1: *const u8, in_buff1_len: usize, in_buff2: *const u8, in_buff2_len: usize, out_buff: *mut u8, out_buff_len: usize, rounding_mode: i32) -> i32;
    fn float_multiply(in_buff1: *const u8, in_buff1_len: usize, in_buff2: *const u8, in_buff2_len: usize, out_buff: *mut u8, out_buff_len: usize, rounding_mode: i32) -> i32;
    fn float_divide(in_buff1: *const u8, in_buff1_len: usize, in_buff2: *const u8, in_buff2_len: usize, out_buff: *mut u8, out_buff_len: usize, rounding_mode: i32) -> i32;
    fn float_pow(in_buff: *const u8, in_buff_len: usize, pow: i32, out_buff: *mut u8, out_buff_len: usize, rounding_mode: i32) -> i32;
    fn float_root(in_buff: *const u8, in_buff_len: usize, root: i32, out_buff: *mut u8, out_buff_len: usize, rounding_mode: i32) -> i32;
    fn float_log(in_buff: *const u8, in_buff_len: usize, out_buff: *mut u8, out_buff_len: usize, rounding_mode: i32) -> i32;

    // Host Function Category: TRACE
    fn trace(msg_read_ptr: *const u8, msg_read_len: usize, data_read_ptr: *const u8, data_read_len: usize, as_hex: i32) -> i32;
    fn trace_num(msg_read_ptr: *const u8, msg_read_len: usize, number: i64) -> i32;
    fn trace_account(msg_read_ptr: *const u8, msg_read_len: usize, account_ptr: *const u8, account_len: usize) -> i32;
    fn trace_opaque_float(msg_read_ptr: *const u8, msg_read_len: usize, opaque_float_ptr: *const u8, opaque_float_len: usize) -> i32;
    fn trace_amount(msg_read_ptr: *const u8, msg_read_len: usize, amount_ptr: *const u8, amount_len: usize) -> i32;

    // Other functions
    fn instance_param(index: i32, st_type_id: i32, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn function_param(index: i32, st_type_id: i32, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    fn get_data_object_field(account_ptr: *const u8, account_len: usize, key_ptr: *const u8, key_len: usize, out_buff_ptr: *const u8, out_buff_len: usize) -> i32;
    fn get_data_nested_object_field(account_ptr: *const u8, account_len: usize, key_ptr: *const u8, key_len: usize, nst_ptr: *const u8, nst_len: usize, out_buff_ptr: *const u8, out_buff_len: usize) -> i32;
    fn get_data_array_element_field(account_ptr: *const u8, account_len: usize, key_ptr: *const u8, key_len: usize, index: i32, out_buff_ptr: *const u8, out_buff_len: usize) -> i32;
    fn get_data_nested_array_element_field(account_ptr: *const u8, account_len: usize, key_ptr: *const u8, key_len: usize, index: i32, nst_ptr: *const u8, nst_len: usize, out_buff_ptr: *const u8, out_buff_len: usize) -> i32;
    fn set_data_object_field(account_ptr: *const u8, account_len: usize, key_ptr: *const u8, key_len: usize, data_ptr: *const u8, data_len: usize) -> i32;
    fn set_data_nested_object_field(account_ptr: *const u8, account_len: usize, key_ptr: *const u8, key_len: usize, nst_ptr: *const u8, nst_len: usize, data_ptr: *const u8, data_len: usize) -> i32;
    fn set_data_array_element_field(account_ptr: *const u8, account_len: usize, key_ptr: *const u8, key_len: usize, index: i32, data_ptr: *const u8, data_len: usize) -> i32;
    fn set_data_nested_array_element_field(account_ptr: *const u8, account_len: usize, key_ptr: *const u8, key_len: usize, index: i32, nst_ptr: *const u8, nst_len: usize, data_ptr: *const u8, data_len: usize) -> i32;
    fn build_txn(txn_type: i32) -> i32;
    fn add_txn_field(index: i32, field: i32, write_ptr: *const u8, write_len: usize) -> i32;
    fn emit_built_txn(index: i32) -> i32;
    fn emit_txn(txn_read_ptr: *const u8, txn_read_len: usize) -> i32;
    fn emit_event(name_ptr: *const u8, name_len: usize, data_ptr: *const u8, data_len: usize) -> i32;
}
