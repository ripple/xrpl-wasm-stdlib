# AGENTS.md

This file provides guidance to AI coding agents (Claude Code, Codex, etc.) when working with code in this repository.

**Keep this file current.** After completing a task, consider whether it changed something this file documents (workspace members, crate boundaries, scripts, architecture) and update the relevant section if so.

## What this repo is

A Rust `no_std` standard library, split across several crates (see below), that lets developers write XRPL smart contracts (currently "Smart Escrows") compiled to WebAssembly. The library wraps a low-level host ABI exposed by `rippled` and offers type-safe accessors for transaction fields, ledger objects, ledger entry IDs, and serialized fields.

Smart escrow WASM modules export `extern "C" fn escrow_finish() -> i32`. Returning a positive value finishes the escrow, `0` rejects it, and a negative value is a host error code.

## Three Cargo workspaces (intentional, do not merge)

| Workspace | Path                   | Members                                                                             |
| --------- | ---------------------- | ----------------------------------------------------------------------------------- |
| Library   | `/Cargo.toml` (root)   | `xrpl-common-stdlib`, `xrpl-macros`, `xrpl-escrow-stdlib`, `xrpl-stdlib-test-utils` |
| Examples  | `examples/Cargo.toml`  | all `examples/smart-escrows/*` cdylibs                                              |
| E2E tests | `e2e-tests/Cargo.toml` | host-function probe contracts + native test crates                                  |

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
cargo test -p xrpl-common-stdlib <test_name>
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

# Regenerate objects/generated/ (per-entry ledger-object field traits) from rippled (requires Node.js)
./scripts/generate-ledger-objects.sh
```

Other scripts not part of the primary workflow above: `scripts/benchmark-gas.sh`, `scripts/check-wasm-exports.sh`, `scripts/docs.sh` (builds and deploys the GitHub Pages docs/UI site), `scripts/host-function-audit.sh` (see below), `scripts/run-markdown.sh`, `scripts/validate-ui.sh`.

Pre-commit hooks (`.pre-commit-config.yaml`) run `cargo fmt --all` and `cargo clippy --all-targets --all-features -- -Dclippy::all` on staged Rust files, plus prettier with `--no-semi --tab-width 2` for JS/MD/YAML.

## Toolchain pinning

`rust-toolchain.toml` pins **Rust 1.89.0** with `rustfmt`, `clippy`, `rust-analyzer`, and the `wasm32v1-none` target. The library uses **edition 2024**. Do not bump these casually — the WASM target and edition affect both the library and every example.

## Architecture: crate ownership (`xrpl-common-stdlib` vs `xrpl-escrow-stdlib` vs `xrpl-macros` vs `xrpl-stdlib-test-utils`)

The library workspace is split into crates with a strict dependency direction: `xrpl-escrow-stdlib` → `xrpl-common-stdlib` → `xrpl-macros`. Never invert this — `xrpl-common-stdlib` must not depend on domain (feature-specific) code.

- **`xrpl-macros`** — proc-macro crate, no runtime dependencies on the other two. Exports:
  - Typed-constant macros: `r_address!`, `hash256!`, `pubkey!`, `currency!`, `blob!` — validate at compile time and emit a typed XRPL value.
  - Entry-point macros: `#[smart_escrow]`, `#[smart_contract]` — wrap a user function in the `extern "C"` symbol the XRPL host calls. Both share a `parse → validate → codegen` pipeline in `entry_point/`; adding a third entry-point macro means adding a new orchestrator file there plus a new `#[proc_macro_attribute]` shim in `lib.rs`.
- **`xrpl-common-stdlib`** — the general-purpose layer: host bindings, transaction/ledger-object field access, ledger entry IDs, types. Contains no feature-specific (e.g. escrow-only) logic.
- **`xrpl-escrow-stdlib`** — Smart Escrow-specific entry-point context (`EscrowFinishContext`, `FinishResult`) and escrow-unique host functions (e.g. `set_data`). Depends on `xrpl-common-stdlib` but does **not** re-export it — contract crates depend on `xrpl-common-stdlib`, `xrpl-macros`, and `xrpl-escrow-stdlib` directly as separate path dependencies (see `examples/smart-escrows/hello_world/Cargo.toml`).
- **`xrpl-stdlib-test-utils`** — test-only harness crate. Depends on `xrpl-common-stdlib` with the `test-host-bindings` feature enabled and re-exports its `HostBindings`/`MockHostBindings`, adding higher-level `EscrowScenario` builder helpers on top (e.g. `EscrowScenario::builder()...install()`) so downstream crates (like `xrpl-escrow-stdlib`'s own tests) don't have to hand-roll mock setups.

**Rule of thumb:** domain-specific code (escrow, and any future smart-contract feature) lives in its own crate and is never added to `xrpl-common-stdlib` with a re-export. `xrpl_common_stdlib::ctx::SmartFeatureContext` (in `xrpl-common-stdlib/src/ctx/mod.rs`) is the narrow, generic trait (`type Tx: TransactionCommonFields`, `fn tx(&self) -> &Self::Tx`) that feature-specific contexts like `EscrowFinishContext` implement — new features add a new context type/crate rather than extending this trait.

## Architecture: the three-implementation host-binding swap

This is the single most important pattern in the repo. `xrpl-common-stdlib/src/host/mod.rs` selects one of three implementations of the same `HostBindings` trait (defined in `host_bindings_trait.rs`) via `cfg`-gated `include!`:

| Config                                                       | Included file            | Purpose                                                                                        |
| ------------------------------------------------------------ | ------------------------ | ---------------------------------------------------------------------------------------------- |
| `cfg(target_arch = "wasm32")`                                | `host_bindings_wasm.rs`  | Real FFI `extern "C"` declarations — used in production WASM builds.                           |
| `cfg(any(test, feature = "test-host-bindings"))` on non-WASM | `host_bindings_test.rs`  | `mockall`-generated mocks — lets unit/coverage tests on the native target stub host functions. |
| Plain `cargo build` on non-WASM                              | `host_bindings_empty.rs` | No-op stubs that just allow native builds to compile (functions panic if called).              |

Consequences:

- `lib.rs` uses `#![cfg_attr(target_arch = "wasm32", no_std)]` — code is `no_std` only when targeting WASM; native builds get `std` so `cargo test` works. This applies to both `xrpl-common-stdlib` and `xrpl-escrow-stdlib`.
- To exercise stdlib code from another crate's tests (e.g. `e2e-tests/`, `xrpl-escrow-stdlib`), enable the `test-host-bindings` feature on `xrpl-common-stdlib` — `dev-dependencies` aren't enough because mockall must be available when the lib is consumed as a regular dep. `xrpl-stdlib-test-utils` wraps this feature and its mocks behind higher-level scenario builders; prefer it over hand-rolling mock setups in new tests.
- Anything new added to `HostBindings` must be implemented in all three files. CI's `scripts/host-function-audit.sh` compares the trait against rippled's exports — keep them in sync.

## Architecture: layering inside `xrpl-common-stdlib`

```
src/
├── lib.rs            # no_std toggle, panic_handler (wasm only), hex decode helpers, re-exports the xrpl-macros constant macros
├── crypto.rs          # crypto helpers
├── type_codes.rs      # XRPL serialized-type code constants
├── ctx/               # SmartFeatureContext trait — narrow contract shared by all feature-specific entry-point contexts
├── current_tx/        # EscrowFinish marker + traits → typed access to the current TX's fields
├── fields/            # Field decoding traits/helpers shared across XRPL field types, incl. locator.rs (nested-field locator paths)
├── host/              # Low-level layer: HostBindings trait + 3 impls, error codes, trace, field_helpers
├── objects/            # Cached ledger entry access: generated/ (per-entry <Entry>Fields traits + structs), ArrayObject, AnyObject, cache.rs (cache_le), traits
├── ledger_entry_ids.rs # Compute ledger entry IDs (escrow_id, oracle_id, credential_id, accountroot_id, amm_id, ...)
├── types/             # AccountID, Amount, Hash{128,160,192,256}, Blob, NFT, OpaqueFloat, Number, constants.rs, etc.
├── sfield.rs          # GENERATED — type-safe SField<T, CODE> constants. Do not hand-edit; rerun generate-sfields.sh
└── tx_flags.rs        # GENERATED, pub(crate) — transaction flag constants (tf*/asf*/tmf*). Do not hand-edit; rerun generate-tx-flags.sh
```

There is no `core/` module — everything above lives directly under `src/`. `CurrentEscrow` (the escrow-specific ledger-object helper) lives in `xrpl-escrow-stdlib/src/ledger_objects/current_escrow.rs`, not in `xrpl-common-stdlib`.

`SField<T, CODE>` encodes the field's Rust type as a const-generic phantom, so `current_tx::get_field(sfield::Account)` infers `AccountID`, `ledger_object::get_field(slot, sfield::Balance)` infers `Amount`, etc. Adding a new field means regenerating `sfield.rs` (see `tools/generateSFields.js` for custom type overrides like `TransactionType`, `ConditionBlob`, `FulfillmentBlob`).

XRPL wire types are mapped to Rust types by `tools/sfieldTypeMap.js`, shared with the ledger-object generator: `typeMap` keys off the wire type, and `customFieldTypes` holds per-field-name overrides that take priority (like `Condition` → `ConditionBlob`). A wire type in neither table needs no registration: its fields are emitted as `SField<Unmapped, CODE>`, where `Unmapped` (defined in `sfield.rs`'s hand-written header) is an uninhabited marker implementing no decoder marker trait — the field code stays usable (raw `le_field` reads, `Locator` segments) while `get_field` on it is a **compile error**, not a runtime failure. The generator lists these fields on stdout each run so a newly-added rippled type stays visible. Wiring one up means adding it to `typeMap` and giving the Rust type a `FieldDecoder` + `FromCurrentTx`/`FromLedger` impl. The `use` statements at the top of `sfield.rs` are _not_ generated — everything above the first `pub const` is preserved verbatim, so a new type's import is added by hand.

`tx_flags.rs` is merged from two rippled branches (see `tools/generateTxFlags.js`): a **base branch** (authoritative) plus a **contract branch** that only adds flags for new transaction types the base branch lacks (never redefining a base flag, so the merge is purely additive). Only individual flags are emitted — rippled's validity masks (`tf*Mask`) are intentionally omitted, since contracts check individual flags rather than validate flag combinations. The constants are `pub(crate)` — crate-internal backing behind a typed flags API, not a public surface.

`xrpl-escrow-stdlib/src/ctx/escrow_finish.rs` shows the pattern for a feature context: a struct holding a `current_tx` marker type (`EscrowFinish`) plus a ledger-object helper (`CurrentEscrow`), implementing `SmartFeatureContext`, with feature-unique host calls as inherent methods (all `unsafe` FFI stays inside the context type — user contract code stays fully safe).

## Ledger-object field accessors (generated)

`xrpl-common-stdlib/src/objects/generated/` holds one file per XRPL ledger-entry type (`oracle.rs`, `account_root.rs`, `bridge.rs`, ...) plus `mod.rs` (private per-entry `mod`s, with a flat `pub use` block as the sole public path — `objects::AccountRoot`, not `objects::generated::account_root::...` — and a `//!` header listing every field whose XRPL wire type has no typed Rust mapping yet, grouped by wire type). All of it — including a `#[cfg(test)] mod tests` block per entry — is produced by `tools/generateLedgerObjects.js` from rippled's `ledger_entries.macro`/`sfields.macro`, invoked via `./scripts/generate-ledger-objects.sh`. Any entry can be configured **slot-only** (`SLOT_ONLY_ENTRIES`) with individual fields excluded (`PER_ENTRY_EXCLUDED_FIELDS`) in the generator: the slot-based `<Entry>Fields` trait + `<Entry>` struct are still emitted (so contracts can read instances of that object), while the entry's current-object accessors and any host-mutable fields stay hand-written in the owning domain crate. Escrow is currently the only such entry — its `Data` (`ContractData`) field is excluded, and its `CurrentEscrowFields` + the `EscrowContractData` extension trait (for `Data`) live in `xrpl-escrow-stdlib`. No `load()` constructor is generated (ledger-entry-ID inputs vary per entry — construct with `new(slot)` after caching a ledger entry ID from `ledger_entry_ids.rs`). Do not hand-edit anything under `generated/` — `./scripts/generate-ledger-objects.sh --check` regenerates and diffs it in CI; fix drift by changing the generator, not the output.

## WASM build profile (matters for size and panic behavior)

The root, `examples/`, and `e2e-tests/` `Cargo.toml` files all set the same release profile:

```toml
opt-level = "s"     # size
lto = true
codegen-units = 1
panic = "abort"     # no_std can't unwind; also avoids pulling in a panic handler
```

The library defines a custom `#[panic_handler]` for `target_arch = "wasm32"` (in `xrpl-common-stdlib/src/lib.rs`) that calls `core::arch::wasm32::unreachable()`. Dev profile uses `panic = "unwind"` so unit tests can run on the host.

## Writing a contract

Minimal template (see `examples/smart-escrows/hello_world/src/lib.rs`):

```rust
#![cfg_attr(target_arch = "wasm32", no_std)]
#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_common_stdlib::host::trace::trace;
use xrpl_escrow_stdlib::{EscrowFinishContext, FinishResult};
use xrpl_macros::smart_escrow;

#[smart_escrow]
fn run(_ctx: EscrowFinishContext) -> FinishResult {
    trace("Hello World");
    FinishResult::succeed()
}
```

The `Cargo.toml` must set `crate-type = ["cdylib"]` and depend on `xrpl-common-stdlib`, `xrpl-macros`, and `xrpl-escrow-stdlib` as separate path dependencies. New examples must be added to `examples/Cargo.toml`'s `[workspace] members`.

Trace output (`trace`, `trace_hex`, `trace_num`) shows up in rippled's `debug.log`.

## Integration test pattern

Each example has a `runTest.js` next to its `Cargo.toml`. `scripts/run-tests.sh` walks all `Cargo.toml`s under `examples/` and `e2e-tests/` and runs `node tests/runSingleTest.js <dir> <release_wasm_path> [endpoint]`. The WASM path is `examples/target/wasm32v1-none/release/<crate>.wasm` or `e2e-tests/target/wasm32v1-none/release/<crate>.wasm`. If a directory under `e2e-tests/` has no `runTest.js`, it's silently skipped.

## File naming (enforced by convention, not tooling)

Per `docs/NAMING_CONVENTIONS.md`: Rust files and module dirs use `snake_case`; crate names use `kebab-case`; JS files use `camelCase`; shell scripts use `kebab-case`; `README.md`/`CONTRIBUTING.md`/`LICENSE` are `SCREAMING_SNAKE_CASE`; other docs use `kebab-case`.

## Manual UI testing

Build with `cargo build --target wasm32v1-none --release`, then upload the `.wasm` at <https://ripple.github.io/xrpl-wasm-stdlib/ui/> to exercise it against local rippled or Devnet. That site is deployed by `.github/workflows/docs.yml` via `scripts/docs.sh`, which builds the release wasm, runs `ui/embed-wasm.sh`, and publishes `ui/` to GitHub Pages on every push to `main`.

## Claude Code skill (`skills/`)

`skills/xrpl-smart-escrows/` is a packaged Claude Code skill (`SKILL.md` + `reference/*.md`) that teaches an AI assistant how to build, test, and debug Smart Escrow contracts against this library. It's referenced by `.claude-plugin/plugin.json` at the repo root, which makes the repo itself a Claude Code plugin — loadable straight from a checkout with `claude --plugin-dir .`, no install step. `.claude-plugin/marketplace.json` sits alongside it and catalogs that same plugin with `"source": "./"`, so the repo also acts as its own single-plugin marketplace: `/plugin marketplace add ripple/xrpl-wasm-stdlib` followed by `/plugin install xrpl-wasm-stdlib@xrpl-wasm-stdlib`. Either path namespaces the skill as `/xrpl-wasm-stdlib:xrpl-smart-escrows`. Validate both manifests with `claude plugin validate .`. It lives outside `docs/` deliberately — `scripts/run-markdown.sh` extracts and executes ` ```bash ` fenced code blocks from files under `docs/`/`examples/`/`scripts/`/`README.md`, and the skill's reference docs contain illustrative shell snippets that must not be executed by CI. It's also outside the Cargo `[workspace] members` list, so it has no effect on `cargo build`/clippy/fmt. Keep its `reference/api-surface.md` and `reference/patterns.md` in sync with the actual public API if crate names, entry-point macros, or example contracts change.
