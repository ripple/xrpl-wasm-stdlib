# xrpl-escrow-stdlib

Smart Escrow types, entry-point context, and host-function wrappers for writing XRPL Smart Escrows
in Rust.

This crate is part of the `xrpl-common-stdlib` workspace. Generic XRPL primitives (`AccountID`,
`Locator`, host bindings, trace, etc.) live in [`xrpl_common_stdlib`]; this crate hosts only what is
tied specifically to escrows: the `EscrowFinish` transaction wrapper, the `Escrow`/`CurrentEscrow`
ledger objects, the escrow-specific field-accessor traits, and [`EscrowFinishContext`] — the control
surface a Smart Escrow author interacts with. Safe, scoped access to escrow-unique host functions
(e.g., `update_data`) is exposed as inherent methods; all unsafe FFI is contained here, so user code
stays fully safe.

## Usage

```rust,ignore
use xrpl_escrow_stdlib::*;
use xrpl_escrow_stdlib::ledger_objects::traits::CurrentEscrowFields;

fn run(ctx: EscrowFinishContext) -> FinishResult {
    let destination = match ctx.escrow().get_destination() {
        host::Result::Ok(d) => d,
        host::Result::Err(e) => return FinishResult::from(e.code()),
    };
    // ... evaluate conditions ...
    FinishResult::succeed()
}
```

The `#[smart_escrow]` entry-point macro (in `xrpl-macros`) constructs the context via
`EscrowFinishContext::default()` and passes it to your function automatically.

## Crate layout

| Module               | Contents                                                              |
| -------------------- | --------------------------------------------------------------------- |
| `ctx::escrow_finish` | `EscrowFinishContext` struct and its host-function methods            |
| `current_tx`         | `EscrowFinish` transaction wrapper and the `EscrowFinishFields` trait |
| `ledger_objects`     | `Escrow`/`CurrentEscrow` objects and their field-accessor traits      |

## `no_std`

This crate is `no_std` when targeting `wasm32`. The `std` crate is available for host (non-WASM)
builds so unit tests run normally.
