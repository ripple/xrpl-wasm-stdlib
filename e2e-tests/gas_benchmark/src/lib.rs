#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_wasm_stdlib::core::current_tx::escrow_finish::{EscrowFinish, get_current_escrow_finish};
use xrpl_wasm_stdlib::core::current_tx::traits::TransactionCommonFields;
use xrpl_wasm_stdlib::core::ledger_objects::current_escrow::{CurrentEscrow, get_current_escrow};
use xrpl_wasm_stdlib::core::ledger_objects::traits::CurrentEscrowFields;
use xrpl_wasm_stdlib::core::locator::Locator;
use xrpl_wasm_stdlib::host::Result;
use xrpl_wasm_stdlib::host::error_codes::{
    match_result_code, match_result_code_optional, match_result_code_with_expected_bytes,
    match_result_code_with_expected_bytes_optional,
};
use xrpl_wasm_stdlib::host::trace::trace;
use xrpl_wasm_stdlib::{decode_hex_20, decode_hex_32, sfield};

const ITERATIONS: usize = 100;

/// Main entry point for the gas benchmark contract
///
/// This contract exercises all optimized helper functions with controlled workloads
/// to measure gas consumption. Each benchmark section is marked with trace() calls
/// to help identify gas usage patterns.
///
/// Benchmarks covered:
/// - Locator operations (pack single, pack nested, repack_last)
/// - Transaction field access (get_account, get_fee)
/// - Blob operations (creation and access)
/// - Result type operations (is_ok, is_err, ok, err)
/// - Error code matching (match_result_code, match_result_code_optional, etc.)
/// - Hex decoding (decode_hex_32, decode_hex_20)
#[unsafe(no_mangle)]
pub extern "C" fn finish() -> i32 {
    let _ = trace("$$$$$ GAS BENCHMARK START $$$$$");

    // Get the current transaction
    let escrow_finish: EscrowFinish = get_current_escrow_finish();
    let escrow: CurrentEscrow = get_current_escrow();

    // Accumulate results to prevent compiler optimization
    let mut accumulator: u64 = 0;

    // Locator operation benchmarks
    let _ = trace("BENCHMARK_SECTION: LOCATOR_OPERATIONS");
    let _ = trace("BENCHMARK: locator_pack_single");
    accumulator = accumulator.wrapping_add(benchmark_locator_pack_single());

    let _ = trace("BENCHMARK: locator_pack_nested");
    accumulator = accumulator.wrapping_add(benchmark_locator_pack_nested());

    let _ = trace("BENCHMARK: locator_repack_last");
    accumulator = accumulator.wrapping_add(benchmark_locator_repack_last());

    // Transaction field access benchmarks
    let _ = trace("BENCHMARK_SECTION: TRANSACTION_FIELD_ACCESS");
    let _ = trace("BENCHMARK: get_account_id_field");
    accumulator = accumulator.wrapping_add(benchmark_account_id_field(&escrow_finish));

    let _ = trace("BENCHMARK: get_fee_field");
    accumulator = accumulator.wrapping_add(benchmark_fee_field(&escrow_finish));

    let _ = trace("BENCHMARK: get_amount_field");
    accumulator = accumulator.wrapping_add(benchmark_amount_field(&escrow_finish));

    let _ = trace("BENCHMARK: get_hash256_field");
    accumulator = accumulator.wrapping_add(benchmark_hash256_field(&escrow));

    let _ = trace("BENCHMARK: get_u16_field");
    accumulator = accumulator.wrapping_add(benchmark_u16_field(&escrow_finish));

    let _ = trace("BENCHMARK: get_u64_field");
    accumulator = accumulator.wrapping_add(benchmark_u64_field(&escrow_finish));

    // Blob benchmarks
    let _ = trace("BENCHMARK_SECTION: BLOB_OPERATIONS");
    let _ = trace("BENCHMARK: blob_creation");
    accumulator = accumulator.wrapping_add(benchmark_blob_creation());

    let _ = trace("BENCHMARK: blob_field_access");
    accumulator = accumulator.wrapping_add(benchmark_blob_field_access(&escrow_finish));

    // Optional field access benchmarks
    let _ = trace("BENCHMARK_SECTION: OPTIONAL_FIELD_ACCESS");
    let _ = trace("BENCHMARK: optional_field_some");
    accumulator = accumulator.wrapping_add(benchmark_optional_field_some(&escrow_finish));

    let _ = trace("BENCHMARK: optional_field_none");
    accumulator = accumulator.wrapping_add(benchmark_optional_field_none(&escrow_finish));

    // Error code matching benchmarks
    let _ = trace("BENCHMARK_SECTION: ERROR_CODE_MATCHING");
    let _ = trace("BENCHMARK: match_result_code");
    accumulator = accumulator.wrapping_add(benchmark_match_result_code());

    let _ = trace("BENCHMARK: match_result_code_optional");
    accumulator = accumulator.wrapping_add(benchmark_match_result_code_optional());

    let _ = trace("BENCHMARK: match_result_code_with_expected_bytes");
    accumulator = accumulator.wrapping_add(benchmark_match_result_code_with_expected_bytes());

    let _ = trace("BENCHMARK: match_result_code_with_expected_bytes_optional");
    accumulator =
        accumulator.wrapping_add(benchmark_match_result_code_with_expected_bytes_optional());

    // Result type method benchmarks
    let _ = trace("BENCHMARK_SECTION: RESULT_TYPE_METHODS");
    let _ = trace("BENCHMARK: is_ok");
    accumulator = accumulator.wrapping_add(benchmark_is_ok(&escrow_finish));

    let _ = trace("BENCHMARK: is_err");
    accumulator = accumulator.wrapping_add(benchmark_is_err(&escrow_finish));

    let _ = trace("BENCHMARK: result_ok");
    accumulator = accumulator.wrapping_add(benchmark_result_ok(&escrow_finish));

    let _ = trace("BENCHMARK: result_err");
    accumulator = accumulator.wrapping_add(benchmark_result_err(&escrow_finish));

    // Hex decoding benchmarks
    let _ = trace("BENCHMARK_SECTION: HEX_DECODING");
    let _ = trace("BENCHMARK: decode_hex_32");
    accumulator = accumulator.wrapping_add(benchmark_decode_hex_32());

    let _ = trace("BENCHMARK: decode_hex_20");
    accumulator = accumulator.wrapping_add(benchmark_decode_hex_20());

    let _ = trace("$$$$$ GAS BENCHMARK END $$$$$");

    // Return 1 if accumulator is non-zero (it always will be), preventing optimization
    if accumulator > 0 { 1 } else { 0 }
}

/// Benchmark get_account_id_field by repeatedly calling get_account()
fn benchmark_account_id_field(escrow_finish: &EscrowFinish) -> u64 {
    let mut count = 0u64;
    for _ in 0..ITERATIONS {
        if escrow_finish.get_account().is_ok() {
            count += 1;
        }
    }
    count
}

/// Benchmark get_fee_field by repeatedly calling get_fee()
fn benchmark_fee_field(escrow_finish: &EscrowFinish) -> u64 {
    let mut count = 0u64;
    for _ in 0..ITERATIONS {
        if escrow_finish.get_fee().is_ok() {
            count += 1;
        }
    }
    count
}

/// Benchmark match_result_code (basic success case)
fn benchmark_match_result_code() -> u64 {
    let mut count = 0u64;
    for _ in 0..ITERATIONS {
        let result: Result<u32> = match_result_code(1, || 42u32);
        if result.is_ok() {
            count += 1;
        }
    }
    count
}

/// Benchmark match_result_code_optional (success with Some)
fn benchmark_match_result_code_optional() -> u64 {
    let mut count = 0u64;
    for _ in 0..ITERATIONS {
        let result: Result<Option<u32>> = match_result_code_optional(1, || Some(42u32));
        if result.is_ok() {
            count += 1;
        }
    }
    count
}

/// Benchmark match_result_code_with_expected_bytes
fn benchmark_match_result_code_with_expected_bytes() -> u64 {
    let mut count = 0u64;
    for _ in 0..ITERATIONS {
        let result: Result<u32> = match_result_code_with_expected_bytes(20, 20, || 42u32);
        if result.is_ok() {
            count += 1;
        }
    }
    count
}

/// Benchmark match_result_code_with_expected_bytes_optional
fn benchmark_match_result_code_with_expected_bytes_optional() -> u64 {
    let mut count = 0u64;
    for _ in 0..ITERATIONS {
        let result: Result<Option<u32>> =
            match_result_code_with_expected_bytes_optional(20, 20, || Some(42u32));
        if result.is_ok() {
            count += 1;
        }
    }
    count
}

/// Benchmark is_ok() checks
fn benchmark_is_ok(escrow_finish: &EscrowFinish) -> u64 {
    let mut count = 0u64;
    for _ in 0..ITERATIONS {
        if escrow_finish.get_account().is_ok() {
            count += 1;
        }
    }
    count
}

/// Benchmark is_err() checks
fn benchmark_is_err(escrow_finish: &EscrowFinish) -> u64 {
    let mut count = 0u64;
    for _ in 0..ITERATIONS {
        if !escrow_finish.get_account().is_err() {
            count += 1;
        }
    }
    count
}

/// Benchmark Result::ok() conversion
fn benchmark_result_ok(escrow_finish: &EscrowFinish) -> u64 {
    let mut count = 0u64;
    for _ in 0..ITERATIONS {
        if escrow_finish.get_account().ok().is_some() {
            count += 1;
        }
    }
    count
}

/// Benchmark Result::err() conversion
fn benchmark_result_err(escrow_finish: &EscrowFinish) -> u64 {
    let mut count = 0u64;
    for _ in 0..ITERATIONS {
        if escrow_finish.get_account().err().is_none() {
            count += 1;
        }
    }
    count
}

/// Benchmark decode_hex_32
fn benchmark_decode_hex_32() -> u64 {
    let hex = *b"0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
    let mut count = 0u64;
    for _ in 0..ITERATIONS {
        if decode_hex_32(&hex).is_some() {
            count += 1;
        }
    }
    count
}

/// Benchmark decode_hex_20
fn benchmark_decode_hex_20() -> u64 {
    let hex = *b"00112233445566778899aabbccddeeff00112233";
    let mut count = 0u64;
    for _ in 0..ITERATIONS {
        if decode_hex_20(&hex).is_some() {
            count += 1;
        }
    }
    count
}

/// Benchmark Locator::pack() - single level field access
fn benchmark_locator_pack_single() -> u64 {
    let mut count = 0u64;
    for _ in 0..ITERATIONS {
        let mut locator = Locator::new();
        if locator.pack(sfield::Account) {
            count += 1;
        }
    }
    count
}

/// Benchmark Locator::pack() - nested field access (3 levels)
fn benchmark_locator_pack_nested() -> u64 {
    let mut count = 0u64;
    for _ in 0..ITERATIONS {
        let mut locator = Locator::new();
        if locator.pack(sfield::Memos) && locator.pack(0) && locator.pack(sfield::MemoType) {
            count += 1;
        }
    }
    count
}

/// Benchmark Locator::repack_last()
fn benchmark_locator_repack_last() -> u64 {
    let mut count = 0u64;
    for _ in 0..ITERATIONS {
        let mut locator = Locator::new();
        locator.pack(sfield::Memos);
        locator.pack(0);
        if locator.repack_last(sfield::MemoData) {
            count += 1;
        }
    }
    count
}

/// Benchmark Blob struct creation and access
fn benchmark_blob_creation() -> u64 {
    let mut count = 0u64;
    for _ in 0..ITERATIONS {
        use xrpl_wasm_stdlib::core::types::blob::Blob;
        let blob = Blob {
            data: [0u8; 1024],
            len: 1024,
        };
        if blob.len > 0 {
            count += 1;
        }
    }
    count
}

/// Benchmark Amount field access (48-byte buffer)
fn benchmark_amount_field(escrow_finish: &EscrowFinish) -> u64 {
    let mut count = 0u64;
    for _ in 0..ITERATIONS {
        if escrow_finish.get_fee().is_ok() {
            count += 1;
        }
    }
    count
}

/// Benchmark Hash256 field access (32-byte buffer)
fn benchmark_hash256_field(escrow: &CurrentEscrow) -> u64 {
    let mut count = 0u64;
    for _ in 0..ITERATIONS {
        // Using get_id as a Hash256 field example
        if escrow.get_previous_txn_id().is_ok() {
            count += 1;
        }
    }
    count
}

/// Benchmark u16 field access
fn benchmark_u16_field(escrow_finish: &EscrowFinish) -> u64 {
    let mut count = 0u64;
    for _ in 0..ITERATIONS {
        // Using get_transaction_type as a u16-like field example
        if escrow_finish.get_transaction_type().is_ok() {
            count += 1;
        }
    }
    count
}

/// Benchmark u64 field access
fn benchmark_u64_field(escrow_finish: &EscrowFinish) -> u64 {
    let mut count = 0u64;
    for _ in 0..ITERATIONS {
        // Using get_computation_allowance as a u32 field
        if escrow_finish.get_computation_allowance().is_ok() {
            count += 1;
        }
    }
    count
}

/// Benchmark Blob field access from transaction
fn benchmark_blob_field_access(escrow_finish: &EscrowFinish) -> u64 {
    let mut count = 0u64;
    for _ in 0..ITERATIONS {
        if escrow_finish.get_txn_signature().is_ok() {
            count += 1;
        }
    }
    count
}

/// Benchmark optional field that exists (Some case)
fn benchmark_optional_field_some(escrow_finish: &EscrowFinish) -> u64 {
    let mut count = 0u64;
    for _ in 0..ITERATIONS {
        if let xrpl_wasm_stdlib::host::Result::Ok(Some(_)) = escrow_finish.get_account_txn_id() {
            count += 1
        }
    }
    count
}

/// Benchmark optional field that doesn't exist (None case)
fn benchmark_optional_field_none(escrow_finish: &EscrowFinish) -> u64 {
    let mut count = 0u64;
    for _ in 0..ITERATIONS {
        if let xrpl_wasm_stdlib::host::Result::Ok(None) = escrow_finish.get_account_txn_id() {
            count += 1
        }
    }
    count
}

#[cfg(test)]
mod coverage_tests {
    use super::*;

    /// Coverage test: exercises all host function categories via finish()
    ///
    /// This test runs the same logic as the integration test, but on native
    /// targets with stub host functions. It's used to measure code coverage
    /// of xrpl-wasm-stdlib.
    ///
    /// Note: The host functions return dummy values (from host_bindings_for_testing.rs),
    /// so this test verifies that the code *runs*, not that it's *correct*.
    /// Correctness is verified by the real integration tests against rippled.
    #[test]
    fn test_finish_exercises_all_host_functions() {
        // On non-wasm targets, finish() uses host_bindings_test.rs
        // which provides stub implementations of all host functions.
        let result = finish();

        // The finish() function returns 1 on success, or a negative error code.
        // With stub host functions, we expect success (though the actual
        // behavior depends on the stub implementations).
        core::assert_eq!(result, 1, "finish() should return 1 on success");
    }
}
