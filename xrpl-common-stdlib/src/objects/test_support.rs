//! Shared mock-host-binding helpers used by the `#[cfg(test)]` blocks that
//! `tools/generateLedgerObjects.js` emits at the bottom of every
//! `objects::generated::<entry>` file.

use crate::host::host_bindings_trait::MockHostBindings;

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
