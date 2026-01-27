#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_wasm_stdlib::host;
use xrpl_wasm_stdlib::host::error_codes::match_result_code;
use xrpl_wasm_stdlib::host::trace::trace_num;

#[unsafe(no_mangle)]
pub extern "C" fn finish() -> i32 {
    unsafe {
        let result_code = host::get_ledger_sqn() as i32;

        let ledger_sequence = match_result_code(result_code, || {
            Some(result_code) // <-- Move the value into a buffer
        })
        .unwrap()
        .unwrap();

        let _ = trace_num("Ledger Sequence", ledger_sequence as i64);
        (ledger_sequence >= 5) as i32 // Return 1 if true (successful outcome), 0 if false (failed outcome)
    }
}
