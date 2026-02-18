# Oracle Smart Escrow

A smart escrow for the XRPL that unlocks based on oracle price data.

## Overview

This smart escrow unlocks based on price, per an oracle.

- Retrieves price data from an XRPL oracle object
- Evaluates whether the price meets a specified threshold (> 1)
- Returns `true` if the escrow should unlock, `false` otherwise

The Rust code demonstrates how to interact with XRPL oracle objects using the XRPL standard library.

## Functionality

### Core Components

- **Oracle Integration**: Connects to XRPL oracle objects using keylet as address
- **Price Retrieval**: Extracts `AssetPrice` data from `PriceDataSeries` within oracle objects
- **Threshold Logic**: Simple escrow unlock condition (price > 1)
- **Error Handling**: Graceful failure when oracle data is unavailable (e.g., if the oracle does not exist)

### Key Functions

- `finish()`: Main entry point that determines escrow unlock status
- `get_price_from_oracle(slot)`: Retrieves price from cached oracle object
- `get_u64_from_buffer(bytes)`: Converts big-endian bytes to u64 price value

## Configuration

The oracle is configured with hardcoded parameters:

```rust
const ORACLE_OWNER: AccountID = AccountID(*b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3");
const ORACLE_DOCUMENT_ID: i32 = 1;
```

## Prerequisites

- Rust toolchain with `wasm32v1-none` target
- Node.js 18+

## Step-by-step: Use on WASM Devnet

This guide uses the public Devnet WASM endpoint at `wss://wasm.devnet.rippletest.net:51233`.

### 1. Install dependencies

```shell
npm install
```

The oracle owner and document ID are hardcoded in the source code. To use different values, edit `src/lib.rs` and modify the `ORACLE_OWNER` and `ORACLE_DOCUMENT_ID` constants.

### 2. Build the WASM

```shell
cargo build --target wasm32v1-none --release
```

Artifact:

```
./target/wasm32v1-none/release/oracle.wasm
```

### 3. Deploy and test on Devnet

Use the test script to deploy an escrow and test the FinishFunction.

```shell
cd ../../..
DEVNET=true ./scripts/run-tests.sh examples/smart-escrows/oracle
```

This will:

- Connect to WASM Devnet
- Create and fund two wallets (Origin and Destination)
- Create an oracle object with price data
- Create an EscrowCreate transaction with your compiled `FinishFunction`
- Submit an `EscrowFinish` transaction
- Verify that the escrow unlocks only if the oracle price is greater than 1

## Testing

### Integration Test Suite

A comprehensive integration test is available at [`../oracle_integration_test/`](../oracle_integration_test/):

```bash
# Build integration tests
cd ../oracle_integration_test
cargo build --target wasm32v1-none
cargo build --target wasm32v1-none --release
```

### Test Coverage

The integration test suite covers:

#### Core Functionality

- Oracle keylet (address derivation) generation with known parameters
- Ledger object caching and retrieval
- Nested field extraction (`PriceDataSeries` → `AssetPrice`)
- Price threshold evaluation logic

#### Data Processing

- Big-endian byte conversion to u64
- Various price scenarios (0, 1, >1, edge cases)
- Buffer handling and data integrity

#### Error Handling

- Invalid oracle owner scenarios
- Missing ledger objects
- Failed price data retrieval
- Graceful degradation

#### Integration Testing

- End-to-end workflow validation
- Exact `finish()` function behavior
- Real-world oracle interaction patterns

### Test Scenarios

| Scenario        | Price | Expected Result   |
| --------------- | ----- | ----------------- |
| Zero price      | 0     | ❌ No unlock      |
| Threshold price | 1     | ❌ No unlock      |
| Above threshold | 2+    | ✅ Unlock         |
| Invalid oracle  | N/A   | ❌ Error handling |
| Missing data    | N/A   | ❌ Error handling |

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        XRP Ledger                               │
│                                                                 │
│  ┌─────────────────┐              ┌─────────────────────────┐   │
│  │   Oracle        │              │    Smart Escrow         │   │
│  │   Object        │              │    (WASM Module)        │   │
│  │                 │              │                         │   │
│  │ ┌─────────────┐ │              │ finish() function:      │   │
│  │ │ PriceData   │ │◄─────────────┤                         │   │
│  │ │ Series      │ │              │ 1. Generate Oracle      │   │
│  │ │             │ │              │    Keylet               │   │
│  │ │ └─AssetPrice│ │              │                         │   │
│  │ └─────────────┘ │              │ 2. Cache Ledger Object  │   │
│  └─────────────────┘              │                         │   │
│                                   │ 3. Extract Price Data   │   │
│                                   │                         │   │
│                                   │ 4. Evaluate Threshold   │   │
│                                   │    (price > 1)          │   │
│                                   │                         │   │
│                                   │ 5. Return bool result   │   │
│                                   └─────────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
```

### Deployment Considerations

1. **Oracle Setup**: Ensure the specified oracle owner and document ID exist
2. **Price Updates**: Oracle price data must be regularly updated by oracle operators
3. **Network Support**: Target network must have required amendments enabled

## Security Considerations

- **Oracle Dependency**: Contract relies on external oracle data availability
- **Price Manipulation**: Susceptible to oracle price manipulation attacks
- **Threshold Logic**: Simple threshold may need enhancement for production use
- **Error Handling**: Fails safely when oracle data is unavailable

## Future Enhancements

- [ ] Configurable price thresholds
- [ ] Multiple oracle data source support
- [ ] Time-based unlock conditions
- [ ] Enhanced error reporting
- [ ] Price trend analysis
- [ ] Oracle reliability scoring
