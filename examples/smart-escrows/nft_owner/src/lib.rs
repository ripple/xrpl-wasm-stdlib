#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_escrow_stdlib::{EscrowFinishContext, FinishResult};
use xrpl_wasm_stdlib::core::ledger_objects::traits::CurrentEscrowFields;
use xrpl_wasm_stdlib::core::locator::Locator;
use xrpl_wasm_stdlib::core::types::nft::{NFT_ID_SIZE, NFToken};
use xrpl_wasm_stdlib::host::get_tx_nested_field;
use xrpl_wasm_stdlib::host::trace::{DataRepr, trace_data, trace_num};
use xrpl_wasm_stdlib::host::{Error, Result, Result::Err, Result::Ok};
use xrpl_wasm_stdlib::sfield;
use xrpl_wasm_stdlib::smart_escrow;
use xrpl_wasm_stdlib::types::{ContractData, XRPL_CONTRACT_DATA_SIZE};

#[unsafe(no_mangle)]
pub fn get_first_memo() -> Result<Option<ContractData>> {
    let mut data: ContractData = [0; XRPL_CONTRACT_DATA_SIZE];
    let mut locator = Locator::new();
    locator.pack(sfield::Memos);
    locator.pack(0);
    locator.pack(sfield::MemoData);
    let result_code = unsafe {
        get_tx_nested_field(
            locator.as_ptr(),
            locator.num_packed_bytes(),
            data.as_mut_ptr(),
            data.len(),
        )
    };

    match result_code {
        result_code if result_code > 0 => {
            Ok(Some(data)) // <-- Move the buffer into an AccountID
        }
        // Zero length is a present-but-empty memo (protocol-valid input); treat it the
        // same as an absent field and let the caller decide.
        0 => Ok(None),
        result_code => Err(Error::from_code(result_code)),
    }
}

#[smart_escrow]
fn nft_owner_finish(ctx: EscrowFinishContext) -> FinishResult {
    let memo: ContractData = match get_first_memo() {
        Ok(v) => {
            match v {
                Some(v) => v,
                None => return FinishResult::reject(), // <-- Do not execute the escrow.
            }
        }
        Err(e) => {
            let _ = trace_num("Error getting first memo:", e.code() as i64);
            return e.code().into(); // <-- Do not execute the escrow.
        }
    };

    // Extract NFT ID from memo (first 32 bytes) and create NFToken
    let nft_id_bytes: [u8; NFT_ID_SIZE] = memo[0..32].try_into().unwrap();
    let nft_token = NFToken::new(nft_id_bytes);
    let _ = trace_data("NFT ID from memo:", nft_token.as_bytes(), DataRepr::AsHex);

    // Demonstrate NFToken field extraction
    if let Ok(nft_flags) = nft_token.flags() {
        let _ = trace_num("NFT Flags:", nft_flags.as_u16() as i64);
        if nft_flags.is_burnable() {
            let _ = trace_num("  - BURNABLE:", 1);
        }
        if nft_flags.is_only_xrp() {
            let _ = trace_num("  - ONLY_XRP:", 1);
        }
        if nft_flags.is_trust_line() {
            let _ = trace_num("  - TRUST_LINE:", 1);
        }
        if nft_flags.is_transferable() {
            let _ = trace_num("  - TRANSFERABLE:", 1);
        }
    }
    if let Ok(transfer_fee) = nft_token.transfer_fee() {
        let _ = trace_num("NFT Transfer Fee:", transfer_fee as i64);
    }
    if let Ok(issuer) = nft_token.issuer() {
        let _ = trace_data("NFT Issuer:", &issuer.0, DataRepr::AsHex);
    }
    if let Ok(taxon) = nft_token.taxon() {
        let _ = trace_num("NFT Taxon:", taxon as i64);
    }
    if let Ok(token_sequence) = nft_token.token_sequence() {
        let _ = trace_num("NFT Token Sequence:", token_sequence as i64);
    }

    let destination = match ctx.escrow().get_destination() {
        Ok(destination) => destination,
        Err(e) => {
            let _ = trace_num("Error getting current ledger destination:", e.code() as i64);
            return e.code().into(); // <-- Do not execute the escrow.
        }
    };

    // Check if destination owns the NFT by attempting to retrieve its URI
    match nft_token.uri(&destination) {
        Ok(_uri) => {
            let _ = trace_data("NFT is owned by destination", &[], DataRepr::AsHex);
            FinishResult::succeed() // <-- Finish the escrow successfully
        }
        Err(e) => {
            let _ = trace_num(
                "NFT is NOT owned by destination. Error code:",
                e.code() as i64,
            );
            FinishResult::reject() // <-- Do not execute the escrow
        }
    }
}
