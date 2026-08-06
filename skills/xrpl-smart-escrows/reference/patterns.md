# Patterns

Six recurring shapes, each grounded in a real example under `examples/smart-escrows/` in the `xrpl-wasm-stdlib` repo. Start from the closest pattern rather than writing a contract from scratch.

## 1. Identity check — `notary`

Release only if a specific account submitted the `EscrowFinish`. The simplest non-trivial contract: read one tx field, compare to a compile-time constant.

```rust
use xrpl_common_stdlib::current_tx::traits::TransactionCommonFields;
use xrpl_common_stdlib::host::trace::trace_num;
use xrpl_common_stdlib::host::Result::{Err, Ok};
use xrpl_common_stdlib::types::account_id::AccountID;
use xrpl_escrow_stdlib::EscrowFinishContext;
use xrpl_macros::{r_address, smart_escrow};

const NOTARY_ACCOUNT: AccountID = r_address!("rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh");

#[smart_escrow]
fn notary_finish(ctx: EscrowFinishContext) -> i32 {
    let tx_account = match ctx.tx().get_account() {
        Ok(v) => v,
        Err(e) => {
            let _ = trace_num("Error in Notary contract", e.code() as i64);
            return e.code();
        }
    };
    (tx_account == NOTARY_ACCOUNT) as i32
}
```

Generalizes to: multi-sig-like approval (check `get_signing_pub_key()` against a known key), source-tag-gated release, etc.

## 2. Oracle / external-price check — `oracle`

Release based on data in an unrelated ledger object, looked up by ledger entry ID. Demonstrates the untyped `LedgerObject` inner-field path for object types without a typed wrapper.

```rust
use xrpl_common_stdlib::host::{self, Error, Result, Result::{Err, Ok}};
use xrpl_common_stdlib::ledger_entry_ids::oracle_id;
use xrpl_common_stdlib::objects::LedgerObject;
use xrpl_common_stdlib::sfield;
use xrpl_common_stdlib::types::account_id::AccountID;
use xrpl_macros::r_address;
use xrpl_escrow_stdlib::{EscrowFinishContext, FinishResult};
use xrpl_macros::smart_escrow;

const ORACLE_OWNER: AccountID = r_address!("rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh");
const ORACLE_DOCUMENT_ID: u32 = 1;

fn get_price_from_oracle(slot: i32) -> Result<u64> {
    let oracle = LedgerObject::new(slot);
    oracle.path()
        .field(sfield::PriceDataSeries)
        .index(0)
        .field(sfield::AssetPrice)
        .get::<u64>()
}

#[smart_escrow]
fn oracle_finish(_ctx: EscrowFinishContext) -> FinishResult {
    let id = match oracle_id(&ORACLE_OWNER, ORACLE_DOCUMENT_ID) {
        Ok(k) => k,
        Err(_) => return FinishResult::reject(),
    };
    let slot = unsafe { host::cache_le(id.as_ptr(), id.len(), 0) };
    if slot < 0 { return FinishResult::reject(); }
    let price = get_price_from_oracle(slot).unwrap_or(0);
    ((price > 1) as i32).into()
}
```

## 3. Credential / NFT gate — `kyc`, `nft_owner`

Release only if the destination holds a specific credential or NFT. Existence of the keyed object _is_ the check — no field comparison needed.

```rust
use xrpl_common_stdlib::ledger_entry_ids::credential_id;
use xrpl_common_stdlib::host;
use xrpl_escrow_stdlib::ledger_objects::traits::CurrentEscrowFields;
use xrpl_escrow_stdlib::{EscrowFinishContext, FinishResult};
use xrpl_macros::smart_escrow;

#[smart_escrow]
fn kyc_finish(ctx: EscrowFinishContext) -> FinishResult {
    let account_id = match ctx.escrow().get_destination() {
        Ok(a) => a,
        Err(_) => return FinishResult::reject(),
    };
    let cred_type: &[u8] = b"termsandconditions";
    let id = match credential_id(&account_id, &account_id, cred_type) {
        Ok(k) => k,
        Err(_) => return FinishResult::reject(),
    };
    let slot = unsafe { host::cache_le(id.as_ptr(), id.len(), 0) };
    if slot < 0 { return FinishResult::reject(); }
    FinishResult::succeed()
}
```

For NFT ownership, use `NFToken::uri(&owner)` instead — a successful read of the URI for a given owner doubles as an ownership proof.

## 4. Time / sequence deadline — `ledger_sqn`

Release once a ledger-derived value crosses a threshold. Never use a host-side clock — only ledger-consensus values are deterministic across validators.

```rust
use xrpl_common_stdlib::host;
use xrpl_common_stdlib::host::error_codes::match_result_code_with_expected_bytes;
use xrpl_escrow_stdlib::EscrowFinishContext;
use xrpl_macros::smart_escrow;

#[smart_escrow]
fn check_ledger_sqn(_ctx: EscrowFinishContext) -> i32 {
    unsafe {
        let mut buf = [0u8; 4];
        let rc = host::ldgr_index(buf.as_mut_ptr(), buf.len());
        if match_result_code_with_expected_bytes(rc, 4, || Some(rc)).is_err() {
            return rc;
        }
        let ledger_sequence = u32::from_be_bytes(buf);
        (ledger_sequence >= 5) as i32
    }
}
```

For wall-clock-like deadlines use `host::parent_ldgr_time` instead of `ldgr_index`, and compare against the escrow's own `get_finish_after()`/`get_cancel_after()` where applicable rather than a hardcoded constant.

## 5. Multi-party state machine — `freelancer_escrow`

Persist typed state across multiple `EscrowFinish` calls (e.g. client/freelancer/arbitrator roles, milestone approvals) using `EscrowStorage` + `load_data`/`save_data`.

```rust
use xrpl_common_stdlib::current_tx::traits::TransactionCommonFields;
use xrpl_escrow_stdlib::ledger_objects::escrow_storage::{EscrowStorage, load_data, save_data};
use xrpl_escrow_stdlib::ledger_objects::traits::CurrentEscrowFields;
use xrpl_escrow_stdlib::{EscrowFinishContext, FinishResult};
use xrpl_macros::smart_escrow;

struct State { approved_by_client: bool, approved_by_freelancer: bool }

impl EscrowStorage for State {
    fn encode(&self, out: &mut [u8]) -> xrpl_common_stdlib::host::Result<usize> {
        out[0] = (self.approved_by_client as u8) | ((self.approved_by_freelancer as u8) << 1);
        Ok(1)
    }
    fn decode(bytes: &[u8]) -> xrpl_common_stdlib::host::Result<Self> {
        let b = bytes.first().copied().unwrap_or(0);
        Ok(State { approved_by_client: b & 1 != 0, approved_by_freelancer: b & 2 != 0 })
    }
}

#[smart_escrow]
fn escrow(ctx: EscrowFinishContext) -> FinishResult {
    let tx_account = ctx.tx().get_account().unwrap_or_default();
    let client = ctx.escrow().get_account().unwrap_or_default();
    let freelancer = ctx.escrow().get_destination().unwrap_or_default();

    let mut state = load_data::<State>(&ctx).ok().flatten()
        .unwrap_or(State { approved_by_client: false, approved_by_freelancer: false });

    if tx_account == client { state.approved_by_client = true; }
    if tx_account == freelancer { state.approved_by_freelancer = true; }

    let done = state.approved_by_client && state.approved_by_freelancer;
    if !done {
        let _ = save_data(&ctx, &state);
        return FinishResult::reject();
    }
    FinishResult::succeed()
}
```

Key idea: every `EscrowFinish` call re-enters this function from scratch — state that must survive between calls has to be explicitly written back with `save_data`/`ctx.set_data()`, since there's no in-memory persistence between invocations.

## 6. Cross-escrow atomic swap — `atomic_swap`

Two escrows, each finished separately, each checking the _other's_ state via a ledger entry ID stored in a memo or in its own `Data` field. Demonstrates `Locator` for reading tx memos and `Escrow::new(slot)` for inspecting a counterpart escrow.

```rust
use xrpl_common_stdlib::fields::locator::Locator;
use xrpl_common_stdlib::host::{self, tx_inner};
use xrpl_common_stdlib::sfield;
use xrpl_escrow_stdlib::ledger_objects::escrow::Escrow;
use xrpl_escrow_stdlib::ledger_objects::traits::CurrentEscrowFields;

// Read Memos[0].MemoData from the current EscrowFinish tx (e.g. the counterpart's ledger entry ID)
let mut locator = Locator::new();
locator.pack(sfield::Memos);
locator.pack(0);
locator.pack(sfield::MemoData);
let mut counterpart_id = [0u8; 32];
let rc = unsafe {
    tx_inner(locator.as_ptr(), locator.num_packed_bytes(),
        counterpart_id.as_mut_ptr(), counterpart_id.len())
};
if rc < 0 { return xrpl_escrow_stdlib::FinishResult::reject(); }

// Load the counterpart escrow and read its fields
let counterpart_slot = unsafe {
    host::cache_le(counterpart_id.as_ptr(), counterpart_id.len(), 0)
};
if counterpart_slot < 0 { return xrpl_escrow_stdlib::FinishResult::reject(); }
let counterpart_escrow = Escrow::new(counterpart_slot);
let counterpart_account = counterpart_escrow.get_account().unwrap_or_default();
```

Each side of the swap is a separate contract deployed on a separate escrow; the integration test (`runTest.js`) deploys and finishes both in the correct order — see [testing.md](testing.md).

## Choosing a pattern

| If the release condition is...                   | Start from            |
| ------------------------------------------------ | --------------------- |
| Who submitted the finish tx                      | notary (1)            |
| A price/value in another ledger object           | oracle (2)            |
| Whether the destination holds a credential/NFT   | kyc / nft_owner (3)   |
| A ledger sequence or time deadline               | ledger_sqn (4)        |
| Multiple parties must approve over several calls | freelancer_escrow (5) |
| Two escrows must resolve together or not at all  | atomic_swap (6)       |

Combine patterns freely — e.g. a deadline (4) that overrides a multi-party approval (5) is just an `||` in the final boolean.
