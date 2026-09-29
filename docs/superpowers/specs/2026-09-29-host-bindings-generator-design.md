# Host bindings generated from rippled — design

**Status:** agreed 2026-09-29 · **Supersedes:** PR #293 (`compareHostFunctions.js` rewrite) · **Follow-ups:** pin migration for the other three generators; e2e probes for the 10 host functions no contract imports today.

> Fenced shell blocks in this file use ` ```sh `, not ` ```bash `, on purpose: `scripts/run-markdown.sh` executes every ` ```bash ` block under `docs/` in CI.

## Problem

`xrpl-common-stdlib/src/host/` hand-maintains four copies of rippled's wasm host ABI: the `HostBindings` trait and three implementations (`host_bindings_wasm.rs`, `host_bindings_empty.rs`, `host_bindings_test.rs`). Nothing structural ties them to rippled. The only guard was `scripts/host-function-audit.sh` → `tools/compareHostFunctions.js`, a regex scraper of two files at the live HEAD of `ripple/se/supported`. It is disabled in CI (`if: false`) because upstream reshaped those files, and the rewrite in PR #293 keeps the same structural weakness (scrapes upstream HEAD, two files, no pin).

The user's constraint: **rippled must be authoritative for the ABI, but the trait must remain a real, viewable file in this repo.**

## What rippled actually provides (verified against `21560cdf82d02771709d6befedb6b192211cf399`)

`crates/xrpl-host-functions/src/lib.rs` holds one `host_functions! { ... }` block. The proc macro (`crates/xrpl-host-functions-macros`) expands it into:

- `pub trait HostFunctions` — the **host-side** shape (`&self`, `&[u8]`, `&mut [u8]`, `u32`, `HostResult<_>`), method names like `get_ledger_sqn`.
- `pub enum HostFunctionSpec` with `const fn wasm_name()`, `gas()`, `wasm_params()` (flattened `[I32, I32, …]`), `wasm_result()`, `ALL`.
- `wasmi_glue!` (engine side, irrelevant to guests).

It emits **no `extern "C"` import block** and nothing a guest can call. The crate is not on crates.io (`0.1.0`, path-only); rippled pins Rust **1.97.1**, we pin **1.89.0**.

Two facts make generation from the declaration block alone sufficient (rippled's own macro documents them, `lib.rs:126-138`):

1. **Declaration order is wasm parameter order.**
2. **Wire lowering is mechanical from declared types:** `i32`/`i64` pass through; `&[u8]`, `&str`, `&mut [u8]` **and `u32`** are `(ptr, len)` regions; `TraceDataType` is an `i32` code; `HostResult<()>` has no wasm result, everything else is one `i32`.

So `register.rs` — which PR #293 parses — is downstream of the same lowering and not needed.

## Decision

Treat host bindings like `sfield.rs` and `objects/generated/`: **committed, generated, reviewable Rust files with a `--check` drift gate in CI**, produced by a **Rust** generator that parses the declaration block with `syn` (the same grammar rippled's proc macro validates on every upstream commit).

### Generated (2 files, `xrpl-common-stdlib/src/host/`)

| File                     | Content                                                                                                                                                                                                                                                                                                                  | Why generated                                          |
| ------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------ |
| `host_bindings_trait.rs` | `pub trait HostBindings` (with the existing `mockall::automock` `cfg_attr`), one `unsafe fn` per declaration in rippled order, named by `#[wasm_name]`. Docs: rippled's `///` lines **verbatim** (intra-doc links rewritten, see below), then generated `**Gas:**`, `# Wire parameters`, `# Returns`, `# Safety` blocks. | This is the file a reader opens.                       |
| `host_bindings_list.rs`  | One `macro_rules! for_each_host_function { ($callback:ident) => { $callback! { fn ldgr_index(out_ptr: *mut u8, out_len: usize) -> i32; … fn trace(…) -> (); } }; }`                                                                                                                                                      | The single signature list the three impls expand from. |

Both start with a `// GENERATED … Source: <label>` header naming the rippled source the run used.

### Hand-written, rewritten once to consume the list (3 files)

Each becomes a short `macro_rules!` callback plus its non-mechanical remainder, and **never changes again when rippled adds a function**:

- `host_bindings_wasm.rs` — `declare_host_imports` (the `#[link(wasm_import_module = "host_lib")] unsafe extern "C"` block) and `impl_wasm_host_bindings` (`impl HostBindings for WasmHostBindings` + the `host::<name>` free-function wrappers).
- `host_bindings_empty.rs` — `stub_host_functions`: returns the last argument as `i32` (today's behaviour, which doc tests rely on) via two tiny private traits (`StubArg::to_i32`, `StubReturn::from_i32`) so `trace`'s `()` needs no special case.
- `host_bindings_test.rs` — `dispatch_to_mock` (thread-local `MockHostBindings` dispatch); `apply_default_expectations` stays hand-written because its defaults are semantic (Amount fields report 8, `check_sig` reports 0, …). A generated test, `every_host_function_has_a_default_expectation`, calls every listed function with zero arguments against `create_default_mock()` so a newly added host function **fails `cargo test`** until its default is written.

`host/mod.rs` gains `include!("host_bindings_list.rs");` immediately after `pub mod host_bindings_trait;` (macro_rules are textually scoped, so the list must precede the three `include!`s).

### Naming and docs (user decisions)

- **Derive parameter names from rippled**, no override table: region `x` → `x_ptr`, `x_len`; scalars keep their name. This changes some names cosmetically (`out_buff_ptr`→`out_ptr`, `cache_num`→`cache_idx`, `sequence_ptr`→`seq_ptr`, `rounding_mode`→`mode`). All call sites, `mockall` closures and `expect_*` calls are positional or name-by-method, so nothing breaks; the compiler confirms.
- **rippled's docs verbatim**, no overlay. Link rewriting: `[`HostFunctions::rust_name`]` → `[`HostBindings::wasm_name`]`; any other `[`Ident`]` (e.g. `HASH_LEN`, `FloatOrdering`) → plain `` `Ident` `` so we add no broken intra-doc links. Non-doc `//` comments inside the block are not tokens and are dropped.
- Generated bullets state the wire mapping (`- `seq_ptr`, `seq_len`: the `seq: u32` region — four little-endian bytes.`) so the rippled↔wire correspondence is on the page.

### Pinning

The generator script defaults to `https://github.com/XRPLF/rippled/tree/<commit>` where `<commit>` is the tag of `XRPLD_DOCKER_IMAGE` in `.github/workflows/test.yml` (the same "single source of truth" `scripts/docker-rippled.sh` already reads). Bindings, the e2e node and the drift gate therefore describe **one rippled commit**, and "bump rippled" is one edit + regenerate. `RIPPLED_REF=<branch|sha>` or a positional source argument overrides; a local rippled checkout directory is accepted too. Migrating `generate-sfields.sh`, `generate-tx-flags.sh`, `generate-ledger-objects.sh` (which still read branch HEAD) to the same pin is a **follow-up**.

### Generator crate

`tools/generate-host-bindings/` — standalone Rust package (`publish = false`, added to the root `[workspace] exclude`, own `Cargo.lock`), library + thin binary:

- `parse.rs` — `syn::parse_file` → find `Item::Macro` named `host_functions` → parse tokens as a sequence of `TraitItemFn` → `Vec<HostFunction>`. Unsupported declared type or `HostResult` payload is a **hard error naming the function** — never a silent `i32`.
- `lower.rs` — declared types → wire params/return; rejects intra-function wire-name collisions.
- `docs.rs` — the link rewriting above.
- `emit.rs` — the two files as strings.
- `main.rs` — `generate-host-bindings <lib.rs> <out-dir> --source-label <text>`; writes both files; runs `rustfmt --edition 2024` on them (repo has no `rustfmt.toml`, so formatting is location-independent).
- `tests/fixtures/rippled_host_functions_lib.rs` — the pinned upstream file, so `cargo test` works offline and pins today's 63-function ABI as a regression.

`scripts/generate-host-bindings.sh [--check] [rippled-source]` mirrors `generate-ledger-objects.sh`: fetches `crates/xrpl-host-functions/src/lib.rs` with `curl` (or copies from a local dir), runs the generator, and in `--check` mode generates into a temp dir and `diff -u`s against the committed files (exit 1 on drift, tree untouched).

Wired into `run-all.sh`, `test.yml` (new `check_host_bindings_generated` job), `clippy.sh`, `fmt.sh`, `build-and-test.sh`. The audit (`scripts/host-function-audit.sh`, `tools/compareHostFunctions.js`) and the superseded `tools/generateTestHostBindings.js` are deleted.

## Alternatives rejected

- **Depend on rippled's `xrpl-host-functions` crate and `impl HostFunctions` for our stubs.** Proves nothing about the guest's `extern "C"` block (different abstraction level); crate unpublished (published crates can't take git deps); toolchain 1.97 vs 1.89; a version pin would block adding a binding before a rippled release.
- **Make the trait a macro expansion too.** Rejected by the user: the trait must be a file one can open. Same reason the three impls stay files rather than one `host_abi!` invocation.
- **Fully generate `host_bindings_test.rs`.** Its defaults are semantic test data, not derivable from rippled; the coverage test gives the same "can't forget" guarantee.
- **Generate at build time (`build.rs`) or during `run-all.sh` only.** Published crates must not fetch at build time; generated code must be committed. `run-all.sh` runs `--check`, not generation.
- **Keep the audit and fix its parser (PR #293).** Fixes the wrong tool: still scrapes upstream HEAD, still two files, still only a link-time check that rippled's own preflight already performs on deploy.

## Longer-term

If rippled later emits a guest `extern "C"` block behind a feature (a ~100-line addition to its `lowering.rs`) and publishes the crate with a `rust-version`, this generator becomes a thin wrapper over that expansion. Nothing here has to be undone.
