#![allow(unused_imports)]
#![allow(unused_variables)]
#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_common_stdlib::decode_hex_32;
use xrpl_common_stdlib::fields::locator::Locator;
use xrpl_common_stdlib::host::trace::{trace, trace_float, trace_hex, trace_num};
use xrpl_common_stdlib::host::{
    RoundingMode, cache_le, float_add, float_cmp, float_div, float_from_int, float_from_mant_exp,
    float_from_uint, float_mult, float_pow, float_root, float_sub, le_arr_len, le_field, le_inner,
};
use xrpl_common_stdlib::sfield;
use xrpl_common_stdlib::types::iou_number::{FLOAT_NEGATIVE_ONE, FLOAT_ONE};

fn test_float_from_host() {
    trace("\n$$$ test_float_from_host $$$");

    let id =
        decode_hex_32(b"97DD92D4F3A791254A530BA769F6669DEBF6B2FC8CCA46842B9031ADCD4D1ADA").unwrap();
    let slot = unsafe { cache_le(id.as_ptr(), id.len(), 0) };
    let mut buf = [0x00; 48];
    let output_len = unsafe {
        le_field(
            slot,
            sfield::LPTokenBalance.into(),
            buf.as_mut_ptr(),
            buf.len(),
        )
    };
    let f_lptokenbalance: [u8; 8] = buf[0..8].try_into().unwrap();
    trace_float("  LPTokenBalance value:", &f_lptokenbalance);

    let mut locator = Locator::new();
    locator.pack(sfield::AuctionSlot);
    locator.pack(sfield::Price);
    let output_len = unsafe {
        le_inner(
            slot,
            locator.as_ptr(),
            locator.num_packed_bytes(),
            buf.as_mut_ptr(),
            buf.len(),
        )
    };
    let f_auctionslot: [u8; 8] = buf[0..8].try_into().unwrap();
    trace_float("  AuctionSlot Price value:", &f_auctionslot);

    let id =
        decode_hex_32(b"D0A063DEE0B0EC9522CF35CD55771B5DCAFA19A133EE46A0295E4D089AF86438").unwrap();
    let slot = unsafe { cache_le(id.as_ptr(), id.len(), 0) };
    let mut buf = [0x00; 48];
    let output_len =
        unsafe { le_field(slot, sfield::TakerPays.into(), buf.as_mut_ptr(), buf.len()) };
    let f_takerpays: [u8; 8] = buf[0..8].try_into().unwrap();
    trace_float("  TakerPays:", &f_takerpays);
}

fn test_float_from_wasm() {
    trace("\n$$$ test_float_from_wasm $$$");

    let mut f: [u8; 8] = [0u8; 8];
    if 8 == unsafe { float_from_int(12300, f.as_mut_ptr(), 8, RoundingMode::ToNearest.into()) } {
        trace_float("  float from i64 12300:", &f);
        trace_hex("  float from i64 12300 as HEX:", &f);
    } else {
        trace("  float from i64 12300: failed");
    }

    let u64_value: u64 = 12300;
    if 8 == unsafe {
        float_from_uint(
            &u64_value as *const u64 as *const u8,
            8,
            f.as_mut_ptr(),
            8,
            RoundingMode::ToNearest.into(),
        )
    } {
        trace_float("  float from u64 12300:", &f);
    } else {
        trace("  float from u64 12300: failed");
    }

    if 8 == unsafe {
        float_from_mant_exp(123, 2, f.as_mut_ptr(), 8, RoundingMode::ToNearest.into())
    } {
        trace_float("  float from exp 2, mantissa 123:", &f);
    } else {
        trace("  float from exp 2, mantissa 3: failed");
    }

    trace_float("  float from const 1:", &FLOAT_ONE);
    trace_float("  float from const -1:", &FLOAT_NEGATIVE_ONE);
}

fn test_float_cmp() {
    trace("\n$$$ test_float_cmp $$$");

    let mut f1: [u8; 8] = [0u8; 8];
    if 8 != unsafe { float_from_int(1, f1.as_mut_ptr(), 8, RoundingMode::ToNearest.into()) } {
        trace("  float from 1: failed");
    } else {
        trace_float("  float from 1:", &f1);
    }

    if 0 == unsafe { float_cmp(f1.as_ptr(), 8, FLOAT_ONE.as_ptr(), 8) } {
        trace("  float from 1 == FLOAT_ONE");
    } else {
        trace("  float from 1 != FLOAT_ONE");
    }

    if 1 == unsafe { float_cmp(f1.as_ptr(), 8, FLOAT_NEGATIVE_ONE.as_ptr(), 8) } {
        trace("  float from 1 > FLOAT_NEGATIVE_ONE");
    } else {
        trace("  float from 1 !> FLOAT_NEGATIVE_ONE");
    }

    if 2 == unsafe { float_cmp(FLOAT_NEGATIVE_ONE.as_ptr(), 8, f1.as_ptr(), 8) } {
        trace("  FLOAT_NEGATIVE_ONE < float from 1");
    } else {
        trace("  FLOAT_NEGATIVE_ONE !< float from 1");
    }
}

fn test_float_add_subtract() {
    trace("\n$$$ test_float_add_subtract $$$");

    let mut f_compute: [u8; 8] = FLOAT_ONE;
    for i in 0..9 {
        unsafe {
            float_add(
                f_compute.as_ptr(),
                8,
                FLOAT_ONE.as_ptr(),
                8,
                f_compute.as_mut_ptr(),
                8,
                RoundingMode::ToNearest.into(),
            )
        };
        // trace_float("  float:", &f_compute);
    }
    let mut f10: [u8; 8] = [0u8; 8];
    if 8 != unsafe { float_from_int(10, f10.as_mut_ptr(), 8, RoundingMode::ToNearest.into()) } {
        // trace("  float from 10: failed");
    }
    if 0 == unsafe { float_cmp(f10.as_ptr(), 8, f_compute.as_ptr(), 8) } {
        trace("  repeated add: good");
    } else {
        trace("  repeated add: bad");
    }

    for i in 0..11 {
        unsafe {
            float_sub(
                f_compute.as_ptr(),
                8,
                FLOAT_ONE.as_ptr(),
                8,
                f_compute.as_mut_ptr(),
                8,
                RoundingMode::ToNearest.into(),
            )
        };
    }
    if 0 == unsafe { float_cmp(f_compute.as_ptr(), 8, FLOAT_NEGATIVE_ONE.as_ptr(), 8) } {
        trace("  repeated subtract: good");
    } else {
        trace("  repeated subtract: bad");
    }
}

fn test_float_mult_divide() {
    trace("\n$$$ test_float_mult_divide $$$");

    let mut f10: [u8; 8] = [0u8; 8];
    unsafe { float_from_int(10, f10.as_mut_ptr(), 8, RoundingMode::ToNearest.into()) };
    let mut f_compute: [u8; 8] = FLOAT_ONE;
    for i in 0..6 {
        unsafe {
            float_mult(
                f_compute.as_ptr(),
                8,
                f10.as_ptr(),
                8,
                f_compute.as_mut_ptr(),
                8,
                RoundingMode::ToNearest.into(),
            )
        };
        // trace_float("  float:", &f_compute);
    }
    let mut f1000000: [u8; 8] = [0u8; 8];
    unsafe {
        float_from_int(
            1000000,
            f1000000.as_mut_ptr(),
            8,
            RoundingMode::ToNearest.into(),
        )
    };

    if 0 == unsafe { float_cmp(f1000000.as_ptr(), 8, f_compute.as_ptr(), 8) } {
        trace("  repeated multiply: good");
    } else {
        trace("  repeated multiply: bad");
    }

    for i in 0..7 {
        unsafe {
            float_div(
                f_compute.as_ptr(),
                8,
                f10.as_ptr(),
                8,
                f_compute.as_mut_ptr(),
                8,
                RoundingMode::ToNearest.into(),
            )
        };
    }
    let mut f01: [u8; 8] = [0u8; 8];
    unsafe { float_from_mant_exp(1, -1, f01.as_mut_ptr(), 8, RoundingMode::ToNearest.into()) };

    if 0 == unsafe { float_cmp(f_compute.as_ptr(), 8, f01.as_ptr(), 8) } {
        trace("  repeated divide: good");
    } else {
        trace("  repeated divide: bad");
    }
}

fn test_float_pow() {
    trace("\n$$$ test_float_pow $$$");

    let mut f_compute: [u8; 8] = [0u8; 8];
    unsafe {
        float_pow(
            FLOAT_ONE.as_ptr(),
            8,
            3,
            f_compute.as_mut_ptr(),
            8,
            RoundingMode::ToNearest.into(),
        )
    };
    trace_float("  float cube of 1:", &f_compute);

    unsafe {
        float_pow(
            FLOAT_NEGATIVE_ONE.as_ptr(),
            8,
            6,
            f_compute.as_mut_ptr(),
            8,
            RoundingMode::ToNearest.into(),
        )
    };
    trace_float("  float 6th power of -1:", &f_compute);

    let mut f9: [u8; 8] = [0u8; 8];
    unsafe { float_from_int(9, f9.as_mut_ptr(), 8, RoundingMode::ToNearest.into()) };
    unsafe {
        float_pow(
            f9.as_ptr(),
            8,
            2,
            f_compute.as_mut_ptr(),
            8,
            RoundingMode::ToNearest.into(),
        )
    };
    trace_float("  float square of 9:", &f_compute);

    unsafe {
        float_pow(
            f9.as_ptr(),
            8,
            0,
            f_compute.as_mut_ptr(),
            8,
            RoundingMode::ToNearest.into(),
        )
    };
    trace_float("  float 0th power of 9:", &f_compute);

    let mut f0: [u8; 8] = [0u8; 8];
    unsafe { float_from_int(0, f0.as_mut_ptr(), 8, RoundingMode::ToNearest.into()) };
    unsafe {
        float_pow(
            f0.as_ptr(),
            8,
            2,
            f_compute.as_mut_ptr(),
            8,
            RoundingMode::ToNearest.into(),
        )
    };
    trace_float("  float square of 0:", &f_compute);

    let r = unsafe {
        float_pow(
            f0.as_ptr(),
            8,
            0,
            f_compute.as_mut_ptr(),
            8,
            RoundingMode::ToNearest.into(),
        )
    };
    trace_num(
        "  float 0th power of 0 (expecting INVALID_PARAMS error):",
        r as i64,
    );
}

fn test_float_root() {
    trace("\n$$$ test_float_root $$$");

    let mut f9: [u8; 8] = [0u8; 8];
    unsafe { float_from_int(9, f9.as_mut_ptr(), 8, RoundingMode::ToNearest.into()) };
    let mut f_compute: [u8; 8] = [0u8; 8];
    unsafe {
        float_root(
            f9.as_ptr(),
            8,
            2,
            f_compute.as_mut_ptr(),
            8,
            RoundingMode::ToNearest.into(),
        )
    };
    trace_float("  float sqrt of 9:", &f_compute);
    unsafe {
        float_root(
            f9.as_ptr(),
            8,
            3,
            f_compute.as_mut_ptr(),
            8,
            RoundingMode::ToNearest.into(),
        )
    };
    trace_float("  float cbrt of 9:", &f_compute);

    let mut f1000000: [u8; 8] = [0u8; 8];
    unsafe {
        float_from_int(
            1000000,
            f1000000.as_mut_ptr(),
            8,
            RoundingMode::ToNearest.into(),
        )
    };
    unsafe {
        float_root(
            f1000000.as_ptr(),
            8,
            3,
            f_compute.as_mut_ptr(),
            8,
            RoundingMode::ToNearest.into(),
        )
    };
    trace_float("  float cbrt of 1000000:", &f_compute);
    unsafe {
        float_root(
            f1000000.as_ptr(),
            8,
            6,
            f_compute.as_mut_ptr(),
            8,
            RoundingMode::ToNearest.into(),
        )
    };
    trace_float("  float 6th root of 1000000:", &f_compute);
}

fn test_float_negate() {
    trace("\n$$$ test_float_negate $$$");

    let mut f_compute: [u8; 8] = [0u8; 8];
    unsafe {
        float_mult(
            FLOAT_ONE.as_ptr(),
            8,
            FLOAT_NEGATIVE_ONE.as_ptr(),
            8,
            f_compute.as_mut_ptr(),
            8,
            RoundingMode::ToNearest.into(),
        )
    };
    // trace_float("  float:", &f_compute);
    if 0 == unsafe { float_cmp(FLOAT_NEGATIVE_ONE.as_ptr(), 8, f_compute.as_ptr(), 8) } {
        trace("  negate const 1: good");
    } else {
        trace("  negate const 1: bad");
    }

    unsafe {
        float_mult(
            FLOAT_NEGATIVE_ONE.as_ptr(),
            8,
            FLOAT_NEGATIVE_ONE.as_ptr(),
            8,
            f_compute.as_mut_ptr(),
            8,
            RoundingMode::ToNearest.into(),
        )
    };
    // trace_float("  float:", &f_compute);
    if 0 == unsafe { float_cmp(FLOAT_ONE.as_ptr(), 8, f_compute.as_ptr(), 8) } {
        trace("  negate const -1: good");
    } else {
        trace("  negate const -1: bad");
    }
}

fn test_float_invert() {
    trace("\n$$$ test_float_invert $$$");

    let mut f_compute: [u8; 8] = [0u8; 8];
    let mut f10: [u8; 8] = [0u8; 8];
    unsafe { float_from_int(10, f10.as_mut_ptr(), 8, RoundingMode::ToNearest.into()) };
    unsafe {
        float_div(
            FLOAT_ONE.as_ptr(),
            8,
            f10.as_ptr(),
            8,
            f_compute.as_mut_ptr(),
            8,
            RoundingMode::ToNearest.into(),
        )
    };
    trace_float("  invert a float from 10:", &f_compute);
    unsafe {
        float_div(
            FLOAT_ONE.as_ptr(),
            8,
            f_compute.as_ptr(),
            8,
            f_compute.as_mut_ptr(),
            8,
            RoundingMode::ToNearest.into(),
        )
    };
    trace_float("  invert again:", &f_compute);

    // if f10's value is 7, then invert twice won't match the original value
    if 0 == unsafe { float_cmp(f10.as_ptr(), 8, f_compute.as_ptr(), 8) } {
        trace("  invert twice: good");
    } else {
        trace("  invert twice: bad");
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn escrow_finish() -> i32 {
    test_float_from_host();
    test_float_from_wasm();
    test_float_cmp();
    test_float_add_subtract();
    test_float_mult_divide();
    test_float_pow();
    test_float_root();
    test_float_negate();
    test_float_invert();

    1 // <-- Finish the escrow to indicate a successful outcome
}

#[cfg(test)]
mod coverage_tests {
    use super::*;

    /// Coverage test: exercises all host function categories via escrow_finish()
    ///
    /// This test runs the same logic as the integration test, but on native
    /// targets with stub host functions. It's used to measure code coverage
    /// of xrpl-common-stdlib.
    ///
    /// Note: The host functions return dummy values (from host_bindings_for_testing.rs),
    /// so this test verifies that the code *runs*, not that it's *correct*.
    /// Correctness is verified by the real integration tests against rippled.
    #[test]
    fn test_finish_exercises_all_host_functions() {
        // On non-wasm targets, escrow_finish() uses host_bindings_for_testing.rs
        // which provides stub implementations of all host functions.
        let result = escrow_finish();

        // The escrow_finish() function returns 1 on success, or a negative error code.
        // With stub host functions, we expect success (though the actual
        // behavior depends on the stub implementations).
        core::assert_eq!(result, 1, "escrow_finish() should return 1 on success");
    }
}
