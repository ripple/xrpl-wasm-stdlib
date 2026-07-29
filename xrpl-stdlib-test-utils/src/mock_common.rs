//! Re-exports of the mock host-bindings machinery that lives inline in `xrpl-wasm-stdlib`.
//!
//! `mockall::automock` generates `MockHostBindings` right next to the `HostBindings` trait
//! definition, so the type itself can't live in this crate. What lives here instead is the
//! author-facing entry point: import from `xrpl_stdlib_test_utils` instead of reaching into
//! `xrpl_wasm_stdlib::host::*` directly.

use xrpl_wasm_stdlib::host::error_codes::BUFFER_TOO_SMALL;
pub use xrpl_wasm_stdlib::host::host_bindings_trait::{HostBindings, MockHostBindings};
pub use xrpl_wasm_stdlib::host::{
    MockGuard, apply_default_expectations, create_default_mock, setup_mock,
};

/// Writes `bytes` into a raw output buffer, mirroring how the real host functions report
/// back the number of bytes written (or `BUFFER_TOO_SMALL` if the caller's buffer is too
/// small). Shared by scenario builders that need to hand fixture bytes back through a mocked
/// host call.
///
/// # Safety
/// `out_buff_ptr` must point to at least `out_buff_len` writable bytes.
pub unsafe fn write_bytes(bytes: &[u8], out_buff_ptr: *mut u8, out_buff_len: usize) -> i32 {
    if out_buff_len < bytes.len() {
        return BUFFER_TOO_SMALL;
    }
    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), out_buff_ptr, bytes.len());
    }
    bytes.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_bytes_returns_buffer_too_small_when_the_caller_buffer_is_undersized() {
        let mut undersized = [0u8; 4];
        let result =
            unsafe { write_bytes(&[1, 2, 3, 4, 5], undersized.as_mut_ptr(), undersized.len()) };
        assert_eq!(result, BUFFER_TOO_SMALL);
    }
}
