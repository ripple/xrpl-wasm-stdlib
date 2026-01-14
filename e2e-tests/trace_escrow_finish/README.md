# trace_escrow_finish e2e-test

This WebAssembly module traces every field of an EscrowFinish transaction to validate functionality between this library and xrpld.

The test exercises:

- All 13 common transaction fields from the `TransactionCommonFields` trait
- All 4 EscrowFinish-specific fields from the `EscrowFinishFields` trait
- Transaction arrays: Memos, Signers, CredentialIDs
- Multi-signed transactions with multiple signers

### 1. Install dependencies

```shell
npm install
```

### 2. Build the WASM

```shell
cargo build --target wasm32v1-none --release
```

### 3a. Deploy and test Locally

```shell
cd ../..
./scripts/run-tests.sh e2e-tests/trace_escrow_finish
```

### 3b. Deploy and test on Devnet

```shell
cd ../..
DEVNET=true ./scripts/run-tests.sh e2e-tests/trace_escrow_finish
```

This will:

- Connect to WASM Devnet (or local rippled)
- Create and fund wallets (source, destination, and signers)
- Set up a SignerList for multi-signing
- Create an EscrowCreate transaction with Condition and your compiled `FinishFunction`
- Finish the escrow with a multi-signed transaction including Fulfillment, Memos, and CredentialIDs
- Execute the WASM which traces all EscrowFinish transaction fields

Expected result: `tesSUCCESS` with comprehensive tracing of all EscrowFinish transaction fields, including multi-signature data.
