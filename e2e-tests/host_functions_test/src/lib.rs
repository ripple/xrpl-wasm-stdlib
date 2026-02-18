#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

//
// Host Functions Test
// Tests 27 host functions (across 7 categories)
//
// With craft you can run this test with:
//   craft test --project host_functions_test --test-case host_functions_test
//
// Amount Format Update:
// - XRP amounts now return as 8-byte serialized rippled objects
// - IOU and MPT amounts return in variable-length serialized format
// - Format details: https://xrpl.org/docs/references/protocol/binary-format#amount-fields
//
// Error Code Ranges:
// -100 to -199: Ledger Header Functions (3 functions)
// -200 to -299: Transaction Data Functions (5 functions)
// -300 to -399: Current Ledger Object Functions (4 functions)
// -400 to -499: Any Ledger Object Functions (5 functions)
// -500 to -599: Keylet Generation Functions (4 functions)
// -600 to -699: Utility Functions (4 functions)
// -700 to -799: Data Update Functions (1 function)
//

use xrpl_wasm_stdlib::core::current_tx::escrow_finish::EscrowFinish;
use xrpl_wasm_stdlib::core::current_tx::traits::TransactionCommonFields;
use xrpl_wasm_stdlib::core::types::account_id::AccountID;
use xrpl_wasm_stdlib::core::types::amount::Amount;
use xrpl_wasm_stdlib::core::types::currency::Currency;
use xrpl_wasm_stdlib::core::types::mpt_id::MptId;
use xrpl_wasm_stdlib::core::types::opaque_float::OpaqueFloat;
use xrpl_wasm_stdlib::host;
use xrpl_wasm_stdlib::host::trace::{
    DataRepr, trace, trace_account_buf, trace_amount, trace_data, trace_num,
};
use xrpl_wasm_stdlib::sfield;

#[unsafe(no_mangle)]
pub extern "C" fn finish() -> i32 {
    let _ = trace("=== HOST FUNCTIONS TEST ===");
    let _ = trace("Testing 27 host functions");

    // Category 1: Ledger Header Data Functions (3 functions)
    // Error range: -100 to -199
    match test_ledger_header_functions() {
        0 => (),
        err => return err,
    }

    // Category 2: Transaction Data Functions (5 functions)
    // Error range: -200 to -299
    match test_transaction_data_functions() {
        0 => (),
        err => return err,
    }

    // Category 3: Current Ledger Object Functions (4 functions)
    // Error range: -300 to -399
    match test_current_ledger_object_functions() {
        0 => (),
        err => return err,
    }

    // Category 4: Any Ledger Object Functions (5 functions)
    // Error range: -400 to -499
    match test_any_ledger_object_functions() {
        0 => (),
        err => return err,
    }

    // Category 5: Keylet Generation Functions (4 functions)
    // Error range: -500 to -599
    match test_keylet_generation_functions() {
        0 => (),
        err => return err,
    }

    // Category 6: Utility Functions (5 functions)
    // Error range: -600 to -699
    match test_utility_functions() {
        0 => (),
        err => return err,
    }

    // Category 7: Data Update Functions (1 function)
    // Error range: -700 to -799
    match test_data_update_functions() {
        0 => (),
        err => return err,
    }

    let _ = trace("SUCCESS: All host function tests passed!");
    1 // Success return code for WASM finish function
}

/// Test Category 1: Ledger Header Data Functions (3 functions)
/// - get_ledger_sqn() - Get ledger sequence number
/// - get_parent_ledger_time() - Get parent ledger timestamp
/// - get_parent_ledger_hash() - Get parent ledger hash
fn test_ledger_header_functions() -> i32 {
    let _ = trace("--- Category 1: Ledger Header Functions ---");

    // Test 1.1: get_ledger_sqn() - should return current ledger sequence number
    let mut sqn_buffer = [0u8; 4];
    let sqn_result = unsafe { host::get_ledger_sqn(sqn_buffer.as_mut_ptr(), sqn_buffer.len()) };

    if sqn_result <= 0 {
        let _ = trace_num("ERROR: get_ledger_sqn failed:", sqn_result as i64);
        return -101; // Ledger sequence number test failed
    }
    let ledger_sqn = u32::from_be_bytes(sqn_buffer);
    let _ = trace_num("Ledger sequence number:", ledger_sqn as i64);

    // Test 1.2: get_parent_ledger_time() - should return parent ledger timestamp
    let mut time_buffer = [0u8; 4];
    let time_result =
        unsafe { host::get_parent_ledger_time(time_buffer.as_mut_ptr(), time_buffer.len()) };

    if time_result <= 0 {
        let _ = trace_num("ERROR: get_parent_ledger_time failed:", time_result as i64);
        return -102; // Parent ledger time test failed
    }
    let parent_ledger_time = u32::from_be_bytes(time_buffer);
    let _ = trace_num("Parent ledger time:", parent_ledger_time as i64);

    // Test 1.3: get_parent_ledger_hash() - should return parent ledger hash (32 bytes)
    let mut hash_buffer = [0u8; 32];
    let hash_result =
        unsafe { host::get_parent_ledger_hash(hash_buffer.as_mut_ptr(), hash_buffer.len()) };

    if hash_result != 32 {
        let _ = trace_num(
            "ERROR: get_parent_ledger_hash wrong length:",
            hash_result as i64,
        );
        return -103; // Parent ledger hash test failed - should be exactly 32 bytes
    }
    let _ = trace_data("Parent ledger hash:", &hash_buffer, DataRepr::AsHex);

    let _ = trace("SUCCESS: Ledger header functions");
    0
}

/// Test Category 2: Transaction Data Functions (5 functions)
/// Tests all functions for accessing current transaction data
fn test_transaction_data_functions() -> i32 {
    let _ = trace("--- Category 2: Transaction Data Functions ---");

    // Test 2.1: get_tx_field() - Basic transaction field access
    // Test with Account field (required, 20 bytes)
    let mut account_buffer = [0u8; 20];
    let account_len = unsafe {
        host::get_tx_field(
            sfield::Account,
            account_buffer.as_mut_ptr(),
            account_buffer.len(),
        )
    };

    if account_len != 20 {
        let _ = trace_num(
            "ERROR: get_tx_field(Account) wrong length:",
            account_len as i64,
        );
        return -201; // Basic transaction field test failed
    }
    let _ = trace_account_buf("Transaction Account:", &account_buffer);

    // Test with Fee field (XRP amount - 8 bytes in new serialized format)
    // New format: XRP amounts are always 8 bytes (positive: value | cPositive flag, negative: just value)
    let mut fee_buffer = [0u8; 8];
    let fee_len =
        unsafe { host::get_tx_field(sfield::Fee, fee_buffer.as_mut_ptr(), fee_buffer.len()) };

    if fee_len != 8 {
        let _ = trace_num(
            "ERROR: get_tx_field(Fee) wrong length (expected 8 bytes for XRP):",
            fee_len as i64,
        );
        return -202; // Fee field test failed - XRP amounts should be exactly 8 bytes
    }
    let _ = trace_num("Transaction Fee length:", fee_len as i64);
    let _ = trace_data(
        "Transaction Fee (serialized XRP amount):",
        &fee_buffer,
        DataRepr::AsHex,
    );

    // Test with Sequence field (required, 4 bytes uint32)
    let mut seq_buffer = [0u8; 4];
    let seq_len =
        unsafe { host::get_tx_field(sfield::Sequence, seq_buffer.as_mut_ptr(), seq_buffer.len()) };

    if seq_len != 4 {
        let _ = trace_num(
            "ERROR: get_tx_field(Sequence) wrong length:",
            seq_len as i64,
        );
        return -203; // Sequence field test failed
    }
    let _ = trace_data("Transaction Sequence:", &seq_buffer, DataRepr::AsHex);

    // NOTE: get_tx_field2() through get_tx_field6() have been deprecated.
    // Use get_tx_field() with appropriate parameters for all transaction field access.

    // Test 2.2: get_tx_nested_field() - Nested field access with locator
    let locator = [0x01, 0x00]; // Simple locator for first element
    let mut nested_buffer = [0u8; 32];
    let nested_result = unsafe {
        host::get_tx_nested_field(
            locator.as_ptr(),
            locator.len(),
            nested_buffer.as_mut_ptr(),
            nested_buffer.len(),
        )
    };

    if nested_result < 0 {
        let _ = trace_num(
            "INFO: get_tx_nested_field not applicable:",
            nested_result as i64,
        );
        // Expected - locator may not match transaction structure
    } else {
        let _ = trace_num("Nested field length:", nested_result as i64);
        let _ = trace_data(
            "Nested field:",
            &nested_buffer[..nested_result as usize],
            DataRepr::AsHex,
        );
    }

    // Test 2.3: get_tx_array_len() - Get array length
    let signers_len = unsafe { host::get_tx_array_len(sfield::Signers) };
    let _ = trace_num("Signers array length:", signers_len as i64);

    let memos_len = unsafe { host::get_tx_array_len(sfield::Memos) };
    let _ = trace_num("Memos array length:", memos_len as i64);

    // Test 2.4: get_tx_nested_array_len() - Get nested array length with locator
    let nested_array_len =
        unsafe { host::get_tx_nested_array_len(locator.as_ptr(), locator.len()) };

    if nested_array_len < 0 {
        let _ = trace_num(
            "INFO: get_tx_nested_array_len not applicable:",
            nested_array_len as i64,
        );
    } else {
        let _ = trace_num("Nested array length:", nested_array_len as i64);
    }

    let _ = trace("SUCCESS: Transaction data functions");
    0
}

/// Test Category 3: Current Ledger Object Functions (4 functions)
/// Tests functions that access the current ledger object being processed
fn test_current_ledger_object_functions() -> i32 {
    let _ = trace("--- Category 3: Current Ledger Object Functions ---");

    // Test 3.1: get_current_ledger_obj_field() - Access field from current ledger object
    // Test with Balance field (XRP amount - 8 bytes in new serialized format)
    let mut balance_buffer = [0u8; 8];
    let balance_result = unsafe {
        host::get_current_ledger_obj_field(
            sfield::Balance,
            balance_buffer.as_mut_ptr(),
            balance_buffer.len(),
        )
    };

    if balance_result <= 0 {
        let _ = trace_num(
            "INFO: get_current_ledger_obj_field(Balance) failed (may be expected):",
            balance_result as i64,
        );
        // This might fail if current ledger object doesn't have balance field
    } else if balance_result == 8 {
        let _ = trace_num(
            "Current object balance length (XRP amount):",
            balance_result as i64,
        );
        let _ = trace_data(
            "Current object balance (serialized XRP amount):",
            &balance_buffer,
            DataRepr::AsHex,
        );
    } else {
        let _ = trace_num(
            "Current object balance length (non-XRP amount):",
            balance_result as i64,
        );
        let _ = trace_data(
            "Current object balance:",
            &balance_buffer[..balance_result as usize],
            DataRepr::AsHex,
        );
    }

    // Test with Account field
    let mut current_account_buffer = [0u8; 20];
    let current_account_result = unsafe {
        host::get_current_ledger_obj_field(
            sfield::Account,
            current_account_buffer.as_mut_ptr(),
            current_account_buffer.len(),
        )
    };

    if current_account_result <= 0 {
        let _ = trace_num(
            "INFO: get_current_ledger_obj_field(Account) failed:",
            current_account_result as i64,
        );
    } else {
        let _ = trace_account_buf("Current ledger object account:", &current_account_buffer);
    }

    // Test 3.2: get_current_ledger_obj_nested_field() - Nested field access
    let locator = [0x01, 0x00]; // Simple locator
    let mut current_nested_buffer = [0u8; 32];
    let current_nested_result = unsafe {
        host::get_current_ledger_obj_nested_field(
            locator.as_ptr(),
            locator.len(),
            current_nested_buffer.as_mut_ptr(),
            current_nested_buffer.len(),
        )
    };

    if current_nested_result < 0 {
        let _ = trace_num(
            "INFO: get_current_ledger_obj_nested_field not applicable:",
            current_nested_result as i64,
        );
    } else {
        let _ = trace_num("Current nested field length:", current_nested_result as i64);
        let _ = trace_data(
            "Current nested field:",
            &current_nested_buffer[..current_nested_result as usize],
            DataRepr::AsHex,
        );
    }

    // Test 3.3: get_current_ledger_obj_array_len() - Array length in current object
    let current_array_len = unsafe { host::get_current_ledger_obj_array_len(sfield::Signers) };
    let _ = trace_num(
        "Current object Signers array length:",
        current_array_len as i64,
    );

    // Test 3.4: get_current_ledger_obj_nested_array_len() - Nested array length
    let current_nested_array_len =
        unsafe { host::get_current_ledger_obj_nested_array_len(locator.as_ptr(), locator.len()) };

    if current_nested_array_len < 0 {
        let _ = trace_num(
            "INFO: get_current_ledger_obj_nested_array_len not applicable:",
            current_nested_array_len as i64,
        );
    } else {
        let _ = trace_num(
            "Current nested array length:",
            current_nested_array_len as i64,
        );
    }

    let _ = trace("SUCCESS: Current ledger object functions");
    0
}

/// Test Category 4: Any Ledger Object Functions (5 functions)
/// Tests functions that work with cached ledger objects
fn test_any_ledger_object_functions() -> i32 {
    let _ = trace("--- Category 4: Any Ledger Object Functions ---");

    // First we need to cache a ledger object to test the other functions
    // Get the account from transaction and generate its keylet
    let escrow_finish = EscrowFinish;
    let account_id = escrow_finish.get_account().unwrap();

    // Test 4.1: cache_ledger_obj() - Cache a ledger object
    let mut keylet_buffer = [0u8; 32];
    let keylet_result = unsafe {
        host::account_keylet(
            account_id.0.as_ptr(),
            account_id.0.len(),
            keylet_buffer.as_mut_ptr(),
            keylet_buffer.len(),
        )
    };

    if keylet_result != 32 {
        let _ = trace_num(
            "ERROR: account_keylet failed for caching test:",
            keylet_result as i64,
        );
        return -401; // Keylet generation failed for caching test
    }

    let cache_result =
        unsafe { host::cache_ledger_obj(keylet_buffer.as_ptr(), keylet_result as usize, 0) };

    if cache_result <= 0 {
        let _ = trace_num(
            "INFO: cache_ledger_obj failed (expected with test fixtures):",
            cache_result as i64,
        );
        // Test fixtures may not contain the account object - this is expected
        // We'll test the interface but expect failures

        // Test 4.2-4.5 with invalid slot (should fail gracefully)
        let mut test_buffer = [0u8; 32];

        // Test get_ledger_obj_field with invalid slot
        let field_result = unsafe {
            host::get_ledger_obj_field(
                1,
                sfield::Balance,
                test_buffer.as_mut_ptr(),
                test_buffer.len(),
            )
        };
        if field_result < 0 {
            let _ = trace_num(
                "INFO: get_ledger_obj_field failed as expected (no cached object):",
                field_result as i64,
            );
        }

        // Test get_ledger_obj_nested_field with invalid slot
        let locator = [0x01, 0x00];
        let nested_result = unsafe {
            host::get_ledger_obj_nested_field(
                1,
                locator.as_ptr(),
                locator.len(),
                test_buffer.as_mut_ptr(),
                test_buffer.len(),
            )
        };
        if nested_result < 0 {
            let _ = trace_num(
                "INFO: get_ledger_obj_nested_field failed as expected:",
                nested_result as i64,
            );
        }

        // Test get_ledger_obj_array_len with invalid slot
        let array_result = unsafe { host::get_ledger_obj_array_len(1, sfield::Signers) };
        if array_result < 0 {
            let _ = trace_num(
                "INFO: get_ledger_obj_array_len failed as expected:",
                array_result as i64,
            );
        }

        // Test get_ledger_obj_nested_array_len with invalid slot
        let nested_array_result =
            unsafe { host::get_ledger_obj_nested_array_len(1, locator.as_ptr(), locator.len()) };
        if nested_array_result < 0 {
            let _ = trace_num(
                "INFO: get_ledger_obj_nested_array_len failed as expected:",
                nested_array_result as i64,
            );
        }

        let _ = trace("SUCCESS: Any ledger object functions (interface tested)");
        return 0;
    }

    // If we successfully cached an object, test the access functions
    let slot = cache_result;
    let _ = trace_num("Successfully cached object in slot:", slot as i64);

    // Test 4.2: get_ledger_obj_field() - Access field from cached object
    let mut cached_balance_buffer = [0u8; 8];
    let cached_balance_result = unsafe {
        host::get_ledger_obj_field(
            slot,
            sfield::Balance,
            cached_balance_buffer.as_mut_ptr(),
            cached_balance_buffer.len(),
        )
    };

    if cached_balance_result <= 0 {
        let _ = trace_num(
            "INFO: get_ledger_obj_field(Balance) failed:",
            cached_balance_result as i64,
        );
    } else if cached_balance_result == 8 {
        let _ = trace_num(
            "Cached object balance length (XRP amount):",
            cached_balance_result as i64,
        );
        let _ = trace_data(
            "Cached object balance (serialized XRP amount):",
            &cached_balance_buffer,
            DataRepr::AsHex,
        );
    } else {
        let _ = trace_num(
            "Cached object balance length (non-XRP amount):",
            cached_balance_result as i64,
        );
        let _ = trace_data(
            "Cached object balance:",
            &cached_balance_buffer[..cached_balance_result as usize],
            DataRepr::AsHex,
        );
    }

    // Test 4.3: get_ledger_obj_nested_field() - Nested field from cached object
    let locator = [0x01, 0x00];
    let mut cached_nested_buffer = [0u8; 32];
    let cached_nested_result = unsafe {
        host::get_ledger_obj_nested_field(
            slot,
            locator.as_ptr(),
            locator.len(),
            cached_nested_buffer.as_mut_ptr(),
            cached_nested_buffer.len(),
        )
    };

    if cached_nested_result < 0 {
        let _ = trace_num(
            "INFO: get_ledger_obj_nested_field not applicable:",
            cached_nested_result as i64,
        );
    } else {
        let _ = trace_num("Cached nested field length:", cached_nested_result as i64);
        let _ = trace_data(
            "Cached nested field:",
            &cached_nested_buffer[..cached_nested_result as usize],
            DataRepr::AsHex,
        );
    }

    // Test 4.4: get_ledger_obj_array_len() - Array length from cached object
    let cached_array_len = unsafe { host::get_ledger_obj_array_len(slot, sfield::Signers) };
    let _ = trace_num(
        "Cached object Signers array length:",
        cached_array_len as i64,
    );

    // Test 4.5: get_ledger_obj_nested_array_len() - Nested array length from cached object
    let cached_nested_array_len =
        unsafe { host::get_ledger_obj_nested_array_len(slot, locator.as_ptr(), locator.len()) };

    if cached_nested_array_len < 0 {
        let _ = trace_num(
            "INFO: get_ledger_obj_nested_array_len not applicable:",
            cached_nested_array_len as i64,
        );
    } else {
        let _ = trace_num(
            "Cached nested array length:",
            cached_nested_array_len as i64,
        );
    }

    let _ = trace("SUCCESS: Any ledger object functions");
    0
}

/// Test Category 5: Keylet Generation Functions (4 functions)
/// Tests keylet generation functions for different ledger entry types
fn test_keylet_generation_functions() -> i32 {
    let _ = trace("--- Category 5: Keylet Generation Functions ---");

    let escrow_finish = EscrowFinish;
    let account_id = escrow_finish.get_account().unwrap();

    // Test 5.1: account_keylet() - Generate keylet for account
    let mut account_keylet_buffer = [0u8; 32];
    let account_keylet_result = unsafe {
        host::account_keylet(
            account_id.0.as_ptr(),
            account_id.0.len(),
            account_keylet_buffer.as_mut_ptr(),
            account_keylet_buffer.len(),
        )
    };

    if account_keylet_result != 32 {
        let _ = trace_num(
            "ERROR: account_keylet failed:",
            account_keylet_result as i64,
        );
        return -501; // Account keylet generation failed
    }
    let _ = trace_data("Account keylet:", &account_keylet_buffer, DataRepr::AsHex);

    // Test 5.2: credential_keylet() - Generate keylet for credential
    let mut credential_keylet_buffer = [0u8; 32];
    let credential_keylet_result = unsafe {
        host::credential_keylet(
            account_id.0.as_ptr(), // Subject
            account_id.0.len(),
            account_id.0.as_ptr(), // Issuer - same account for test
            account_id.0.len(),
            b"TestType".as_ptr(), // Credential type
            9usize,               // Length of "TestType"
            credential_keylet_buffer.as_mut_ptr(),
            credential_keylet_buffer.len(),
        )
    };

    if credential_keylet_result <= 0 {
        let _ = trace_num(
            "INFO: credential_keylet failed (expected - interface issue):",
            credential_keylet_result as i64,
        );
        // This is expected to fail due to unusual parameter types
    } else {
        let _ = trace_data(
            "Credential keylet:",
            &credential_keylet_buffer[..credential_keylet_result as usize],
            DataRepr::AsHex,
        );
    }

    // Test 5.3: escrow_keylet() - Generate keylet for escrow
    let mut escrow_keylet_buffer = [0u8; 32];
    let seq: i32 = 1000;
    let seq_bytes = seq.to_be_bytes();
    let escrow_keylet_result = unsafe {
        host::escrow_keylet(
            account_id.0.as_ptr(),
            account_id.0.len(),
            seq_bytes.as_ptr(),
            seq_bytes.len(),
            escrow_keylet_buffer.as_mut_ptr(),
            escrow_keylet_buffer.len(),
        )
    };

    if escrow_keylet_result != 32 {
        let _ = trace_num("ERROR: escrow_keylet failed:", escrow_keylet_result as i64);
        return -503; // Escrow keylet generation failed
    }
    let _ = trace_data("Escrow keylet:", &escrow_keylet_buffer, DataRepr::AsHex);

    // Test 5.4: oracle_keylet() - Generate keylet for oracle
    let mut oracle_keylet_buffer = [0u8; 32];
    let document_id: i32 = 42;
    let document_id_bytes = document_id.to_be_bytes();
    let oracle_keylet_result = unsafe {
        host::oracle_keylet(
            account_id.0.as_ptr(),
            account_id.0.len(),
            document_id_bytes.as_ptr(),
            document_id_bytes.len(),
            oracle_keylet_buffer.as_mut_ptr(),
            oracle_keylet_buffer.len(),
        )
    };

    if oracle_keylet_result != 32 {
        let _ = trace_num("ERROR: oracle_keylet failed:", oracle_keylet_result as i64);
        return -504; // Oracle keylet generation failed
    }
    let _ = trace_data("Oracle keylet:", &oracle_keylet_buffer, DataRepr::AsHex);

    let _ = trace("SUCCESS: Keylet generation functions");
    0
}

/// Test Category 6: Utility Functions (5 functions)
/// Tests utility functions for hashing, NFT access, and tracing
fn test_utility_functions() -> i32 {
    let _ = trace("--- Category 6: Utility Functions ---");

    // Test 6.1: compute_sha512_half() - SHA512 hash computation (first 32 bytes)
    let test_data = b"Hello, XRPL WASM world!";
    let mut hash_output = [0u8; 32];
    let hash_result = unsafe {
        host::compute_sha512_half(
            test_data.as_ptr(),
            test_data.len(),
            hash_output.as_mut_ptr(),
            hash_output.len(),
        )
    };

    if hash_result != 32 {
        let _ = trace_num("ERROR: compute_sha512_half failed:", hash_result as i64);
        return -601; // SHA512 half computation failed
    }
    let _ = trace_data("Input data:", test_data, DataRepr::AsHex);
    let _ = trace_data("SHA512 half hash:", &hash_output, DataRepr::AsHex);

    // Test 6.2: get_nft() - NFT data retrieval
    let escrow_finish = EscrowFinish;
    let account_id = escrow_finish.get_account().unwrap();
    let nft_id = [0u8; 32]; // Dummy NFT ID for testing
    let mut nft_buffer = [0u8; 256];
    let nft_result = unsafe {
        host::get_nft(
            account_id.0.as_ptr(),
            account_id.0.len(),
            nft_id.as_ptr(),
            nft_id.len(),
            nft_buffer.as_mut_ptr(),
            nft_buffer.len(),
        )
    };

    if nft_result <= 0 {
        let _ = trace_num(
            "INFO: get_nft failed (expected - no such NFT):",
            nft_result as i64,
        );
        // This is expected - test account likely doesn't own the dummy NFT
    } else {
        let _ = trace_num("NFT data length:", nft_result as i64);
        let _ = trace_data(
            "NFT data:",
            &nft_buffer[..nft_result as usize],
            DataRepr::AsHex,
        );
    }

    // Test 6.3: trace() - Debug logging with data
    let trace_message = b"Test trace message";
    let trace_data_payload = b"payload";
    let trace_result = unsafe {
        host::trace(
            trace_message.as_ptr(),
            trace_message.len(),
            trace_data_payload.as_ptr(),
            trace_data_payload.len(),
            1, // as_hex = true
        )
    };

    if trace_result < 0 {
        let _ = trace_num("ERROR: trace() failed:", trace_result as i64);
        return -603; // Trace function failed
    }
    let _ = trace_num("Trace function bytes written:", trace_result as i64);

    // Test 6.4: trace_num() - Debug logging with number
    let test_number = 42i64;
    let trace_num_result = trace_num("Test number trace", test_number);

    use xrpl_wasm_stdlib::host::Result;
    match trace_num_result {
        Result::Ok(_) => {
            let _ = trace_num("Trace_num function succeeded", 0);
        }
        Result::Err(_) => {
            let _ = trace_num(
                "ERROR: trace_num() failed:",
                trace_num_result.err().unwrap().code() as i64,
            );
            return -604; // Trace number function failed
        }
    }

    // Test 6.5: trace_amount() - Debug logging with Amount
    match test_trace_amount_functions() {
        0 => (),
        err => return err,
    }

    let _ = trace("SUCCESS: Utility functions");
    0
}

/// Test Category 7: Data Update Functions (1 function)
/// Tests the function for modifying the current ledger entry
fn test_data_update_functions() -> i32 {
    let _ = trace("--- Category 7: Data Update Functions ---");

    // Test 7.1: update_data() - Update current ledger entry data
    let update_payload = b"Updated ledger entry data from WASM test";

    let update_result = unsafe { host::update_data(update_payload.as_ptr(), update_payload.len()) };

    if update_result != 0 {
        let _ = trace_num("ERROR: update_data failed:", update_result as i64);
        return -701; // Data update failed
    }

    let _ = trace_data(
        "Successfully updated ledger entry with:",
        update_payload,
        DataRepr::AsHex,
    );
    let _ = trace("SUCCESS: Data update functions");
    1 // <-- Finish the escrow to indicate a successful outcome
}

/// Test trace_amount() function with different Amount types
/// Tests the trace_amount host function with XRP, IOU, and MPT amounts
fn test_trace_amount_functions() -> i32 {
    let _ = trace("--- Testing trace_amount() function ---");

    // Test 6.5.1: trace_amount() with XRP amount (positive)
    let xrp_amount = Amount::XRP {
        num_drops: 1_000_000, // 1 XRP
    };
    let trace_result = trace_amount("Test XRP amount (1 XRP)", &xrp_amount);
    match trace_result {
        host::Result::Ok(_) => {
            let _ = trace("SUCCESS: trace_amount with positive XRP");
        }
        host::Result::Err(_) => {
            let _ = trace_num(
                "ERROR: trace_amount XRP failed:",
                trace_result.err().unwrap().code() as i64,
            );
            return -605; // Trace amount XRP failed
        }
    }

    // Test 6.5.2: trace_amount() with negative XRP amount
    let negative_xrp_amount = Amount::XRP {
        num_drops: -500_000, // -0.5 XRP
    };
    let trace_result = trace_amount("Test negative XRP amount (-0.5 XRP)", &negative_xrp_amount);
    match trace_result {
        host::Result::Ok(_) => {
            let _ = trace("SUCCESS: trace_amount with negative XRP");
        }
        host::Result::Err(_) => {
            let _ = trace_num(
                "ERROR: trace_amount negative XRP failed:",
                trace_result.err().unwrap().code() as i64,
            );
            return -606; // Trace amount negative XRP failed
        }
    }

    // Test 6.5.3: trace_amount() with zero XRP amount
    // TODO: uncomment when new devnet is deployed
    // let zero_xrp_amount = Amount::XRP { num_drops: 0 };
    // let trace_result = trace_amount("Test zero XRP amount", &zero_xrp_amount);
    // match trace_result {
    //     host::Result::Ok(_) => {
    //         let _ = trace("SUCCESS: trace_amount with zero XRP");
    //     }
    //     host::Result::Err(_) => {
    //         let _ = trace_num(
    //             "ERROR: trace_amount zero XRP failed:",
    //             trace_result.err().unwrap().code() as i64,
    //         );
    //         return -607; // Trace amount zero XRP failed
    //     }
    // }

    // Test 6.5.4: trace_amount() with small XRP amount (fee-like)
    let fee_amount = Amount::XRP { num_drops: 10 }; // 10 drops (typical fee)
    let trace_result = trace_amount("Test small XRP amount (10 drops)", &fee_amount);
    match trace_result {
        host::Result::Ok(_) => {
            let _ = trace("SUCCESS: trace_amount with small XRP");
        }
        host::Result::Err(_) => {
            let _ = trace_num(
                "ERROR: trace_amount small XRP failed:",
                trace_result.err().unwrap().code() as i64,
            );
            return -608; // Trace amount small XRP failed
        }
    }

    // Test 6.5.5: trace_amount() with large XRP amount
    let large_xrp_amount = Amount::XRP {
        num_drops: 100_000_000_000, // 100,000 XRP
    };
    let trace_result = trace_amount("Test large XRP amount (100,000 XRP)", &large_xrp_amount);
    match trace_result {
        host::Result::Ok(_) => {
            let _ = trace("SUCCESS: trace_amount with large XRP");
        }
        host::Result::Err(_) => {
            let _ = trace_num(
                "ERROR: trace_amount large XRP failed:",
                trace_result.err().unwrap().code() as i64,
            );
            return -609; // Trace amount large XRP failed
        }
    }

    let _ = trace("SUCCESS: trace_amount XRP tests completed");

    // Test 6.5.6: trace_amount() with IOU amount
    // USD currency code: 20 bytes with "USD" at positions 12-14, rest zeros
    let mut currency_bytes = [0u8; 20];
    currency_bytes[12..15].copy_from_slice(b"USD");
    let issuer_bytes = [3u8; 20]; // Test issuer

    // Create a valid IOU amount: $5 USD
    // Mantissa = 5000000000000000, Exponent = -15 (raw exponent = 82)
    // Actual value = 5000000000000000 × 10^-15 = 5
    // Format: [Type=1][Sign=1][Exponent=82][Mantissa=54bits]
    // Type bit (bit 63) = 1, Sign bit (bit 62) = 1
    // Exponent 82 = 0b01010010
    // Top byte: 0b11010100 = 0xD4
    // Second byte: 0b10010001 = 0x91
    let amount_bytes = [0xD4, 0x91, 0xC3, 0x79, 0x37, 0xE0, 0x80, 0x00]; // Valid IOU: $5 USD

    let currency = Currency::from(currency_bytes);
    let issuer = AccountID::from(issuer_bytes);
    let amount = OpaqueFloat(amount_bytes);

    let iou_amount = Amount::IOU {
        amount,
        issuer,
        currency,
    };
    let trace_result = trace_amount("Test IOU amount", &iou_amount);
    match trace_result {
        host::Result::Ok(_) => {
            let _ = trace("SUCCESS: trace_amount with IOU");
        }
        host::Result::Err(_) => {
            let _ = trace_num(
                "ERROR: trace_amount IOU failed:",
                trace_result.err().unwrap().code() as i64,
            );
            return -610; // Trace amount IOU failed
        }
    }

    // Test 6.5.7: trace_amount() with MPT amount (positive)
    const MPT_VALUE: u64 = 500_000;
    const MPT_SEQUENCE_NUM: u32 = 12345;
    const MPT_ISSUER_BYTES: [u8; 20] = [1u8; 20];

    let mpt_issuer = AccountID::from(MPT_ISSUER_BYTES);
    let mpt_id = MptId::new(MPT_SEQUENCE_NUM, mpt_issuer);
    let mpt_amount = Amount::MPT {
        num_units: MPT_VALUE,
        is_positive: true,
        mpt_id,
    };
    let trace_result = trace_amount("Test positive MPT amount", &mpt_amount);
    match trace_result {
        host::Result::Ok(_) => {
            let _ = trace("SUCCESS: trace_amount with positive MPT");
        }
        host::Result::Err(_) => {
            let _ = trace_num(
                "ERROR: trace_amount positive MPT failed:",
                trace_result.err().unwrap().code() as i64,
            );
            return -611; // Trace amount positive MPT failed
        }
    }

    // Test 6.5.8: trace_amount() with MPT amount (negative)
    let negative_mpt_amount = Amount::MPT {
        num_units: MPT_VALUE,
        is_positive: false,
        mpt_id,
    };
    let trace_result = trace_amount("Test negative MPT amount", &negative_mpt_amount);
    match trace_result {
        host::Result::Ok(_) => {
            let _ = trace("SUCCESS: trace_amount with negative MPT");
        }
        host::Result::Err(_) => {
            let _ = trace_num(
                "ERROR: trace_amount negative MPT failed:",
                trace_result.err().unwrap().code() as i64,
            );
            return -612; // Trace amount negative MPT failed
        }
    }

    // Test 6.5.9: trace_amount() with zero MPT amount
    // TODO: uncomment when new devnet is deployed
    // let zero_mpt_amount = Amount::MPT {
    //     num_units: 0,
    //     is_positive: true,
    //     mpt_id,
    // };
    // let trace_result = trace_amount("Test zero MPT amount", &zero_mpt_amount);
    // match trace_result {
    //     host::Result::Ok(_) => {
    //         let _ = trace("SUCCESS: trace_amount with zero MPT");
    //     }
    //     host::Result::Err(_) => {
    //         let _ = trace_num(
    //             "ERROR: trace_amount zero MPT failed:",
    //             trace_result.err().unwrap().code() as i64,
    //         );
    //         return -613; // Trace amount zero MPT failed
    //     }
    // }

    let _ = trace("SUCCESS: All trace_amount tests completed");
    1
}

#[cfg(test)]
mod coverage_tests {
    use super::*;

    /// Coverage test: exercises all host function categories via finish()
    ///
    /// This test runs the same logic as the integration test, but on native
    /// targets with stub host functions. It's used to measure code coverage
    /// of xrpl-wasm-stdlib.
    ///
    /// Note: The host functions return dummy values (from host_bindings_for_testing.rs),
    /// so this test verifies that the code *runs*, not that it's *correct*.
    /// Correctness is verified by the real integration tests against rippled.
    #[test]
    fn test_finish_exercises_all_host_functions() {
        // On non-wasm targets, finish() uses host_bindings_for_testing.rs
        // which provides stub implementations of all host functions.
        let result = finish();

        // The finish() function returns 1 on success, or a negative error code.
        // With stub host functions, we expect success (though the actual
        // behavior depends on the stub implementations).
        core::assert_eq!(result, 1, "finish() should return 1 on success");
    }
}
