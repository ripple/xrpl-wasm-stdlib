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
// -500 to -599: Ledger entry ID Generation Functions (4 functions)
// -600 to -699: Utility Functions (4 functions)
// -700 to -799: Data Update Functions (1 function)
//

use xrpl_common_stdlib::current_tx::traits::TransactionCommonFields;
use xrpl_common_stdlib::host;
use xrpl_common_stdlib::host::trace::{
    TraceDataType, trace, trace_acct_buf, trace_amt, trace_hex, trace_num,
};
use xrpl_common_stdlib::sfield;
use xrpl_common_stdlib::types::account_id::AccountID;
use xrpl_common_stdlib::types::amount::Amount;
use xrpl_common_stdlib::types::currency::Currency;
use xrpl_common_stdlib::types::iou_number::IOUNumber;
use xrpl_common_stdlib::types::mpt_id::MptId;
use xrpl_escrow_stdlib::current_tx::escrow_finish::EscrowFinish;

#[unsafe(no_mangle)]
pub extern "C" fn escrow_finish() -> i32 {
    trace("=== HOST FUNCTIONS TEST ===");
    trace("Testing 27 host functions");

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

    // Category 5: Ledger entry ID Generation Functions (4 functions)
    // Error range: -500 to -599
    match test_id_generation_functions() {
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

    trace("SUCCESS: All host function tests passed!");
    1 // Success return code for WASM finish function
}

/// Test Category 1: Ledger Header Data Functions (3 functions)
/// - ldgr_index() - Get ledger sequence number
/// - parent_ldgr_time() - Get parent ledger timestamp
/// - parent_ldgr_hash() - Get parent ledger hash
fn test_ledger_header_functions() -> i32 {
    trace("--- Category 1: Ledger Header Functions ---");

    // Test 1.1: ldgr_index() - should return current ledger sequence number
    let mut sqn_buffer = [0u8; 4];
    let sqn_result = unsafe { host::ldgr_index(sqn_buffer.as_mut_ptr(), sqn_buffer.len()) };

    if sqn_result <= 0 {
        trace_num("ERROR: ldgr_index failed:", sqn_result as i64);
        return -101; // Ledger sequence number test failed
    }
    let ledger_sqn = u32::from_be_bytes(sqn_buffer);
    trace_num("Ledger sequence number:", ledger_sqn as i64);

    // Test 1.2: parent_ldgr_time() - should return parent ledger timestamp
    let mut time_buffer = [0u8; 4];
    let time_result =
        unsafe { host::parent_ldgr_time(time_buffer.as_mut_ptr(), time_buffer.len()) };

    if time_result <= 0 {
        trace_num("ERROR: parent_ldgr_time failed:", time_result as i64);
        return -102; // Parent ledger time test failed
    }
    let parent_ledger_time = u32::from_be_bytes(time_buffer);
    trace_num("Parent ledger time:", parent_ledger_time as i64);

    // Test 1.3: parent_ldgr_hash() - should return parent ledger hash (32 bytes)
    let mut hash_buffer = [0u8; 32];
    let hash_result =
        unsafe { host::parent_ldgr_hash(hash_buffer.as_mut_ptr(), hash_buffer.len()) };

    if hash_result != 32 {
        trace_num("ERROR: parent_ldgr_hash wrong length:", hash_result as i64);
        return -103; // Parent ledger hash test failed - should be exactly 32 bytes
    }
    trace_hex("Parent ledger hash:", &hash_buffer);

    trace("SUCCESS: Ledger header functions");
    0
}

/// Test Category 2: Transaction Data Functions (5 functions)
/// Tests all functions for accessing current transaction data
fn test_transaction_data_functions() -> i32 {
    trace("--- Category 2: Transaction Data Functions ---");

    // Test 2.1: tx_field() - Basic transaction field access
    // Test with Account field (required, 20 bytes)
    let mut account_buffer = [0u8; 20];
    let account_len = unsafe {
        host::tx_field(
            sfield::Account.into(),
            account_buffer.as_mut_ptr(),
            account_buffer.len(),
        )
    };

    if account_len != 20 {
        trace_num("ERROR: tx_field(Account) wrong length:", account_len as i64);
        return -201; // Basic transaction field test failed
    }
    trace_acct_buf("Transaction Account:", &account_buffer);

    // Test with Fee field (XRP amount - 8 bytes in new serialized format)
    // New format: XRP amounts are always 8 bytes (positive: value | cPositive flag, negative: just value)
    let mut fee_buffer = [0u8; 8];
    let fee_len = unsafe {
        host::tx_field(
            sfield::Fee.into(),
            fee_buffer.as_mut_ptr(),
            fee_buffer.len(),
        )
    };

    if fee_len != 8 {
        trace_num(
            "ERROR: tx_field(Fee) wrong length (expected 8 bytes for XRP):",
            fee_len as i64,
        );
        return -202; // Fee field test failed - XRP amounts should be exactly 8 bytes
    }
    trace_num("Transaction Fee length:", fee_len as i64);
    trace_hex("Transaction Fee (serialized XRP amount):", &fee_buffer);

    // Test with Sequence field (required, 4 bytes uint32)
    let mut seq_buffer = [0u8; 4];
    let seq_len = unsafe {
        host::tx_field(
            sfield::Sequence.into(),
            seq_buffer.as_mut_ptr(),
            seq_buffer.len(),
        )
    };

    if seq_len != 4 {
        trace_num("ERROR: tx_field(Sequence) wrong length:", seq_len as i64);
        return -203; // Sequence field test failed
    }
    trace_hex("Transaction Sequence:", &seq_buffer);

    // NOTE: tx_field2() through tx_field6() have been deprecated.
    // Use tx_field() with appropriate parameters for all transaction field access.

    // Test 2.2: tx_inner() - Inner field access with locator
    let locator = [0x01, 0x00]; // Simple locator for first element
    let mut inner_buffer = [0u8; 32];
    let inner_result = unsafe {
        host::tx_inner(
            locator.as_ptr(),
            locator.len(),
            inner_buffer.as_mut_ptr(),
            inner_buffer.len(),
        )
    };

    if inner_result < 0 {
        trace_num("INFO: tx_inner not applicable:", inner_result as i64);
        // Expected - locator may not match transaction structure
    } else {
        trace_num("Inner field length:", inner_result as i64);
        trace_hex("Inner field:", &inner_buffer[..inner_result as usize]);
    }

    // Test 2.3: tx_arr_len() - Get array length
    let signers_len = unsafe { host::tx_arr_len(sfield::Signers.into()) };
    trace_num("Signers array length:", signers_len as i64);

    let memos_len = unsafe { host::tx_arr_len(sfield::Memos.into()) };
    trace_num("Memos array length:", memos_len as i64);

    // Test 2.4: tx_inner_arr_len() - Get inner array length with locator
    let inner_array_len = unsafe { host::tx_inner_arr_len(locator.as_ptr(), locator.len()) };

    if inner_array_len < 0 {
        trace_num(
            "INFO: tx_inner_arr_len not applicable:",
            inner_array_len as i64,
        );
    } else {
        trace_num("Inner array length:", inner_array_len as i64);
    }

    trace("SUCCESS: Transaction data functions");
    0
}

/// Test Category 3: Current Ledger Object Functions (4 functions)
/// Tests functions that access the current ledger object being processed
fn test_current_ledger_object_functions() -> i32 {
    trace("--- Category 3: Current Ledger Object Functions ---");

    // Test 3.1: home_le_field() - Access field from current ledger object
    // Test with Balance field (XRP amount - 8 bytes in new serialized format)
    let mut balance_buffer = [0u8; 8];
    let balance_result = unsafe {
        host::home_le_field(
            sfield::Balance.into(),
            balance_buffer.as_mut_ptr(),
            balance_buffer.len(),
        )
    };

    if balance_result <= 0 {
        trace_num(
            "INFO: home_le_field(Balance) failed (may be expected):",
            balance_result as i64,
        );
        // This might fail if current ledger object doesn't have balance field
    } else if balance_result == 8 {
        trace_num(
            "Current object balance length (XRP amount):",
            balance_result as i64,
        );
        trace_hex(
            "Current object balance (serialized XRP amount):",
            &balance_buffer,
        );
    } else {
        trace_num(
            "Current object balance length (non-XRP amount):",
            balance_result as i64,
        );
        trace_hex(
            "Current object balance:",
            &balance_buffer[..balance_result as usize],
        );
    }

    // Test with Account field
    let mut current_account_buffer = [0u8; 20];
    let current_account_result = unsafe {
        host::home_le_field(
            sfield::Account.into(),
            current_account_buffer.as_mut_ptr(),
            current_account_buffer.len(),
        )
    };

    if current_account_result <= 0 {
        trace_num(
            "INFO: home_le_field(Account) failed:",
            current_account_result as i64,
        );
    } else {
        trace_acct_buf("Current ledger object account:", &current_account_buffer);
    }

    // Test 3.2: home_le_inner() - Inner field access
    let locator = [0x01, 0x00]; // Simple locator
    let mut current_inner_buffer = [0u8; 32];
    let current_inner_result = unsafe {
        host::home_le_inner(
            locator.as_ptr(),
            locator.len(),
            current_inner_buffer.as_mut_ptr(),
            current_inner_buffer.len(),
        )
    };

    if current_inner_result < 0 {
        trace_num(
            "INFO: home_le_inner not applicable:",
            current_inner_result as i64,
        );
    } else {
        trace_num("Current inner field length:", current_inner_result as i64);
        trace_hex(
            "Current inner field:",
            &current_inner_buffer[..current_inner_result as usize],
        );
    }

    // Test 3.3: home_le_arr_len() - Array length in current object
    let current_array_len = unsafe { host::home_le_arr_len(sfield::Signers.into()) };
    trace_num(
        "Current object Signers array length:",
        current_array_len as i64,
    );

    // Test 3.4: home_le_inner_arr_len() - Inner array length
    let current_inner_array_len =
        unsafe { host::home_le_inner_arr_len(locator.as_ptr(), locator.len()) };

    if current_inner_array_len < 0 {
        trace_num(
            "INFO: home_le_inner_arr_len not applicable:",
            current_inner_array_len as i64,
        );
    } else {
        trace_num(
            "Current inner array length:",
            current_inner_array_len as i64,
        );
    }

    trace("SUCCESS: Current ledger object functions");
    0
}

/// Test Category 4: Any Ledger Object Functions (5 functions)
/// Tests functions that work with cached ledger objects
fn test_any_ledger_object_functions() -> i32 {
    trace("--- Category 4: Any Ledger Object Functions ---");

    // First we need to cache a ledger object to test the other functions
    // Get the account from transaction and generate its ledger entry ID
    let escrow_finish = EscrowFinish;
    let account_id = escrow_finish.get_account().unwrap();

    // Test 4.1: cache_le() - Cache a ledger object
    let mut id_buffer = [0u8; 32];
    let id_result = unsafe {
        host::accountroot_id(
            account_id.0.as_ptr(),
            account_id.0.len(),
            id_buffer.as_mut_ptr(),
            id_buffer.len(),
        )
    };

    if id_result != 32 {
        trace_num(
            "ERROR: accountroot_id failed for caching test:",
            id_result as i64,
        );
        return -401; // LedgerEntryId generation failed for caching test
    }

    let cache_result = unsafe { host::cache_le(id_buffer.as_ptr(), id_result as usize, 0) };

    if cache_result <= 0 {
        trace_num(
            "INFO: cache_le failed (expected with test fixtures):",
            cache_result as i64,
        );
        // Test fixtures may not contain the account object - this is expected
        // We'll test the interface but expect failures

        // Test 4.2-4.5 with invalid slot (should fail gracefully)
        let mut test_buffer = [0u8; 32];

        // Test le_field with invalid slot
        let field_result = unsafe {
            host::le_field(
                1,
                sfield::Balance.into(),
                test_buffer.as_mut_ptr(),
                test_buffer.len(),
            )
        };
        if field_result < 0 {
            trace_num(
                "INFO: le_field failed as expected (no cached object):",
                field_result as i64,
            );
        }

        // Test le_inner with invalid slot
        let locator = [0x01, 0x00];
        let inner_result = unsafe {
            host::le_inner(
                1,
                locator.as_ptr(),
                locator.len(),
                test_buffer.as_mut_ptr(),
                test_buffer.len(),
            )
        };
        if inner_result < 0 {
            trace_num("INFO: le_inner failed as expected:", inner_result as i64);
        }

        // Test le_arr_len with invalid slot
        let array_result = unsafe { host::le_arr_len(1, sfield::Signers.into()) };
        if array_result < 0 {
            trace_num("INFO: le_arr_len failed as expected:", array_result as i64);
        }

        // Test le_inner_arr_len with invalid slot
        let inner_array_result =
            unsafe { host::le_inner_arr_len(1, locator.as_ptr(), locator.len()) };
        if inner_array_result < 0 {
            trace_num(
                "INFO: le_inner_arr_len failed as expected:",
                inner_array_result as i64,
            );
        }

        trace("SUCCESS: Any ledger object functions (interface tested)");
        return 0;
    }

    // If we successfully cached an object, test the access functions
    let slot = cache_result;
    trace_num("Successfully cached object in slot:", slot as i64);

    // Test 4.2: le_field() - Access field from cached object
    let mut cached_balance_buffer = [0u8; 8];
    let cached_balance_result = unsafe {
        host::le_field(
            slot,
            sfield::Balance.into(),
            cached_balance_buffer.as_mut_ptr(),
            cached_balance_buffer.len(),
        )
    };

    if cached_balance_result <= 0 {
        trace_num(
            "INFO: le_field(Balance) failed:",
            cached_balance_result as i64,
        );
    } else if cached_balance_result == 8 {
        trace_num(
            "Cached object balance length (XRP amount):",
            cached_balance_result as i64,
        );
        trace_hex(
            "Cached object balance (serialized XRP amount):",
            &cached_balance_buffer,
        );
    } else {
        trace_num(
            "Cached object balance length (non-XRP amount):",
            cached_balance_result as i64,
        );
        trace_hex(
            "Cached object balance:",
            &cached_balance_buffer[..cached_balance_result as usize],
        );
    }

    // Test 4.3: le_inner() - Inner field from cached object
    let locator = [0x01, 0x00];
    let mut cached_inner_buffer = [0u8; 32];
    let cached_inner_result = unsafe {
        host::le_inner(
            slot,
            locator.as_ptr(),
            locator.len(),
            cached_inner_buffer.as_mut_ptr(),
            cached_inner_buffer.len(),
        )
    };

    if cached_inner_result < 0 {
        trace_num("INFO: le_inner not applicable:", cached_inner_result as i64);
    } else {
        trace_num("Cached inner field length:", cached_inner_result as i64);
        trace_hex(
            "Cached inner field:",
            &cached_inner_buffer[..cached_inner_result as usize],
        );
    }

    // Test 4.4: le_arr_len() - Array length from cached object
    let cached_array_len = unsafe { host::le_arr_len(slot, sfield::Signers.into()) };
    trace_num(
        "Cached object Signers array length:",
        cached_array_len as i64,
    );

    // Test 4.5: le_inner_arr_len() - Inner array length from cached object
    let cached_inner_array_len =
        unsafe { host::le_inner_arr_len(slot, locator.as_ptr(), locator.len()) };

    if cached_inner_array_len < 0 {
        trace_num(
            "INFO: le_inner_arr_len not applicable:",
            cached_inner_array_len as i64,
        );
    } else {
        trace_num("Cached inner array length:", cached_inner_array_len as i64);
    }

    trace("SUCCESS: Any ledger object functions");
    0
}

/// Test Category 5: Ledger entry ID Generation Functions (4 functions)
/// Tests ledger entry ID generation functions for different ledger entry types
fn test_id_generation_functions() -> i32 {
    trace("--- Category 5: LedgerEntryId Generation Functions ---");

    let escrow_finish = EscrowFinish;
    let account_id = escrow_finish.get_account().unwrap();

    // Test 5.1: accountroot_id() - Generate ledger entry ID for account
    let mut accountroot_id_buffer = [0u8; 32];
    let accountroot_id_result = unsafe {
        host::accountroot_id(
            account_id.0.as_ptr(),
            account_id.0.len(),
            accountroot_id_buffer.as_mut_ptr(),
            accountroot_id_buffer.len(),
        )
    };

    if accountroot_id_result != 32 {
        trace_num(
            "ERROR: accountroot_id failed:",
            accountroot_id_result as i64,
        );
        return -501; // AccountRoot ledger entry ID generation failed
    }
    trace_hex("AccountRoot ledger entry ID:", &accountroot_id_buffer);

    // Test 5.2: credential_id() - Generate ledger entry ID for credential
    let mut credential_id_buffer = [0u8; 32];
    let credential_id_result = unsafe {
        host::credential_id(
            account_id.0.as_ptr(), // Subject
            account_id.0.len(),
            account_id.0.as_ptr(), // Issuer - same account for test
            account_id.0.len(),
            b"TestType".as_ptr(), // Credential type
            9usize,               // Length of "TestType"
            credential_id_buffer.as_mut_ptr(),
            credential_id_buffer.len(),
        )
    };

    if credential_id_result <= 0 {
        trace_num(
            "INFO: credential_id failed (expected - interface issue):",
            credential_id_result as i64,
        );
        // This is expected to fail due to unusual parameter types
    } else {
        trace_hex(
            "Credential ledger entry ID:",
            &credential_id_buffer[..credential_id_result as usize],
        );
    }

    // Test 5.3: escrow_id() - Generate ledger entry ID for escrow
    let mut escrow_id_buffer = [0u8; 32];
    let seq: i32 = 1000;
    let seq_bytes = seq.to_be_bytes();
    let escrow_id_result = unsafe {
        host::escrow_id(
            account_id.0.as_ptr(),
            account_id.0.len(),
            seq_bytes.as_ptr(),
            seq_bytes.len(),
            escrow_id_buffer.as_mut_ptr(),
            escrow_id_buffer.len(),
        )
    };

    if escrow_id_result != 32 {
        trace_num("ERROR: escrow_id failed:", escrow_id_result as i64);
        return -503; // Escrow ledger entry ID generation failed
    }
    trace_hex("Escrow ledger entry ID:", &escrow_id_buffer);

    // Test 5.4: oracle_id() - Generate ledger entry ID for oracle
    let mut oracle_id_buffer = [0u8; 32];
    let document_id: i32 = 42;
    let document_id_bytes = document_id.to_be_bytes();
    let oracle_id_result = unsafe {
        host::oracle_id(
            account_id.0.as_ptr(),
            account_id.0.len(),
            document_id_bytes.as_ptr(),
            document_id_bytes.len(),
            oracle_id_buffer.as_mut_ptr(),
            oracle_id_buffer.len(),
        )
    };

    if oracle_id_result != 32 {
        trace_num("ERROR: oracle_id failed:", oracle_id_result as i64);
        return -504; // Oracle ledger entry ID generation failed
    }
    trace_hex("Oracle ledger entry ID:", &oracle_id_buffer);

    trace("SUCCESS: LedgerEntryId generation functions");
    0
}

/// Test Category 6: Utility Functions (5 functions)
/// Tests utility functions for hashing, NFT access, and tracing
fn test_utility_functions() -> i32 {
    trace("--- Category 6: Utility Functions ---");

    // Test 6.1: sha512_half() - SHA512 hash computation (first 32 bytes)
    let test_data = b"Hello, XRPL WASM world!";
    let mut hash_output = [0u8; 32];
    let hash_result = unsafe {
        host::sha512_half(
            test_data.as_ptr(),
            test_data.len(),
            hash_output.as_mut_ptr(),
            hash_output.len(),
        )
    };

    if hash_result != 32 {
        trace_num("ERROR: sha512_half failed:", hash_result as i64);
        return -601; // SHA512 half computation failed
    }
    trace_hex("Input data:", test_data);
    trace_hex("SHA512 half hash:", &hash_output);

    // Test 6.2: nft_uri() - NFT data retrieval
    let escrow_finish = EscrowFinish;
    let account_id = escrow_finish.get_account().unwrap();
    let nft_id = [0u8; 32]; // Dummy NFT ID for testing
    let mut nft_buffer = [0u8; 256];
    let nft_result = unsafe {
        host::nft_uri(
            account_id.0.as_ptr(),
            account_id.0.len(),
            nft_id.as_ptr(),
            nft_id.len(),
            nft_buffer.as_mut_ptr(),
            nft_buffer.len(),
        )
    };

    if nft_result <= 0 {
        trace_num(
            "INFO: nft_uri failed (expected - no such NFT):",
            nft_result as i64,
        );
        // This is expected - test account likely doesn't own the dummy NFT
    } else {
        trace_num("NFT data length:", nft_result as i64);
        trace_hex("NFT data:", &nft_buffer[..nft_result as usize]);
    }

    // Test 6.3: trace() - Debug logging with data
    let trace_message = b"Test trace message";
    let trace_data_payload = b"payload";
    unsafe {
        host::trace(
            trace_message.as_ptr(),
            trace_message.len(),
            TraceDataType::AsHex as i32,
            trace_data_payload.as_ptr(),
            trace_data_payload.len(),
        )
    };

    // Test 6.4: trace_num() - Debug logging with number
    let test_number = 42i64;
    trace_num("Test number trace", test_number);
    trace_num("Trace_num function succeeded", 0);

    // Test 6.5: trace_amt() - Debug logging with Amount
    match test_trace_amt_functions() {
        0 => (),
        err => return err,
    }

    trace("SUCCESS: Utility functions");
    0
}

/// Test Category 7: Data Update Functions (1 function)
/// Tests the function for modifying the current ledger entry
fn test_data_update_functions() -> i32 {
    trace("--- Category 7: Data Update Functions ---");

    // Test 7.1: set_data() - Update current ledger entry data
    let update_payload = b"Updated ledger entry data from WASM test";

    let update_result = unsafe { host::set_data(update_payload.as_ptr(), update_payload.len()) };

    if update_result != 0 {
        trace_num("ERROR: set_data failed:", update_result as i64);
        return -701; // Data update failed
    }

    trace_hex("Successfully updated ledger entry with:", update_payload);
    trace("SUCCESS: Data update functions");
    1 // <-- Finish the escrow to indicate a successful outcome
}

/// Test trace_amt() function with different Amount types
/// Tests the trace_amt host function with XRP, IOU, and MPT amounts
fn test_trace_amt_functions() -> i32 {
    trace("--- Testing trace_amt() function ---");

    // Test 6.5.1: trace_amt() with XRP amount (positive)
    let xrp_amount = Amount::XRP {
        num_drops: 1_000_000, // 1 XRP
    };
    trace_amt("Test XRP amount (1 XRP)", &xrp_amount);
    trace("SUCCESS: trace_amt with positive XRP");

    // Test 6.5.2: trace_amt() with negative XRP amount
    let negative_xrp_amount = Amount::XRP {
        num_drops: -500_000, // -0.5 XRP
    };
    trace_amt("Test negative XRP amount (-0.5 XRP)", &negative_xrp_amount);
    trace("SUCCESS: trace_amt with negative XRP");

    // Test 6.5.3: trace_amt() with zero XRP amount
    // TODO: uncomment when new devnet is deployed
    // let zero_xrp_amount = Amount::XRP { num_drops: 0 };
    // let trace_result = trace_amt("Test zero XRP amount", &zero_xrp_amount);
    // match trace_result {
    //     host::Result::Ok(_) => {
    //         trace("SUCCESS: trace_amt with zero XRP");
    //     }
    //     host::Result::Err(_) => {
    //         trace_num(
    //             "ERROR: trace_amt zero XRP failed:",
    //             trace_result.err().unwrap().code() as i64,
    //         );
    //         return -607; // Trace amount zero XRP failed
    //     }
    // }

    // Test 6.5.4: trace_amt() with small XRP amount (fee-like)
    let fee_amount = Amount::XRP { num_drops: 10 }; // 10 drops (typical fee)
    trace_amt("Test small XRP amount (10 drops)", &fee_amount);
    trace("SUCCESS: trace_amt with small XRP");

    // Test 6.5.5: trace_amt() with large XRP amount
    let large_xrp_amount = Amount::XRP {
        num_drops: 100_000_000_000, // 100,000 XRP
    };
    trace_amt("Test large XRP amount (100,000 XRP)", &large_xrp_amount);
    trace("SUCCESS: trace_amt with large XRP");

    trace("SUCCESS: trace_amt XRP tests completed");

    // Test 6.5.6: trace_amt() with IOU amount
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
    let amount = IOUNumber(amount_bytes);

    let iou_amount = Amount::IOU {
        amount,
        issuer,
        currency,
    };
    trace_amt("Test IOU amount", &iou_amount);
    trace("SUCCESS: trace_amt with IOU");

    // Test 6.5.7: trace_amt() with MPT amount (positive)
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
    trace_amt("Test positive MPT amount", &mpt_amount);
    trace("SUCCESS: trace_amt with positive MPT");

    // Test 6.5.8: trace_amt() with MPT amount (negative)
    let negative_mpt_amount = Amount::MPT {
        num_units: MPT_VALUE,
        is_positive: false,
        mpt_id,
    };
    trace_amt("Test negative MPT amount", &negative_mpt_amount);
    trace("SUCCESS: trace_amt with negative MPT");

    // Test 6.5.9: trace_amt() with zero MPT amount
    // TODO: uncomment when new devnet is deployed
    // let zero_mpt_amount = Amount::MPT {
    //     num_units: 0,
    //     is_positive: true,
    //     mpt_id,
    // };
    // let trace_result = trace_amt("Test zero MPT amount", &zero_mpt_amount);
    // match trace_result {
    //     host::Result::Ok(_) => {
    //         trace("SUCCESS: trace_amt with zero MPT");
    //     }
    //     host::Result::Err(_) => {
    //         trace_num(
    //             "ERROR: trace_amt zero MPT failed:",
    //             trace_result.err().unwrap().code() as i64,
    //         );
    //         return -613; // Trace amount zero MPT failed
    //     }
    // }

    trace("SUCCESS: All trace_amt tests completed");
    1
}

#[cfg(test)]
mod coverage_tests {
    use super::*;

    /// Coverage test: exercises all host function categories via escrow_finish()
    ///
    /// This test runs the same logic as the integration test, but on native
    /// targets with stub host functions. It's used to measure code coverage
    /// of xrpl-common-stdlib.
    ///
    /// Note: The host functions return dummy values (from host_bindings_for_testing.rs),
    /// so this test verifies that the code *runs*, not that it's *correct*.
    /// Correctness is verified by the real integration tests against rippled.
    #[test]
    fn test_finish_exercises_all_host_functions() {
        // On non-wasm targets, escrow_finish() uses host_bindings_for_testing.rs
        // which provides stub implementations of all host functions.
        let result = escrow_finish();

        // The escrow_finish() function returns 1 on success, or a negative error code.
        // With stub host functions, we expect success (though the actual
        // behavior depends on the stub implementations).
        core::assert_eq!(result, 1, "escrow_finish() should return 1 on success");
    }
}
