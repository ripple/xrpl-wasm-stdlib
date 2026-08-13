# xrpl-macros

⚠️ **INTERNAL CRATE - DO NOT USE DIRECTLY** ⚠️

This is an internal procedural macro crate for `xrpl-common-stdlib`.

**Users should add `xrpl-common-stdlib` to their dependencies, NOT this crate.**

Rust requires procedural macros to live in their own crate, so this is published alongside
`xrpl-common-stdlib` but is not intended for direct use. Every macro here is re-exported from
`xrpl-common-stdlib` for your convenience, and the code these macros generate refers to
`xrpl_common_stdlib` paths — so this crate cannot be used on its own regardless.

It provides the compile-time typed constants (`r_address!`, `hash256!`, `pubkey!`, `currency!`,
`blob!`) and the entry-point attributes (`#[smart_escrow]`, `#[smart_contract]`).

## For Users

Add the [xrpl-common-stdlib crate](https://crates.io/crates/xrpl-common-stdlib) to your `Cargo.toml`.

Then use the macro:

```rust
use xrpl_common_stdlib::r_address;
use xrpl_common_stdlib::types::account_id::AccountID;

const ACCOUNT: AccountID = r_address!("rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh");
```

This crate also provides the `smart_escrow` and `smart_contract` entry-point attribute macros, both
re-exported from `xrpl-common-stdlib` as well. `smart_escrow` generates code referencing
`EscrowFinishContext` and `FinishResult`, so a contract using it must also depend on
[`xrpl-escrow-stdlib`](https://github.com/ripple/xrpl-wasm-stdlib/tree/main/xrpl-escrow-stdlib) —
see that crate's README for usage.

---

## About This Crate

A compile-time macro for converting XRPL classic addresses (r-addresses) to typed `AccountID` values.

## Features

- **Zero runtime overhead**: Address decoding happens at compile time
- **Type safe**: Invalid addresses cause compilation errors; the result is a typed `AccountID`
- **No binary bloat**: The final WASM contains only the raw 20-byte `AccountID`, no decoding logic
- **no-std compatible**: The macro runs at compile time on the host, so its dependencies never affect the target
  environment

## no-std Compatibility

**Important**: Procedural macros run at compile time on your development machine, NOT in the target environment. This
means:

- The macro's dependencies (`bs58`, `sha2`, `syn`, `quote`) run during compilation only
- These dependencies are NEVER included in your final WASM binary
- The `xrpl-common-stdlib` library remains fully `no-std` compatible
- The macro only outputs an `AccountID(...)` literal containing the 20 decoded bytes

For example, this code:

```rust
const ACCOUNT: AccountID = r_address!("rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh");
```

Gets expanded at compile time to:

```rust
const ACCOUNT: AccountID = AccountID([132, 45, 67, 89, ...]); // actual 20 bytes
```

No runtime code from the macro or its dependencies exists in the final binary.

## Usage

```rust
use xrpl_common_stdlib::r_address;
use xrpl_common_stdlib::types::account_id::AccountID;

// Convert r-address to AccountID at compile time
const ACCOUNT: AccountID = r_address!("rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh");

// Multiple accounts can be defined
const NOTARY: AccountID = r_address!("rN7n7otQDd6FczFgLdSqtcsAUxDkw6fzRH");
const ADMIN: AccountID = r_address!("rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn");
```

## Why Use This Macro?

This macro provides a clean, compile-time solution for embedding XRPL addresses in smart contracts:

- **Simple**: Just use `r_address!("r...")` directly in your code
- **Safe**: Invalid addresses are caught at compile time
- **Efficient**: No runtime overhead or extra dependencies in the final WASM
