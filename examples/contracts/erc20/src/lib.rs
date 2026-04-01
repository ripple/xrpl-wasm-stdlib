//! # ERC-20 Wrapper for Multi-Purpose Tokens (MPTs)
//!
//! This contract provides a standard ERC-20-like interface for XRPL Multi-Purpose
//! Tokens (XLS-33). It wraps native MPT operations behind familiar ERC-20 function
//! names while using the contract's persistent storage for allowance management.
//!
//! ## Exported Functions
//!
//! - `total_supply()` - Returns the outstanding amount of the wrapped MPT
//! - `balance_of(account)` - Returns the MPT balance of an account
//! - `transfer(to, amount)` - Transfers MPTs from the contract to a recipient
//! - `approve(spender, amount)` - Sets an allowance for a spender
//! - `allowance(owner, spender)` - Returns the current allowance
//! - `transfer_from(from, to, amount)` - Transfers MPTs using an allowance
//!
//! ## Instance Parameters
//!
//! The contract is deployed with a single instance parameter:
//! - `mpt_amount` (Amount::MPT) - An MPT amount that identifies the wrapped token

#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_wasm_stdlib::core::current_tx::contract_call::{ContractCall, get_current_contract_call};
use xrpl_wasm_stdlib::core::current_tx::traits::{ContractCallFields, TransactionCommonFields};
use xrpl_wasm_stdlib::core::data::codec::{get_nested_data, set_nested_data};
use xrpl_wasm_stdlib::core::event::codec_v3::{EventBuffer, event_add};
use xrpl_wasm_stdlib::core::keylets::{mpt_issuance_keylet, mptoken_keylet};
use xrpl_wasm_stdlib::core::ledger_objects::ledger_object;
use xrpl_wasm_stdlib::core::types::account_id::AccountID;
use xrpl_wasm_stdlib::core::types::amount::Amount;
use xrpl_wasm_stdlib::core::types::mpt_id::MptId;
use xrpl_wasm_stdlib::host;
use xrpl_wasm_stdlib::host::trace::trace;
use xrpl_wasm_stdlib::sfield;
use xrpl_wasm_stdlib::wasm_export;

// ============================================================================
// Constants
// ============================================================================

const SUCCESS: i32 = 0;
const ERR_TRANSFER: i32 = -4;
const ERR_KEYLET: i32 = -5;
const ERR_SLOT: i32 = -6;
const ERR_FIELD: i32 = -7;
const ERR_INSUFFICIENT_ALLOWANCE: i32 = -8;

/// Storage key prefix for allowances.
const ALLOWANCES_KEY: &str = "allowances";

// ============================================================================
// Helper Functions
// ============================================================================

/// Exit handler for `wasm_export` – logs a message and returns the error code.
fn exit(message: &str, error_code: i32) -> i32 {
    let _ = trace(message);
    error_code
}

/// Build a 40-byte storage key from owner (20 bytes) + spender (20 bytes).
fn allowance_key(owner: &AccountID, spender: &AccountID) -> [u8; 40] {
    let mut key = [0u8; 40];
    let mut i = 0;
    while i < 20 {
        key[i] = owner.0[i];
        i += 1;
    }
    while i < 40 {
        key[i] = spender.0[i - 20];
        i += 1;
    }
    key
}

/// Read the stored allowance for (owner, spender) from contract data.
fn get_allowance_value(contract: &AccountID, owner: &AccountID, spender: &AccountID) -> u64 {
    let key = allowance_key(owner, spender);
    get_nested_data::<u64>(contract, ALLOWANCES_KEY, &key[..]).unwrap_or(0)
}

/// Write the allowance for (owner, spender) to contract data.
fn set_allowance_value(contract: &AccountID, owner: &AccountID, spender: &AccountID, value: u64) {
    let key = allowance_key(owner, spender);
    let _ = set_nested_data::<u64>(contract, ALLOWANCES_KEY, &key[..], value);
}

/// Emit a Transfer event with from, to, and value fields.
fn emit_transfer(from: &AccountID, to: &AccountID, value: u64) {
    let mut buf = EventBuffer::new();
    let _ = event_add(&mut buf, "from", from);
    let _ = event_add(&mut buf, "to", to);
    let _ = event_add(&mut buf, "value", &value);
    let _ = buf.emit("Transfer");
}

/// Emit an Approval event with owner, spender, and value fields.
fn emit_approval(owner: &AccountID, spender: &AccountID, value: u64) {
    let mut buf = EventBuffer::new();
    let _ = event_add(&mut buf, "owner", owner);
    let _ = event_add(&mut buf, "spender", spender);
    let _ = event_add(&mut buf, "value", &value);
    let _ = buf.emit("Approval");
}

/// Read the MPT balance for `holder` from the on-chain MPToken ledger object.
fn read_mpt_balance(mpt_id: &MptId, holder: &AccountID) -> i32 {
    let keylet = match mptoken_keylet(mpt_id, holder) {
        host::Result::Ok(k) => k,
        host::Result::Err(_) => return ERR_KEYLET,
    };
    let slot = unsafe { host::cache_ledger_obj(keylet.as_ptr(), keylet.len(), 0) };
    if slot < 0 {
        return 0; // No MPToken object means zero balance
    }
    match ledger_object::get_field(slot, sfield::MPTAmount) {
        host::Result::Ok(amount) => amount as i32,
        host::Result::Err(_) => 0,
    }
}

/// Read the total outstanding supply from the MPTokenIssuance ledger object.
fn read_total_supply(mpt_id: &MptId) -> i32 {
    let keylet = match mpt_issuance_keylet(&mpt_id.get_issuer(), mpt_id.get_sequence_num()) {
        host::Result::Ok(k) => k,
        host::Result::Err(_) => return ERR_KEYLET,
    };
    let slot = unsafe { host::cache_ledger_obj(keylet.as_ptr(), keylet.len(), 0) };
    if slot < 0 {
        return ERR_SLOT;
    }
    match ledger_object::get_field(slot, sfield::OutstandingAmount) {
        host::Result::Ok(amount) => amount as i32,
        host::Result::Err(_) => ERR_FIELD,
    }
}

/// Build an `Amount::MPT` from the instance-param MPT ID and a u64 value.
fn build_mpt_amount(mpt_id: &MptId, value: u64) -> Amount {
    Amount::MPT {
        num_units: value,
        is_positive: true,
        mpt_id: *mpt_id,
    }
}

/// Extract the `MptId` from an `Amount::MPT` instance parameter.
fn extract_mpt_id(amount: &Amount) -> MptId {
    match amount {
        Amount::MPT { mpt_id, .. } => *mpt_id,
        _ => panic!("Instance parameter must be an MPT amount"),
    }
}

// ============================================================================
// ERC-20 Exported Functions
// ============================================================================

/// Returns the total outstanding supply of the wrapped MPT.
///
/// Instance params: `mpt_amount` (Amount::MPT identifying the token)
/// Function params: none
#[wasm_export(
    exit = exit,
    instance(mpt_amount: Amount)
)]
fn total_supply() -> i32 {
    let mpt_id = extract_mpt_id(&mpt_amount);
    read_total_supply(&mpt_id)
}

/// Returns the MPT balance of the given account.
///
/// Instance params: `mpt_amount` (Amount::MPT identifying the token)
/// Function params: `account` (AccountID to query)
#[wasm_export(
    exit = exit,
    instance(mpt_amount: Amount)
)]
fn balance_of(account: AccountID) -> i32 {
    let mpt_id = extract_mpt_id(&mpt_amount);
    read_mpt_balance(&mpt_id, &account)
}

/// Transfers MPTs from the contract to the recipient by emitting a Payment.
///
/// Instance params: `mpt_amount` (Amount::MPT identifying the token)
/// Function params: `to` (recipient AccountID), `amount` (u64 units to transfer)
#[wasm_export(
    exit = exit,
    instance(mpt_amount: Amount)
)]
fn transfer(to: AccountID, amount: u64) -> i32 {
    let mpt_id = extract_mpt_id(&mpt_amount);
    let caller = {
        let cc: ContractCall = get_current_contract_call();
        cc.get_account().unwrap()
    };

    let payment = build_mpt_amount(&mpt_id, amount);
    let result = payment.transfer(&to);
    if result < 0 {
        return exit("Payment emission failed", ERR_TRANSFER);
    }

    emit_transfer(&caller, &to, amount);
    SUCCESS
}

/// Sets the allowance that `spender` may transfer on behalf of the caller.
///
/// Instance params: `mpt_amount` (Amount::MPT identifying the token)
/// Function params: `spender` (AccountID), `amount` (u64 allowance)
#[wasm_export(
    exit = exit,
    instance(mpt_amount: Amount)
)]
fn approve(spender: AccountID, amount: u64) -> i32 {
    let _ = &mpt_amount; // MPT identity not needed for allowance logic
    let contract = {
        let cc: ContractCall = get_current_contract_call();
        cc.get_contract_account().unwrap()
    };
    let caller = {
        let cc: ContractCall = get_current_contract_call();
        cc.get_account().unwrap()
    };

    set_allowance_value(&contract, &caller, &spender, amount);
    emit_approval(&caller, &spender, amount);
    SUCCESS
}

/// Returns the remaining allowance that `spender` may transfer on behalf of `owner`.
///
/// Instance params: `mpt_amount` (Amount::MPT identifying the token)
/// Function params: `owner` (AccountID), `spender` (AccountID)
#[wasm_export(
    exit = exit,
    instance(mpt_amount: Amount)
)]
fn allowance(owner: AccountID, spender: AccountID) -> i32 {
    let _ = &mpt_amount;
    let contract = {
        let cc: ContractCall = get_current_contract_call();
        cc.get_contract_account().unwrap()
    };

    get_allowance_value(&contract, &owner, &spender) as i32
}

/// Transfers MPTs from `from` to `to` using a pre-approved allowance.
///
/// Instance params: `mpt_amount` (Amount::MPT identifying the token)
/// Function params: `from` (owner AccountID), `to` (recipient AccountID), `amount` (u64)
#[wasm_export(
    exit = exit,
    instance(mpt_amount: Amount)
)]
fn transfer_from(from: AccountID, to: AccountID, amount: u64) -> i32 {
    let mpt_id = extract_mpt_id(&mpt_amount);
    let contract = {
        let cc: ContractCall = get_current_contract_call();
        cc.get_contract_account().unwrap()
    };
    let caller = {
        let cc: ContractCall = get_current_contract_call();
        cc.get_account().unwrap()
    };

    // Check allowance
    let current_allowance = get_allowance_value(&contract, &from, &caller);
    if current_allowance < amount {
        return exit("Insufficient allowance", ERR_INSUFFICIENT_ALLOWANCE);
    }

    // Emit the MPT Payment
    let payment = build_mpt_amount(&mpt_id, amount);
    let result = payment.transfer(&to);
    if result < 0 {
        return exit("Payment emission failed", ERR_TRANSFER);
    }

    // Decrease the allowance
    set_allowance_value(&contract, &from, &caller, current_allowance - amount);
    emit_transfer(&from, &to, amount);
    SUCCESS
}
