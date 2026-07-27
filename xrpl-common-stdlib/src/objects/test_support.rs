//! Shared mock-host-binding helpers used by the `#[cfg(test)]` blocks that
//! `tools/generateLedgerObjects.js` emits at the bottom of every
//! `objects::generated::<entry>` file.

use crate::host::host_bindings_trait::MockHostBindings;
use crate::keylets::XRPL_KEYLET_SIZE;
use crate::types::account_id::AccountID;
use crate::types::currency::Currency;
use crate::types::issue::{Issue, XrpIssue};
use crate::types::mpt_id::MptId;

/// Wires `get_ledger_obj_field` and `get_current_ledger_obj_field` to always succeed:
/// the output buffer is zero-filled and the call returns the requested buffer length as
/// the result code. This satisfies both the fixed-size getters (which require the result
/// code to exactly equal the expected size) and the variable-size getters (which only
/// require a non-negative result code), so it works uniformly as a "field present" mock
/// for every getter the generator emits, without a `.with(...)` predicate limiting which
/// field/slot/call count it applies to.
pub fn mock_all_fields_present(mock: &mut MockHostBindings) {
    mock.expect_get_ledger_obj_field().returning(
        |_cache_num, _field, out_buff_ptr, out_buff_len| {
            unsafe { core::ptr::write_bytes(out_buff_ptr, 0, out_buff_len) };
            out_buff_len as i32
        },
    );
    mock.expect_get_current_ledger_obj_field()
        .returning(|_field, out_buff_ptr, out_buff_len| {
            unsafe { core::ptr::write_bytes(out_buff_ptr, 0, out_buff_len) };
            out_buff_len as i32
        });
}

/// Wires `get_ledger_obj_field` and `get_current_ledger_obj_field` to always report
/// `FIELD_NOT_FOUND`, regardless of field/slot. Buffers are zero-filled defensively even
/// though a not-found result usually means the caller doesn't read the buffer.
pub fn mock_all_fields_not_found(mock: &mut MockHostBindings) {
    use crate::host::error_codes::FIELD_NOT_FOUND;

    mock.expect_get_ledger_obj_field().returning(
        |_cache_num, _field, out_buff_ptr, out_buff_len| {
            unsafe { core::ptr::write_bytes(out_buff_ptr, 0, out_buff_len) };
            FIELD_NOT_FOUND
        },
    );
    mock.expect_get_current_ledger_obj_field()
        .returning(|_field, out_buff_ptr, out_buff_len| {
            unsafe { core::ptr::write_bytes(out_buff_ptr, 0, out_buff_len) };
            FIELD_NOT_FOUND
        });
}

/// Writes `0xCC` into a keylet output buffer and returns `XRPL_KEYLET_SIZE` as the
/// success result code. The byte value is arbitrary -- `cache_ledger_obj` is mocked
/// separately and never reads the buffer; what matters is that the keylet's storage is
/// initialized before downstream code reads it back.
fn write_keylet_to_buffer(out_buff_ptr: *mut u8, out_buff_len: usize) {
    unsafe {
        core::ptr::write_bytes(out_buff_ptr, 0xCC, out_buff_len);
    }
    debug_assert_eq!(out_buff_len, XRPL_KEYLET_SIZE);
}

pub fn mock_account_keylet_success(mock: &mut MockHostBindings) {
    mock.expect_account_keylet()
        .returning(|_, _, out_buff_ptr, out_buff_len| {
            write_keylet_to_buffer(out_buff_ptr, out_buff_len);
            XRPL_KEYLET_SIZE as i32
        });
}

pub fn mock_amm_keylet_success(mock: &mut MockHostBindings) {
    mock.expect_amm_keylet()
        .returning(|_, _, _, _, out_buff_ptr, out_buff_len| {
            write_keylet_to_buffer(out_buff_ptr, out_buff_len);
            XRPL_KEYLET_SIZE as i32
        });
}

pub fn mock_check_keylet_success(mock: &mut MockHostBindings) {
    mock.expect_check_keylet()
        .returning(|_, _, _, _, out_buff_ptr, out_buff_len| {
            write_keylet_to_buffer(out_buff_ptr, out_buff_len);
            XRPL_KEYLET_SIZE as i32
        });
}

pub fn mock_credential_keylet_success(mock: &mut MockHostBindings) {
    mock.expect_credential_keylet()
        .returning(|_, _, _, _, _, _, out_buff_ptr, out_buff_len| {
            write_keylet_to_buffer(out_buff_ptr, out_buff_len);
            XRPL_KEYLET_SIZE as i32
        });
}

pub fn mock_delegate_keylet_success(mock: &mut MockHostBindings) {
    mock.expect_delegate_keylet()
        .returning(|_, _, _, _, out_buff_ptr, out_buff_len| {
            write_keylet_to_buffer(out_buff_ptr, out_buff_len);
            XRPL_KEYLET_SIZE as i32
        });
}

pub fn mock_deposit_preauth_keylet_success(mock: &mut MockHostBindings) {
    mock.expect_deposit_preauth_keylet()
        .returning(|_, _, _, _, out_buff_ptr, out_buff_len| {
            write_keylet_to_buffer(out_buff_ptr, out_buff_len);
            XRPL_KEYLET_SIZE as i32
        });
}

pub fn mock_did_keylet_success(mock: &mut MockHostBindings) {
    mock.expect_did_keylet()
        .returning(|_, _, out_buff_ptr, out_buff_len| {
            write_keylet_to_buffer(out_buff_ptr, out_buff_len);
            XRPL_KEYLET_SIZE as i32
        });
}

pub fn mock_line_keylet_success(mock: &mut MockHostBindings) {
    mock.expect_line_keylet()
        .returning(|_, _, _, _, _, _, out_buff_ptr, out_buff_len| {
            write_keylet_to_buffer(out_buff_ptr, out_buff_len);
            XRPL_KEYLET_SIZE as i32
        });
}

pub fn mock_mpt_issuance_keylet_success(mock: &mut MockHostBindings) {
    mock.expect_mpt_issuance_keylet()
        .returning(|_, _, _, _, out_buff_ptr, out_buff_len| {
            write_keylet_to_buffer(out_buff_ptr, out_buff_len);
            XRPL_KEYLET_SIZE as i32
        });
}

pub fn mock_mptoken_keylet_success(mock: &mut MockHostBindings) {
    mock.expect_mptoken_keylet()
        .returning(|_, _, _, _, out_buff_ptr, out_buff_len| {
            write_keylet_to_buffer(out_buff_ptr, out_buff_len);
            XRPL_KEYLET_SIZE as i32
        });
}

pub fn mock_nft_offer_keylet_success(mock: &mut MockHostBindings) {
    mock.expect_nft_offer_keylet()
        .returning(|_, _, _, _, out_buff_ptr, out_buff_len| {
            write_keylet_to_buffer(out_buff_ptr, out_buff_len);
            XRPL_KEYLET_SIZE as i32
        });
}

pub fn mock_offer_keylet_success(mock: &mut MockHostBindings) {
    mock.expect_offer_keylet()
        .returning(|_, _, _, _, out_buff_ptr, out_buff_len| {
            write_keylet_to_buffer(out_buff_ptr, out_buff_len);
            XRPL_KEYLET_SIZE as i32
        });
}

pub fn mock_oracle_keylet_success(mock: &mut MockHostBindings) {
    mock.expect_oracle_keylet()
        .returning(|_, _, _, _, out_buff_ptr, out_buff_len| {
            write_keylet_to_buffer(out_buff_ptr, out_buff_len);
            XRPL_KEYLET_SIZE as i32
        });
}

pub fn mock_paychan_keylet_success(mock: &mut MockHostBindings) {
    mock.expect_paychan_keylet()
        .returning(|_, _, _, _, _, _, out_buff_ptr, out_buff_len| {
            write_keylet_to_buffer(out_buff_ptr, out_buff_len);
            XRPL_KEYLET_SIZE as i32
        });
}

pub fn mock_permissioned_domain_keylet_success(mock: &mut MockHostBindings) {
    mock.expect_permissioned_domain_keylet()
        .returning(|_, _, _, _, out_buff_ptr, out_buff_len| {
            write_keylet_to_buffer(out_buff_ptr, out_buff_len);
            XRPL_KEYLET_SIZE as i32
        });
}

pub fn mock_signers_keylet_success(mock: &mut MockHostBindings) {
    mock.expect_signers_keylet()
        .returning(|_, _, out_buff_ptr, out_buff_len| {
            write_keylet_to_buffer(out_buff_ptr, out_buff_len);
            XRPL_KEYLET_SIZE as i32
        });
}

pub fn mock_ticket_keylet_success(mock: &mut MockHostBindings) {
    mock.expect_ticket_keylet()
        .returning(|_, _, _, _, out_buff_ptr, out_buff_len| {
            write_keylet_to_buffer(out_buff_ptr, out_buff_len);
            XRPL_KEYLET_SIZE as i32
        });
}

pub fn mock_vault_keylet_success(mock: &mut MockHostBindings) {
    mock.expect_vault_keylet()
        .returning(|_, _, _, _, out_buff_ptr, out_buff_len| {
            write_keylet_to_buffer(out_buff_ptr, out_buff_len);
            XRPL_KEYLET_SIZE as i32
        });
}

/// Mocks `cache_ledger_obj` to always return the given slot number.
pub fn mock_cache_ledger_obj_success(mock: &mut MockHostBindings, slot: i32) {
    mock.expect_cache_ledger_obj()
        .returning(move |_, _, _| slot);
}

/// Mocks `cache_ledger_obj` to always return the given (negative) error code.
pub fn mock_cache_ledger_obj_error(mock: &mut MockHostBindings, error_code: i32) {
    mock.expect_cache_ledger_obj()
        .returning(move |_, _, _| error_code);
}

/// Sample values covering every distinct keylet-constructor parameter name/type that
/// appears across `KEYLET_ROUTING` in `tools/generateLedgerObjects.js`. Each value is
/// deterministic and arbitrary -- only its type and presence matter for the generated
/// smoke tests, not its content.
pub mod sample {
    use super::*;

    /// A generic sample account, for keylets that only need one account.
    pub fn account_id() -> AccountID {
        AccountID::from([0xAB; 20])
    }

    /// A second, distinct sample account -- for keylets needing two different accounts
    /// (e.g. `line_keylet`'s `account1`/`account2`, `delegate_keylet`'s
    /// `account`/`authorize`, `paychan_keylet`'s `account`/`destination`).
    pub fn account_id_b() -> AccountID {
        AccountID::from([0xCD; 20])
    }

    /// A sample account sequence number, used for every `seq: u32` keylet parameter.
    pub fn seq() -> u32 {
        42
    }

    /// A sample oracle document ID, used for `oracle_keylet`'s `document_id: u32`.
    pub fn document_id() -> u32 {
        7
    }

    /// A sample credential type blob, used for `credential_keylet`'s
    /// `credential_type: &[u8]`.
    pub fn credential_type() -> &'static [u8] {
        b"termsandconditions"
    }

    /// A sample `Issue`, used for `amm_keylet`'s `issue1`/`issue2: &Issue` parameters.
    pub fn issue() -> Issue {
        Issue::XRP(XrpIssue {})
    }

    /// A sample `Currency`, used for `line_keylet`'s `currency: &Currency` parameter.
    pub fn currency() -> Currency {
        Currency::from(*b"USD")
    }

    /// A sample `MptId`, used for `mptoken_keylet`'s `mptid: &MptId` parameter.
    pub fn mpt_id() -> MptId {
        MptId::new(seq(), account_id())
    }
}
