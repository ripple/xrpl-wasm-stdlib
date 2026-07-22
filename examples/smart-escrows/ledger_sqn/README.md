# Ledger Sequence Escrow

This WebAssembly module implements a simple ledger sequence-based escrow finish condition.

## How it works

The contract retrieves the current ledger sequence number and checks if it's greater than 5. Since ledger sequences start from 1 and increment with each ledger, this condition is almost always met in practice, making this a basic demonstration example.

## Function

The entry point, `check_ledger_sqn(_ctx: EscrowFinishContext) -> i32`, is annotated with `#[smart_escrow]`, which
generates the `extern "C" fn finish() -> i32` export the XRPL host calls. It returns `1` directly (as `i32`, no
`FinishResult` wrapper needed here) if ledger sequence > 5 (allow), `0` otherwise (deny). On host errors, the
function panics rather than propagating an error code — see [`src/lib.rs`](./src/lib.rs).

## Prerequisites

- Rust toolchain with `wasm32v1-none` target
- Node.js 18+

## Step-by-step: Use on WASM Devnet

This guide uses the public Devnet WASM endpoint at `wss://wasm.devnet.rippletest.net:51233`.

### 1. Install dependencies

```shell
npm install
```

### 2. Build the WASM

```shell
cargo build --target wasm32v1-none --release
```

Artifact:

```
./target/wasm32v1-none/release/ledger_sqn.wasm
```

### 3. Deploy and test on Devnet

Use the test script to deploy an escrow and test the FinishFunction.

```shell
cd ../../..
DEVNET=true ./scripts/run-tests.sh examples/smart-escrows/ledger_sqn
```

This will:

- Connect to WASM Devnet
- Create and fund two wallets (Origin and Destination)
- Create an EscrowCreate transaction with your compiled `FinishFunction`
- Finish the escrow, executing the `ledger_sqn` WASM

Expected result: `tesSUCCESS` and "Escrow finished successfully!" (since ledger sequence will be > 5).

## Notes

- This is a basic demonstration example - the condition (ledger sequence > 5) is almost always true
- In practice, you might use higher sequence numbers or time-based conditions for more realistic scenarios
- The contract demonstrates how to access ledger state information from within WASM
- Useful as a starting point for building more complex time or block-based escrow conditions
