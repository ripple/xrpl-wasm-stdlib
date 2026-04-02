#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_wasm_stdlib::core::params::function::get_function_param;
use xrpl_wasm_stdlib::core::params::instance::get_instance_param;
use xrpl_wasm_stdlib::core::types::amount::Amount;
use xrpl_wasm_stdlib::host::trace::{trace, trace_num, trace_data, DataRepr};

// ============================================================================
// Instance Parameter Tests
// ============================================================================

/// Test XRP Amount as instance parameter
/// Expects instance param 0 = XRP Amount of 1,000,000 drops (1 XRP)
#[unsafe(no_mangle)]
pub extern "C" fn instance_amount_xrp() -> i32 {
    let _ = trace("=== Instance Amount XRP ===");

    let amount = match get_instance_param::<Amount>(0) {
        Ok(a) => a,
        Err(err) => {
            let _ = trace_num("XRP instance param error:", err as i64);
            return -1;
        }
    };

    match amount {
        Amount::XRP { num_drops } => {
            let _ = trace_num("XRP drops:", num_drops);
            if num_drops != 1_000_000 {
                let _ = trace_num("Expected 1000000, got:", num_drops);
                return -2;
            }
        }
        _ => {
            let _ = trace("Expected XRP variant, got something else");
            return -3;
        }
    }

    let _ = trace("Instance Amount XRP: PASS");
    0
}

/// Test IOU Amount as instance parameter
/// Expects instance param 1 = IOU Amount of USD 1.2
#[unsafe(no_mangle)]
pub extern "C" fn instance_amount_iou() -> i32 {
    let _ = trace("=== Instance Amount IOU ===");

    let amount = match get_instance_param::<Amount>(1) {
        Ok(a) => a,
        Err(err) => {
            let _ = trace_num("IOU instance param error:", err as i64);
            return -1;
        }
    };

    match &amount {
        Amount::IOU { amount: opaque, issuer, currency } => {
            let _ = trace_data("IOU amount bytes:", &opaque.0, DataRepr::AsHex);
            let _ = trace_data("IOU issuer:", &issuer.0, DataRepr::AsHex);
            let _ = trace_data("IOU currency:", &currency.0, DataRepr::AsHex);

            // Verify currency is USD (0x00..00 + "USD" + 0x00..00)
            let expected_usd: [u8; 20] = [
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x55, 0x53, 0x44, 0x00,
                0x00, 0x00, 0x00, 0x00
            ];
            if currency.0 != expected_usd {
                let _ = trace("Currency mismatch - expected USD");
                return -2;
            }

            // Verify issuer is not zero
            let zero_account = [0u8; 20];
            if issuer.0 == zero_account {
                let _ = trace("Issuer is zero account");
                return -3;
            }
        }
        _ => {
            let _ = trace("Expected IOU variant, got something else");
            return -4;
        }
    }

    let _ = trace("Instance Amount IOU: PASS");
    0
}

/// Test MPT Amount as instance parameter
/// Expects instance param 2 = MPT Amount of 500 units
#[unsafe(no_mangle)]
pub extern "C" fn instance_amount_mpt() -> i32 {
    let _ = trace("=== Instance Amount MPT ===");

    let amount = match get_instance_param::<Amount>(2) {
        Ok(a) => a,
        Err(err) => {
            let _ = trace_num("MPT instance param error:", err as i64);
            return -1;
        }
    };

    match &amount {
        Amount::MPT { num_units, is_positive, mpt_id } => {
            let _ = trace_num("MPT units:", *num_units as i64);
            let _ = trace_num("MPT positive:", *is_positive as i64);
            let _ = trace_data("MPT id:", mpt_id.as_bytes(), DataRepr::AsHex);

            if *num_units != 500 {
                let _ = trace_num("Expected 500, got:", *num_units as i64);
                return -2;
            }
            if !is_positive {
                let _ = trace("Expected positive amount");
                return -3;
            }
        }
        _ => {
            let _ = trace("Expected MPT variant, got something else");
            return -4;
        }
    }

    let _ = trace("Instance Amount MPT: PASS");
    0
}

// ============================================================================
// Function Parameter Tests
// ============================================================================

/// Test all three Amount types as function parameters in one call
/// Expects:
///   param 0 = XRP Amount (2,000,000 drops = 2 XRP)
///   param 1 = IOU Amount (USD 3.5)
///   param 2 = MPT Amount (1000 units)
#[unsafe(no_mangle)]
pub extern "C" fn function_amount_all() -> i32 {
    let _ = trace("=== Function Amount All ===");

    // XRP
    let xrp = match get_function_param::<Amount>(0) {
        Ok(a) => a,
        Err(err) => {
            let _ = trace_num("XRP function param error:", err as i64);
            return -1;
        }
    };
    match xrp {
        Amount::XRP { num_drops } => {
            let _ = trace_num("XRP drops:", num_drops);
            if num_drops != 2_000_000 {
                let _ = trace_num("Expected 2000000, got:", num_drops);
                return -2;
            }
        }
        _ => {
            let _ = trace("Param 0: expected XRP variant");
            return -3;
        }
    }

    // IOU
    let iou = match get_function_param::<Amount>(1) {
        Ok(a) => a,
        Err(err) => {
            let _ = trace_num("IOU function param error:", err as i64);
            return -4;
        }
    };
    match &iou {
        Amount::IOU { amount: opaque, issuer, currency } => {
            let _ = trace_data("IOU amount:", &opaque.0, DataRepr::AsHex);
            let _ = trace_data("IOU issuer:", &issuer.0, DataRepr::AsHex);
            let _ = trace_data("IOU currency:", &currency.0, DataRepr::AsHex);

            let expected_usd: [u8; 20] = [
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x55, 0x53, 0x44, 0x00,
                0x00, 0x00, 0x00, 0x00
            ];
            if currency.0 != expected_usd {
                let _ = trace("Param 1: currency mismatch - expected USD");
                return -5;
            }
        }
        _ => {
            let _ = trace("Param 1: expected IOU variant");
            return -6;
        }
    }

    // MPT
    let mpt = match get_function_param::<Amount>(2) {
        Ok(a) => a,
        Err(err) => {
            let _ = trace_num("MPT function param error:", err as i64);
            return -7;
        }
    };
    match &mpt {
        Amount::MPT { num_units, is_positive, mpt_id } => {
            let _ = trace_num("MPT units:", *num_units as i64);
            let _ = trace_num("MPT positive:", *is_positive as i64);
            let _ = trace_data("MPT id:", mpt_id.as_bytes(), DataRepr::AsHex);

            if *num_units != 1000 {
                let _ = trace_num("Expected 1000, got:", *num_units as i64);
                return -8;
            }
            if !is_positive {
                let _ = trace("Expected positive MPT amount");
                return -9;
            }
        }
        _ => {
            let _ = trace("Param 2: expected MPT variant");
            return -10;
        }
    }

    let _ = trace("Function Amount All: PASS");
    0
}
