# xrpl-escrow-stdlib

Smart Escrow entry-point context and host-function wrappers for writing XRPL Smart Escrows in Rust.

This crate is part of the `xrpl-wasm-stdlib` workspace. It provides [`EscrowFinishContext`], the
single entry point a Smart Escrow author interacts with, along with safe wrappers around the
escrow-unique host functions (e.g., `update_data`). All unsafe FFI is contained here; user code
stays fully safe.

## Usage

```rust,ignore
use xrpl_escrow_stdlib::*;

fn run(ctx: EscrowFinishContext) -> Result<bool, ()> {
    let destination = ctx.escrow().fetch_destination()?;
    // ... evaluate conditions ...
    Ok(true)
}
```

The `#[smart_escrow]` entry-point macro (in `xrpl-macros`) constructs the context via
`EscrowFinishContext::default()` and passes it to your function automatically.

## Crate layout

| Module               | Contents                                                   |
| -------------------- | ---------------------------------------------------------- |
| `ctx::escrow_finish` | `EscrowFinishContext` struct and its host-function methods |

## `no_std`

This crate is `no_std` when targeting `wasm32`. The `std` crate is available for host (non-WASM)
builds so unit tests run normally.
