# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

**Keep this file current.** After completing a task, consider whether it changed something this file documents (workspace members, crate boundaries, scripts, architecture) and update the relevant section if so.

## What this repo is

A Rust `no_std` standard library that lets developers write XRPL smart contracts (currently "Smart Escrows") compiled to WebAssembly. The library wraps a low-level host ABI exposed by `rippled` and offers type-safe accessors for transaction fields, ledger objects, keylets, and serialized fields.

Smart escrow WASM modules export `extern "C" fn finish() -> i32`. Returning a positive value finishes the escrow, `0` rejects it, and a negative value is a host error code.

## Three Cargo workspaces (intentional, do not merge)

| Workspace | Path                   | Members                                                 |
| --------- | ---------------------- | ------------------------------------------------------- |
| Library   | `/Cargo.toml` (root)   | `xrpl-wasm-stdlib`, `xrpl-macros`, `xrpl-escrow-stdlib` |
| Examples  | `examples/Cargo.toml`  | all `examples/smart-escrows/*` cdylibs                  |
| E2E tests | `e2e-tests/Cargo.toml` | host-function probe contracts + native test crates      |

The root workspace explicitly `exclude`s `examples` and `e2e-tests` because they target `wasm32v1-none` with `crate-type = ["cdylib"]`. Build/clippy scripts `cd` into each workspace separately — if you add a new top-level workspace, mirror that in `scripts/build.sh` and `scripts/clippy.sh`.

## Common commands

All scripts assume you have run `./scripts/setup.sh` once. They mirror the GitHub Actions workflow in `.github/workflows/test.yml` and set `RUSTFLAGS="-Dwarnings"`.

```shell
# Full CI suite locally (clippy, fmt, host-function audit, wasm-exports check, build+test, markdown, e2e)
./scripts/run-all.sh

# Build everything (native + wasm32v1-none for both examples/ and e2e-tests/, debug + release)
./scripts/build.sh
./scripts/build.sh release          # release-only

# Native unit tests across the library workspace
./scripts/build-and-test.sh         # builds wasm + runs `cargo test --workspace`
cargo test --workspace              # just the unit tests (root workspace)

# Single unit test
cargo test --workspace <test_name>
cargo test -p xrpl-wasm-stdlib <test_name>
cargo test -p xrpl-escrow-stdlib <test_name>

# Clippy / fmt across all three workspaces
./scripts/clippy.sh
./scripts/fmt.sh

# Integration tests (requires a rippled node — local on ws://localhost:6006 by default)
./scripts/run-tests.sh                                     # all examples + e2e contracts that have runTest.js
./scripts/run-tests.sh examples/smart-escrows/hello_world  # single example
DEVNET=true ./scripts/run-tests.sh                         # run against wss://wasm.devnet.rippletest.net:51233

# Coverage (uses test-host-bindings feature; requires `cargo install cargo-llvm-cov`)
./scripts/coverage.sh

# Regenerate src/sfield.rs from rippled (requires Node.js)
./scripts/generate-sfields.sh

# Regenerate src/tx_flags.rs (tf*/asf*/tmf* constants) from rippled (requires Node.js)
./scripts/generate-tx-flags.sh
```

Pre-commit hooks (`.pre-commit-config.yaml`) run `cargo fmt --all` and `cargo clippy --all-targets --all-features -- -Dclippy::all` on staged Rust files, plus prettier with `--no-semi --tab-width 2` for JS/MD/YAML.

## Toolchain pinning

`rust-toolchain.toml` pins **Rust 1.89.0** with `rustfmt`, `clippy`, and the `wasm32v1-none` target. The library uses **edition 2024**. Do not bump these casually — the WASM target and edition affect both the library and every example.

## Architecture: crate ownership (`xrpl-wasm-stdlib` vs `xrpl-escrow-stdlib` vs `xrpl-macros`)

The library workspace is split into three crates with a strict dependency direction: `xrpl-escrow-stdlib` → `xrpl-wasm-stdlib` → `xrpl-macros`. Never invert this — `xrpl-wasm-stdlib` must not depend on domain (feature-specific) code.

- **`xrpl-macros`** — proc-macro crate, no runtime dependencies on the other two. Exports:
  - Typed-constant macros: `r_address!`, `hash256!`, `pubkey!`, `currency!`, `blob!` — validate at compile time and emit a typed XRPL value.
  - Entry-point macros: `#[smart_escrow]`, `#[smart_contract]` — wrap a user function in the `extern "C"` symbol the XRPL host calls. Both share a `parse → validate → codegen` pipeline in `entry_point/`; adding a third entry-point macro means adding a new orchestrator file there plus a new `#[proc_macro_attribute]` shim in `lib.rs`.
- **`xrpl-wasm-stdlib`** — the general-purpose layer: host bindings, transaction/ledger-object field access, keylets, types. Contains no feature-specific (e.g. escrow-only) logic.
- **`xrpl-escrow-stdlib`** — Smart Escrow-specific entry-point context (`EscrowFinishContext`, `FinishResult`) and escrow-unique host functions (e.g. `update_data`). Re-exports `xrpl_wasm_stdlib::*`, so contract code typically only needs to depend on `xrpl-escrow-stdlib`.

**Rule of thumb:** domain-specific code (escrow, and any future smart-contract feature) lives in its own crate and is never added to `xrpl-wasm-stdlib` with a re-export. `xrpl-wasm-stdlib::ctx::SmartFeatureContext` is the narrow, generic trait (`type Tx: TransactionCommonFields`, `fn tx(&self) -> &Self::Tx`) that feature-specific contexts like `EscrowFinishContext` implement — new features add a new context type/crate rather than extending this trait.

## Architecture: the three-implementation host-binding swap

This is the single most important pattern in the repo. `xrpl-wasm-stdlib/src/host/mod.rs` selects one of three implementations of the same `HostBindings` trait (defined in `host_bindings_trait.rs`) via `cfg`-gated `include!`:

| Config                                                       | Included file            | Purpose                                                                                        |
| ------------------------------------------------------------ | ------------------------ | ---------------------------------------------------------------------------------------------- |
| `cfg(target_arch = "wasm32")`                                | `host_bindings_wasm.rs`  | Real FFI `extern "C"` declarations — used in production WASM builds.                           |
| `cfg(any(test, feature = "test-host-bindings"))` on non-WASM | `host_bindings_test.rs`  | `mockall`-generated mocks — lets unit/coverage tests on the native target stub host functions. |
| Plain `cargo build` on non-WASM                              | `host_bindings_empty.rs` | No-op stubs that just allow native builds to compile (functions panic if called).              |

Consequences:

- `lib.rs` uses `#![cfg_attr(target_arch = "wasm32", no_std)]` — code is `no_std` only when targeting WASM; native builds get `std` so `cargo test` works. This applies to both `xrpl-wasm-stdlib` and `xrpl-escrow-stdlib`.
- To exercise stdlib code from another crate's tests (e.g. `e2e-tests/`, `xrpl-escrow-stdlib`), enable the `test-host-bindings` feature on `xrpl-wasm-stdlib` — `dev-dependencies` aren't enough because mockall must be available when the lib is consumed as a regular dep.
- Anything new added to `HostBindings` must be implemented in all three files. CI's `host-function-audit.sh` compares the trait against rippled's exports — keep them in sync.

## Architecture: layering inside `xrpl-wasm-stdlib`

```
src/
├── lib.rs            # no_std toggle, panic_handler (wasm only), hex decode helpers, re-exports the xrpl-macros constant macros
├── ctx/               # SmartFeatureContext trait — narrow contract shared by all feature-specific entry-point contexts
├── fields/            # Field decoding traits/helpers shared across XRPL field types
├── host/              # Low-level layer: HostBindings trait + 3 impls, error codes, trace, field_helpers
├── core/              # High-level safe API — what contract authors should call
│   ├── current_tx/    # EscrowFinish marker + traits → typed access to the current TX's fields
│   ├── ledger_objects/  # Cached ledger entry access (Escrow, AccountRoot, etc.) + CurrentEscrow helper
│   ├── keylets.rs     # Compute keylets (escrow_keylet, oracle_keylet, credential_keylet, ...)
│   ├── locator.rs     # Builds nested-field locator paths for `get_*_nested_field`
│   ├── types/         # AccountID, Amount, Hash{128,160,192,256}, Blob, NFT, OpaqueFloat, etc.
│   └── constants.rs
├── sfield.rs          # GENERATED — type-safe SField<T, CODE> constants. Do not hand-edit; rerun generate-sfields.sh
├── tx_flags.rs        # GENERATED, pub(crate) — transaction flag constants (tf*/asf*/tmf*). Do not hand-edit; rerun generate-tx-flags.sh
└── types.rs           # Top-level type re-exports
```

`SField<T, CODE>` encodes the field's Rust type as a const-generic phantom, so `current_tx::get_field(sfield::Account)` infers `AccountID`, `ledger_object::get_field(slot, sfield::Balance)` infers `Amount`, etc. Adding a new field means regenerating `sfield.rs` (see `tools/generateSFields.js` for custom type overrides like `TransactionType`, `ConditionBlob`, `FulfillmentBlob`).

`tx_flags.rs` is merged from two rippled branches (see `tools/generateTxFlags.js`): a **base branch** (authoritative) plus a **contract branch** that only adds flags for new transaction types the base branch lacks (never redefining a base flag, so the merge is purely additive). Only individual flags are emitted — rippled's validity masks (`tf*Mask`) are intentionally omitted, since contracts check individual flags rather than validate flag combinations. The constants are `pub(crate)` — crate-internal backing behind a typed flags API, not a public surface.

`xrpl-escrow-stdlib/src/ctx/escrow_finish.rs` shows the pattern for a feature context: a struct holding a `current_tx` marker type (`EscrowFinish`) plus a ledger-object helper (`CurrentEscrow`), implementing `SmartFeatureContext`, with feature-unique host calls as inherent methods (all `unsafe` FFI stays inside the context type — user contract code stays fully safe).

## WASM build profile (matters for size and panic behavior)

Both the root and `examples/` `Cargo.toml` set the same release profile:

```toml
opt-level = "s"     # size
lto = true
codegen-units = 1
panic = "abort"     # no_std can't unwind; also avoids pulling in a panic handler
```

The library defines a custom `#[panic_handler]` for `target_arch = "wasm32"` that calls `wasm32::unreachable()`. Dev profile uses `panic = "unwind"` so unit tests can run on the host.

## Writing a contract

Minimal template (see `examples/smart-escrows/hello_world/src/lib.rs`):

```rust
#![cfg_attr(target_arch = "wasm32", no_std)]
#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_escrow_stdlib::{EscrowFinishContext, FinishResult, smart_escrow};
use xrpl_wasm_stdlib::host::trace::trace;

#[smart_escrow]
fn run(_ctx: EscrowFinishContext) -> FinishResult {
    let _ = trace("Hello World");
    FinishResult::succeed()
}
```

The `Cargo.toml` must set `crate-type = ["cdylib"]` and depend on `xrpl-wasm-stdlib` via path. New examples must be added to `examples/Cargo.toml`'s `[workspace] members`.

Trace output (`trace`, `trace_data`, `trace_num`) shows up in rippled's `debug.log`.

## Integration test pattern

Each example has a `runTest.js` next to its `Cargo.toml`. `scripts/run-tests.sh` walks all `Cargo.toml`s under `examples/` and `e2e-tests/` and runs `node tests/runSingleTest.js <dir> <release_wasm_path> [endpoint]`. The WASM path is `examples/target/wasm32v1-none/release/<crate>.wasm` or `e2e-tests/target/wasm32v1-none/release/<crate>.wasm`. If a directory under `e2e-tests/` has no `runTest.js`, it's silently skipped.

## File naming (enforced by convention, not tooling)

Per `docs/NAMING_CONVENTIONS.md`: Rust files and module dirs use `snake_case`; crate names use `kebab-case`; JS files use `camelCase`; shell scripts use `kebab-case`; `README.md`/`CONTRIBUTING.md`/`LICENSE` are `SCREAMING_SNAKE_CASE`; other docs use `kebab-case`.

## Manual UI testing

Build with `cargo build --target wasm32v1-none --release`, then upload the `.wasm` at <https://ripple.github.io/xrpl-wasm-stdlib/ui/> to exercise it against local rippled or Devnet.
