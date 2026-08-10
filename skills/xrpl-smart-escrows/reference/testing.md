# Testing

## Native unit tests

Contract-side logic that doesn't need real host calls (encoding/decoding, pure functions) can be unit-tested natively:

```shell
cargo test --workspace
cargo test -p <your-crate> <test_name>
```

These run against the `mockall`-based `HostBindings` mock (see [architecture.md](architecture.md)), enabled via `cfg(test)` or the `test-host-bindings` feature — no real rippled node needed.

## Building for WASM

```shell
rustup target add wasm32v1-none   # one-time
cargo build --target wasm32v1-none --release
```

Output: `target/wasm32v1-none/release/<crate_name>.wasm`. Common build failures:

| Symptom                                                        | Cause                                                                                                           |
| -------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------- |
| Build succeeds but produces no `.wasm`, or produces an `.rlib` | Missing `crate-type = ["cdylib"]` in `Cargo.toml`                                                               |
| Linker error about missing `finish` export                     | Missing `#[smart_escrow]` on the entry function, or the function isn't reachable (not `pub`, wrong signature)   |
| `can't find crate for 'std'`                                   | Missing `#![cfg_attr(target_arch = "wasm32", no_std)]` or building without the `wasm32v1-none` target installed |

## Integration tests against a real node

Each contract directory has a `runTest.js` next to its `Cargo.toml`, exporting `async function test(testContext)`:

```js
async function test(testContext) {
  const { deploy, finish, submit, sourceWallet, destWallet } = testContext

  const escrowResult = await deploy(sourceWallet, destWallet, finish)

  const tx = {
    TransactionType: "EscrowFinish",
    Account: sourceWallet.address,
    Owner: sourceWallet.address,
    OfferSequence: parseInt(escrowResult.sequence),
    Gas: 1000000, // required: WASM execution budget
  }

  const response = await submit(tx, sourceWallet)
  if (response.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error(
      "Failed to finish escrow:",
      response.result.meta.TransactionResult,
    )
    process.exit(1)
  }
}

module.exports = { test }
```

The harness supplies `testContext`: `deploy(sourceWallet, destWallet, wasmBytes)` creates an `EscrowCreate` with the compiled WASM as `Bytecode`; `finish` is the compiled WASM bytes; `submit(tx, wallet)` signs, submits, and waits for validation; `sourceWallet`/`destWallet` are pre-funded test accounts. Multi-contract patterns (atomic swap) call `deploy`/`submit` twice, once per side, in the order the swap requires.

Run it:

```shell
./scripts/run-tests.sh                                     # every example + e2e contract with a runTest.js
./scripts/run-tests.sh examples/smart-escrows/hello_world   # single contract
DEVNET=true ./scripts/run-tests.sh                          # against wss://wasm.devnet.rippletest.net:51233 instead of local rippled
```

Requires a running rippled node with the Smart Escrow amendment (local default: `ws://localhost:6006`; build from `XRPLF/rippled` branch with WASM support, or point at the WASM Devnet).

## Manual UI testing

Build release, then upload the `.wasm` at `https://ripple.github.io/xrpl-wasm-stdlib/ui/` to exercise the contract against local rippled or Devnet without writing a `runTest.js` — useful for quick iteration during development.

## Debugging runtime failures

| Symptom                                          | Likely cause                                                                                                                           |
| ------------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------- |
| `FieldNotFound`                                  | Reading an optional field that isn't set on this tx/object, or a typo'd `sfield` constant / wrong nesting via `Locator`                |
| Buffer overflow / truncated read                 | Destination buffer smaller than the field's actual size (check `Blob<N>` capacity, `ContractData` is 1024 bytes)                       |
| `NoFreeSlots`                                    | Too many `cache_le` calls in one execution — cache the slot and reuse it instead of re-resolving the same ledger entry ID              |
| Escrow finishes when it shouldn't, or vice versa | Check the actual `i32` returned — `FinishResult`/`i32` boolean coercion (`(cond as i32)`) is easy to get backwards; positive = release |

Add `trace`/`trace_num` calls on both success and failure paths during development (see [api-surface.md](api-surface.md)); they show up in rippled's `debug.log`. Always create a **fresh escrow per test run** — an already-finished or already-cancelled escrow can't be re-tested.

## Full local CI

```shell
./scripts/run-all.sh    # clippy, fmt, host-function audit, wasm-exports check, build+test, markdown lint, e2e
```
