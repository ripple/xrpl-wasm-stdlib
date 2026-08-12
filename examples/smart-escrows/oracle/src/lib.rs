#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_common_stdlib::host::trace::{trace, trace_num};
use xrpl_common_stdlib::host::{Error, Result, Result::Err, Result::Ok};
use xrpl_common_stdlib::ledger_entry_ids::oracle_id;
use xrpl_common_stdlib::objects::traits::LedgerObjectCommonFields;
use xrpl_common_stdlib::objects::{LedgerObject, cache_ledger_entry};
use xrpl_common_stdlib::r_address;
use xrpl_common_stdlib::sfield;
use xrpl_common_stdlib::types::account_id::AccountID;
use xrpl_escrow_stdlib::{EscrowFinishContext, FinishResult};
use xrpl_macros::smart_escrow;

const ORACLE_OWNER: AccountID = r_address!("rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh");
const ORACLE_DOCUMENT_ID: u32 = 1;

pub fn get_price_from_oracle(slot: i32) -> Result<u64> {
    // The Oracle entry has no typed wrapper, so reach its inner fields through the untyped slot
    // handle. Check the series is non-empty before indexing into it, rather than relying on the
    // read of [0] to fail.
    let oracle = LedgerObject::new(slot);

    let series_len = match oracle.path().field(sfield::PriceDataSeries).array_len() {
        Ok(len) => len,
        Err(error) => {
            trace_num("Error getting PriceDataSeries length", error.code() as i64);
            return Err(error);
        }
    };
    trace_num(
        "get_price_from_oracle: price_data_series_len=",
        series_len as i64,
    );
    if series_len == 0 {
        trace("get_price_from_oracle: oracle has no price data");
        return Err(Error::FieldNotFound);
    }

    // PriceDataSeries[0].AssetPrice
    let asset_price = oracle
        .path()
        .field(sfield::PriceDataSeries)
        .index(0)
        .field(sfield::AssetPrice)
        .get::<u64>();

    match asset_price {
        Ok(price) => {
            trace_num("get_price_from_oracle: asset_price=", price as i64);
            Ok(price)
        }
        Err(error) => {
            trace_num("Error getting asset_price", error.code() as i64);
            Err(error) // Must return to short circuit.
        }
    }
}

#[smart_escrow]
fn oracle_finish(_ctx: EscrowFinishContext) -> FinishResult {
    let oracle_id = match oracle_id(&ORACLE_OWNER, ORACLE_DOCUMENT_ID) {
        Ok(id) => id,
        Err(error) => {
            trace_num("finish: oracle_id error_code=", error.code() as i64);
            return error.code().into();
        }
    };

    let slot = match cache_ledger_entry(&oracle_id) {
        Ok(slot) => {
            trace_num("finish: cached oracle at slot=", slot as i64);
            slot
        }
        Err(error) => {
            trace_num(
                "finish: caching oracle failed, error_code=",
                error.code() as i64,
            );
            return FinishResult::reject();
        }
    };

    let price = match get_price_from_oracle(slot) {
        Ok(v) => v,
        Err(e) => return e.code().into(),
    };

    // <-- Finish the escrow to indicate a successful outcome
    ((price > 1) as i32).into()
}
