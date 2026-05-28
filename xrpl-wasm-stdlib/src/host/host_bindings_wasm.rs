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
        pub(super) fn ldgr_index(out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
        pub(super) fn parent_ldgr_time(out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
        pub(super) fn parent_ldgr_hash(out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
        pub(super) fn base_fee(out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
        pub(super) fn amendment_enabled(amendment_ptr: *const u8, amendment_len: usize) -> i32;
        pub(super) fn cache_le(keylet_ptr: *const u8, keylet_len: usize, cache_num: i32) -> i32;
        pub(super) fn tx_field(field: i32, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
        pub(super) fn home_le_field(field: i32, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
        pub(super) fn le_field(
            cache_num: i32,
            field: i32,
            out_buff_ptr: *mut u8,
            out_buff_len: usize,
        ) -> i32;
        pub(super) fn tx_inner(
            locator_ptr: *const u8,
            locator_len: usize,
            out_buff_ptr: *mut u8,
            out_buff_len: usize,
        ) -> i32;
        pub(super) fn home_le_inner(
            locator_ptr: *const u8,
            locator_len: usize,
            out_buff_ptr: *mut u8,
            out_buff_len: usize,
        ) -> i32;
        pub(super) fn le_inner(
            cache_num: i32,
            locator_ptr: *const u8,
            locator_len: usize,
            out_buff_ptr: *mut u8,
            out_buff_len: usize,
        ) -> i32;
        pub(super) fn tx_arr_len(field: i32) -> i32;
        pub(super) fn home_le_arr_len(field: i32) -> i32;
        pub(super) fn le_arr_len(cache_num: i32, field: i32) -> i32;
        pub(super) fn tx_inner_arr_len(locator_ptr: *const u8, locator_len: usize) -> i32;
        pub(super) fn home_le_inner_arr_len(locator_ptr: *const u8, locator_len: usize) -> i32;
        pub(super) fn le_inner_arr_len(
            cache_num: i32,
            locator_ptr: *const u8,
            locator_len: usize,
        ) -> i32;
        pub(super) fn set_data(data_ptr: *const u8, data_len: usize) -> i32;
        pub(super) fn sha512_half(
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
        pub(super) fn accountroot_id(
            account_ptr: *const u8,
            account_len: usize,
            out_buff_ptr: *mut u8,
            out_buff_len: usize,
        ) -> i32;
        pub(super) fn amm_id(
            issue1_ptr: *const u8,
            issue1_len: usize,
            issue2_ptr: *const u8,
            issue2_len: usize,
            out_buff_ptr: *mut u8,
            out_buff_len: usize,
        ) -> i32;
        pub(super) fn check_id(
            account_ptr: *const u8,
            account_len: usize,
            sequence_ptr: *const u8,
            sequence_len: usize,
            out_buff_ptr: *mut u8,
            out_buff_len: usize,
        ) -> i32;
        pub(super) fn credential_id(
            subject_ptr: *const u8,
            subject_len: usize,
            issuer_ptr: *const u8,
            issuer_len: usize,
            cred_type_ptr: *const u8,
            cred_type_len: usize,
            out_buff_ptr: *mut u8,
            out_buff_len: usize,
        ) -> i32;
        pub(super) fn delegate_id(
            account_ptr: *const u8,
            account_len: usize,
            authorize_ptr: *const u8,
            authorize_len: usize,
            out_buff_ptr: *mut u8,
            out_buff_len: usize,
        ) -> i32;
        pub(super) fn deposit_preauth_id(
            account_ptr: *const u8,
            account_len: usize,
            authorize_ptr: *const u8,
            authorize_len: usize,
            out_buff_ptr: *mut u8,
            out_buff_len: usize,
        ) -> i32;
        pub(super) fn did_id(
            account_ptr: *const u8,
            account_len: usize,
            out_buff_ptr: *mut u8,
            out_buff_len: usize,
        ) -> i32;
        pub(super) fn escrow_id(
            account_ptr: *const u8,
            account_len: usize,
            sequence_ptr: *const u8,
            sequence_len: usize,
            out_buff_ptr: *mut u8,
            out_buff_len: usize,
        ) -> i32;
        pub(super) fn trustline_id(
            account1_ptr: *const u8,
            account1_len: usize,
            account2_ptr: *const u8,
            account2_len: usize,
            currency_ptr: *const u8,
            currency_len: usize,
            out_buff_ptr: *mut u8,
            out_buff_len: usize,
        ) -> i32;
        pub(super) fn mpt_issuance_id(
            issuer_ptr: *const u8,
            issuer_len: usize,
            sequence_ptr: *const u8,
            sequence_len: usize,
            out_buff_ptr: *mut u8,
            out_buff_len: usize,
        ) -> i32;
        pub(super) fn mptoken_id(
            mptid_ptr: *const u8,
            mptid_len: usize,
            holder_ptr: *const u8,
            holder_len: usize,
            out_buff_ptr: *mut u8,
            out_buff_len: usize,
        ) -> i32;
        pub(super) fn nft_offer_id(
            account_ptr: *const u8,
            account_len: usize,
            sequence_ptr: *const u8,
            sequence_len: usize,
            out_buff_ptr: *mut u8,
            out_buff_len: usize,
        ) -> i32;
        pub(super) fn offer_id(
            account_ptr: *const u8,
            account_len: usize,
            sequence_ptr: *const u8,
            sequence_len: usize,
            out_buff_ptr: *mut u8,
            out_buff_len: usize,
        ) -> i32;
        pub(super) fn oracle_id(
            account_ptr: *const u8,
            account_len: usize,
            document_id_ptr: *const u8,
            document_id_len: usize,
            out_buff_ptr: *mut u8,
            out_buff_len: usize,
        ) -> i32;
        pub(super) fn paychan_id(
            account_ptr: *const u8,
            account_len: usize,
            destination_ptr: *const u8,
            destination_len: usize,
            sequence_ptr: *const u8,
            sequence_len: usize,
            out_buff_ptr: *mut u8,
            out_buff_len: usize,
        ) -> i32;
        pub(super) fn permissioned_domain_id(
            account_ptr: *const u8,
            account_len: usize,
            sequence_ptr: *const u8,
            sequence_len: usize,
            out_buff_ptr: *mut u8,
            out_buff_len: usize,
        ) -> i32;
        pub(super) fn signers_id(
            account_ptr: *const u8,
            account_len: usize,
            out_buff_ptr: *mut u8,
            out_buff_len: usize,
        ) -> i32;
        pub(super) fn ticket_id(
            account_ptr: *const u8,
            account_len: usize,
            sequence_ptr: *const u8,
            sequence_len: usize,
            out_buff_ptr: *mut u8,
            out_buff_len: usize,
        ) -> i32;
        pub(super) fn vault_id(
            account_ptr: *const u8,
            account_len: usize,
            sequence_ptr: *const u8,
            sequence_len: usize,
            out_buff_ptr: *mut u8,
            out_buff_len: usize,
        ) -> i32;
        pub(super) fn nft_uri(
            account_ptr: *const u8,
            account_len: usize,
            nft_id_ptr: *const u8,
            nft_id_len: usize,
            out_buff_ptr: *mut u8,
            out_buff_len: usize,
        ) -> i32;
        pub(super) fn nft_issuer(
            nft_id_ptr: *const u8,
            nft_id_len: usize,
            out_buff_ptr: *mut u8,
            out_buff_len: usize,
        ) -> i32;
        pub(super) fn nft_taxon(
            nft_id_ptr: *const u8,
            nft_id_len: usize,
            out_buff_ptr: *mut u8,
            out_buff_len: usize,
        ) -> i32;
        pub(super) fn nft_flags(nft_id_ptr: *const u8, nft_id_len: usize) -> i32;
        pub(super) fn nft_xfer_fee(nft_id_ptr: *const u8, nft_id_len: usize) -> i32;
        pub(super) fn nft_serial(
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
        pub(super) fn float_from_mant_exp(
            mantissa: i64,
            exponent: i32,
            out_buff: *mut u8,
            out_buff_len: usize,
            rounding_mode: i32,
        ) -> i32;
        pub(super) fn float_from_stamount(
            in_buff: *const u8,
            in_buff_len: usize,
            out_buff: *mut u8,
            out_buff_len: usize,
            rounding_mode: i32,
        ) -> i32;
        pub(super) fn float_from_stnumber(
            in_buff: *const u8,
            in_buff_len: usize,
            out_buff: *mut u8,
            out_buff_len: usize,
            rounding_mode: i32,
        ) -> i32;
        pub(super) fn float_to_int(
            in_buff: *const u8,
            in_buff_len: usize,
            out_buff: *mut u8,
            out_buff_len: usize,
            rounding_mode: i32,
        ) -> i32;
        pub(super) fn float_to_mant_exp(
            in_buff: *const u8,
            in_buff_len: usize,
            mant_buff: *mut u8,
            mant_buff_len: usize,
            exp_buff: *mut u8,
            exp_buff_len: usize,
        ) -> i32;
        pub(super) fn float_cmp(
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
        pub(super) fn float_sub(
            in_buff1: *const u8,
            in_buff1_len: usize,
            in_buff2: *const u8,
            in_buff2_len: usize,
            out_buff: *mut u8,
            out_buff_len: usize,
            rounding_mode: i32,
        ) -> i32;
        pub(super) fn float_mult(
            in_buff1: *const u8,
            in_buff1_len: usize,
            in_buff2: *const u8,
            in_buff2_len: usize,
            out_buff: *mut u8,
            out_buff_len: usize,
            rounding_mode: i32,
        ) -> i32;
        pub(super) fn float_div(
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
        pub(super) fn float_root(
            in_buff: *const u8,
            in_buff_len: usize,
            root: i32,
            out_buff: *mut u8,
            out_buff_len: usize,
            rounding_mode: i32,
        ) -> i32;
        pub(super) fn trace(
            msg_read_ptr: *const u8,
            msg_read_len: usize,
            data_read_ptr: *const u8,
            data_read_len: usize,
            as_hex: i32,
        ) -> i32;
        pub(super) fn trace_num(msg_read_ptr: *const u8, msg_read_len: usize, number: i64) -> i32;
        pub(super) fn trace_acct(
            msg_read_ptr: *const u8,
            msg_read_len: usize,
            account_ptr: *const u8,
            account_len: usize,
        ) -> i32;
        pub(super) fn trace_xfloat(
            msg_read_ptr: *const u8,
            msg_read_len: usize,
            opaque_float_ptr: *const u8,
            opaque_float_len: usize,
        ) -> i32;
        pub(super) fn trace_amt(
            msg_read_ptr: *const u8,
            msg_read_len: usize,
            amount_ptr: *const u8,
            amount_len: usize,
        ) -> i32;
    }
}

/// Implementation of host bindings for WASM targets.
pub struct WasmHostBindings;

/// WASM implementation of HostBindings.
impl HostBindings for WasmHostBindings {
    unsafe fn ldgr_index(&self, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32 {
        unsafe { host_defined_functions::ldgr_index(out_buff_ptr, out_buff_len) }
    }

    unsafe fn parent_ldgr_time(&self, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32 {
        unsafe { host_defined_functions::parent_ldgr_time(out_buff_ptr, out_buff_len) }
    }

    unsafe fn parent_ldgr_hash(&self, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32 {
        unsafe { host_defined_functions::parent_ldgr_hash(out_buff_ptr, out_buff_len) }
    }

    unsafe fn base_fee(&self, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32 {
        unsafe { host_defined_functions::base_fee(out_buff_ptr, out_buff_len) }
    }

    unsafe fn amendment_enabled(&self, amendment_ptr: *const u8, amendment_len: usize) -> i32 {
        unsafe { host_defined_functions::amendment_enabled(amendment_ptr, amendment_len) }
    }

    unsafe fn cache_le(&self, keylet_ptr: *const u8, keylet_len: usize, cache_num: i32) -> i32 {
        unsafe { host_defined_functions::cache_le(keylet_ptr, keylet_len, cache_num) }
    }

    unsafe fn tx_field(&self, field: i32, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32 {
        unsafe { host_defined_functions::tx_field(field, out_buff_ptr, out_buff_len) }
    }

    unsafe fn home_le_field(&self, field: i32, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32 {
        unsafe { host_defined_functions::home_le_field(field, out_buff_ptr, out_buff_len) }
    }

    unsafe fn le_field(
        &self,
        cache_num: i32,
        field: i32,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32 {
        unsafe { host_defined_functions::le_field(cache_num, field, out_buff_ptr, out_buff_len) }
    }

    unsafe fn tx_inner(
        &self,
        locator_ptr: *const u8,
        locator_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::tx_inner(locator_ptr, locator_len, out_buff_ptr, out_buff_len)
        }
    }

    unsafe fn home_le_inner(
        &self,
        locator_ptr: *const u8,
        locator_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::home_le_inner(
                locator_ptr,
                locator_len,
                out_buff_ptr,
                out_buff_len,
            )
        }
    }

    unsafe fn le_inner(
        &self,
        cache_num: i32,
        locator_ptr: *const u8,
        locator_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::le_inner(
                cache_num,
                locator_ptr,
                locator_len,
                out_buff_ptr,
                out_buff_len,
            )
        }
    }

    unsafe fn tx_arr_len(&self, field: i32) -> i32 {
        unsafe { host_defined_functions::tx_arr_len(field) }
    }

    unsafe fn home_le_arr_len(&self, field: i32) -> i32 {
        unsafe { host_defined_functions::home_le_arr_len(field) }
    }

    unsafe fn le_arr_len(&self, cache_num: i32, field: i32) -> i32 {
        unsafe { host_defined_functions::le_arr_len(cache_num, field) }
    }

    unsafe fn tx_inner_arr_len(&self, locator_ptr: *const u8, locator_len: usize) -> i32 {
        unsafe { host_defined_functions::tx_inner_arr_len(locator_ptr, locator_len) }
    }

    unsafe fn home_le_inner_arr_len(&self, locator_ptr: *const u8, locator_len: usize) -> i32 {
        unsafe { host_defined_functions::home_le_inner_arr_len(locator_ptr, locator_len) }
    }

    unsafe fn le_inner_arr_len(
        &self,
        cache_num: i32,
        locator_ptr: *const u8,
        locator_len: usize,
    ) -> i32 {
        unsafe { host_defined_functions::le_inner_arr_len(cache_num, locator_ptr, locator_len) }
    }

    unsafe fn set_data(&self, data_ptr: *const u8, data_len: usize) -> i32 {
        unsafe { host_defined_functions::set_data(data_ptr, data_len) }
    }

    unsafe fn sha512_half(
        &self,
        data_ptr: *const u8,
        data_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::sha512_half(data_ptr, data_len, out_buff_ptr, out_buff_len)
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

    unsafe fn accountroot_id(
        &self,
        account_ptr: *const u8,
        account_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::accountroot_id(
                account_ptr,
                account_len,
                out_buff_ptr,
                out_buff_len,
            )
        }
    }

    unsafe fn amm_id(
        &self,
        issue1_ptr: *const u8,
        issue1_len: usize,
        issue2_ptr: *const u8,
        issue2_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::amm_id(
                issue1_ptr,
                issue1_len,
                issue2_ptr,
                issue2_len,
                out_buff_ptr,
                out_buff_len,
            )
        }
    }

    unsafe fn check_id(
        &self,
        account_ptr: *const u8,
        account_len: usize,
        sequence_ptr: *const u8,
        sequence_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::check_id(
                account_ptr,
                account_len,
                sequence_ptr,
                sequence_len,
                out_buff_ptr,
                out_buff_len,
            )
        }
    }

    unsafe fn credential_id(
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
            host_defined_functions::credential_id(
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

    unsafe fn delegate_id(
        &self,
        account_ptr: *const u8,
        account_len: usize,
        authorize_ptr: *const u8,
        authorize_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::delegate_id(
                account_ptr,
                account_len,
                authorize_ptr,
                authorize_len,
                out_buff_ptr,
                out_buff_len,
            )
        }
    }

    unsafe fn deposit_preauth_id(
        &self,
        account_ptr: *const u8,
        account_len: usize,
        authorize_ptr: *const u8,
        authorize_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::deposit_preauth_id(
                account_ptr,
                account_len,
                authorize_ptr,
                authorize_len,
                out_buff_ptr,
                out_buff_len,
            )
        }
    }

    unsafe fn did_id(
        &self,
        account_ptr: *const u8,
        account_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::did_id(account_ptr, account_len, out_buff_ptr, out_buff_len)
        }
    }

    unsafe fn escrow_id(
        &self,
        account_ptr: *const u8,
        account_len: usize,
        sequence_ptr: *const u8,
        sequence_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::escrow_id(
                account_ptr,
                account_len,
                sequence_ptr,
                sequence_len,
                out_buff_ptr,
                out_buff_len,
            )
        }
    }

    unsafe fn trustline_id(
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
            host_defined_functions::trustline_id(
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

    unsafe fn mpt_issuance_id(
        &self,
        issuer_ptr: *const u8,
        issuer_len: usize,
        sequence_ptr: *const u8,
        sequence_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::mpt_issuance_id(
                issuer_ptr,
                issuer_len,
                sequence_ptr,
                sequence_len,
                out_buff_ptr,
                out_buff_len,
            )
        }
    }

    unsafe fn mptoken_id(
        &self,
        mptid_ptr: *const u8,
        mptid_len: usize,
        holder_ptr: *const u8,
        holder_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::mptoken_id(
                mptid_ptr,
                mptid_len,
                holder_ptr,
                holder_len,
                out_buff_ptr,
                out_buff_len,
            )
        }
    }

    unsafe fn nft_offer_id(
        &self,
        account_ptr: *const u8,
        account_len: usize,
        sequence_ptr: *const u8,
        sequence_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::nft_offer_id(
                account_ptr,
                account_len,
                sequence_ptr,
                sequence_len,
                out_buff_ptr,
                out_buff_len,
            )
        }
    }

    unsafe fn offer_id(
        &self,
        account_ptr: *const u8,
        account_len: usize,
        sequence_ptr: *const u8,
        sequence_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::offer_id(
                account_ptr,
                account_len,
                sequence_ptr,
                sequence_len,
                out_buff_ptr,
                out_buff_len,
            )
        }
    }

    unsafe fn oracle_id(
        &self,
        account_ptr: *const u8,
        account_len: usize,
        document_id_ptr: *const u8,
        document_id_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::oracle_id(
                account_ptr,
                account_len,
                document_id_ptr,
                document_id_len,
                out_buff_ptr,
                out_buff_len,
            )
        }
    }

    unsafe fn paychan_id(
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
            host_defined_functions::paychan_id(
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

    unsafe fn permissioned_domain_id(
        &self,
        account_ptr: *const u8,
        account_len: usize,
        sequence_ptr: *const u8,
        sequence_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::permissioned_domain_id(
                account_ptr,
                account_len,
                sequence_ptr,
                sequence_len,
                out_buff_ptr,
                out_buff_len,
            )
        }
    }

    unsafe fn signers_id(
        &self,
        account_ptr: *const u8,
        account_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::signers_id(account_ptr, account_len, out_buff_ptr, out_buff_len)
        }
    }

    unsafe fn ticket_id(
        &self,
        account_ptr: *const u8,
        account_len: usize,
        sequence_ptr: *const u8,
        sequence_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::ticket_id(
                account_ptr,
                account_len,
                sequence_ptr,
                sequence_len,
                out_buff_ptr,
                out_buff_len,
            )
        }
    }

    unsafe fn vault_id(
        &self,
        account_ptr: *const u8,
        account_len: usize,
        sequence_ptr: *const u8,
        sequence_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::vault_id(
                account_ptr,
                account_len,
                sequence_ptr,
                sequence_len,
                out_buff_ptr,
                out_buff_len,
            )
        }
    }

    unsafe fn nft_uri(
        &self,
        account_ptr: *const u8,
        account_len: usize,
        nft_id_ptr: *const u8,
        nft_id_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::nft_uri(
                account_ptr,
                account_len,
                nft_id_ptr,
                nft_id_len,
                out_buff_ptr,
                out_buff_len,
            )
        }
    }

    unsafe fn nft_issuer(
        &self,
        nft_id_ptr: *const u8,
        nft_id_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::nft_issuer(nft_id_ptr, nft_id_len, out_buff_ptr, out_buff_len)
        }
    }

    unsafe fn nft_taxon(
        &self,
        nft_id_ptr: *const u8,
        nft_id_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::nft_taxon(nft_id_ptr, nft_id_len, out_buff_ptr, out_buff_len)
        }
    }

    unsafe fn nft_flags(&self, nft_id_ptr: *const u8, nft_id_len: usize) -> i32 {
        unsafe { host_defined_functions::nft_flags(nft_id_ptr, nft_id_len) }
    }

    unsafe fn nft_xfer_fee(&self, nft_id_ptr: *const u8, nft_id_len: usize) -> i32 {
        unsafe { host_defined_functions::nft_xfer_fee(nft_id_ptr, nft_id_len) }
    }

    unsafe fn nft_serial(
        &self,
        nft_id_ptr: *const u8,
        nft_id_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::nft_serial(nft_id_ptr, nft_id_len, out_buff_ptr, out_buff_len)
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

    unsafe fn float_from_mant_exp(
        &self,
        mantissa: i64,
        exponent: i32,
        out_buff: *mut u8,
        out_buff_len: usize,
        rounding_mode: i32,
    ) -> i32 {
        unsafe {
            host_defined_functions::float_from_mant_exp(
                mantissa,
                exponent,
                out_buff,
                out_buff_len,
                rounding_mode,
            )
        }
    }

    unsafe fn float_from_stamount(
        &self,
        in_buff: *const u8,
        in_buff_len: usize,
        out_buff: *mut u8,
        out_buff_len: usize,
        rounding_mode: i32,
    ) -> i32 {
        unsafe {
            host_defined_functions::float_from_stamount(
                in_buff,
                in_buff_len,
                out_buff,
                out_buff_len,
                rounding_mode,
            )
        }
    }

    unsafe fn float_from_stnumber(
        &self,
        in_buff: *const u8,
        in_buff_len: usize,
        out_buff: *mut u8,
        out_buff_len: usize,
        rounding_mode: i32,
    ) -> i32 {
        unsafe {
            host_defined_functions::float_from_stnumber(
                in_buff,
                in_buff_len,
                out_buff,
                out_buff_len,
                rounding_mode,
            )
        }
    }

    unsafe fn float_to_int(
        &self,
        in_buff: *const u8,
        in_buff_len: usize,
        out_buff: *mut u8,
        out_buff_len: usize,
        rounding_mode: i32,
    ) -> i32 {
        unsafe {
            host_defined_functions::float_to_int(
                in_buff,
                in_buff_len,
                out_buff,
                out_buff_len,
                rounding_mode,
            )
        }
    }

    unsafe fn float_to_mant_exp(
        &self,
        in_buff: *const u8,
        in_buff_len: usize,
        mant_buff: *mut u8,
        mant_buff_len: usize,
        exp_buff: *mut u8,
        exp_buff_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::float_to_mant_exp(
                in_buff,
                in_buff_len,
                mant_buff,
                mant_buff_len,
                exp_buff,
                exp_buff_len,
            )
        }
    }

    unsafe fn float_cmp(
        &self,
        in_buff1: *const u8,
        in_buff1_len: usize,
        in_buff2: *const u8,
        in_buff2_len: usize,
    ) -> i32 {
        unsafe { host_defined_functions::float_cmp(in_buff1, in_buff1_len, in_buff2, in_buff2_len) }
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

    unsafe fn float_sub(
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
            host_defined_functions::float_sub(
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

    unsafe fn float_mult(
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
            host_defined_functions::float_mult(
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

    unsafe fn float_div(
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
            host_defined_functions::float_div(
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
        unsafe {
            host_defined_functions::float_root(
                in_buff,
                in_buff_len,
                root,
                out_buff,
                out_buff_len,
                rounding_mode,
            )
        }
    }

    unsafe fn trace(
        &self,
        msg_read_ptr: *const u8,
        msg_read_len: usize,
        data_read_ptr: *const u8,
        data_read_len: usize,
        as_hex: i32,
    ) -> i32 {
        unsafe {
            host_defined_functions::trace(
                msg_read_ptr,
                msg_read_len,
                data_read_ptr,
                data_read_len,
                as_hex,
            )
        }
    }

    unsafe fn trace_num(&self, msg_read_ptr: *const u8, msg_read_len: usize, number: i64) -> i32 {
        unsafe { host_defined_functions::trace_num(msg_read_ptr, msg_read_len, number) }
    }

    unsafe fn trace_acct(
        &self,
        msg_read_ptr: *const u8,
        msg_read_len: usize,
        account_ptr: *const u8,
        account_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::trace_acct(msg_read_ptr, msg_read_len, account_ptr, account_len)
        }
    }

    unsafe fn trace_xfloat(
        &self,
        msg_read_ptr: *const u8,
        msg_read_len: usize,
        opaque_float_ptr: *const u8,
        opaque_float_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::trace_xfloat(
                msg_read_ptr,
                msg_read_len,
                opaque_float_ptr,
                opaque_float_len,
            )
        }
    }

    unsafe fn trace_amt(
        &self,
        msg_read_ptr: *const u8,
        msg_read_len: usize,
        amount_ptr: *const u8,
        amount_len: usize,
    ) -> i32 {
        unsafe {
            host_defined_functions::trace_amt(msg_read_ptr, msg_read_len, amount_ptr, amount_len)
        }
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
    fn float_from_mant_exp(mantissa: i64, exponent: i32, out_buff: *mut u8, out_buff_len: usize, rounding_mode: i32) -> i32;
    fn float_from_stamount(in_buff: *const u8, in_buff_len: usize, out_buff: *mut u8, out_buff_len: usize, rounding_mode: i32) -> i32;
    fn float_from_stnumber(in_buff: *const u8, in_buff_len: usize, out_buff: *mut u8, out_buff_len: usize, rounding_mode: i32) -> i32;
    fn float_to_int(in_buff: *const u8, in_buff_len: usize, out_buff: *mut u8, out_buff_len: usize, rounding_mode: i32) -> i32;
    fn float_to_mant_exp(in_buff: *const u8, in_buff_len: usize, mant_buff: *mut u8, mant_buff_len: usize, exp_buff: *mut u8, exp_buff_len: usize) -> i32;
    fn float_cmp(in_buff1: *const u8, in_buff1_len: usize, in_buff2: *const u8, in_buff2_len: usize) -> i32;
    fn float_add(in_buff1: *const u8, in_buff1_len: usize, in_buff2: *const u8, in_buff2_len: usize, out_buff: *mut u8, out_buff_len: usize, rounding_mode: i32) -> i32;
    fn float_sub(in_buff1: *const u8, in_buff1_len: usize, in_buff2: *const u8, in_buff2_len: usize, out_buff: *mut u8, out_buff_len: usize, rounding_mode: i32) -> i32;
    fn float_mult(in_buff1: *const u8, in_buff1_len: usize, in_buff2: *const u8, in_buff2_len: usize, out_buff: *mut u8, out_buff_len: usize, rounding_mode: i32) -> i32;
    fn float_div(in_buff1: *const u8, in_buff1_len: usize, in_buff2: *const u8, in_buff2_len: usize, out_buff: *mut u8, out_buff_len: usize, rounding_mode: i32) -> i32;
    fn float_pow(in_buff: *const u8, in_buff_len: usize, pow: i32, out_buff: *mut u8, out_buff_len: usize, rounding_mode: i32) -> i32;
    fn float_root(in_buff: *const u8, in_buff_len: usize, root: i32, out_buff: *mut u8, out_buff_len: usize, rounding_mode: i32) -> i32;

    // Host Function Category: TRACE
    fn trace(msg_read_ptr: *const u8, msg_read_len: usize, data_read_ptr: *const u8, data_read_len: usize, as_hex: i32) -> i32;
    fn trace_num(msg_read_ptr: *const u8, msg_read_len: usize, number: i64) -> i32;
    fn trace_acct(msg_read_ptr: *const u8, msg_read_len: usize, account_ptr: *const u8, account_len: usize) -> i32;
    fn trace_xfloat(msg_read_ptr: *const u8, msg_read_len: usize, opaque_float_ptr: *const u8, opaque_float_len: usize) -> i32;
    fn trace_amt(msg_read_ptr: *const u8, msg_read_len: usize, amount_ptr: *const u8, amount_len: usize) -> i32;

}
