# Hello World Escrow

This WebAssembly module implements a simple escrow finish function that emits the text "Hello World" into the trace log
and always returns 1. This example is useful for getting started with a new Smart Escrow.

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
./target/wasm32v1-none/release/helloworld.wasm
```

### 3. Deploy and test on Devnet

Use the test script to deploy an escrow and test the FinishFunction.

```shell
cd ../../..
DEVNET=true ./scripts/run-tests.sh examples/smart-escrows/hello_world
```

This will:

- Connect to WASM Devnet
- Create and fund two wallets (Origin and Destination)
- Create an EscrowCreate transaction with your compiled `FinishFunction`
- Finish the escrow, executing the `helloworld` WASM

Expected result: `tesSUCCESS` and “Escrow finished successfully!”.
