# XRPL WebAssembly Standard Library - Complete Guide

This comprehensive guide covers everything you need to develop smart escrows using the XRPL WebAssembly Standard Library.

## Table of Contents

- [XRPL WebAssembly Standard Library - Complete Guide](#xrpl-webassembly-standard-library---complete-guide)
  - [Table of Contents](#table-of-contents)
  - [Getting Started](#getting-started)
    - [Prerequisites](#prerequisites)
    - [Installation](#installation)
    - [Your First Contract](#your-first-contract)
    - [Core Concepts](#core-concepts)
      - [Smart Escrow Basics](#smart-escrow-basics)
      - [Contract Structure](#contract-structure)
      - [Host Environment](#host-environment)
  - [API Reference](#api-reference)
    - [Transaction Access](#transaction-access)
      - [EscrowFinish Transaction](#escrowfinish-transaction)
      - [Field Access](#field-access)
    - [Ledger Objects](#ledger-objects)
      - [Account Information](#account-information)
      - [NFT Objects](#nft-objects)
    - [Type System](#type-system)
      - [Core Types](#core-types)
      - [Keylet Generation](#keylet-generation)
    - [Host Functions](#host-functions)
      - [Ledger Access](#ledger-access)
      - [Transaction Fields](#transaction-fields)
    - [Error Handling](#error-handling)
  - [Examples](#examples)
    - [Hello World](#hello-world)
    - [Oracle Example](#oracle-example)
    - [KYC Example](#kyc-example)
    - [Advanced Examples](#advanced-examples)
      - [Multi-Signature Notary](#multi-signature-notary)
      - [NFT Ownership Verification](#nft-ownership-verification)
      - [Time-Based Ledger Sequence](#time-based-ledger-sequence)
      - [Atomic Swap Examples](#atomic-swap-examples)
  - [Testing and Debugging](#testing-and-debugging)
    - [Test Networks](#test-networks)
    - [Key Testing Considerations](#key-testing-considerations)
    - [Test Using the Web UI](#test-using-the-web-ui)
    - [Performance Optimization](#performance-optimization)
      - [Binary Size Optimization](#binary-size-optimization)
      - [Runtime Optimization](#runtime-optimization)
    - [Troubleshooting](#troubleshooting)
      - [Common Build Issues](#common-build-issues)
      - [Common Runtime Issues](#common-runtime-issues)
      - [Debugging Techniques](#debugging-techniques)
  - [Additional Resources](#additional-resources)
  - [Contributing](#contributing)

---

## Getting Started

### Prerequisites

Before building smart escrows, ensure you have:

1. **Rust toolchain** (stable or nightly)
2. **WASM target** (`wasm32v1-none`)
3. **Node.js** (for testing tools)
4. **Basic understanding** of XRPL concepts

**Quick setup:**

```shell
# Run the automated setup script
./scripts/setup.sh

# Or install manually:
# Follow the instructions at https://rust-lang.org/tools/install/
rustup target add wasm32v1-none
npm install
```

### Installation

1. **Clone the repository:**

   ```shell
   git clone https://github.com/ripple/xrpl-wasm-stdlib.git
   cd xrpl-wasm-stdlib
   ```

2. **Run setup script:**

   ```shell
   ./scripts/setup.sh
   ```

3. **Verify installation:**
   ```shell
   ./scripts/run-tests.sh examples/smart-escrows/hello_world
   ```

### Your First Contract

Let's create a simple escrow that releases funds when an account balance exceeds 10 XRP:

```rust

use xrpl_wasm_stdlib::core::current_tx::escrow_finish::EscrowFinish;
use xrpl_wasm_stdlib::core::current_tx::traits::TransactionCommonFields;
use xrpl_wasm_stdlib::core::ledger_objects::account_root::get_account_balance;
use xrpl_wasm_stdlib::core::types::amount::Amount;
use xrpl_wasm_stdlib::host::Result::{Ok, Err};

#[unsafe(no_mangle)]
pub extern "C" fn finish() -> i32 {
    let tx = EscrowFinish;

    // Get the account trying to finish the escrow
    let account = match tx.get_account() {
        Ok(acc) => acc,
        Err(_) => return 0, // Invalid transaction
    };

    // Check account balance
    match get_account_balance(&account) {
        Ok(Some(Amount::XRP { num_drops })) if num_drops > 10_000_000 => 1, // Release (>10 XRP)
        _ => 0, // Keep locked
    }
}
```

**Build and test:**

```shell
# Add the contract code above to src/lib.rs
# Configure Cargo.toml:

[package]
name = "my-escrow"
version = "0.1.0"
edition = "2021"

[dependencies]
xrpl-wasm-stdlib = { path = "../xrpl-wasm-stdlib" }

[lib]
crate-type = ["cdylib"]

[profile.release]
opt-level = "s"
lto = true
panic = "abort"

# Build the contract
cargo build --target wasm32v1-none --release

# Test with provided tools
node ../examples/smart-escrows/hello_world/runTest.js
```

### Core Concepts

#### Smart Escrow Basics

Smart escrows are **conditional payment contracts** that:

- Lock XRP between parties
- Execute custom logic to determine release conditions
- Run deterministically across all network validators
- Have read-only access to ledger data

#### Contract Structure

Every smart escrow must:

1. **Export a `finish()` function** with signature `extern "C" fn finish() -> i32`
2. **Return 1 to release** funds or **0 to keep locked**
3. **Be deterministic** - same inputs always produce same outputs
4. **Use `#![no_std]`** - no standard library available (use ours instead 😉)

#### Host Environment

Smart escrows run in a constrained WebAssembly environment:

- No heap allocation - stack-based memory only
- No file system or network access
- Limited execution time and memory
- Read-only ledger access (except for escrow state updates)

---

## API Reference

> **⚠️ IMPORTANT:** The code examples in this API reference section are **illustrative only** and may not compile due to API changes.
>
> **For working, tested code that is guaranteed to compile and run correctly, please refer to the [complete examples](#examples).**
>
> The examples below demonstrate concepts and patterns, but the actual API may have changed. Always refer to the working examples for copy-pastable code.

### Transaction Access

The XRPL WASM Standard Library provides type-safe access to transaction data through the `current_tx` module.

#### EscrowFinish Transaction

```rust ignore
use xrpl_wasm_stdlib::core::current_tx::escrow_finish::EscrowFinish;

let tx = EscrowFinish;

// Get the account finishing the escrow
let account = tx.get_account().unwrap();

// Get the destination account (receives funds if released)
let destination = tx.get_destination().unwrap();

// Get the escrow sequence number
let escrow_sequence = tx.get_escrow_sequence().unwrap();
```

#### Field Access

```rust
use xrpl_wasm_stdlib::core::current_tx::escrow_finish::EscrowFinish;
use xrpl_wasm_stdlib::core::current_tx::traits::TransactionCommonFields;
use xrpl_wasm_stdlib::sfield;

let tx = EscrowFinish;

// Access transaction fields using trait methods
let fee_amount = tx.get_fee().ok(); // Returns Amount
let account_id = tx.get_account().ok(); // Returns AccountID
let sequence = tx.get_sequence().ok(); // Returns u32

// EscrowFinish-specific fields (when using EscrowFinishFields trait)
// let owner = tx.get_owner().ok();
// let offer_sequence = tx.get_offer_sequence().ok();
```

### Ledger Objects

Access current ledger state through the `ledger_objects` module.

#### Account Information

```rust
use xrpl_wasm_stdlib::core::ledger_objects::account_root::get_account_balance;
use xrpl_wasm_stdlib::core::types::account_id::AccountID;
use xrpl_wasm_stdlib::sfield;

let account = AccountID::from([0u8; 20]); // Replace with real account

// Get XRP balance (in drops) - returns Option<Amount>
let balance = get_account_balance(&account);

// Use host functions to get account fields directly
// (Note: Specific helper functions may vary based on current API)
```

#### NFT Objects

```rust ignore
// NFT functionality uses the NFToken type
use xrpl_wasm_stdlib::core::types::nft::NFToken;
use xrpl_wasm_stdlib::core::types::account_id::AccountID;

let owner = AccountID::from([0u8; 20]);
let nft_id_bytes = [0u8; 32]; // 32-byte NFT identifier
let nft_token = NFToken::new(nft_id_bytes);

// Check ownership
let is_owned = nft_token.is_owned_by(&owner);

// Get NFT metadata
let nft_flags = nft_token.flags()?;
let transfer_fee = nft_token.transfer_fee()?;
let issuer = nft_token.issuer()?;
let taxon = nft_token.taxon()?;
let token_sequence = nft_token.token_sequence()?;

// Check individual flags efficiently (no additional host calls)
if nft_flags.is_burnable() {
    // NFT can be burned by issuer
}
if nft_flags.is_transferable() {
    // NFT can be transferred
}

// Get NFT URI
let uri = nft_token.uri(&owner)?;
```

### Type System

#### Core Types

```rust ignore
use xrpl_wasm_stdlib::core::types::{
    account_id::AccountID,           // 20-byte XRPL account identifier
    amount::Amount, // Token amounts (XRP, IOU, MPT)
};
use xrpl_wasm_stdlib::types::NFT;      // [u8; 32] NFT identifier

// Create AccountID from r-address (if r_address macro exists)
// let account = xrpl_wasm_stdlib::r_address!("rN7n7otQDd6FczFgLdSqtcsAUxDkw6fzRH");

// Create from raw bytes
let account = AccountID::from([0u8; 20]);

// NFT as byte array
let nft: NFT = [0u8; 32];

// Note: High-level string parsing functions may not be available
// Use the working examples for guaranteed compilable code
```

#### Keylet Generation

Keylets are used to locate objects in the ledger:

```rust ignore
use xrpl_wasm_stdlib::core::types::keylets::{
    account_keylet,
    line_keylet,
    escrow_keylet,
    oracle_keylet,
};
use xrpl_wasm_stdlib::core::types::account_id::AccountID;
use xrpl_wasm_stdlib::core::types::amount::asset::Asset;

let account = AccountID::from([0u8; 20]);
let sequence = 12345i32;

// Account keylet
let keylet = account_keylet(&account);

// Trust line keylet (requires Asset types)
let asset1 = Asset::XRP(XrpAsset {});
let asset2 = Asset::IOU(IouAsset::new(issuer, currency));
let keylet = line_keylet(&account, &asset1, &asset2);

// Escrow keylet
let keylet = escrow_keylet(&account, sequence);

// Oracle keylet
let document_id = 1i32;
let keylet = oracle_keylet(&account, document_id);
```

### Host Functions

Low-level host function access through the `host` module.

#### Ledger Access

```rust
// Use the high-level trait methods instead of low-level host functions
use xrpl_wasm_stdlib::core::ledger_objects::account_root::AccountRoot;
use xrpl_wasm_stdlib::core::ledger_objects::traits::AccountFields;
use xrpl_wasm_stdlib::core::types::account_id::AccountID;
use xrpl_wasm_stdlib::core::types::keylets::account_keylet;
use xrpl_wasm_stdlib::host::cache_ledger_obj;
use xrpl_wasm_stdlib::host::Error;

// The correct approach is to use the trait methods
fn main() {
    let account = AccountID::from(*b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3");
    let account_keylet = account_keylet(&account).unwrap_or_panic();
    let slot = unsafe { cache_ledger_obj(account_keylet.as_ptr(), account_keylet.len(), 0) };
    if slot < 0 {
        return;
    }

    let account_root = AccountRoot { slot_num: slot };
    let balance = account_root.balance();  // Returns Option<Amount>
    let sequence = account_root.sequence(); // Returns u32
}
```

#### Transaction Fields

```rust
// Use the high-level trait methods instead of low-level host functions
use xrpl_wasm_stdlib::core::current_tx::escrow_finish::EscrowFinish;
use xrpl_wasm_stdlib::core::current_tx::traits::{TransactionCommonFields, EscrowFinishFields};

fn main() {
    let tx = EscrowFinish;

    // Access common transaction fields
    let account = tx.get_account(); // AccountID
    let fee = tx.get_fee(); // Amount
    let sequence = tx.get_sequence(); // u32

    // Access EscrowFinish-specific fields
    let owner = tx.get_owner(); // AccountID
    let offer_sequence = tx.get_offer_sequence(); // u32
    let condition = tx.get_condition(); // Option<Condition>
}
```

### Error Handling

The library uses custom `Result` types for comprehensive error handling:

```rust
use xrpl_wasm_stdlib::core::current_tx::escrow_finish::EscrowFinish;
use xrpl_wasm_stdlib::core::current_tx::traits::TransactionCommonFields;
use xrpl_wasm_stdlib::core::ledger_objects::account_root::{get_account_balance, AccountRoot};
use xrpl_wasm_stdlib::core::ledger_objects::traits::AccountFields;
use xrpl_wasm_stdlib::core::types::account_id::AccountID;
use xrpl_wasm_stdlib::core::types::amount::Amount;
use xrpl_wasm_stdlib::core::types::keylets::account_keylet;
use xrpl_wasm_stdlib::host::{cache_ledger_obj, Error, Result};
use xrpl_wasm_stdlib::host::Result::{Ok, Err};

fn process_escrow() -> Result<i32> {
    let tx = EscrowFinish;

    // Chain operations with ?
    let account = match tx.get_account() {
        Ok(acc) => acc,
        Err(e) => return Err(e), // Invalid transaction
    };

    let balance = get_account_balance(&account);

    // Handle specific errors - create AccountRoot to access account fields
    let account_keylet = match account_keylet(&account) {
        Ok(keylet) => keylet,
        Err(e) => return Err(e), // Invalid account
    };

    let slot = unsafe { cache_ledger_obj(account_keylet.as_ptr(), account_keylet.len(), 0) };
    if slot < 0 {
        return Err(Error::from_code(slot));
    }

    let account_root = AccountRoot { slot_num: slot };
    match account_root.sequence() {
        Ok(sequence) => {
            // Use sequence
        },
        Err(e) => {
            // Handle missing field or other error
            return Err(e);
        },
    }

    return Ok(match balance {
        Ok(Some(Amount::XRP { num_drops })) if num_drops > 10_000_000 => 1,
        _ => 0,
    })
}
```

**Common error patterns:**

- `WasmError::ObjectNotFound` - Ledger object doesn't exist
- `WasmError::FieldNotFound` - Required field missing
- `WasmError::InvalidField` - Field data malformed
- `WasmError::BufferTooSmall` - Output buffer insufficient
- `WasmError::CacheSlotNotFound` - Cached object evicted

**Error Code Debugging:**

Host functions return negative integers for errors. You can use trace functions to log error codes for debugging:

```rust ignore
use xrpl_wasm_stdlib::host::trace::trace_num;

let result = unsafe { some_host_function(params) };
if result < 0 {
    let _ = trace_num("Host function failed with error:", result as i64);
    return result; // or handle appropriately
}
```

---

## Examples

### Hello World

The simplest possible smart escrow that demonstrates basic concepts.

**📁 View complete example:** [`examples/smart-escrows/hello_world/`](https://github.com/ripple/xrpl-wasm-stdlib/tree/main/examples/smart-escrows/hello_world/)

**Key learning points:**

- Basic contract structure with `#![no_std]` and `#![no_main]`
- Using `#[unsafe(no_mangle)]` for the entry point function
- Simple error handling with pattern matching
- Trace logging for debugging

**Files:**

- [`src/lib.rs`](https://github.com/ripple/xrpl-wasm-stdlib/blob/main/examples/smart-escrows/hello_world/src/lib.rs) - Main contract code
- [`README.md`](https://github.com/ripple/xrpl-wasm-stdlib/blob/main/examples/smart-escrows/hello_world/README.md) - Detailed explanation
- [`runTest.js`](https://github.com/ripple/xrpl-wasm-stdlib/blob/main/examples/smart-escrows/hello_world/runTest.js) - Integration test

### Oracle Example

A price-based escrow that releases funds when an asset price meets conditions.

**📁 View complete example:** [`examples/smart-escrows/oracle/`](https://github.com/ripple/xrpl-wasm-stdlib/tree/main/examples/smart-escrows/oracle/)

**Key concepts demonstrated:**

- External data integration through oracles
- Price threshold logic
- Error handling for missing oracle data
- Real-world conditional logic

**Files:**

- [`src/lib.rs`](https://github.com/ripple/xrpl-wasm-stdlib/blob/main/examples/smart-escrows/oracle/src/lib.rs) - Oracle price checking logic
- [`README.md`](https://github.com/ripple/xrpl-wasm-stdlib/blob/main/examples/smart-escrows/oracle/README.md) - Oracle integration guide
- [`runTest.js`](https://github.com/ripple/xrpl-wasm-stdlib/blob/main/examples/smart-escrows/oracle/runTest.js) - Price simulation test

### KYC Example

A compliance-focused escrow that requires credential verification.

**📁 View complete example:** [`examples/smart-escrows/kyc/`](https://github.com/ripple/xrpl-wasm-stdlib/tree/main/examples/smart-escrows/kyc/)

**Key concepts demonstrated:**

- Credential-based verification
- Trusted issuer validation
- Signature verification
- Expiration checking
- Compliance patterns

**Files:**

- [`src/lib.rs`](https://github.com/ripple/xrpl-wasm-stdlib/blob/main/examples/smart-escrows/kyc/src/lib.rs) - KYC credential verification
- [`README.md`](https://github.com/ripple/xrpl-wasm-stdlib/blob/main/examples/smart-escrows/kyc/README.md) - Compliance implementation guide
- [`runTest.js`](https://github.com/ripple/xrpl-wasm-stdlib/blob/main/examples/smart-escrows/kyc/runTest.js) - Credential verification test

### Advanced Examples

#### Multi-Signature Notary

**📁 [`examples/smart-escrows/notary/`](https://github.com/ripple/xrpl-wasm-stdlib/tree/main/examples/smart-escrows/notary/)**

- Requires multiple signature approvals
- Implements threshold signing logic
- Demonstrates complex authorization patterns

#### NFT Ownership Verification

**📁 [`examples/smart-escrows/nft_owner/`](https://github.com/ripple/xrpl-wasm-stdlib/tree/main/examples/smart-escrows/nft_owner/)**

- Releases funds based on NFT ownership
- Shows how to query NFT ledger objects
- Demonstrates asset-based conditions

#### Time-Based Ledger Sequence

**📁 [`examples/smart-escrows/ledger_sqn/`](https://github.com/ripple/xrpl-wasm-stdlib/tree/main/examples/smart-escrows/ledger_sqn/)**

- Uses ledger sequence numbers for timing
- Implements time-locked escrows
- Shows sequence-based logic

#### Atomic Swap Examples

These examples demonstrate:

- Cross-escrow reference patterns (memo-based and data field-based)
- Account validation between related escrows
- Timing coordination and deadline management
- Atomic failure handling when escrows are consumed
- Multi-phase execution patterns

**📁 [`examples/smart-escrows/atomic_swap1/`](https://github.com/ripple/xrpl-wasm-stdlib/tree/main/examples/smart-escrows/atomic_swap1/) - Memo-Based Atomic Swap**

- Stateless atomic swap using transaction memos
- Cross-escrow validation and account reversal
- Demonstrates mutual escrow validation patterns

**📁 [`examples/smart-escrows/atomic_swap2/`](https://github.com/ripple/xrpl-wasm-stdlib/tree/main/examples/smart-escrows/atomic_swap2/) - Data Field-Based Atomic Swap**

- Stateful atomic swap using escrow data fields
- Two-phase execution with built-in timing validation
- Shows state persistence and deadline management

## Testing and Debugging

### Test Networks

| Network         | Endpoint                                 | Purpose             |
| --------------- | ---------------------------------------- | ------------------- |
| **WASM Devnet** | `wss://wasm.devnet.rippletest.net:51233` | Integration testing |
| **Local Node**  | `ws://localhost:6006`                    | Local Development   |

Follow the instructions [here](https://xrpl.org/docs/infrastructure/installation/build-on-linux-mac-windows) with [this branch](https://github.com/XRPLF/rippled/tree/ripple/se/supported) if you would like to build and run rippled locally.

### Key Testing Considerations

- Always create fresh escrows for each test to avoid consumption issues
- Use extensive trace output to debug coordination timing issues (you can access these traces by running rippled locally)
- Test both success and failure paths for atomic behavior
- Understand that escrow consumption affects subsequent tests

### Test Using the Web UI

**🌐 Open the web UI:** [https://ripple.github.io/xrpl-wasm-stdlib/ui/](https://ripple.github.io/xrpl-wasm-stdlib/ui/)

The web UI allows you to:

- Upload and test any WASM contract directly
- Configure test transactions and ledger state
- Execute contracts and see results with trace output
- Test on different networks (Devnet, Testnet)
- Debug without local setup

1. **Build your contract:**

   ```shell
   cargo build --target wasm32v1-none --release
   ```

2. **Upload your WASM file:**

- Open the testing interface in your browser
- Click "Choose File" and select your `.wasm` file from `target/wasm32v1-none/release/`
- The contract will be loaded automatically

3. **Test your contract:**

- Set up test scenarios using the interface
- Configure transaction data and ledger state
- Execute and see results with debug output

### Performance Optimization

#### Binary Size Optimization

**Cargo.toml optimizations:**

```toml
[profile.release]
opt-level = "s"           # Optimize for size over speed
lto = true               # Link-time optimization
panic = "abort"          # Remove panic handling code
codegen-units = 1        # Single codegen unit for better optimization
strip = true             # Strip debug symbols
```

**Code patterns for smaller binaries:**

```rust ignore
// Use fixed-size arrays instead of vectors
let mut buffer = [0u8; 32];  // Stack allocation

// Minimize string usage
const ERROR_MSG: &str = "Error"; // Use constants

// Efficient error handling
match operation() {
    Ok(result) => result,
    Err(_) => return 0,  // Simple error path
}
```

#### Runtime Optimization

**Minimize host function calls:**

```rust ignore
// Good: Call once, use cached result
let account = tx.get_account();
let balance = get_account_balance(&account);
// Create AccountRoot to access account fields
let account_keylet = account_keylet(&account);
let slot = cache_ledger_obj(&account_keylet);
let account_root = AccountRoot { slot_num: slot };
let sequence = account_root.sequence();

// Bad: Multiple calls for same data
let balance = get_account_balance(&tx.get_account());
// Bad: Multiple calls - should cache the account and keylet
let account_keylet = account_keylet(&tx.get_account());
let slot = cache_ledger_obj(&account_keylet);
let account_root = AccountRoot { slot_num: slot };
let sequence = account_root.sequence();
```

**Efficient ledger object access:**

```rust ignore
// Cache ledger objects for multiple field access using traits
let account = AccountID::from(*b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3");
let account_keylet = account_keylet(&account).unwrap_or_panic();
let slot = unsafe { cache_ledger_obj(account_keylet.as_ptr(), account_keylet.len(), 0) };
let account_root = AccountRoot { slot_num: slot };

// Use trait methods to access fields efficiently
let balance = account_root.balance();        // Option<Amount>
let sequence = account_root.sequence();      // u32
let owner_count = account_root.owner_count(); // u32
```

**Memory usage optimization:**

```rust ignore
// Use stack-based allocation
let mut accounts = [AccountID::default(); 10];

// Reuse buffers for transaction fields
let mut buffer = [0u8; 64];
let len1 = unsafe { get_tx_field(sfield::Account, buffer[..20].as_mut_ptr(), 20) };
let len2 = unsafe { get_tx_field(sfield::Destination, buffer[20..40].as_mut_ptr(), 20) };
```

### Troubleshooting

#### Common Build Issues

| Issue                            | Solution                                             |
| -------------------------------- | ---------------------------------------------------- |
| `wasm32v1-none` target not found | `rustup target add wasm32v1-none`                    |
| Link errors                      | Check `crate-type = ["cdylib"]` in Cargo.toml        |
| Binary too large                 | Use release profile optimizations                    |
| Missing exports                  | Ensure `#[unsafe(no_mangle)]` on `finish()` function |
| Compilation errors               | Check `#![no_std]` and avoid std library usage       |

#### Common Runtime Issues

| Issue                    | Cause                   | Solution                                    |
| ------------------------ | ----------------------- | ------------------------------------------- |
| Function not found       | WASM export missing     | Check `#[unsafe(no_mangle)]` on entry point |
| Memory access violation  | Buffer overflow         | Verify buffer sizes and bounds              |
| Cache full (NoFreeSlots) | Too many cached objects | Minimize `cache_ledger_obj` calls           |
| Field not found          | Missing ledger field    | Handle `FieldNotFound` errors               |
| Invalid field data       | Malformed field         | Validate input data                         |

#### Debugging Techniques

**Add trace statements:**

```rust
use xrpl_wasm_stdlib::host::trace::{trace, trace_data, trace_num, DataRepr};
use xrpl_wasm_stdlib::core::current_tx::escrow_finish::EscrowFinish;
use xrpl_wasm_stdlib::core::current_tx::traits::TransactionCommonFields;
use xrpl_wasm_stdlib::host::Result::{Ok, Err};

#[unsafe(no_mangle)]
pub extern "C" fn finish() -> i32 {
    trace("Contract starting").ok();

    let tx = EscrowFinish;
    let account = match tx.get_account() {
        Ok(acc) => {
            trace_data("Account", &acc.0, DataRepr::AsHex).ok();
            acc
        },
        Err(e) => {
            trace_num("Error getting account: {:?}", e as i64).ok();
            return 0;
        }
    };

    // More logic with tracing...
    1 // Return 1 to complete the function
}
```

**Inspect WASM binary:**

```shell
# Detailed binary analysis
wasm-objdump -x target/wasm32v1-none/release/my_escrow.wasm

# Size analysis
wasm-objdump -h target/wasm32v1-none/release/my_escrow.wasm
```

---

## Additional Resources

The XRPL WebAssembly Standard Library is designed to make smart escrow development accessible while maintaining the security and determinism required for production use on the XRPL network.

For additional help:

- Review the examples in `examples/smart-escrows/`
- Check the API documentation generated by `cargo doc`
- Join the XRPL developer community
- Submit issues or questions on GitHub

## Contributing

If you're interested in contributing to the XRPL WebAssembly Standard Library, please see our [CONTRIBUTING.md](https://github.com/ripple/xrpl-wasm-stdlib/blob/main/CONTRIBUTING.md) for detailed guidelines on:

- Development setup and workflow
- Code standards and style guidelines
- Pull request process
- Testing requirements
- Release procedures

We welcome contributions of all kinds, from bug fixes and documentation improvements to new examples and library features!
