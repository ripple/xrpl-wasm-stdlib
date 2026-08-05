---
name: xrpl-smart-escrows
description: Build, test, and debug XRPL Smart Escrow contracts written in Rust against xrpl-wasm-stdlib (crates xrpl-common-stdlib, xrpl-escrow-stdlib, xrpl-macros), compiled to wasm32v1-none. Use when the user wants to write a smart escrow / EscrowFinish WASM contract, asks about the XRPL WASM host functions, ledger entry IDs, ledger object access, or mentions xrpl-wasm-stdlib, EscrowFinishContext, FinishResult, or #[smart_escrow].
---

# XRPL Smart Escrows

Smart Escrows are Rust `no_std` WASM modules that gate whether an `EscrowFinish` transaction succeeds. The host calls a generated `finish() -> i32` export: positive releases the escrow, zero/negative keeps it locked (negative is a host error code).

## Quick start

```rust
#![cfg_attr(target_arch = "wasm32", no_std)]
#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_common_stdlib::host::trace::trace;
use xrpl_escrow_stdlib::{EscrowFinishContext, FinishResult};
use xrpl_macros::smart_escrow;

#[smart_escrow]
fn run(_ctx: EscrowFinishContext) -> FinishResult {
    let _ = trace("Hello World");
    FinishResult::succeed()
}
```

`Cargo.toml` needs `crate-type = ["cdylib"]` and path deps on `xrpl-common-stdlib`, `xrpl-escrow-stdlib`, `xrpl-macros`. The function name must not be `finish` (that's the generated export symbol) and must take exactly `EscrowFinishContext`, returning `FinishResult` or `i32`.

## Core workflow

1. **Decide what the escrow checks** — an account identity (see notary pattern), a price/oracle value, a credential/NFT the destination holds, a ledger sequence/time deadline, or multi-step state across several `EscrowFinish` calls. See [reference/patterns.md](reference/patterns.md) for a worked example of each.
2. **Read whatever data the check needs**:
   - Current tx fields → `ctx.tx()` (`TransactionCommonFields` + `EscrowFinishFields`)
   - The escrow being finished → `ctx.escrow()` (`CurrentEscrowFields`)
   - Any other ledger object → ledger entry ID → `cache_le` → typed wrapper or `LedgerObject`
   - See [reference/api-surface.md](reference/api-surface.md) for the full method/type inventory.
3. **Return `FinishResult::succeed()` / `reject()`** (or `.succeed_with::<N>()` / `.reject_with::<N>()` to pass a custom code).
4. **Build and test** per [reference/testing.md](reference/testing.md) — native `cargo test`, wasm build via `wasm32v1-none`, and a `runTest.js` integration test against a rippled node.

## Key constraints (always true)

- `no_std`, no heap allocation, no network/filesystem access, read-only ledger access except the escrow's own `Data` field (via `ctx.update_data()` / `escrow_storage::save_data`).
- Execution must be deterministic — no wall-clock time, no randomness; use `parent_ledger_time`/`ledger_sqn` for time/sequence, not host-side clocks.
- Compare token amounts via the `Amount`/`Number`/`IOUNumber` types (host-delegated decimal math), never raw floats.
- Minimize host calls (`cache_le`, ledger entry ID computation, field reads) — cache results instead of repeating identical calls; `NoFreeSlots` and execution budget (`Gas`) are real limits.
- Debug via `trace`/`trace_num`/`trace_data`/`trace_acct`/`trace_amt` — output lands in rippled's `debug.log`. Convention: on every error path, `trace_num("<context>", e.code() as i64)` before returning.

## Reference

- [reference/architecture.md](reference/architecture.md) — crate boundaries, host-binding pattern, WASM build profile, why things are `no_std`
- [reference/api-surface.md](reference/api-surface.md) — every public type/trait/macro a contract author calls, with signatures
- [reference/patterns.md](reference/patterns.md) — full worked examples: identity check, oracle price, credential/NFT gate, time/sequence deadline, multi-party state machine, cross-escrow atomic swap
- [reference/testing.md](reference/testing.md) — native unit tests, wasm build, `runTest.js` integration harness, manual UI testing, troubleshooting table
