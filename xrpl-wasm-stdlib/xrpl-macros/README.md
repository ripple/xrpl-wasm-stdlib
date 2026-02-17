# xrpl-macros

⚠️ **INTERNAL CRATE - DO NOT USE DIRECTLY** ⚠️

This is an internal procedural macro crate for `xrpl-wasm-stdlib`.

**Users should add `xrpl-wasm-stdlib` to their dependencies, NOT this crate.**

Due to Rust's requirement that procedural macros must be in a separate crate, this is published
alongside `xrpl-wasm-stdlib` but is not intended for direct use. The macro is re-exported from
`xrpl-wasm-stdlib` for your convenience.

## For Users

Add the [xrpl-wasm-stdlib crate](https://crates.io/crates/xrpl-wasm-stdlib) to your `Cargo.toml`.

Then use the macro:

```rust
use xrpl_wasm_stdlib::r_address;

const ACCOUNT: [u8; 20] = r_address!("rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh");
```

---

## About This Crate

A compile-time macro for converting XRPL classic addresses (r-addresses) to 20-byte arrays.

## Features

- **Zero runtime overhead**: Address decoding happens at compile time
- **Type safe**: Invalid addresses cause compilation errors
- **No binary bloat**: The final WASM contains only the raw 20-byte array, no decoding logic
- **no-std compatible**: The macro runs at compile time on the host, so its dependencies never affect the target
  environment

## no-std Compatibility

**Important**: Procedural macros run at compile time on your development machine, NOT in the target environment. This
means:

- The macro's dependencies (`bs58`, `sha2`, `syn`, `quote`) run during compilation only
- These dependencies are NEVER included in your final WASM binary
- The `xrpl-wasm-stdlib` library remains fully `no-std` compatible
- The macro only outputs a simple `[u8; 20]` array literal in your code

For example, this code:

```rust
const ACCOUNT: [u8; 20] = r_address!("rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh");
```

Gets expanded at compile time to:

```rust
const ACCOUNT: [u8; 20] = [132, 45, 67, 89, ...]; // actual 20 bytes
```

No runtime code from the macro or its dependencies exists in the final binary.

## Usage

```rust
use xrpl_wasm_stdlib::r_address;

// Convert r-address to [u8; 20] at compile time
const ACCOUNT: [u8; 20] = r_address!("rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh");

// Multiple accounts can be defined
const NOTARY: [u8; 20] = r_address!("rN7n7otQDd6FczFgLdSqtcsAUxDkw6fzRH");
const ADMIN: [u8; 20] = r_address!("rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn");
```

## Why Use This Macro?

This macro provides a clean, compile-time solution for embedding XRPL addresses in smart contracts:

- **Simple**: Just use `r_address!("r...")` directly in your code
- **Safe**: Invalid addresses are caught at compile time
- **Efficient**: No runtime overhead or extra dependencies in the final WASM
