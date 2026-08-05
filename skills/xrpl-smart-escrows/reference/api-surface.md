# API Surface

Everything here is what a contract author calls directly. Internal host-binding plumbing is out of scope (see [architecture.md](architecture.md) if curious).

## Entry point

```rust
use xrpl_escrow_stdlib::{EscrowFinishContext, FinishResult};
use xrpl_macros::smart_escrow;

#[smart_escrow]
fn my_check(ctx: EscrowFinishContext) -> FinishResult { /* ... */ }
```

Rules enforced at compile time: the annotated fn takes exactly `EscrowFinishContext` and returns `FinishResult` or `i32`; no attribute arguments; the fn must not be named `escrow_finish` (that's the generated `extern "C"` export symbol). Import `smart_escrow` via `xrpl_escrow_stdlib`/`xrpl_macros` — the generated code references `xrpl_escrow_stdlib` types.

`#[smart_contract]` is the analogous entry-point macro for a future non-escrow smart-contract feature family; no example in this repo uses it yet.

### `FinishResult`

```rust
impl FinishResult {
    pub const fn succeed() -> Self;                     // -> 1
    pub const fn reject() -> Self;                       // -> 0
    pub fn succeed_with<const N: i32>() -> Self;          // N must be > 0
    pub fn reject_with<const N: i32>() -> Self;           // N must be <= 0
}
```

`From<i32> for FinishResult` exists, so `return e.code().into();` propagates a host error code as a rejection.

### `EscrowFinishContext`

```rust
impl EscrowFinishContext {
    pub fn escrow(&self) -> &CurrentEscrow;
    pub fn tx(&self) -> &EscrowFinish;                                 // via SmartFeatureContext trait
    pub fn update_data(&self, data: &[u8]) -> host::Result<()>;        // write the escrow's Data field
}
```

## Reading the current transaction — `ctx.tx()`

`EscrowFinish` implements `TransactionCommonFields` (shared by every tx type) plus `EscrowFinishFields` (escrow-finish-specific).

```rust
use xrpl_common_stdlib::current_tx::traits::TransactionCommonFields;

pub trait TransactionCommonFields {
    fn path(&self) -> TxPathBuilder;                       // nested-field access (memos, arrays)
    fn get_account(&self) -> Result<AccountID>;
    fn get_transaction_type(&self) -> Result<TransactionType>;
    fn get_gas(&self) -> Result<u32>;
    fn get_fee(&self) -> Result<Amount>;
    fn get_sequence(&self) -> Result<u32>;
    fn get_account_txn_id(&self) -> Result<Option<Hash256>>;
    fn get_flags(&self) -> Result<Option<u32>>;
    fn get_last_ledger_sequence(&self) -> Result<Option<u32>>;
    fn get_network_id(&self) -> Result<Option<u32>>;
    fn get_source_tag(&self) -> Result<Option<u32>>;
    fn get_signing_pub_key(&self) -> Result<Option<PublicKey>>;
    fn get_ticket_sequence(&self) -> Result<Option<u32>>;
    fn get_txn_signature(&self) -> Result<SignatureBlob>;
}
```

`EscrowFinishFields` adds: `get_owner()`, `get_offer_sequence()`, `get_condition()`, `get_fulfillment()` — the Owner/OfferSequence/Condition/Fulfillment identifying which `EscrowCreate` this finish targets.

Variable-length data (Memos, arrays) isn't exposed as a typed method — read it via `Locator`:

```rust
use xrpl_common_stdlib::fields::locator::Locator;
use xrpl_common_stdlib::host::get_tx_nested_field;

let mut locator = Locator::new();
locator.pack(sfield::Memos);
locator.pack(0);                    // index 0
locator.pack(sfield::MemoData);
let rc = unsafe {
    get_tx_nested_field(locator.as_ptr(), locator.num_packed_bytes(), buf.as_mut_ptr(), buf.len())
};
```

## Reading the escrow being finished — `ctx.escrow()`

`CurrentEscrow` implements `CurrentLedgerObjectCommonFields` (`path()`, `get_flags()`, `get_ledger_entry_type()`) plus `CurrentEscrowFields`:

```rust
pub trait CurrentEscrowFields: CurrentLedgerObjectCommonFields {
    fn get_account(&self) -> Result<AccountID>;
    fn get_amount(&self) -> Result<Amount>;
    fn get_cancel_after(&self) -> Result<Option<u32>>;
    fn get_condition(&self) -> Result<Option<ConditionBlob>>;
    fn get_destination(&self) -> Result<AccountID>;
    fn get_destination_node(&self) -> Result<Option<u64>>;
    fn get_destination_tag(&self) -> Result<Option<u32>>;
    fn get_finish_after(&self) -> Result<Option<u32>>;
    fn get_owner_node(&self) -> Result<u64>;
    fn get_previous_txn_id(&self) -> Result<Hash256>;
    fn get_previous_txn_lgr_seq(&self) -> Result<u32>;
    fn get_source_tag(&self) -> Result<Option<u32>>;
    fn get_bytecode(&self) -> Result<Option<WasmBlob>>;
    fn get_data(&self) -> Result<ContractData>;
    fn update_current_escrow_data(data: ContractData) -> Result<()>;
}
```

### Persistent contract state across finishes

```rust
use xrpl_escrow_stdlib::ledger_objects::escrow_storage::{EscrowStorage, load_data, save_data};

trait EscrowStorage: Sized {
    fn encode(&self, out: &mut [u8]) -> Result<usize>;
    fn decode(bytes: &[u8]) -> Result<Self>;
}
fn load_data<T: EscrowStorage>(ctx: &EscrowFinishContext) -> Result<Option<T>>;
fn save_data<T: EscrowStorage>(ctx: &EscrowFinishContext, data: &T) -> Result<()>;
```

Implement `EscrowStorage` for a plain struct to persist a small state machine (roles, flags, deadlines) in the escrow's 1024-byte `Data` field between `EscrowFinish` calls. See the `freelancer_escrow` pattern in [patterns.md](patterns.md).

## Reading other ledger objects

Two-step pattern for any object besides the current escrow:

```rust
use xrpl_common_stdlib::keylets::escrow_keylet;   // or account_keylet, oracle_keylet, credential_keylet, ...
use xrpl_common_stdlib::host;

let keylet = escrow_keylet(&owner, sequence)?;
let slot = unsafe { host::cache_ledger_obj(keylet.as_ptr(), keylet.len(), 0) };
if slot < 0 { return FinishResult::reject(); }
```

Then wrap the slot:

- **Typed wrapper**, if one exists — `AccountRoot { slot_num: slot }` (via `AccountFields` trait: `get_account`, `balance()`, `sequence`, `owner_count`, `domain`, `regular_key`, ... — full field set mirrors `AccountRoot` ledger entries), or `Escrow::new(slot)` (via `EscrowFields`, same method set as `CurrentEscrowFields` above, for inspecting an escrow _other than_ the one being finished — e.g. an atomic-swap counterpart).
- **Untyped `LedgerObject::new(slot)`**, for object types with no typed wrapper (Oracle, SignerList, NFTokenPage, RippleState). Read nested fields via the path builder:

```rust
use xrpl_common_stdlib::objects::LedgerObject;
use xrpl_common_stdlib::sfield;

let oracle = LedgerObject::new(slot);
let price: u64 = oracle.path()
    .field(sfield::PriceDataSeries)
    .index(0)
    .field(sfield::AssetPrice)
    .get::<u64>()?;
```

Convenience one-shot for the common case of "just give me this account's XRP/token balance":

```rust
pub fn get_account_balance(account_id: &AccountID) -> host::Result<Option<Amount>>;
```

### Keylet functions (`xrpl_common_stdlib::keylets`)

All return `Result<[u8; 32]>` (`KeyletBytes`, `XRPL_KEYLET_SIZE = 32`):

```
account_keylet(account_id)
amm_keylet(issue1, issue2)
check_keylet(owner, seq)
credential_keylet(subject, issuer, credential_type)
delegate_keylet(account, authorize)
deposit_preauth_keylet(account, authorize)
did_keylet(account_id)
escrow_keylet(owner, seq)
line_keylet(account1, account2, currency)
mpt_issuance_keylet(owner, seq)
mptoken_keylet(mptid, holder)
nft_offer_keylet(owner, seq)
offer_keylet(owner, seq)
oracle_keylet(owner, document_id)
paychan_keylet(account, destination, seq)
permissioned_domain_keylet(account, seq)
signers_keylet(account_id)
ticket_keylet(owner, seq)
vault_keylet(account, seq)
```

## Core types (`xrpl_common_stdlib::types`)

| Type                                                                                     | Purpose                                                                                                               | Key methods                                                                                                                                     |
| ---------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| `AccountID([u8;20])`                                                                     | XRPL account                                                                                                          | `From<[u8;20]>`; usable directly with `r_address!`                                                                                              |
| `Amount`                                                                                 | `XRP { num_drops: i64 }` \| `IOU { amount: IOUNumber, issuer, currency }` \| `MPT { num_units, is_positive, mpt_id }` | Pattern-match this for balance/payment checks. `to_stamount_bytes()`, `from_bytes()`. `AMOUNT_SIZE = 48`.                                       |
| `Number([u8;12])`                                                                        | Opaque decimal (mantissa × 10^exponent) for precise math delegated to the host                                        | `float_from_int/uint`, `from_mant_exp`, `float_to_int()`, `float_to_mant_exp()`, `compare() -> Ordering`, constants `ZERO`/`ONE`/`NEGATIVE_ONE` |
| `IOUNumber([u8;8])`                                                                      | The 8-byte mantissa/exponent inside an IOU `Amount`                                                                   | `is_positive()`, `is_zero()`, `exponent()`, `mantissa()`, `to_number()`                                                                         |
| `UInt<const N>` (+ `Hash128/160/192/256` aliases)                                        | Fixed-size hash/hex types                                                                                             | `as_bytes()`                                                                                                                                    |
| `Blob<const N>` (+ `ConditionBlob`, `WasmBlob`, `UriBlob`, `SignatureBlob`, ... aliases) | Fixed-capacity byte buffer                                                                                            | `new()`, `from_slice()`, `len()`, `is_empty()`, `capacity()`, `as_slice()`                                                                      |
| `Currency([u8;20])`                                                                      | Currency code                                                                                                         | `new(code)`, `as_bytes()`; `From<[u8;3]>` for standard codes                                                                                    |
| `NFToken([u8;32])` + `NftFlags(u16)`                                                     | NFT ID and flags                                                                                                      | `flags()`, `transfer_fee()`, `issuer()`, `taxon()`, `token_sequence()`, `uri(&owner)` (also doubles as an ownership check)                      |
| `Issue` enum (`XrpIssue`/`IouIssue`/`MptIssue`)                                          | Asset identity for `amm_keylet`/`line_keylet`                                                                         | `IouIssue::new(issuer, currency)`, `as_bytes()`                                                                                                 |
| `MptId([u8;24])`                                                                         | Multi-purpose token ID                                                                                                | `new(sequence_num, issuer)`, `get_sequence_num()`, `get_issuer()`                                                                               |
| `PublicKey([u8;33])`                                                                     | Signing key                                                                                                           | `From<[u8;33]>`, `From<[u8;64]>`                                                                                                                |
| `ContractData`                                                                           | Escrow `Data` field buffer, 1024 bytes                                                                                | returned by `get_data()`, consumed by `update_current_escrow_data()`                                                                            |
| `TransactionType` enum                                                                   | Result of `get_transaction_type()`                                                                                    | `From<[u8;2]>`, `From<i16>`                                                                                                                     |
| `constants` module                                                                       | —                                                                                                                     | `ACCOUNT_ZERO`, `ACCOUNT_ONE`, `ONE_DROP = 1`, `MAX_XRP = 100_000_000_000`, `MAX_DROPS`                                                         |

## Typed-constant macros (`xrpl_macros`)

Compile-time validated, literals only:

```rust
const NOTARY: AccountID = r_address!("rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh");
const H: Hash256 = hash256!("0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF");
const KEY: PublicKey = pubkey!("02C7387FFC25C156CA7F8A6D760C8D01EF642CEE9CE4680C33FFB3FF39AFECFE70");
const C: Currency = currency!("USD");                                       // 3-char code
const C2: Currency = currency!("0158415500000000C1F76FF6ECB0BAC600000000"); // 40-hex non-standard
const B: Blob<4> = blob!("DEADBEEF");                                       // exact-fit
const B2: Blob<8> = blob!("DEADBEEF", 8);                                   // zero-padded to capacity
```

## Trace / debug (`xrpl_common_stdlib::host::trace`)

```rust
pub fn trace(msg: &str) -> Result<i32>;
pub fn trace_data(msg: &str, data: &[u8], data_repr: DataRepr) -> Result<i32>;  // DataRepr::AsUTF8 | AsHex
pub fn trace_num(msg: &str, number: i64) -> Result<i32>;
pub fn trace_account(msg: &str, account_id: &AccountID) -> Result<i32>;
pub fn trace_amount(msg: &str, amount: &Amount) -> Result<i32>;
pub fn trace_float(msg: &str, f: &[u8; 8]) -> Result<i32>;
```

Output lands in rippled's `debug.log`. Convention used throughout every example: on an error path, call `trace_num("<context>", e.code() as i64)` before returning the rejection, and discard the trace call's own result with `let _ = ...`.
