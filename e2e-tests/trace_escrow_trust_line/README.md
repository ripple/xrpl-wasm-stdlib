# trace_escrow_trust_line e2e-test

This WebAssembly module loads a RippleState (trust line) ledger object and traces every field, to
validate functionality between this library and xrpld.

Its distinguishing purpose is to exercise the **IOU `Amount` decode path against real rippled bytes**.
Trust lines are the only ledger object whose `Balance`/`LowLimit`/`HighLimit` are always IOU amounts
(a 48-byte STAmount with sign/exponent/mantissa + currency + issuer). The other trace tests
(`trace_escrow_account`, `trace_escrow_ledger_object`) only ever decode XRP amounts, and the mocked
unit tests zero-fill their buffers — which routes through the XRP branch of `Amount::from_bytes`. So
this is the one place a real IOU `Amount` is decoded off the ledger end-to-end.

The test exercises:

- Loading a RippleState object via `TrustLine::load(account1, account2, currency)`
  (`line_keylet` + `cache_ledger_obj`)
- All `TrustLineFields` accessors: `balance`, `low_limit`, `high_limit`, `low_node`, `high_node`,
  the four quality fields, and `previous_txn_id` / `previous_txn_lgr_seq`
- Asserting (WASM only) that `Balance`/`LowLimit`/`HighLimit` decode as `Amount::IOU` in the
  configured currency, and that the two limit issuers are the trust line's two parties

## Test setup (`runTest.js`)

1. Fund a **holder** account and enable `DefaultRipple` on the **issuer** (`destWallet`).
2. Create a `USD` trust line between holder and issuer (`TrustSet`).
3. Issue IOU to the holder (`Payment`) so the line carries a non-zero balance.
4. Create an escrow owned by the holder with the issuer as `Destination` and this contract as its
   `FinishFunction`.
5. Finish the escrow to trigger WASM execution.

The contract reconstructs the trust line's two parties from the finishing account (holder) and the
escrow destination (issuer).

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
./scripts/run-tests.sh e2e-tests/trace_escrow_trust_line
```

### 3b. Deploy and test on Devnet

```shell
cd ../..
DEVNET=true ./scripts/run-tests.sh e2e-tests/trace_escrow_trust_line
```
