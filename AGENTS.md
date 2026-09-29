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

Other scripts not part of the primary workflow above: `scripts/benchmark-gas.sh`, `scripts/check-wasm-exports.sh`, `scripts/docs.sh` (builds and deploys the GitHub Pages docs/UI site), `scripts/host-function-audit.sh` (see below), `scripts/run-markdown.sh`, `scripts/validate-ui.sh`, `scripts/wasm-opt.sh` (called by `build.sh`; see below), `scripts/cargo-deny.sh` (RustSec advisories on the library workspace; advisory-only in CI until the first clean run).

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
- **`xrpl-stdlib-test-utils`** — test-only harness crate, but a **published** one: external contract authors need it for the same reason in-repo tests do. Depends on `xrpl-common-stdlib` with the `test-host-bindings` feature enabled and re-exports its `HostBindings`/`MockHostBindings`, adding higher-level `EscrowScenario` builder helpers on top (e.g. `EscrowScenario::builder()...install()`) so downstream crates (like `xrpl-escrow-stdlib`'s own tests) don't have to hand-roll mock setups. Depends on `xrpl-common-stdlib` only — never on `xrpl-escrow-stdlib`, despite carrying the escrow scenario builders, because it wires expectations at the host-binding level.

**Publishing.** All four crates go to crates.io. In-workspace deps between published crates carry both `version` and `path` — cargo builds against `path` and swaps to the registry range when packaging, so omitting `version` fails `cargo publish` with `all dependencies must have a version specified`. Note that `xrpl-stdlib-test-utils`' mocks are generated from `xrpl-common-stdlib`'s `HostBindings` trait, so those two are tightly coupled in practice whatever the versioning scheme. Under the 1.89 pin each crate publishes individually in dependency order (macros → common → test-utils → escrow; test-utils depends only on common); `cargo publish --workspace` needs 1.90. Releasing is **entirely manual** — there is no publish workflow, and neither a merge to `main` nor a `v*` tag uploads anything. A maintainer runs `./scripts/run-all.sh`, then `cargo publish -p <crate>` in that order, then tags and cuts a GitHub Release as a record of what shipped. Nothing in CI enforces the pre-publish gate, so it is a checklist item in `CONTRIBUTING.md`'s "Release Process" — that's the maintainer-facing sequence. The `deprecated/xrpl-wasm-stdlib` signpost is a one-time publish.

`xrpl-stdlib-test-utils` enables `xrpl-common-stdlib`'s `test-host-bindings` feature as a **normal** dependency — it _is_ the harness, so the feature is not optional for it. Consumers keep `mockall` out of their WASM builds by declaring the harness under `[target.'cfg(not(target_arch = "wasm32"))'.dev-dependencies]`; document that gate in anything user-facing, because `mockall` is not `no_std` and a non-gated dep breaks `wasm32v1-none`.

The name `xrpl-wasm-stdlib` refers to the **repository** and to the old pre-split crate (published through 0.8.0, now deprecated on crates.io) — never to the current general-layer crate, which is `xrpl-common-stdlib`.

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

`scripts/build.sh` finishes by running `scripts/wasm-opt.sh`, which rewrites the **release** artifacts of both WASM workspaces in place with Binaryen's `wasm-opt` (~30% smaller, which matters because rippled charges gas per instruction and caps contract size). In place, rather than a separate `.opt.wasm`, so `tests/runSingleTest.js`, `ui/embed-wasm.sh`, and the gas benchmark all exercise the bytes that would ship on-ledger. Debug artifacts are left alone.

There is deliberately **no result caching**. Writing in place breaks the hardlink cargo makes from `deps/`, so cargo re-uplifts the unoptimized artifact on every build — meaning each `build.sh` run is already exactly one `wasm-opt` pass over pristine output, and a skip-cache keyed on the file would never legitimately hit. An earlier stamp-file version of this was removed: it cost ~0.7s of savings that never materialized, and it silently skipped re-optimization when `WASM_OPT_FLAGS` or the wasm-opt version changed.

The flag set is `-Oz --mvp-features --enable-sign-ext --enable-mutable-globals --strip-debug --strip-producers`. Starting from `--mvp-features` and re-enabling exactly sign-extension and mutable-globals is deliberate: that pair _is_ `wasm32v1-none`, and it stops wasm-opt from emitting post-MVP instructions (bulk memory, SIMD, reference types) that rippled's engine would reject. Don't widen it without checking what the host actually accepts.

The **tool** is mandatory — `scripts/build.sh` calls `wasm-opt.sh` outside its release/debug branch, so even a debug build fails if `wasm-opt` can't be obtained. It resolves one from `$WASM_OPT`, then `PATH`, then `$CARGO_HOME/bin`; finding none it runs `cargo install wasm-opt --version <pin> --locked` and fails the build if that fails. Having no release WASM to optimize is a different matter and exits 0 quietly, so a debug-only build isn't broken by the step — but a dir or file named explicitly on the command line that's missing or holds no `.wasm` is treated as a typo and fails. The pinned crate version lives in that script and is the single source of truth: `scripts/setup.sh` and both CI workflows call `./scripts/wasm-opt.sh --ensure-tool` to resolve/install up front rather than stalling a build mid-way. CI's `~/.cargo/bin` cache carries the compiled binary between runs.

The `wasm-opt` crate isn't a reimplementation — its binary links Binaryen's real `wasm_opt_main`, so it's the upstream CLI with upstream flags, just built from source (needs a C++17 toolchain, a few minutes on first install). It tracks Binaryen **116** while upstream is at 133; output measured within 0.4% of 132 on every example, so the lag costs nothing that matters. It's pinned because a different Binaryen means different bytes, which means different numbers out of `scripts/benchmark-gas.sh`. A native Binaryen already on `PATH` wins over the pin — respecting a developer's install beats a multi-minute compile.

A **cargo-managed** `wasm-opt` (the one at `$CARGO_HOME/bin`), by contrast, is version-checked against the pin and reinstalled if it doesn't match, then re-checked so a bad install fails loudly. This matters because CI caches `~/.cargo/bin` under a key derived from `Cargo.lock`, which bumping `WASM_OPT_CRATE_VERSION` in the script does not change — without the check, a pin bump would silently keep using the stale cached binary and the pin would be a lie. The expected Binaryen version is derived from the crate version, which is published as `0.<binaryen>.<patch>`; if that convention ever breaks, fix `PINNED_BINARYEN_VERSION` in the script. An explicit `$WASM_OPT` and a system Binaryen are both exempt from this check.

Note that `cargo install` is invoked with `RUSTFLAGS=""`: the repo-wide `-Dwarnings` would otherwise fail the build on any warning in a third-party crate.

Other knobs: `SKIP_WASM_OPT=true` (the one deliberate opt-out), `WASM_OPT_NO_INSTALL=true` (fail rather than auto-install), `WASM_OPT=<path>`, `WASM_OPT_FLAGS="..."`. The script also takes explicit dirs/files as arguments.

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

`runSingleTest.js` injects a `testContext` into each `runTest.js`'s exported `test(ctx)` function: `client`, `submit`, `sourceWallet`, `destWallet`, `fundWallet`, `deploy` (from `tests/deployWasmCode.js`), `finish` (hex of the built WASM), and the helpers from `tests/harness.js` spread in — `finishEscrow(ctx, wallet, { Owner, OfferSequence, Gas, expect, Memos, ... })`, `expectResult(response, expected, label)`, `expectEscrowConsumed`/`expectEscrowSurvived`, `getLedgerCloseTime`/`getLedgerCloseTimeIso(client)`, and `loadWasmHex(callerDir, relative)` for pulling a sibling example's release WASM (used by `atomic_swap1` to load `atomic_swap2.wasm`). Both `finishEscrow` and `deploy()` take an options bag: any PascalCase XRPL transaction field (`Data`, `Amount`, `CancelAfter`, `Condition`, `SourceTag`, `DestinationTag`, ...) is spread verbatim onto the tx so new fields work without harness edits; the only camelCase keys are harness metadata — `expect` (default `"tesSUCCESS"`) and, on `deploy()`, `cancelAfterOffset` (default `2000`s added to the validated `close_time`). The harness stops at the escrow surface: multi-sign flows (`trace_escrow_finish`) and rich per-test setup (freelancer's 27-byte `Data` layout, NFT builders in `nft_owner`) stay inline because they'd be single-caller and over-abstract poorly.

## File naming (enforced by convention, not tooling)

Per `docs/NAMING_CONVENTIONS.md`: Rust files and module dirs use `snake_case`; crate names use `kebab-case`; JS files use `camelCase`; shell scripts use `kebab-case`; `README.md`/`CONTRIBUTING.md`/`LICENSE` are `SCREAMING_SNAKE_CASE`; other docs use `kebab-case`.

## Manual UI testing

Build with `cargo build --target wasm32v1-none --release`, then upload the `.wasm` at <https://ripple.github.io/xrpl-wasm-stdlib/ui/> to exercise it against local rippled or Devnet. That site is deployed by `.github/workflows/docs.yml` via `scripts/docs.sh`, which builds the release wasm, runs `ui/embed-wasm.sh`, and publishes `ui/` to GitHub Pages on every push to `main`.

## Claude Code skill (`skills/`)

`skills/xrpl-smart-escrows/` is a packaged Claude Code skill (`SKILL.md` + `reference/*.md`) that teaches an AI assistant how to build, test, and debug Smart Escrow contracts against this library. It's referenced by `.claude-plugin/plugin.json` at the repo root, which makes the repo itself a Claude Code plugin — loadable straight from a checkout with `claude --plugin-dir .`, no install step. `.claude-plugin/marketplace.json` sits alongside it and catalogs that same plugin with `"source": "./"`, so the repo also acts as its own single-plugin marketplace: `/plugin marketplace add ripple/xrpl-wasm-stdlib` followed by `/plugin install xrpl-wasm-stdlib@xrpl-wasm-stdlib`. Either path namespaces the skill as `/xrpl-wasm-stdlib:xrpl-smart-escrows`. Validate both manifests with `claude plugin validate .`. It lives outside `docs/` deliberately — `scripts/run-markdown.sh` extracts and executes ` ```bash ` fenced code blocks from files under `docs/`/`examples/`/`scripts/`/`README.md`, and the skill's reference docs contain illustrative shell snippets that must not be executed by CI. It's also outside the Cargo `[workspace] members` list, so it has no effect on `cargo build`/clippy/fmt. Keep its `reference/api-surface.md` and `reference/patterns.md` in sync with the actual public API if crate names, entry-point macros, or example contracts change.
