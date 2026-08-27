# Agent task board

Prioritized queue of [open GitHub issues](https://github.com/ripple/xrpl-wasm-stdlib/issues) so agents can pick work without colliding.

**Snapshot:** 2026-08-27. Re-check GitHub before starting — this file lags the tracker.

**Source of truth:** the GitHub issue. This board is a parallel-work index (priority, claim status, file collisions), not a second tracker.

## How to claim (required)

1. Pick **one** issue from [Ready now](#ready-now-pick-these). Do not start anything in [Do not start](#do-not-start).
2. Confirm the GitHub issue is still **open**, **unassigned**, and has **no open PR**.
3. Assign yourself on the issue and open a branch named `feat/<issue-number>-<short-slug>` (or `fix/` / `chore/` as appropriate).
4. Add a row to [In progress](#in-progress) in this file on that same branch (or note the claim in the PR body if you are not editing this file).
5. Stay inside that issue's suggested files. If two ready issues share a collision group, only one agent may hold that group.
6. When the PR is up, link `Fixes #<n>` (or `Closes #<n>`). Move the board row to the PR.

Do not "just start coding" on an unclaimed issue. Two agents in `current_tx/traits.rs` will conflict.

## In progress

| Issue                                                                            | Agent / PR                                                             | Branch                       | Collision group           |
| -------------------------------------------------------------------------------- | ---------------------------------------------------------------------- | ---------------------------- | ------------------------- |
| [#145](https://github.com/ripple/xrpl-wasm-stdlib/issues/145) Add `get_delegate` | [PR #284](https://github.com/ripple/xrpl-wasm-stdlib/pull/284) (draft) | `feat/add-delegate-tx-field` | `TransactionCommonFields` |

## Ready now (pick these)

Independent, bounded, and currently unblocked. Ordered by **impact / effort** (do the small complete wins first). Parallel agents should pick **different collision groups**.

| Pri | Issue                                                                                                         | What to do                                                                                                                                                                                                                | Suggested files                                                                                             | Collision group  | Verify                                                                                                                      |
| --- | ------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------- | ---------------- | --------------------------------------------------------------------------------------------------------------------------- |
| 1   | [#115](https://github.com/ripple/xrpl-wasm-stdlib/issues/115) Mark e2e-test crates `publish = false`          | Only `e2e-tests/test_utils/Cargo.toml` has `publish = false`. Add it to every other e2e package (and the workspace root if needed).                                                                                       | `e2e-tests/**/Cargo.toml`                                                                                   | `e2e-manifests`  | `cargo metadata --manifest-path e2e-tests/Cargo.toml --format-version 1` — each member `"publish": false`                   |
| 2   | [#203](https://github.com/ripple/xrpl-wasm-stdlib/issues/203) `get_first_memo` typed as `ContractData`        | Memo data is `sfield::MemoData` (`StandardBlob`), not escrow `ContractData`. Same bug in `nft_owner`. Use `StandardBlob` (or a memo-specific alias) and stop returning a leftover length beside the blob.                 | `examples/smart-escrows/atomic_swap/atomic_swap1/src/lib.rs`, `examples/smart-escrows/nft_owner/src/lib.rs` | `examples-memos` | `cargo clippy` in `examples/`; wasm build of those crates                                                                   |
| 3   | [#250](https://github.com/ripple/xrpl-wasm-stdlib/issues/250) Named buffer-size constants                     | `MptId` already has `MPT_ID_SIZE`. Remaining: `IouIssue` still uses a bare `[u8; 40]`. Add `IOU_ISSUE_SIZE` (or similar) and replace magic 40/20 splits. `Number` already has `NUMBER_SIZE`.                              | `xrpl-common-stdlib/src/types/issue.rs`                                                                     | `types-sizes`    | `cargo test -p xrpl-common-stdlib`                                                                                          |
| 4   | [#126](https://github.com/ripple/xrpl-wasm-stdlib/issues/126) Experiment with `cargo-deny`                    | Add `deny.toml` + a CI/script check. Advisory-only first (do not fail the whole suite on the first run unless it is clean).                                                                                               | `deny.toml`, `.github/workflows/test.yml` or `scripts/`                                                     | `ci-deny`        | `cargo deny check` locally                                                                                                  |
| 5   | [#53](https://github.com/ripple/xrpl-wasm-stdlib/issues/53) Example/e2e that traces ledger-header fields      | Mirror `e2e-tests/trace_escrow_ledger_object`: one contract that reads every ledger-header field the host exposes and traces it.                                                                                          | new crate under `e2e-tests/` + `e2e-tests/Cargo.toml` member                                                | `e2e-new-crate`  | `./scripts/build.sh` wasm for that crate; `runTest.js` if you add one                                                       |
| 6   | [#58](https://github.com/ripple/xrpl-wasm-stdlib/issues/58) E2E that traces all NFT fields                    | Same pattern as #53, for NFT / `NFToken` fields. Comment on the issue says this should be an e2e test, not only an example.                                                                                               | new crate under `e2e-tests/`                                                                                | `e2e-new-crate`  | same as #53. **Do not** take this if another agent already holds `e2e-new-crate` unless you coordinate distinct crate names |
| 7   | [#239](https://github.com/ripple/xrpl-wasm-stdlib/issues/239) Newtype wrappers to prevent argument-order bugs | `check_sig(msg, sig, key)` takes two `&[u8]`. Introduce newtypes (or reuse `SignatureBlob`) so swapping args is a type error. Audit similar public fns; do not boil the ocean.                                            | `xrpl-common-stdlib/src/crypto.rs` and callers                                                              | `crypto-api`     | `cargo test -p xrpl-common-stdlib crypto`                                                                                   |
| 8   | [#7](https://github.com/ripple/xrpl-wasm-stdlib/issues/7) Unwrap trace calls                                  | Port [craft#232](https://github.com/ripple/craft/pull/232) / [craft#216](https://github.com/ripple/craft/issues/216): tracing helpers should not force `unwrap` on `Result`. Assigned to `sappenin` — ping before taking. | `xrpl-common-stdlib/src/host/trace.rs`, example/e2e call sites                                              | `trace-api`      | `cargo test -p xrpl-common-stdlib`; clippy on examples                                                                      |

#53 and #58 can run in parallel if crate names do not clash. Prefer one agent per new e2e crate to keep `e2e-tests/Cargo.toml` mergeable (rebase the member list).

## Next up (blocked, assigned, or needs a product decision)

Do not start these unless the blocker is gone and you have re-checked GitHub.

| Issue                                                                                                                                              | Why it is not ready                                                                                                                                                                                                                                    | Unblock by                                                           |
| -------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | -------------------------------------------------------------------- |
| [#145](https://github.com/ripple/xrpl-wasm-stdlib/issues/145) `get_delegate`                                                                       | [PR #284](https://github.com/ripple/xrpl-wasm-stdlib/pull/284)                                                                                                                                                                                         | Merge or take over that PR                                           |
| [#150](https://github.com/ripple/xrpl-wasm-stdlib/issues/150) Optional `get_signing_pub_key` / `get_txn_signature`                                 | Assigned `sappenin`. `get_signing_pub_key` is **already** `Result<Option<PublicKey>>`. Remaining work is `get_txn_signature` (still required). Same files as #145/#90                                                                                  | Wait for #284; then only the signature getter                        |
| [#90](https://github.com/ripple/xrpl-wasm-stdlib/issues/90) `get_signers`                                                                          | Empty body; relates to [#112](https://github.com/ripple/xrpl-wasm-stdlib/issues/112). Signers are an array (`sfield::Signers`), not a single field — likely a locator/`ArrayObject` helper, not a flat getter. Collides with `TransactionCommonFields` | After #284; design the array API first                               |
| [#112](https://github.com/ripple/xrpl-wasm-stdlib/issues/112) Iterate `Vector256` via Locator in e2e                                               | Related to #90. Locator paths already exist; this is e2e coverage                                                                                                                                                                                      | Independent of #145; can follow #90 or proceed if you only touch e2e |
| [#132](https://github.com/ripple/xrpl-wasm-stdlib/issues/132) Generate host-function derivatives in Rust                                           | [PR #281](https://github.com/ripple/xrpl-wasm-stdlib/pull/281)                                                                                                                                                                                         | Review/land that PR                                                  |
| [#169](https://github.com/ripple/xrpl-wasm-stdlib/issues/169) `XRPL_CONTRACT_DATA_SIZE` → 1024                                                     | **Likely done.** `DEFAULT_BLOB_SIZE` is 1024 and `XRPL_CONTRACT_DATA_SIZE` aliases it. Assigned `pwang200`                                                                                                                                             | Confirm on `main`, then close                                        |
| [#250](https://github.com/ripple/xrpl-wasm-stdlib/issues/250) (MptId half)                                                                         | `MPT_ID_SIZE` already exists                                                                                                                                                                                                                           | Close the MptId portion when landing the Issue constant              |
| [#151](https://github.com/ripple/xrpl-wasm-stdlib/issues/151) Host function `float_to_int`                                                         | Needs a rippled host export first; then all three `host_bindings_*.rs` + audit script                                                                                                                                                                  | Wait for rippled                                                     |
| [#152](https://github.com/ripple/xrpl-wasm-stdlib/issues/152) Opaque-float operator overloading                                                    | Overlaps open float PRs [#283](https://github.com/ripple/xrpl-wasm-stdlib/pull/283) (`float_root` removal), [#206](https://github.com/ripple/xrpl-wasm-stdlib/pull/206) (Amount/XFloat)                                                                | After those PRs settle                                               |
| [#105](https://github.com/ripple/xrpl-wasm-stdlib/issues/105) Float example                                                                        | Same float collision group as #152/#151                                                                                                                                                                                                                | After float API is stable                                            |
| [#282](https://github.com/ripple/xrpl-wasm-stdlib/issues/282) Rename crates `xrpl-wasm-*`                                                          | Assigned `sappenin`; naming still in discussion (`xrpl-wasm-common` vs `xrpl-stdlib-*`). Touches **every** `Cargo.toml`                                                                                                                                | Human decision, then a dedicated rename PR                           |
| [#46](https://github.com/ripple/xrpl-wasm-stdlib/issues/46) Finish `host_functions_test`                                                           | Assigned `mvadari`; empty body                                                                                                                                                                                                                         | Ask assignee what is left                                            |
| [#59](https://github.com/ripple/xrpl-wasm-stdlib/issues/59) pre-commit.ci                                                                          | Assigned `mvadari`; they marked it low priority                                                                                                                                                                                                        | Skip unless you are that assignee                                    |
| [#54](https://github.com/ripple/xrpl-wasm-stdlib/issues/54) Credential-field trace                                                                 | Open design disagreement (every ledger type vs one-of-each SType). Tied to #93                                                                                                                                                                         | Get a decision on the issue                                          |
| [#93](https://github.com/ripple/xrpl-wasm-stdlib/issues/93) Reorganize e2e tests                                                                   | Broad; comments said wait for other PRs. Overlaps #53/#54/#58/#46                                                                                                                                                                                      | Human scope first                                                    |
| [#107](https://github.com/ripple/xrpl-wasm-stdlib/issues/107) Tests for `CurrentTxFieldGetter` / `FieldGetter`                                     | Those trait names look **gone** after the decoder/`get_field` refactor. Waited on #102 (merged)                                                                                                                                                        | Confirm staleness; close or retarget at current decoder tests        |
| [#55](https://github.com/ripple/xrpl-wasm-stdlib/issues/55) / [#60](https://github.com/ripple/xrpl-wasm-stdlib/issues/60) Broad unit-test coverage | Overlapping umbrellas, not a single PR                                                                                                                                                                                                                 | Carve a concrete file/module; do not "add tests for everything"      |
| [#61](https://github.com/ripple/xrpl-wasm-stdlib/issues/61) Base58 tracing                                                                         | May need a rippled `trace` encoding mode, not just stdlib                                                                                                                                                                                              | Confirm host ABI before coding                                       |
| [#120](https://github.com/ripple/xrpl-wasm-stdlib/issues/120) Point generators at rippled `develop`/`master`                                       | Wait until Smart Escrow is in rippled                                                                                                                                                                                                                  | External merge                                                       |
| [#35](https://github.com/ripple/xrpl-wasm-stdlib/issues/35) Public Docker image                                                                    | Empty body; infra                                                                                                                                                                                                                                      | Needs a spec                                                         |

## Design / architecture (do not implement from this board)

These need a human design choice before code. An agent may **research and comment** on the issue; do not ship a speculative API.

| Issue                                                         | Topic                                                                                                                        |
| ------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------- |
| [#238](https://github.com/ripple/xrpl-wasm-stdlib/issues/238) | Distinct host vs guest types                                                                                                 |
| [#205](https://github.com/ripple/xrpl-wasm-stdlib/issues/205) | Zero-copy host/guest transfers (audit first)                                                                                 |
| [#82](https://github.com/ripple/xrpl-wasm-stdlib/issues/82)   | How much abstraction the library should contain                                                                              |
| [#84](https://github.com/ripple/xrpl-wasm-stdlib/issues/84)   | Folder reorg — **stale** (pre-crate-split; proposed `src/` at repo root and `xrpl-address-macro`). Do not execute as written |
| [#86](https://github.com/ripple/xrpl-wasm-stdlib/issues/86)   | Caps on host returned-data / trace payloads (rippled + stdlib)                                                               |
| [#116](https://github.com/ripple/xrpl-wasm-stdlib/issues/116) | Cached property-style accessors vs methods                                                                                   |

## Do not start

Open PRs that already cover an area. Do not duplicate them; review or wait.

| PR                                                          | Title                           | Overlaps                                                |
| ----------------------------------------------------------- | ------------------------------- | ------------------------------------------------------- |
| [#284](https://github.com/ripple/xrpl-wasm-stdlib/pull/284) | `get_delegate`                  | #145, `TransactionCommonFields`                         |
| [#283](https://github.com/ripple/xrpl-wasm-stdlib/pull/283) | Remove `float_root`             | float API                                               |
| [#281](https://github.com/ripple/xrpl-wasm-stdlib/pull/281) | Migrate `tools/` JS to Rust     | #132                                                    |
| [#280](https://github.com/ripple/xrpl-wasm-stdlib/pull/280) | Publish on `v*` tag             | CI / release                                            |
| [#279](https://github.com/ripple/xrpl-wasm-stdlib/pull/279) | Bump to `0.10.0-dev`            | versions                                                |
| [#273](https://github.com/ripple/xrpl-wasm-stdlib/pull/273) | Fallible `PublicKey` from slice | `types/public_key.rs`                                   |
| [#232](https://github.com/ripple/xrpl-wasm-stdlib/pull/232) | Inject version into wasm        | wasm build                                              |
| [#222](https://github.com/ripple/xrpl-wasm-stdlib/pull/222) | E2E coverage                    | e2e / CI                                                |
| [#206](https://github.com/ripple/xrpl-wasm-stdlib/pull/206) | Amount + XFloat                 | float / amount                                          |
| [#184](https://github.com/ripple/xrpl-wasm-stdlib/pull/184) | Docker README                   | docs                                                    |
| [#168](https://github.com/ripple/xrpl-wasm-stdlib/pull/168) | Getting-started docs            | docs                                                    |
| [#161](https://github.com/ripple/xrpl-wasm-stdlib/pull/161) | ERC20/MPT example               | examples                                                |
| [#158](https://github.com/ripple/xrpl-wasm-stdlib/pull/158) | AI assistant dotfiles           | `AGENTS.md` / skills                                    |
| [#64](https://github.com/ripple/xrpl-wasm-stdlib/pull/64)   | Amount/Issue/Asset redesign     | `types/amount.rs`, `issue.rs` — **conflicts with #250** |

## Collision groups (one holder at a time)

| Group                     | Files / area                                  | Issues                           |
| ------------------------- | --------------------------------------------- | -------------------------------- |
| `TransactionCommonFields` | `xrpl-common-stdlib/src/current_tx/traits.rs` | #145, #150, #90                  |
| `float-api`               | float/amount/XFloat types, host float fns     | #151, #152, #105, PRs 283/206/64 |
| `types-sizes`             | `types/issue.rs`, amount buffers              | #250, PR #64                     |
| `e2e-reorg`               | `e2e-tests/` layout                           | #93, #46, #54                    |
| `e2e-new-crate`           | `e2e-tests/Cargo.toml` members                | #53, #58, #112                   |
| `e2e-manifests`           | e2e `Cargo.toml` `publish` keys               | #115                             |
| `crate-rename`            | every package name                            | #282                             |
| `trace-api`               | `host/trace.rs`                               | #7, #61                          |
| `crypto-api`              | `crypto.rs`                                   | #239                             |
| `examples-memos`          | atomic_swap / nft_owner                       | #203                             |
| `ci-deny`                 | `deny.toml`, workflows                        | #126                             |

## House rules for implementers

- Three workspaces stay split (`/`, `examples/`, `e2e-tests/`). See `AGENTS.md`.
- Domain/escrow code stays out of `xrpl-common-stdlib`.
- New `HostBindings` methods must land in all three of `host_bindings_wasm.rs`, `host_bindings_test.rs`, `host_bindings_empty.rs`.
- Do not hand-edit `sfield.rs`, `tx_flags.rs`, or `objects/generated/` — regenerate.
- `scripts/run-markdown.sh` executes fenced `bash` blocks under `docs/`. Do not put runnable `bash` fences in this file.
- Prefer `xrpl-stdlib-test-utils` (`EscrowScenario`) over hand-rolled mocks.
- After finishing, update this board (move the issue out of Ready / In progress) if you are already editing docs on that branch.
