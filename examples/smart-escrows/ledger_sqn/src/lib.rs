#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_wasm_stdlib::host;
use xrpl_wasm_stdlib::host::error_codes::match_result_code_with_expected_bytes;
use xrpl_wasm_stdlib::host::trace::trace_num;

#[unsafe(no_mangle)]
pub extern "C" fn finish() -> i32 {
    unsafe {
        let mut ledger_sqn_buffer = [0u8; 4];
        let result_code =
            host::get_ledger_sqn(ledger_sqn_buffer.as_mut_ptr(), ledger_sqn_buffer.len());

        match_result_code_with_expected_bytes(result_code, 4, || {
            Some(result_code) // <-- Move the value into a buffer
        })
        .unwrap()
        .unwrap();

        let ledger_sequence = u32::from_be_bytes(ledger_sqn_buffer);
        let _ = trace_num("Ledger Sequence", ledger_sequence as i64);
        (ledger_sequence >= 5) as i32 // Return 1 if true (successful outcome), 0 if false (failed outcome)
    }
}
