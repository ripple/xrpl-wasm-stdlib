#![allow(unused_imports)]
#![allow(unused_variables)]
#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_wasm_stdlib::core::locator::Locator;
use xrpl_wasm_stdlib::decode_hex_32;
use xrpl_wasm_stdlib::host::trace::DataRepr::AsHex;
use xrpl_wasm_stdlib::host::trace::{DataRepr, trace, trace_data, trace_float, trace_num};
use xrpl_wasm_stdlib::host::{
    FLOAT_ROUNDING_MODES_TO_NEAREST, cache_ledger_obj, float_abs, float_add, float_compare,
    float_divide, float_from_int, float_from_stamount, float_from_stnumber, float_from_uint,
    float_log, float_multiply, float_negate, float_pow, float_root, float_set, float_subtract,
    float_to_int, float_to_mantissa_and_exponent, get_ledger_obj_array_len, get_ledger_obj_field,
    get_ledger_obj_nested_field, trace_opaque_float,
};
use xrpl_wasm_stdlib::sfield;
use xrpl_wasm_stdlib::sfield::{
    Account, AccountTxnID, Balance, Domain, EmailHash, Flags, LedgerEntryType, MessageKey,
    OwnerCount, PreviousTxnID, PreviousTxnLgrSeq, RegularKey, Sequence, TicketCount, TransferRate,
};

/// Helper: create float from i64
fn make_float(val: i64) -> [u8; 12] {
    let mut f = [0u8; 12];
    let r = unsafe { float_from_int(val, f.as_mut_ptr(), 12, FLOAT_ROUNDING_MODES_TO_NEAREST) };
    if r != 12 {
        let _ = trace_num("  make_float failed for value:", val);
        let _ = trace_num("  error code:", r as i64);
    }
    f
}

fn test_float_from_host() {
    let _ = trace("\n$$$ test_float_from_host $$$");

    let keylet =
        decode_hex_32(b"97DD92D4F3A791254A530BA769F6669DEBF6B2FC8CCA46842B9031ADCD4D1ADA").unwrap();
    let slot = unsafe { cache_ledger_obj(keylet.as_ptr(), keylet.len(), 0) };
    let mut buf = [0x00; 48];
    let output_len = unsafe {
        get_ledger_obj_field(
            slot,
            sfield::LPTokenBalance.into(),
            buf.as_mut_ptr(),
            buf.len(),
        )
    };
    // Convert STAmount to 12-byte float
    let mut f_lptokenbalance = [0u8; 12];
    unsafe {
        float_from_stamount(
            buf.as_ptr(),
            output_len as usize,
            f_lptokenbalance.as_mut_ptr(),
            12,
            FLOAT_ROUNDING_MODES_TO_NEAREST,
        )
    };
    let _ = trace_float("  LPTokenBalance value:", &f_lptokenbalance);

    let mut locator = Locator::new();
    locator.pack(sfield::AuctionSlot);
    locator.pack(sfield::Price);
    let output_len = unsafe {
        get_ledger_obj_nested_field(
            slot,
            locator.as_ptr(),
            locator.num_packed_bytes(),
            buf.as_mut_ptr(),
            buf.len(),
        )
    };
    // Convert STAmount to 12-byte float
    let mut f_auctionslot = [0u8; 12];
    unsafe {
        float_from_stamount(
            buf.as_ptr(),
            output_len as usize,
            f_auctionslot.as_mut_ptr(),
            12,
            FLOAT_ROUNDING_MODES_TO_NEAREST,
        )
    };
    let _ = trace_float("  AuctionSlot Price value:", &f_auctionslot);

    let keylet =
        decode_hex_32(b"D0A063DEE0B0EC9522CF35CD55771B5DCAFA19A133EE46A0295E4D089AF86438").unwrap();
    let slot = unsafe { cache_ledger_obj(keylet.as_ptr(), keylet.len(), 0) };
    let mut buf = [0x00; 48];
    let output_len = unsafe {
        get_ledger_obj_field(slot, sfield::TakerPays.into(), buf.as_mut_ptr(), buf.len())
    };
    // Convert STAmount to 12-byte float
    let mut f_takerpays = [0u8; 12];
    unsafe {
        float_from_stamount(
            buf.as_ptr(),
            output_len as usize,
            f_takerpays.as_mut_ptr(),
            12,
            FLOAT_ROUNDING_MODES_TO_NEAREST,
        )
    };
    let _ = trace_float("  TakerPays:", &f_takerpays);
}

fn test_float_from_wasm() {
    let _ = trace("\n$$$ test_float_from_wasm $$$");

    let mut f: [u8; 12] = [0u8; 12];
    if 12 == unsafe { float_from_int(12300, f.as_mut_ptr(), 12, FLOAT_ROUNDING_MODES_TO_NEAREST) } {
        let _ = trace_float("  float from i64 12300:", &f);
        let _ = trace_data("  float from i64 12300 as HEX:", &f, AsHex);
    } else {
        let _ = trace("  float from i64 12300: failed");
    }

    let u64_value: u64 = 12300;
    if 12
        == unsafe {
            float_from_uint(
                &u64_value as *const u64 as *const u8,
                8,
                f.as_mut_ptr(),
                12,
                FLOAT_ROUNDING_MODES_TO_NEAREST,
            )
        }
    {
        let _ = trace_float("  float from u64 12300:", &f);
    } else {
        let _ = trace("  float from u64 12300: failed");
    }

    if 12 == unsafe { float_set(2, 123, f.as_mut_ptr(), 12, FLOAT_ROUNDING_MODES_TO_NEAREST) } {
        let _ = trace_float("  float from exp 2, mantissa 123:", &f);
    } else {
        let _ = trace("  float from exp 2, mantissa 3: failed");
    }

    let float_one = make_float(1);
    let float_neg_one = make_float(-1);
    let _ = trace_float("  float from int 1:", &float_one);
    let _ = trace_float("  float from int -1:", &float_neg_one);
}

fn test_float_compare() {
    let _ = trace("\n$$$ test_float_compare $$$");

    let float_one = make_float(1);
    let float_neg_one = make_float(-1);

    let mut f1: [u8; 12] = [0u8; 12];
    if 12 != unsafe { float_from_int(1, f1.as_mut_ptr(), 12, FLOAT_ROUNDING_MODES_TO_NEAREST) } {
        let _ = trace("  float from 1: failed");
    } else {
        let _ = trace_float("  float from 1:", &f1);
    }

    if 0 == unsafe { float_compare(f1.as_ptr(), 12, float_one.as_ptr(), 12) } {
        let _ = trace("  float from 1 == FLOAT_ONE");
    } else {
        let _ = trace("  float from 1 != FLOAT_ONE");
    }

    if 1 == unsafe { float_compare(f1.as_ptr(), 12, float_neg_one.as_ptr(), 12) } {
        let _ = trace("  float from 1 > FLOAT_NEGATIVE_ONE");
    } else {
        let _ = trace("  float from 1 !> FLOAT_NEGATIVE_ONE");
    }

    if 2 == unsafe { float_compare(float_neg_one.as_ptr(), 12, f1.as_ptr(), 12) } {
        let _ = trace("  FLOAT_NEGATIVE_ONE < float from 1");
    } else {
        let _ = trace("  FLOAT_NEGATIVE_ONE !< float from 1");
    }
}

fn test_float_add_subtract() {
    let _ = trace("\n$$$ test_float_add_subtract $$$");

    let float_one = make_float(1);
    let float_neg_one = make_float(-1);

    let mut f_compute: [u8; 12] = float_one;
    for _i in 0..9 {
        unsafe {
            float_add(
                f_compute.as_ptr(),
                12,
                float_one.as_ptr(),
                12,
                f_compute.as_mut_ptr(),
                12,
                FLOAT_ROUNDING_MODES_TO_NEAREST,
            )
        };
    }
    let mut f10: [u8; 12] = [0u8; 12];
    unsafe { float_from_int(10, f10.as_mut_ptr(), 12, FLOAT_ROUNDING_MODES_TO_NEAREST) };
    if 0 == unsafe { float_compare(f10.as_ptr(), 12, f_compute.as_ptr(), 12) } {
        let _ = trace("  repeated add: good");
    } else {
        let _ = trace("  repeated add: bad");
    }

    for _i in 0..11 {
        unsafe {
            float_subtract(
                f_compute.as_ptr(),
                12,
                float_one.as_ptr(),
                12,
                f_compute.as_mut_ptr(),
                12,
                FLOAT_ROUNDING_MODES_TO_NEAREST,
            )
        };
    }
    if 0 == unsafe { float_compare(f_compute.as_ptr(), 12, float_neg_one.as_ptr(), 12) } {
        let _ = trace("  repeated subtract: good");
    } else {
        let _ = trace("  repeated subtract: bad");
    }
}

fn test_float_multiply_divide() {
    let _ = trace("\n$$$ test_float_multiply_divide $$$");

    let float_one = make_float(1);

    let mut f10: [u8; 12] = [0u8; 12];
    unsafe { float_from_int(10, f10.as_mut_ptr(), 12, FLOAT_ROUNDING_MODES_TO_NEAREST) };
    let mut f_compute: [u8; 12] = float_one;
    for _i in 0..6 {
        unsafe {
            float_multiply(
                f_compute.as_ptr(),
                12,
                f10.as_ptr(),
                12,
                f_compute.as_mut_ptr(),
                12,
                FLOAT_ROUNDING_MODES_TO_NEAREST,
            )
        };
    }
    let mut f1000000: [u8; 12] = [0u8; 12];
    unsafe {
        float_from_int(
            1000000,
            f1000000.as_mut_ptr(),
            12,
            FLOAT_ROUNDING_MODES_TO_NEAREST,
        )
    };

    if 0 == unsafe { float_compare(f1000000.as_ptr(), 12, f_compute.as_ptr(), 12) } {
        let _ = trace("  repeated multiply: good");
    } else {
        let _ = trace("  repeated multiply: bad");
    }

    for _i in 0..7 {
        unsafe {
            float_divide(
                f_compute.as_ptr(),
                12,
                f10.as_ptr(),
                12,
                f_compute.as_mut_ptr(),
                12,
                FLOAT_ROUNDING_MODES_TO_NEAREST,
            )
        };
    }
    let mut f01: [u8; 12] = [0u8; 12];
    unsafe { float_set(-1, 1, f01.as_mut_ptr(), 12, FLOAT_ROUNDING_MODES_TO_NEAREST) };

    if 0 == unsafe { float_compare(f_compute.as_ptr(), 12, f01.as_ptr(), 12) } {
        let _ = trace("  repeated divide: good");
    } else {
        let _ = trace("  repeated divide: bad");
    }
}

fn test_float_pow() {
    let _ = trace("\n$$$ test_float_pow $$$");

    let float_one = make_float(1);
    let float_neg_one = make_float(-1);

    let mut f_compute: [u8; 12] = [0u8; 12];
    unsafe {
        float_pow(
            float_one.as_ptr(),
            12,
            3,
            f_compute.as_mut_ptr(),
            12,
            FLOAT_ROUNDING_MODES_TO_NEAREST,
        )
    };
    let _ = trace_float("  float cube of 1:", &f_compute);

    unsafe {
        float_pow(
            float_neg_one.as_ptr(),
            12,
            6,
            f_compute.as_mut_ptr(),
            12,
            FLOAT_ROUNDING_MODES_TO_NEAREST,
        )
    };
    let _ = trace_float("  float 6th power of -1:", &f_compute);

    let mut f9: [u8; 12] = [0u8; 12];
    unsafe { float_from_int(9, f9.as_mut_ptr(), 12, FLOAT_ROUNDING_MODES_TO_NEAREST) };
    unsafe {
        float_pow(
            f9.as_ptr(),
            12,
            2,
            f_compute.as_mut_ptr(),
            12,
            FLOAT_ROUNDING_MODES_TO_NEAREST,
        )
    };
    let _ = trace_float("  float square of 9:", &f_compute);

    unsafe {
        float_pow(
            f9.as_ptr(),
            12,
            0,
            f_compute.as_mut_ptr(),
            12,
            FLOAT_ROUNDING_MODES_TO_NEAREST,
        )
    };
    let _ = trace_float("  float 0th power of 9:", &f_compute);

    let mut f0: [u8; 12] = [0u8; 12];
    unsafe { float_from_int(0, f0.as_mut_ptr(), 12, FLOAT_ROUNDING_MODES_TO_NEAREST) };
    unsafe {
        float_pow(
            f0.as_ptr(),
            12,
            2,
            f_compute.as_mut_ptr(),
            12,
            FLOAT_ROUNDING_MODES_TO_NEAREST,
        )
    };
    let _ = trace_float("  float square of 0:", &f_compute);

    let r = unsafe {
        float_pow(
            f0.as_ptr(),
            12,
            0,
            f_compute.as_mut_ptr(),
            12,
            FLOAT_ROUNDING_MODES_TO_NEAREST,
        )
    };
    let _ = trace_num(
        "  float 0th power of 0 (expecting INVALID_PARAMS error):",
        r as i64,
    );
}

fn test_float_root() {
    let _ = trace("\n$$$ test_float_root $$$");

    let mut f9: [u8; 12] = [0u8; 12];
    unsafe { float_from_int(9, f9.as_mut_ptr(), 12, FLOAT_ROUNDING_MODES_TO_NEAREST) };
    let mut f_compute: [u8; 12] = [0u8; 12];
    unsafe {
        float_root(
            f9.as_ptr(),
            12,
            2,
            f_compute.as_mut_ptr(),
            12,
            FLOAT_ROUNDING_MODES_TO_NEAREST,
        )
    };
    let _ = trace_float("  float sqrt of 9:", &f_compute);
    unsafe {
        float_root(
            f9.as_ptr(),
            12,
            3,
            f_compute.as_mut_ptr(),
            12,
            FLOAT_ROUNDING_MODES_TO_NEAREST,
        )
    };
    let _ = trace_float("  float cbrt of 9:", &f_compute);

    let mut f1000000: [u8; 12] = [0u8; 12];
    unsafe {
        float_from_int(
            1000000,
            f1000000.as_mut_ptr(),
            12,
            FLOAT_ROUNDING_MODES_TO_NEAREST,
        )
    };
    unsafe {
        float_root(
            f1000000.as_ptr(),
            12,
            3,
            f_compute.as_mut_ptr(),
            12,
            FLOAT_ROUNDING_MODES_TO_NEAREST,
        )
    };
    let _ = trace_float("  float cbrt of 1000000:", &f_compute);
    unsafe {
        float_root(
            f1000000.as_ptr(),
            12,
            6,
            f_compute.as_mut_ptr(),
            12,
            FLOAT_ROUNDING_MODES_TO_NEAREST,
        )
    };
    let _ = trace_float("  float 6th root of 1000000:", &f_compute);
}

fn test_float_log() {
    let _ = trace("\n$$$ test_float_log $$$");

    let mut f1000000: [u8; 12] = [0u8; 12];
    unsafe {
        float_from_int(
            1000000,
            f1000000.as_mut_ptr(),
            12,
            FLOAT_ROUNDING_MODES_TO_NEAREST,
        )
    };
    let mut f_compute: [u8; 12] = [0u8; 12];
    unsafe {
        float_log(
            f1000000.as_ptr(),
            12,
            f_compute.as_mut_ptr(),
            12,
            FLOAT_ROUNDING_MODES_TO_NEAREST,
        )
    };
    let _ = trace_float("  log_10 of 1000000:", &f_compute);
}

fn test_float_negate() {
    let _ = trace("\n$$$ test_float_negate $$$");

    let float_one = make_float(1);
    let float_neg_one = make_float(-1);

    // Test using float_negate host function
    let mut f_compute: [u8; 12] = [0u8; 12];
    unsafe { float_negate(float_one.as_ptr(), 12, f_compute.as_mut_ptr(), 12) };
    if 0 == unsafe { float_compare(float_neg_one.as_ptr(), 12, f_compute.as_ptr(), 12) } {
        let _ = trace("  negate const 1: good");
    } else {
        let _ = trace("  negate const 1: bad");
    }

    unsafe { float_negate(float_neg_one.as_ptr(), 12, f_compute.as_mut_ptr(), 12) };
    if 0 == unsafe { float_compare(float_one.as_ptr(), 12, f_compute.as_ptr(), 12) } {
        let _ = trace("  negate const -1: good");
    } else {
        let _ = trace("  negate const -1: bad");
    }
}

fn test_float_abs() {
    let _ = trace("\n$$$ test_float_abs $$$");

    let float_one = make_float(1);
    let float_neg_one = make_float(-1);

    let mut f_compute: [u8; 12] = [0u8; 12];
    unsafe { float_abs(float_neg_one.as_ptr(), 12, f_compute.as_mut_ptr(), 12) };
    if 0 == unsafe { float_compare(float_one.as_ptr(), 12, f_compute.as_ptr(), 12) } {
        let _ = trace("  abs of -1: good");
    } else {
        let _ = trace("  abs of -1: bad");
    }

    unsafe { float_abs(float_one.as_ptr(), 12, f_compute.as_mut_ptr(), 12) };
    if 0 == unsafe { float_compare(float_one.as_ptr(), 12, f_compute.as_ptr(), 12) } {
        let _ = trace("  abs of 1: good");
    } else {
        let _ = trace("  abs of 1: bad");
    }
}

fn test_float_invert() {
    let _ = trace("\n$$$ test_float_invert $$$");

    let float_one = make_float(1);

    let mut f_compute: [u8; 12] = [0u8; 12];
    let mut f10: [u8; 12] = [0u8; 12];
    unsafe { float_from_int(10, f10.as_mut_ptr(), 12, FLOAT_ROUNDING_MODES_TO_NEAREST) };
    unsafe {
        float_divide(
            float_one.as_ptr(),
            12,
            f10.as_ptr(),
            12,
            f_compute.as_mut_ptr(),
            12,
            FLOAT_ROUNDING_MODES_TO_NEAREST,
        )
    };
    let _ = trace_float("  invert a float from 10:", &f_compute);
    unsafe {
        float_divide(
            float_one.as_ptr(),
            12,
            f_compute.as_ptr(),
            12,
            f_compute.as_mut_ptr(),
            12,
            FLOAT_ROUNDING_MODES_TO_NEAREST,
        )
    };
    let _ = trace_float("  invert again:", &f_compute);

    if 0 == unsafe { float_compare(f10.as_ptr(), 12, f_compute.as_ptr(), 12) } {
        let _ = trace("  invert twice: good");
    } else {
        let _ = trace("  invert twice: bad");
    }
}

fn test_float_to_int() {
    let _ = trace("\n$$$ test_float_to_int $$$");

    let f42 = make_float(42);
    let mut int_buf = [0u8; 8];
    let r = unsafe {
        float_to_int(
            f42.as_ptr(),
            12,
            int_buf.as_mut_ptr(),
            8,
            FLOAT_ROUNDING_MODES_TO_NEAREST,
        )
    };
    let _ = trace_num("  float_to_int(42) result:", r as i64);
    let val = i64::from_le_bytes(int_buf);
    let _ = trace_num("  float_to_int(42) value:", val);
}

fn test_float_to_mantissa_and_exponent() {
    let _ = trace("\n$$$ test_float_to_mantissa_and_exponent $$$");

    let f123 = make_float(123);
    let mut mantissa_buf = [0u8; 8];
    let mut exp_buf = [0u8; 4];
    let r = unsafe {
        float_to_mantissa_and_exponent(
            f123.as_ptr(),
            12,
            mantissa_buf.as_mut_ptr(),
            8,
            exp_buf.as_mut_ptr(),
            4,
        )
    };
    let _ = trace_num("  float_to_mantissa_and_exponent(123) result:", r as i64);
}

#[unsafe(no_mangle)]
pub extern "C" fn finish() -> i32 {
    test_float_from_host();
    test_float_from_wasm();
    test_float_compare();
    test_float_add_subtract();
    test_float_multiply_divide();
    test_float_pow();
    test_float_root();
    test_float_log();
    test_float_negate();
    test_float_abs();
    test_float_invert();
    test_float_to_int();
    test_float_to_mantissa_and_exponent();

    1 // <-- Finish the escrow to indicate a successful outcome
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
        // On non-wasm targets, finish() uses host_bindings_for_testing.rs
        // which provides stub implementations of all host functions.
        let result = finish();

        // The finish() function returns 1 on success, or a negative error code.
        // With stub host functions, we expect success (though the actual
        // behavior depends on the stub implementations).
        core::assert_eq!(result, 1, "finish() should return 1 on success");
    }
}
