# NFT Owner Smart Escrow

A smart escrow for the XRPL that unlocks based on NFT ownership verification.

## Overview

This smart escrow unlocks when the escrow destination account owns a specific NFT. The NFT ID is provided through the
transaction memo field, and the contract verifies ownership before allowing the escrow to complete.

- Retrieves NFT ID from the transaction memo data
- Verifies that the escrow destination account owns the specified NFT
- Returns `true` if the NFT is owned by the destination, `false` otherwise

The Rust code demonstrates how to interact with XRPL NFT objects and escrow data using the XRPL standard library.

## Functionality

### Core Components

- **Memo Processing**: Extracts NFT ID from the first memo in the transaction
- **Escrow Integration**: Retrieves the destination account from the current escrow object
- **NFT Verification**: Checks if the destination account owns the specified NFT
- **Error Handling**: Graceful failure when memo data is missing or NFT lookup fails

### Key Functions

- `finish()`: Main entry point that determines escrow unlock status
- `get_first_memo()`: Extracts the first memo data from the transaction
- NFT ownership verification using `get_nft()` from the XRPL standard library

## How it Works

The contract follows this workflow:

1. **Extract NFT ID**: Reads the first 32 bytes from the transaction memo as the NFT ID
2. **Get Destination**: Retrieves the destination account from the current escrow
3. **Verify Ownership**: Checks if the destination account owns the specified NFT
4. **Return Result**: Returns `1` (true) if owned, error code otherwise

Pseudo-code:

```
function finish() {
    nftId = getFirstMemo()[0:32]
    destination = getCurrentEscrow().destination
    return hasNFT(destination, nftId)
}
```

## Configuration

The contract expects:

- **NFT ID**: 32-byte NFT identifier provided in the transaction memo
- **Destination Account**: Automatically retrieved from the escrow object

## Building

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
./target/wasm32v1-none/release/nft_owner.wasm
```

### 3. Deploy and test on Devnet

Use the test script to deploy an escrow and test the FinishFunction.

```shell
cd ../../..
DEVNET=true ./scripts/run-tests.sh examples/smart-escrows/nft_owner
```

This will:

- Connect to WASM Devnet
- Create and fund two wallets (Origin and Destination)
- Create an EscrowCreate transaction with your compiled `FinishFunction`
- Submit an `EscrowFinish` transaction with the NFT ID in the memo field
- Verify that the escrow unlocks only if the destination account owns the specified NFT

## Use Cases

This NFT ownership pattern can be used for:

1. **Collectible Trading** - Escrows that unlock when the buyer proves ownership of a specific NFT
2. **Membership Verification** - Access control based on NFT membership tokens
3. **Proof of Achievement** - Unlocking rewards when users demonstrate ownership of achievement NFTs
4. **Gaming Assets** - Escrows tied to ownership of specific in-game NFT items
5. **Digital Art Sales** - Conditional payments based on NFT ownership verification

## Transaction Structure

The transaction must include a memo with the NFT ID:

```json
{
  "TransactionType": "EscrowFinish",
  "Account": "rAccount...",
  "Destination": "rDestination...",
  "Memos": [
    {
      "Memo": {
        "MemoData": "NFT_ID_32_BYTES_HEX_ENCODED"
      }
    }
  ]
}
```

## Error Handling

The contract handles various error scenarios:

| Scenario                     | Behavior        | Return Code |
| ---------------------------- | --------------- | ----------- |
| Missing memo                 | Escrow fails    | `0`         |
| Invalid memo format          | Escrow fails    | Error code  |
| NFT not found                | Escrow fails    | Error code  |
| NFT not owned by destination | Escrow fails    | Error code  |
| Valid NFT ownership          | Escrow succeeds | `1`         |

## Project Structure

This project is intentionally kept as an independent Rust project, separate from the main workspace. This allows:

- Independent building and testing
- Project-specific target directory
- Clear separation of the WASM module from the host application

## Security Considerations

- **Memo Validation**: Contract assumes first 32 bytes of memo contain valid NFT ID
- **Ownership Verification**: Relies on XRPL ledger state for NFT ownership
- **Error Propagation**: Fails safely when NFT data is unavailable
- **Input Sanitization**: Limited validation of memo data format

## Future Enhancements

- [ ] Support for multiple NFT IDs in memo
- [ ] Configurable memo field selection
- [ ] NFT metadata validation
- [ ] Time-based ownership requirements
- [ ] Enhanced error reporting with specific failure reasons
- [ ] Support for NFT collection-based verification
