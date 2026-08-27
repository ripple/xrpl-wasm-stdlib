#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_common_stdlib::ctx::SmartFeatureContext;
use xrpl_common_stdlib::current_tx::traits::TransactionCommonFields;
use xrpl_common_stdlib::host::trace::{trace_hex, trace_num};
use xrpl_common_stdlib::host::{Result, Result::Err, Result::Ok};
use xrpl_common_stdlib::sfield;
use xrpl_common_stdlib::types::blob::StandardBlob;
use xrpl_common_stdlib::types::nft::{NFT_ID_SIZE, NFToken};
use xrpl_escrow_stdlib::ledger_objects::traits::CurrentEscrowFields;
use xrpl_escrow_stdlib::{EscrowFinishContext, FinishResult};
use xrpl_macros::smart_escrow;

/// Extracts the first memo from the transaction.
///
/// `Memos[0].MemoData` is a `StandardBlob`, not escrow contract data. An empty
/// or absent memo is `Ok(None)` so the caller can reject it. A missing `Memos`
/// field used to surface as `FieldNotFound` (a negative host code from this
/// helper); it is now treated the same as empty, so `escrow_finish` returns `0`
/// rather than that error code.
fn get_first_memo(tx: &impl TransactionCommonFields) -> Result<Option<StandardBlob>> {
    tx.path()
        .field(sfield::Memos)
        .index(0)
        .field(sfield::MemoData)
        .get_optional::<StandardBlob>()
        .map(|opt| opt.filter(|data| !data.is_empty()))
}

#[smart_escrow]
fn nft_owner_finish(ctx: EscrowFinishContext) -> FinishResult {
    let memo = match get_first_memo(ctx.tx()) {
        Ok(v) => {
            match v {
                Some(v) => v,
                None => return FinishResult::reject(), // <-- Do not execute the escrow.
            }
        }
        Err(e) => {
            trace_num("Error getting first memo:", e.code() as i64);
            return e.code().into(); // <-- Do not execute the escrow.
        }
    };

    // MemoData must be exactly the 32-byte NFT ID — extra bytes are rejected.
    if memo.len() != NFT_ID_SIZE {
        return FinishResult::reject();
    }
    let nft_id_bytes: [u8; NFT_ID_SIZE] = memo.as_slice().try_into().unwrap();
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
