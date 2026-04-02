# ERC-20 MPT Wrapper Contract

An XLS-101 smart contract that exposes an [ERC-20](https://eips.ethereum.org/EIPS/eip-20)-compatible interface on top of XRPL native [Multi-Purpose Tokens (XLS-33)](https://github.com/XRPLF/XRPL-Standards/blob/master/XLS-0033d-multi-purpose-tokens/README.md).

## Why?

This contract makes it easier to transition from the EVM world to the XRPL world by providing a familiar ERC-20-like API, while using better mechanisms internally (native MPT operations, on-chain state, inner transactions).

## Design Overview

### Pseudo-Account Issuer Model

The contract's pseudo-account **is** the MPT issuer. During `ContractCreate`, the hardcoded `init` function reads `max_amount` from instance parameters and emits an `MPTokenIssuanceCreate` inner transaction with `tfMPTCanTransfer | tfMPTCanClawback` flags. The resulting sequence number is stored in `ContractData` so subsequent calls can derive the `MptId`.

### Transfer Architecture: Clawback + Payment

Since the contract is the issuer, it cannot simply "send" tokens on behalf of users. Instead, transfers use two inner transactions:

1. **Clawback** — the issuer (contract) pulls tokens from the sender's account
2. **Payment** — the issuer (contract) sends tokens to the recipient's account

This leverages XLS-34 (Clawback) and works because the issuance was created with `tfMPTCanClawback`.

### No Readonly Functions

Functions like `total_supply`, `balance_of`, and `allowance` are intentionally omitted. On XRPL, ledger state is publicly queryable — anyone can read MPT balances and supply directly from the ledger objects (`MPToken`, `MPTokenIssuance`) via standard XRPL APIs without invoking the contract.

### Storage Layout

Allowances are the only state the contract manages, stored under the key prefix `"allowances"` using a 40-byte composite key (owner `AccountID` ‖ spender `AccountID`). The MPT issuance sequence is stored under `"mpt_seq"`.

```
ContractData["mpt_seq"]                                    → u32
ContractData["allowances"][owner (20B) || spender (20B)]   → u64
```

### Events

| Event      | Fields                      |
| ---------- | --------------------------- |
| `Transfer` | `from`, `to`, `value`       |
| `Approval` | `owner`, `spender`, `value` |

## Exported Functions

| Function        | Parameters                                             | Returns      | Description                                                                      |
| --------------- | ------------------------------------------------------ | ------------ | -------------------------------------------------------------------------------- |
| `init`          | _(none; reads `max_amount: u64` from instance params)_ | 0 on success | Creates the MPT issuance during `ContractCreate`                                 |
| `transfer`      | `to: AccountID`, `amount: u64`                         | 0 on success | Clawback from caller + Payment to `to`                                           |
| `approve`       | `spender: AccountID`, `amount: u64`                    | 0 on success | Stores the allowance in `ContractData`                                           |
| `transfer_from` | `from: AccountID`, `to: AccountID`, `amount: u64`      | 0 on success | Validates allowance, Clawback from `from`, Payment to `to`, decrements allowance |

## Error Codes

| Code | Constant                     | Meaning                                       |
| ---- | ---------------------------- | --------------------------------------------- |
| `0`  | `SUCCESS`                    | Operation completed successfully              |
| `-1` | `ERR_CLAWBACK`               | Clawback inner transaction failed             |
| `-2` | `ERR_PAYMENT`                | Payment inner transaction failed              |
| `-3` | `ERR_ISSUANCE`               | MPT issuance creation failed                  |
| `-4` | `ERR_SEQUENCE`               | Failed to read the contract account sequence  |
| `-5` | `ERR_STORE`                  | Failed to store data in `ContractData`        |
| `-6` | `ERR_INSUFFICIENT_ALLOWANCE` | `transfer_from` caller's allowance is too low |

## Running the Tests

```bash
./scripts/run-tests.sh examples/contracts/erc20
```

Requires a rippled instance with XLS-101 (smart contracts), XLS-33 (MPT), and XLS-34 (Clawback) support enabled.
