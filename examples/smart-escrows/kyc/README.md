# KYC Credential Escrow

This WebAssembly module implements a KYC (Know Your Customer) credential-based escrow finish condition.

## How it works

The contract checks whether the destination account has a credential with the type "termsandconditions". If the credential exists in the ledger, it returns 1 (allow), otherwise it returns 0 (deny).

## Function

`finish() -> i32` — returns 1 to allow finishing the escrow, 0 to reject. On host errors, the function returns a non-zero error code from the host.

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
./target/wasm32v1-none/release/kyc.wasm
```

### 3. Deploy and test on Devnet

Use the test script to deploy an escrow and test the FinishFunction.

```shell
cd ../../..
DEVNET=true ./scripts/run-tests.sh examples/smart-escrows/kyc
```

This will:

- Connect to WASM Devnet
- Create and fund two wallets (Origin and Destination)
- Create an EscrowCreate transaction with your compiled `FinishFunction`
- First attempt to finish the escrow (should fail with `tecWASM_REJECTED`)
- Create a "termsandconditions" credential for the destination account
- Second attempt to finish the escrow (should succeed with `tesSUCCESS`)

Expected result: `tesSUCCESS` and "Escrow finished successfully!" on the second attempt after the credential is created.

## Testing Flow

1. **Initial Finish Attempt**: The escrow finish will fail because no credential exists yet
2. **Credential Creation**: A `CredentialCreate` transaction adds the required "termsandconditions" credential
3. **Successful Finish**: The escrow finish succeeds because the credential now exists

## Notes

- The contract looks for a credential with type "termsandconditions" on the destination account
- The credential must exist in the ledger state for the escrow to be finishable
- This pattern can be used for compliance scenarios where users must complete KYC before accessing funds
