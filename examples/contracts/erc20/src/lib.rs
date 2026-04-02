//! # ERC-20 Wrapper for Multi-Purpose Tokens (MPTs)
//!
//! This contract provides a standard ERC-20-like interface for XRPL Multi-Purpose
//! Tokens (XLS-33). The contract acts as the MPT issuer (pseudo-account model),
//! creating the MPT during initialization and using Clawback + Payment for transfers.
//!
//! This makes it easier to transition from the EVM world to the XRPL world with a
//! familiar API, while using better mechanisms internally.
//!
//! ## Exported Functions
//!
//! - `init()` - Creates the MPT issuance (called during `ContractCreate`; reads `max_amount` from instance params)
//! - `transfer(to, amount)` - Clawbacks from caller, pays to recipient
//! - `approve(spender, amount)` - Sets an allowance for a spender
//! - `transfer_from(from, to, amount)` - Transfers MPTs using an allowance
//!
//! ## Design
//!
//! - Readonly functions (`total_supply`, `balance_of`, `allowance`) are omitted
//!   because ledger state is publicly queryable on XRPL.
//! - The contract is the MPT issuer, so transfers use Clawback (from sender) +
//!   Payment (to receiver) via inner transactions.
//! - Allowances are the only state the contract stores, in `ContractData`.

#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_wasm_stdlib::core::current_tx;
use xrpl_wasm_stdlib::core::current_tx::contract_call::{ContractCall, get_current_contract_call};
use xrpl_wasm_stdlib::core::current_tx::traits::{ContractCallFields, TransactionCommonFields};
use xrpl_wasm_stdlib::core::data::codec::{get_data, get_nested_data, set_data, set_nested_data};
use xrpl_wasm_stdlib::core::event::codec_v3::{EventBuffer, event_add};
use xrpl_wasm_stdlib::core::keylets::account_keylet;
use xrpl_wasm_stdlib::core::ledger_objects::account_root::AccountRoot;
use xrpl_wasm_stdlib::core::ledger_objects::traits::AccountFields;
use xrpl_wasm_stdlib::core::types::account_id::AccountID;
use xrpl_wasm_stdlib::core::types::amount::Amount;
use xrpl_wasm_stdlib::core::types::mpt_id::MptId;
use xrpl_wasm_stdlib::core::types::transaction_type::TransactionType;
use xrpl_wasm_stdlib::host;
use xrpl_wasm_stdlib::host::trace::trace;
use xrpl_wasm_stdlib::host::{add_txn_field, build_txn, emit_built_txn};
use xrpl_wasm_stdlib::sfield;
use xrpl_wasm_stdlib::sflags::{tfMPTCanClawback, tfMPTCanTransfer};
use xrpl_wasm_stdlib::wasm_export;

// ============================================================================
// Constants
// ============================================================================

const SUCCESS: i32 = 0;
const ERR_CLAWBACK: i32 = -1;
const ERR_PAYMENT: i32 = -2;
const ERR_ISSUANCE: i32 = -3;
const ERR_SEQUENCE: i32 = -4;
const ERR_STORE: i32 = -5;
const ERR_INSUFFICIENT_ALLOWANCE: i32 = -6;

/// Storage key for the MPT issuance sequence number.
const MPT_SEQ_KEY: &str = "mpt_seq";

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

/// Get the contract's own AccountID from the current transaction context.
fn get_contract_account() -> AccountID {
    let cc: ContractCall = get_current_contract_call();
    cc.get_contract_account().unwrap()
}

/// Get the caller's AccountID from the current transaction context.
fn get_caller() -> AccountID {
    let cc: ContractCall = get_current_contract_call();
    cc.get_account().unwrap()
}

/// Derive the MptId from the stored sequence number and the contract account.
fn get_mpt_id(contract: &AccountID) -> MptId {
    let seq: u32 = get_data(contract, MPT_SEQ_KEY).unwrap();
    MptId::new(seq, *contract)
}

/// Build an `Amount::MPT` from a MptId and a u64 value.
fn build_mpt_amount(mpt_id: &MptId, value: u64) -> Amount {
    Amount::MPT {
        num_units: value,
        is_positive: true,
        mpt_id: *mpt_id,
    }
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

/// Emit a Clawback inner transaction to pull MPTs from a holder.
fn emit_clawback(mpt_id: &MptId, holder: &AccountID, amount: u64) -> i32 {
    unsafe {
        let txn_index = build_txn(TransactionType::Clawback as i32);
        if txn_index < 0 {
            return txn_index;
        }

        // Add Amount field (the MPT amount to claw back)
        let mpt_amount = build_mpt_amount(mpt_id, amount);
        let (amount_bytes, _) = mpt_amount.to_stamount_bytes();
        if add_txn_field(
            txn_index,
            sfield::Amount.into(),
            amount_bytes.as_ptr(),
            amount_bytes.len(),
        ) < 0
        {
            return -1;
        }

        // Add Holder field (account to claw back from)
        let mut holder_buffer = [0u8; 21];
        holder_buffer[0] = 0x14; // Account ID type prefix
        holder_buffer[1..21].copy_from_slice(&holder.0);
        if add_txn_field(
            txn_index,
            sfield::Holder.into(),
            holder_buffer.as_ptr(),
            holder_buffer.len(),
        ) < 0
        {
            return -1;
        }

        emit_built_txn(txn_index)
    }
}

/// Emit a Payment inner transaction to send MPTs to a recipient.
fn emit_payment(mpt_id: &MptId, recipient: &AccountID, amount: u64) -> i32 {
    let mpt_amount = build_mpt_amount(mpt_id, amount);
    mpt_amount.transfer(recipient)
}

// ============================================================================
// ERC-20 Exported Functions
// ============================================================================

/// Initializes the contract by creating an MPT issuance.
///
/// Called during `ContractCreate`. Emits an `MPTokenIssuanceCreate` inner
/// transaction with `tfMPTCanTransfer | tfMPTCanClawback` flags, then stores
/// the issuance sequence number so subsequent calls can derive the MptId.
///
/// Instance params: `max_amount` (u64 maximum supply of the MPT)
#[wasm_export(exit = exit, instance(max_amount: u64))]
fn init() -> i32 {
    // Get the contract's own account and its current sequence number
    let contract: AccountID = current_tx::get_field(sfield::ContractAccount).unwrap();

    let keylet = match account_keylet(&contract) {
        host::Result::Ok(k) => k,
        host::Result::Err(_) => return exit("Failed to get account keylet", ERR_SEQUENCE),
    };
    let slot = unsafe { host::cache_ledger_obj(keylet.as_ptr(), keylet.len(), 0) };
    if slot < 0 {
        return exit("Failed to cache account root", ERR_SEQUENCE);
    }
    let account_root = AccountRoot { slot_num: slot };
    let sequence = match account_root.sequence() {
        host::Result::Ok(s) => s,
        host::Result::Err(_) => return exit("Failed to read sequence", ERR_SEQUENCE),
    };

    // Build and emit MPTokenIssuanceCreate
    unsafe {
        let txn_index = build_txn(TransactionType::MPTokenIssuanceCreate as i32);
        if txn_index < 0 {
            return exit("Failed to build issuance txn", ERR_ISSUANCE);
        }

        // Add Flags: tfMPTCanTransfer | tfMPTCanClawback
        let flags: u32 = tfMPTCanTransfer | tfMPTCanClawback;
        let flags_bytes = flags.to_be_bytes();
        if add_txn_field(
            txn_index,
            sfield::Flags.into(),
            flags_bytes.as_ptr(),
            flags_bytes.len(),
        ) < 0
        {
            return exit("Failed to add Flags", ERR_ISSUANCE);
        }

        // Add MaximumAmount
        let max_bytes = max_amount.to_be_bytes();
        if add_txn_field(
            txn_index,
            sfield::MaximumAmount.into(),
            max_bytes.as_ptr(),
            max_bytes.len(),
        ) < 0
        {
            return exit("Failed to add MaximumAmount", ERR_ISSUANCE);
        }

        let result = emit_built_txn(txn_index);
        if result != 0 {
            return exit("Failed to emit issuance txn", ERR_ISSUANCE);
        }
    }

    // Store the sequence number so other functions can derive the MptId
    if set_data::<u32>(&contract, MPT_SEQ_KEY, sequence).is_err() {
        return exit("Failed to store MPT sequence", ERR_STORE);
    }

    SUCCESS
}

/// Transfers MPTs from the caller to the recipient.
///
/// Uses Clawback (from caller) + Payment (to recipient) since the contract
/// is the MPT issuer.
///
/// Function params: `to` (recipient AccountID), `amount` (u64 units to transfer)
#[wasm_export(exit = exit)]
fn transfer(to: AccountID, amount: u64) -> i32 {
    let contract = get_contract_account();
    let caller = get_caller();
    let mpt_id = get_mpt_id(&contract);

    // Clawback from caller
    let result = emit_clawback(&mpt_id, &caller, amount);
    if result != 0 {
        return exit("Clawback failed", ERR_CLAWBACK);
    }

    // Payment to recipient
    let result = emit_payment(&mpt_id, &to, amount);
    if result != 0 {
        return exit("Payment failed", ERR_PAYMENT);
    }

    emit_transfer(&caller, &to, amount);
    SUCCESS
}

/// Sets the allowance that `spender` may transfer on behalf of the caller.
///
/// Function params: `spender` (AccountID), `amount` (u64 allowance)
#[wasm_export(exit = exit)]
fn approve(spender: AccountID, amount: u64) -> i32 {
    let contract = get_contract_account();
    let caller = get_caller();

    set_allowance_value(&contract, &caller, &spender, amount);
    emit_approval(&caller, &spender, amount);
    SUCCESS
}

/// Transfers MPTs from `from` to `to` using a pre-approved allowance.
///
/// Uses Clawback (from `from`) + Payment (to `to`) since the contract
/// is the MPT issuer.
///
/// Function params: `from` (owner AccountID), `to` (recipient AccountID), `amount` (u64)
#[wasm_export(exit = exit)]
fn transfer_from(from: AccountID, to: AccountID, amount: u64) -> i32 {
    let contract = get_contract_account();
    let caller = get_caller();
    let mpt_id = get_mpt_id(&contract);

    // Check allowance
    let current_allowance = get_allowance_value(&contract, &from, &caller);
    if current_allowance < amount {
        return exit("Insufficient allowance", ERR_INSUFFICIENT_ALLOWANCE);
    }

    // Clawback from `from`
    let result = emit_clawback(&mpt_id, &from, amount);
    if result != 0 {
        return exit("Clawback failed", ERR_CLAWBACK);
    }

    // Payment to `to`
    let result = emit_payment(&mpt_id, &to, amount);
    if result != 0 {
        return exit("Payment failed", ERR_PAYMENT);
    }

    // Decrease the allowance
    set_allowance_value(&contract, &from, &caller, current_allowance - amount);
    emit_transfer(&from, &to, amount);
    SUCCESS
}
