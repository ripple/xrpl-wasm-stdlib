//! Shared mock-host-binding helpers used by the `#[cfg(test)]` blocks that
//! `tools/generateLedgerObjects.js` emits at the bottom of every
//! `objects::generated::<entry>` file.

use crate::host::host_bindings_trait::MockHostBindings;

/// Wires `le_field` and `home_le_field` to always succeed:
/// the output buffer is filled and the call returns the requested buffer length as the
/// result code. This satisfies both the fixed-size getters (which require the result code
/// to exactly equal the expected size) and the variable-size getters (which only require a
/// non-negative result code), so it works uniformly as a "field present" mock for every
/// getter the generator emits, without a `.with(...)` predicate limiting which
/// field/slot/call count it applies to.
///
/// The first byte is set to `0x80` rather than `0`: for an `Amount` field the decoder reads
/// the leading bit to pick the wire variant, and only the IOU variant (bit 7 set) has an
/// expected length equal to the full `AMOUNT_SIZE` buffer this mock reports. A zero lead byte
/// would parse as XRP (expected length 8) and be rejected against the 48-byte report. Every
/// other field type ignores the lead byte's value here (they only care about the length), so
/// `0x80` is a safe uniform fill.
fn write_present(out_buff_ptr: *mut u8, out_buff_len: usize) {
    unsafe {
        core::ptr::write_bytes(out_buff_ptr, 0, out_buff_len);
        if out_buff_len > 0 {
            *out_buff_ptr = 0x80;
        }
    }
}

pub fn mock_all_fields_present(mock: &mut MockHostBindings) {
    mock.expect_le_field()
        .returning(|_cache_num, _field, out_buff_ptr, out_buff_len| {
            write_present(out_buff_ptr, out_buff_len);
            out_buff_len as i32
        });
    mock.expect_home_le_field()
        .returning(|_field, out_buff_ptr, out_buff_len| {
            write_present(out_buff_ptr, out_buff_len);
            out_buff_len as i32
        });
}

/// Wires `le_field` and `home_le_field` to always report
/// `FIELD_NOT_FOUND`, regardless of field/slot. Buffers are zero-filled defensively even
/// though a not-found result usually means the caller doesn't read the buffer.
pub fn mock_all_fields_not_found(mock: &mut MockHostBindings) {
    use crate::host::error_codes::FIELD_NOT_FOUND;

    mock.expect_le_field()
        .returning(|_cache_num, _field, out_buff_ptr, out_buff_len| {
            unsafe { core::ptr::write_bytes(out_buff_ptr, 0, out_buff_len) };
            FIELD_NOT_FOUND
        });
    mock.expect_home_le_field()
        .returning(|_field, out_buff_ptr, out_buff_len| {
            unsafe { core::ptr::write_bytes(out_buff_ptr, 0, out_buff_len) };
            FIELD_NOT_FOUND
        });
}
