//! # Trace Escrow Trust Line Test
//!
//! This test ensures that a RippleState (trust line) ledger object can be loaded and that every
//! field on it can be successfully accessed from within a WASM smart contract.
//!
//! Its primary purpose is to exercise the **IOU `Amount` decode path against real rippled bytes**.
//! Trust lines are the only ledger object whose `Balance`/`LowLimit`/`HighLimit` are always IOU
//! amounts (48-byte STAmount with sign/exponent/mantissa + currency + issuer). Every other e2e
//! trace test (`trace_escrow_account`, `trace_escrow_ledger_object`) only ever decodes XRP amounts,
//! and the mocked unit tests zero-fill their buffers, which routes through the XRP variant of
//! `Amount::from_bytes`. So this is the one place where a real IOU `Amount` is decoded off the
//! ledger end-to-end.
//!
//! The test harness (`runTest.js`) sets up a trust line for currency "USD" between the finishing
//! account (holder) and the escrow destination (issuer), issues an IOU balance on that line, then
//! creates an escrow finished by the holder with this contract as the finish condition.
#![cfg_attr(target_arch = "wasm32", no_std)]

use xrpl_common_stdlib::current_tx::traits::TransactionCommonFields;
use xrpl_common_stdlib::host::trace::{
    DataRepr, trace, trace_account, trace_amount, trace_data, trace_num,
};
use xrpl_common_stdlib::objects::traits::TrustLineFields;
use xrpl_common_stdlib::objects::trust_line::TrustLine;
use xrpl_common_stdlib::types::currency::Currency;
use xrpl_escrow_stdlib::current_tx::escrow_finish::{EscrowFinish, get_current_escrow_finish};
use xrpl_escrow_stdlib::ledger_objects::current_escrow::{CurrentEscrow, get_current_escrow};
use xrpl_escrow_stdlib::ledger_objects::traits::CurrentEscrowFields;

// NOTE: The IOU-specific assertions are only compiled for WASM targets. On native (the coverage
// build) the host functions are stubbed with zero-filled buffers, so amounts decode as `Amount::XRP`
// and the assertions below would not hold. This mirrors `trace_escrow_account`.
#[cfg(target_arch = "wasm32")]
use xrpl_common_stdlib::types::amount::Amount;

#[unsafe(no_mangle)]
pub extern "C" fn finish() -> i32 {
    let _ = trace("$$$$$ STARTING WASM EXECUTION $$$$$");
    let _ = trace("");

    // The account finishing the escrow is the trust line holder (see runTest.js).
    let escrow_finish: EscrowFinish = get_current_escrow_finish();
    let holder = escrow_finish.get_account().unwrap();
    let _ = trace_account("Holder (finishing account):", &holder);

    // The escrow destination is the IOU issuer (see runTest.js).
    let current_escrow: CurrentEscrow = get_current_escrow();
    let issuer = current_escrow.get_destination().unwrap();
    let _ = trace_account("Issuer (escrow destination):", &issuer);

    // The trust line currency configured by the harness. Standard 3-char currency codes live in
    // bytes 12..15 of the 20-byte currency field.
    let currency = Currency::from(*b"USD");

    // ########################################
    // Load the RippleState object and trace every field.
    // ########################################
    let _ = trace("### Load Trust Line (RippleState)");
    let line = TrustLine::load(&holder, &issuer, &currency).unwrap();

    let _ = trace("{ ");

    // Trace Field: Balance (IOU amount from the low account's perspective)
    let balance = line.balance().unwrap();
    let _ = trace_amount("  Balance:", &balance);

    // Trace Field: LowLimit (IOU amount; issuer = low account)
    let low_limit = line.low_limit().unwrap();
    let _ = trace_amount("  LowLimit:", &low_limit);

    // Trace Field: HighLimit (IOU amount; issuer = high account)
    let high_limit = line.high_limit().unwrap();
    let _ = trace_amount("  HighLimit:", &high_limit);

    // Trace Field: LowNode / HighNode (u64 directory hints)
    let low_node = line.low_node().unwrap();
    let _ = trace_num("  LowNode:", low_node as i64);
    let high_node = line.high_node().unwrap();
    let _ = trace_num("  HighNode:", high_node as i64);

    // Trace Field: quality fields (optional u32)
    let low_quality_in = line.low_quality_in().unwrap().unwrap_or(0);
    let _ = trace_num("  LowQualityIn:", low_quality_in as i64);
    let low_quality_out = line.low_quality_out().unwrap().unwrap_or(0);
    let _ = trace_num("  LowQualityOut:", low_quality_out as i64);
    let high_quality_in = line.high_quality_in().unwrap().unwrap_or(0);
    let _ = trace_num("  HighQualityIn:", high_quality_in as i64);
    let high_quality_out = line.high_quality_out().unwrap().unwrap_or(0);
    let _ = trace_num("  HighQualityOut:", high_quality_out as i64);

    // Trace Field: PreviousTxnID / PreviousTxnLgrSeq (metadata common to every ledger object)
    let previous_txn_id = line.previous_txn_id().unwrap();
    let _ = trace_data("  PreviousTxnID:", &previous_txn_id.0, DataRepr::AsHex);
    let previous_txn_lgr_seq = line.previous_txn_lgr_seq().unwrap();
    let _ = trace_num("  PreviousTxnLgrSeq:", previous_txn_lgr_seq as i64);

    let _ = trace("} ");

    // ########################################
    // IOU-specific assertions (WASM / real rippled only).
    // ########################################
    #[cfg(target_arch = "wasm32")]
    {
        // The balance must decode as an IOU amount in the trust line currency. This is the core
        // signal: it proves the IOU branch of `Amount::from_bytes` correctly parses real STAmount
        // bytes off the ledger (as opposed to the XRP branch that zero-filled mocks always hit).
        match &balance {
            Amount::IOU { currency: c, .. } => {
                test_utils::assert!(*c == currency, "Balance currency should be USD");
            }
            _ => panic!("Balance should be an IOU amount, not XRP/MPT"),
        }

        // Both limits must decode as IOU amounts in the same currency, and their issuers must be the
        // two trust line parties (the ledger stores them sorted low/high, so accept either order).
        let low_issuer = match &low_limit {
            Amount::IOU {
                issuer,
                currency: c,
                ..
            } => {
                test_utils::assert!(*c == currency, "LowLimit currency should be USD");
                *issuer
            }
            _ => panic!("LowLimit should be an IOU amount, not XRP/MPT"),
        };
        let high_issuer = match &high_limit {
            Amount::IOU {
                issuer,
                currency: c,
                ..
            } => {
                test_utils::assert!(*c == currency, "HighLimit currency should be USD");
                *issuer
            }
            _ => panic!("HighLimit should be an IOU amount, not XRP/MPT"),
        };

        let issuers_match = (low_issuer == holder && high_issuer == issuer)
            || (low_issuer == issuer && high_issuer == holder);
        test_utils::assert!(
            issuers_match,
            "LowLimit/HighLimit issuers should be the holder and the issuer (in either order)"
        );
    }

    let _ = trace("$$$$$ WASM EXECUTION COMPLETE $$$$$");
    1 // <-- Finish the escrow to indicate a successful outcome
}

#[cfg(test)]
mod coverage_tests {
    use super::*;

    /// Coverage test: exercises the trust line accessor + IOU decode code paths via finish().
    ///
    /// On non-wasm targets, finish() uses the default stub host bindings, which return zero-filled
    /// buffers. This verifies the code *runs* (loads the object and calls every accessor), not that
    /// the values are *correct* — correctness is verified by the real integration test against
    /// rippled. The IOU-specific assertions in finish() are compiled out on native for this reason.
    #[test]
    fn test_finish_exercises_all_host_functions() {
        let result = finish();
        core::assert_eq!(result, 1, "finish() should return 1 on success");
    }
}
