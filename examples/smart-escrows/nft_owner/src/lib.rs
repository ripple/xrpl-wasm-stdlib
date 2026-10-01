#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_common_stdlib::ctx::SmartFeatureContext;
use xrpl_common_stdlib::current_tx::traits::TransactionCommonFields;
use xrpl_common_stdlib::host::trace::{trace_hex, trace_num};
use xrpl_common_stdlib::host::{Result::Err, Result::Ok};
use xrpl_common_stdlib::types::memo::Memo;
use xrpl_common_stdlib::types::nft::{NFT_ID_SIZE, NFToken};
use xrpl_escrow_stdlib::ledger_objects::traits::CurrentEscrowFields;
use xrpl_escrow_stdlib::{EscrowFinishContext, FinishResult};
use xrpl_macros::smart_escrow;

#[smart_escrow]
fn nft_owner_finish(ctx: EscrowFinishContext) -> FinishResult {
    // The NFT to check is carried in Memos[0].MemoData. No memo, no MemoData, or too few
    // bytes for an NFT ID all reject the finish.
    let memo = match ctx.tx().get_memo(0) {
        Ok(Some(Memo {
            memo_data: Some(data),
            ..
        })) => data,
        Ok(_) => return FinishResult::reject(), // <-- Do not execute the escrow.
        Err(e) => {
            trace_num("Error getting first memo:", e.code() as i64);
            return e.code().into(); // <-- Do not execute the escrow.
        }
    };
    if memo.len < NFT_ID_SIZE {
        trace_num("MemoData too short for an NFT ID, got:", memo.len as i64);
        return FinishResult::reject();
    }

    // Extract NFT ID from memo (first 32 bytes) and create NFToken
    let nft_id_bytes: [u8; NFT_ID_SIZE] = memo.data[0..NFT_ID_SIZE].try_into().unwrap();
    let nft_token = NFToken::new(nft_id_bytes);
    trace_hex("NFT ID from memo:", nft_token.as_bytes());

    // Demonstrate NFToken field extraction
    if let Ok(nft_flags) = nft_token.flags() {
        trace_num("NFT Flags:", nft_flags.as_u16() as i64);
        if nft_flags.is_burnable() {
            trace_num("  - BURNABLE:", 1);
        }
        if nft_flags.is_only_xrp() {
            trace_num("  - ONLY_XRP:", 1);
        }
        if nft_flags.is_trust_line() {
            trace_num("  - TRUST_LINE:", 1);
        }
        if nft_flags.is_transferable() {
            trace_num("  - TRANSFERABLE:", 1);
        }
    }
    if let Ok(transfer_fee) = nft_token.transfer_fee() {
        trace_num("NFT Transfer Fee:", transfer_fee as i64);
    }
    if let Ok(issuer) = nft_token.issuer() {
        trace_hex("NFT Issuer:", &issuer.0);
    }
    if let Ok(taxon) = nft_token.taxon() {
        trace_num("NFT Taxon:", taxon as i64);
    }
    if let Ok(token_sequence) = nft_token.token_sequence() {
        trace_num("NFT Token Sequence:", token_sequence as i64);
    }

    let destination = match ctx.escrow().get_destination() {
        Ok(destination) => destination,
        Err(e) => {
            trace_num("Error getting current ledger destination:", e.code() as i64);
            return e.code().into(); // <-- Do not execute the escrow.
        }
    };

    // Check if destination owns the NFT by attempting to retrieve its URI
    match nft_token.uri(&destination) {
        Ok(_uri) => {
            trace_hex("NFT is owned by destination", &[]);
            FinishResult::succeed() // <-- Finish the escrow successfully
        }
        Err(e) => {
            trace_num(
                "NFT is NOT owned by destination. Error code:",
                e.code() as i64,
            );
            FinishResult::reject() // <-- Do not execute the escrow
        }
    }
}
