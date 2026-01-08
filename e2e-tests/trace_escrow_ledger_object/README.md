# trace_escrow_ledger_object e2e-test

This WebAssembly module traces every field of an Escrow ledger object to validate functionality between this library and xrpld.

The test exercises:

- All Escrow ledger object fields from the `CurrentEscrowFields` trait
- Common ledger object fields from the `CurrentLedgerObjectCommonFields` trait
- Loading the current escrow ledger object using `get_current_escrow()`
- Crypto-condition fields (Condition) in full 39-byte format

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
./scripts/run-tests.sh e2e-tests/trace_escrow_ledger_object
```

### 3b. Deploy and test on Devnet

```shell
cd ../..
DEVNET=true ./scripts/run-tests.sh e2e-tests/trace_escrow_ledger_object
```

This will:

- Connect to WASM Devnet (or local rippled)
- Create and fund two wallets (source and destination)
- Create an EscrowCreate transaction with Condition and your compiled `FinishFunction`
- Finish the escrow with Fulfillment and ComputationAllowance
- Execute the WASM which traces all Escrow ledger object fields

Expected result: `tesSUCCESS` with comprehensive tracing of all Escrow ledger object fields, including the Condition in full crypto-condition format.
