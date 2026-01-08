# Notary Escrow FinishFunction

This WebAssembly module implements a notary-based escrow finish condition. It verifies that only a designated notary
account is allowed to finish the escrow.

### How it works

The contract checks whether the account submitting EscrowFinish matches the embedded notary account. If it matches, it
returns 1 (allow), otherwise 0 (deny).

### Function

`finish() -> i32` — returns 1 to allow finishing the escrow, 0 to reject (deny finishing). On host errors, the function
returns a non-zero error code from the host.

## Prerequisites

- Rust toolchain with `wasm32v1-none` target
- Node.js 18+
- Dependencies installed in `reference/js`:

## Step-by-step: Use on WASM Devnet

This guide uses the public Devnet WASM endpoint at `wss://wasm.devnet.rippletest.net:51233`.

### 1. Install dependencies

```shell
npm install
```

The notary address is hardcoded in the source code. To use a different account, edit `src/lib.rs` and modify the `NOTARY_ACCOUNT` constant.

### 2. Build the WASM

```shell
cargo build --target wasm32v1-none --release
```

Artifact:

```
./target/wasm32v1-none/release/notary.wasm
```

### 3. Deploy and test on Devnet

Use the test script to deploy an escrow and test the FinishFunction.

```shell
cd ../../..
DEVNET=true ./scripts/run-tests.sh examples/smart-escrows/notary
```

This will:

- Connect to WASM Devnet
- Create and fund wallets (including the notary account)
- Create an EscrowCreate transaction with your compiled `FinishFunction`
- Attempt to finish the escrow from a non-notary account (should fail with `tecWASM_REJECTED`)
- Finish the escrow from the notary account (should succeed with `tesSUCCESS`)

The escrow will only unlock if the transaction is submitted by the designated notary account.

## Local testing with wasm-host-simulator (optional)

You can also run the WASM locally with the included host emulator:

```shell
cd ../../../../
cargo run --package wasm-host-simulator --bin wasm-host-simulator -- --dir examples/smart-escrows/notary --project notary
```

## Modifying the notary account

The notary account is defined as a constant in `src/lib.rs` using the `r_address!` macro:

```rust
const NOTARY_ACCOUNT: [u8; 20] = r_address!("rN7n7otQDd6FczFgLdSqtcsAUxDkw6fzRH");
```

To use a different notary account, simply edit this line with your desired r-address. The macro validates the address at compile time and converts it to the 20-byte AccountID.

## Notes

- The contract compares raw 20-byte AccountIDs. Classic addresses are converted at compile-time by the `r_address!` macro.
- Make sure the hardcoded notary address in `src/lib.rs` matches the account you'll use in step 4 to submit `EscrowFinish`.
