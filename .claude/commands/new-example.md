---
description: Create a new smart escrow example with proper structure
argument-hint: [example-name]
---

## Creating New Smart Escrow Example: $ARGUMENTS

I'll create a new smart escrow example with the proper structure:

1. **Create directory structure**
   - `examples/smart-escrows/$ARGUMENTS/`
   - `examples/smart-escrows/$ARGUMENTS/src/`

2. **Required files to create:**
   - `Cargo.toml` - Package configuration with WASM settings
   - `src/lib.rs` - Contract implementation with `#![no_std]` and `#![no_main]`
   - `README.md` - Comprehensive documentation
   - `runTest.js` - Integration test

3. **Cargo.toml template:**

```toml
[package]
name = "$ARGUMENTS"
version = "0.1.0"
edition = "2021"
description = "Smart escrow example: [DESCRIPTION]"
license = "ISC"

[lib]
crate-type = ["cdylib"]

[dependencies]
xrpl-wasm-stdlib = { path = "../../../xrpl-wasm-stdlib" }

[profile.release]
opt-level = "s"
lto = true
codegen-units = 1
panic = "abort"
```

4. **src/lib.rs template:**

```rust
#![no_std]
#![no_main]

use xrpl_wasm_stdlib::host::{Result, Error};
use xrpl_wasm_stdlib::host::Result::{Ok, Err};

#[unsafe(no_mangle)]
pub extern "C" fn finish() -> i32 {
    // Contract logic here
    1 // Success
}
```

5. **Next steps:**
   - Add to `examples/Cargo.toml` workspace members
   - Implement contract logic
   - Write comprehensive README
   - Create integration test
   - Test with `./scripts/run-tests.sh examples/smart-escrows/$ARGUMENTS`
   - Add to main README examples list

What functionality should this smart escrow implement?
