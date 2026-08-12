# xrpl-escrow-stdlib

Smart Escrow types, entry-point context, and host-function wrappers for writing XRPL Smart Escrows
in Rust.

Generic XRPL primitives (`AccountID`,
`Locator`, host bindings, trace, etc.) live in [`xrpl_common_stdlib`]; this crate hosts only what is
tied specifically to escrows: the `EscrowFinish` transaction wrapper, the `Escrow`/`CurrentEscrow`
ledger objects, the escrow-specific field-accessor traits, and [`EscrowFinishContext`] — the control
surface a Smart Escrow author interacts with. Safe, scoped access to escrow-unique host functions
(e.g., `set_data`) is exposed as inherent methods; all unsafe FFI is contained here, so user code
stays fully safe.

## Usage

```rust,ignore
use xrpl_escrow_stdlib::ledger_objects::traits::CurrentEscrowFields;
use xrpl_escrow_stdlib::{EscrowFinishContext, FinishResult};
use xrpl_macros::smart_escrow;

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

The `#[smart_escrow]` entry-point macro lives in `xrpl-macros` and is re-exported from
`xrpl-common-stdlib` — this crate does not re-export it, so import it from either of those. It
constructs the context via `EscrowFinishContext::default()` and passes it to your function
automatically, then converts your `FinishResult` (or `i32`) into the
`extern "C" fn escrow_finish() -> i32` the XRPL host calls.

A Smart Escrow depends on two crates, versioned in lockstep:

```toml
[dependencies]
xrpl-common-stdlib = "0.9"
xrpl-escrow-stdlib = "0.9"
```

## Crate layout

| Module               | Contents                                                              |
| -------------------- | --------------------------------------------------------------------- |
| `ctx::escrow_finish` | `EscrowFinishContext` struct and its host-function methods            |
| `current_tx`         | `EscrowFinish` transaction wrapper and the `EscrowFinishFields` trait |
| `ledger_objects`     | `Escrow`/`CurrentEscrow` objects and their field-accessor traits      |

## `no_std`

This crate is `no_std` when targeting `wasm32`. The `std` crate is available for host (non-WASM)
builds so unit tests run normally.
