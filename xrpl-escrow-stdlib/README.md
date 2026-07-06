# xrpl-escrow-stdlib

Smart Escrow entry-point context and host-function wrappers for writing XRPL Smart Escrows in Rust.

This crate is part of the `xrpl-wasm-stdlib` workspace. It provides [`EscrowFinishContext`], the
control surface a Smart Escrow author interacts with, along with safe, scoped access to
escrow-unique host functions (e.g., `update_data`). All unsafe FFI is contained here; user code
stays fully safe.

## Usage

```rust,ignore
use xrpl_escrow_stdlib::*;
use xrpl_wasm_stdlib::core::ledger_objects::traits::CurrentEscrowFields;

#[smart_escrow]
fn run(ctx: EscrowFinishContext) -> FinishResult {
    let destination = match ctx.escrow().get_destination() {
        Ok(d) => d,
        Err(e) => return e.code().into(),
    };
    // ... evaluate conditions ...
    FinishResult::succeed()
}
```

The `#[smart_escrow]` entry-point macro (in `xrpl-macros`, re-exported here) constructs the context via
`EscrowFinishContext::default()` and passes it to your function automatically, then converts your `FinishResult`
(or `i32`) into the `extern "C" fn finish() -> i32` the XRPL host calls.

## Crate layout

| Module               | Contents                                                   |
| -------------------- | ---------------------------------------------------------- |
| `ctx::escrow_finish` | `EscrowFinishContext` struct and its host-function methods |

## `no_std`

This crate is `no_std` when targeting `wasm32`. The `std` crate is available for host (non-WASM)
builds so unit tests run normally.
