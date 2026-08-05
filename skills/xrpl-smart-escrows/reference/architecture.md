# Architecture

## Crate boundaries

A Smart Escrow project (whether inside `xrpl-wasm-stdlib`'s own `examples/` or a standalone contract repo) depends on three path/crates-io crates, in a strict dependency direction:

```
xrpl-escrow-stdlib → xrpl-common-stdlib → xrpl-macros
```

- **`xrpl-macros`** — proc-macro crate. Exports the typed-constant macros (`r_address!`, `hash256!`, `pubkey!`, `currency!`, `blob!`) and the entry-point macros (`#[smart_escrow]`, `#[smart_contract]`). No runtime dependency on the other two crates.
- **`xrpl-common-stdlib`** — general-purpose layer: host bindings, transaction/ledger-object field access, ledger entry IDs, core types (`AccountID`, `Amount`, `Number`, `Hash256`, `Blob<N>`, ...). Contains no feature-specific (escrow-only) logic.
- **`xrpl-escrow-stdlib`** — Smart-Escrow-specific entry-point context (`EscrowFinishContext`, `FinishResult`), the `CurrentEscrow`/`Escrow` ledger-object wrappers, escrow persistent-storage helpers, and escrow-only host calls (`set_data`). Re-exports `xrpl_common_stdlib::*`, so a contract typically only needs to depend on `xrpl-escrow-stdlib` directly plus `xrpl-macros` for constants.

> Historical note: earlier revisions of this library named the common crate `xrpl-wasm-stdlib`; it has since been split into `xrpl-common-stdlib` (generic) + `xrpl-escrow-stdlib` (escrow-specific). If you see `xrpl-wasm-stdlib` in older docs or blog posts, read it as `xrpl-common-stdlib`.

## The three-implementation host-binding swap

Contract code never sees this directly, but it explains why the library builds and tests natively despite being `no_std` on WASM. `HostBindings` (a trait covering every host function: `cache_le`, `tx_field`, `trace`, etc.) has three implementations selected by `cfg`:

| Target                                       | Implementation                                                                          |
| -------------------------------------------- | --------------------------------------------------------------------------------------- |
| `wasm32`                                     | Real `extern "C"` FFI declarations — used in production WASM builds                     |
| Native + `test`/`test-host-bindings` feature | `mockall`-generated mocks — lets `cargo test` exercise stdlib logic without a real host |
| Native, plain build                          | No-op stubs — let `cargo check`/IDE tooling work; panic if actually called              |

Consequence for contract authors: your contract crate is `no_std` only under `#[cfg_attr(target_arch = "wasm32", no_std)]`, with an `extern crate std;` shim for non-wasm builds — this lets `cargo test` run natively while `cargo build --target wasm32v1-none --release` produces the real deployable artifact.

## WASM build profile

Both the root stdlib workspace and any downstream contract's `Cargo.toml` should set:

```toml
[profile.release]
opt-level = "s"     # optimize for size
lto = true
codegen-units = 1
panic = "abort"      # no_std can't unwind; avoids pulling in a panic handler
strip = true
```

The stdlib defines a `#[panic_handler]` for `target_arch = "wasm32"` that calls `wasm32::unreachable()`. The dev profile uses `panic = "unwind"` so unit tests can run on the host target.

## Why determinism and read-only access matter

Smart Escrows execute identically on every validator during consensus. That rules out:

- Heap allocation (no allocator is wired up under `no_std`; use fixed-size stack arrays / `Blob<N>`)
- Any host clock, randomness, network, or filesystem access
- Writing to ledger state other than the escrow's own `Data` field (via `ctx.set_data()`)

Time- and sequence-based logic must go through host-provided, ledger-derived values (`parent_ldgr_time`, `ldgr_index`) rather than anything computed locally, since those are the only values every validator agrees on.

## Contract crate layout

A minimal contract crate:

```
my-escrow/
├── Cargo.toml       # crate-type = ["cdylib"], path deps on the three crates above
├── src/
│   └── lib.rs       # #![cfg_attr(target_arch = "wasm32", no_std)] + #[smart_escrow] fn
└── runTest.js        # integration test, see reference/testing.md
```

If developing inside the `xrpl-wasm-stdlib` repo itself, new example contracts go under `examples/smart-escrows/<name>/` and must be added to `examples/Cargo.toml`'s `[workspace] members`.
