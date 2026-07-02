---
paths:
  - "examples/smart-escrows/**/*.rs"
  - "e2e-tests/**/*.rs"
---

# WASM Smart Contract Rules

## Critical Requirements

### 1. File Attributes

Every WASM contract MUST start with:

```rust
#![no_std]
#![no_main]
```

### 2. Entry Point

Every contract MUST have exactly one entry point:

```rust
#[unsafe(no_mangle)]
pub extern "C" fn finish() -> i32 {
    // Return 1 for success, 0 for failure
}
```

### 3. NO Heap Allocations

**NEVER use these types:**

- `Vec<T>`
- `String`
- `Box<T>`
- `HashMap<K, V>`
- `BTreeMap<K, V>`
- Any type that allocates on the heap

**ALWAYS use:**

- Fixed-size arrays: `[0u8; 32]`
- Stack-allocated data structures
- Buffer sizes from `core::constants`

### 4. Error Handling

**NEVER use** `std::result::Result`

**ALWAYS use** the library's custom Result type:

```rust
use xrpl_wasm_stdlib::host::{Result, Error};
use xrpl_wasm_stdlib::host::Result::{Ok, Err};
```

### 5. Build Target

**ALWAYS build with:**

```bash
cargo build --target wasm32v1-none --release
```

### 6. Cargo.toml Configuration

```toml
[lib]
crate-type = ["cdylib"]

[profile.release]
opt-level = "s"      # Optimize for size
lto = true           # Link-time optimization
codegen-units = 1    # Better optimization
panic = "abort"      # Required for no_std
```

## Common Patterns

### Reading Transaction Fields

```rust
use xrpl_wasm_stdlib::core::current_tx::escrow_finish::EscrowFinish;
use xrpl_wasm_stdlib::core::current_tx::traits::TransactionCommonFields;

let account = match EscrowFinish.get_account() {
    Ok(acc) => acc,
    Err(_) => return 0,
};
```

### Debugging with Trace

```rust
use xrpl_wasm_stdlib::host::trace::{trace, trace_data, DataRepr};

trace("Contract starting").ok();
trace_data("Account", &account.as_bytes(), DataRepr::AsHex).ok();
```

### Reading Ledger Objects

```rust
use xrpl_wasm_stdlib::core::ledger_objects::account_root::AccountRoot;
use xrpl_wasm_stdlib::core::keylets::account_keylet;
use xrpl_wasm_stdlib::host::cache_ledger_obj;

let keylet = account_keylet(&account_id);
match cache_ledger_obj(&keylet) {
    Ok(slot) => {
        let balance = AccountRoot(slot).get_balance()?;
        // Use balance
    }
    Err(_) => return 0,
}
```

## Size Optimization

Target: < 64KB for WASM binary

**Techniques:**

- Use `opt-level = "s"` in release profile
- Enable LTO: `lto = true`
- Single codegen unit: `codegen-units = 1`
- Minimize dependencies
- Use constants instead of string literals
- Avoid complex generic types

## Testing

Every new contract MUST have:

1. Unit tests (if applicable)
2. Integration test (`runTest.js`)
3. Comprehensive README

Test with:

```bash
./scripts/run-tests.sh examples/smart-escrows/[name]
```
