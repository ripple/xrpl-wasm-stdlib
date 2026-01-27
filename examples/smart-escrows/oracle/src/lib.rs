#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_wasm_stdlib::core::locator::Locator;
use xrpl_wasm_stdlib::core::types::account_id::AccountID;
use xrpl_wasm_stdlib::core::types::keylets::oracle_keylet;
use xrpl_wasm_stdlib::host::error_codes::match_result_code;
use xrpl_wasm_stdlib::host::trace::{DataRepr, trace_data, trace_num};
use xrpl_wasm_stdlib::host::{Result, Result::Err, Result::Ok};
use xrpl_wasm_stdlib::r_address;
use xrpl_wasm_stdlib::{host, sfield};

const ORACLE_OWNER: AccountID = AccountID(r_address!("rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh"));
const ORACLE_DOCUMENT_ID: i64 = 1;

// TODO: Update this function to handle errors and return a Result<u64> instead.
pub fn get_u64_from_buffer(bytes: &[u8]) -> u64 {
    let mut result: u64 = 0;

    // rippled uses big-endian: most significant byte is first
    let mut i = 0;
    while i < 8 {
        result = (result << 8) | (bytes[i] as u64);
        i += 1;
    }

    result
}

pub fn get_price_from_oracle(slot: i32) -> Result<u64> {
    let mut locator = Locator::new();
    locator.pack(sfield::PriceDataSeries);
    locator.pack(0);
    locator.pack(sfield::AssetPrice);

    let mut data: [u8; 8] = [0; 8];
    let result_code = unsafe {
        host::get_ledger_obj_nested_field(
            slot,
            locator.as_ptr(),
            locator.num_packed_bytes(),
            data.as_mut_ptr(),
            data.len(),
        )
    };
    let asset_price = match match_result_code(result_code, || data) {
        Ok(asset_bytes) => get_u64_from_buffer(&asset_bytes[0..8]),
        Err(error) => {
            let _ = trace_num("Error getting asset_price", error.code() as i64);
            return Err(error); // Must return to short circuit.
        }
    };
    Ok(asset_price)
}

#[unsafe(no_mangle)]
pub extern "C" fn finish() -> i32 {
    let oracle_keylet = match oracle_keylet(&ORACLE_OWNER, ORACLE_DOCUMENT_ID) {
        Ok(keylet) => keylet,
        Err(error) => {
            let _ = trace_data(
                "Failed to get oracle_keylet for account_id=",
                &ORACLE_OWNER.0,
                DataRepr::AsHex,
            );
            let _ = trace_num(
                "Failed to get oracle_keylet for document_id=",
                ORACLE_DOCUMENT_ID,
            );
            return error.code(); // <-- Do not execute the escrow; return the error code instead.
        }
    };

    let slot: i32;
    unsafe {
        slot = host::cache_ledger_obj(oracle_keylet.as_ptr(), oracle_keylet.len(), 0);
        if slot < 0 {
            return 0;
        };
    }

    let price = match get_price_from_oracle(slot) {
        Ok(v) => v,
        Err(e) => return e.code(),
    };

    (price > 1) as i32 // <-- Finish the escrow to indicate a successful outcome
}
