# trace_escrow_account e2e-test

This WebAssembly module traces every field of an AccountRoot ledger object to validate functionality between this library and xrpld.

The test exercises:

- All 21 AccountRoot fields from the `AccountFields` trait
- Common ledger object fields (Flags, LedgerEntryType) from the `LedgerObjectCommonFields` trait
- Loading arbitrary ledger objects using keylets and `cache_ledger_obj()`
- Asserting specific static values configured in the test (Domain, EmailHash, MessageKey, etc.)

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
./scripts/run-tests.sh e2e-tests/trace_escrow_account
```

### 3b. Deploy and test on Devnet

```shell
cd ../..
DEVNET=true ./scripts/run-tests.sh e2e-tests/trace_escrow_account
```

This will:

- Connect to WASM Devnet (or local rippled)
- Create and fund two wallets (Origin and Destination)
- Create an EscrowCreate transaction with your compiled `FinishFunction`
- Finish the escrow, executing the WASM which traces AccountRoot fields

Expected result: `tesSUCCESS` with comprehensive tracing of all AccountRoot fields.
