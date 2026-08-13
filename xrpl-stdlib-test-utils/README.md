# xrpl-stdlib-test-utils

Test harness for XRPL Smart Escrow contracts: mock host bindings plus scenario builders that let a
test state escrow facts instead of wiring individual host-function expectations.

Contracts normally read the ledger through host functions that only exist inside `rippled`. This
crate stands in for the host on your development machine, so contract logic can be unit-tested with
plain `cargo test` — no node, no WASM, no integration harness.

## Installation

Declare it as a **dev-dependency gated to non-WASM targets**, alongside the two crates the contract
itself needs:

```shell
cargo add xrpl-common-stdlib xrpl-escrow-stdlib
cargo add --dev --target 'cfg(not(target_arch = "wasm32"))' xrpl-stdlib-test-utils
```

The second command lands it under `[target.'cfg(not(target_arch = "wasm32"))'.dev-dependencies]`
rather than plain `[dev-dependencies]`.

The `cfg` gate matters. This crate pulls in `mockall`, which is not `no_std` and cannot build for
`wasm32v1-none`. Gating it keeps the harness entirely out of the target your contract actually ships
to, so `cargo build --target wasm32v1-none --release` never sees it.

## Usage

`EscrowScenario` builds a mock pre-wired with sensible defaults, overriding only the facts your test
cares about. `install()` returns a guard that keeps the mock active for the rest of the scope:

```rust,ignore
use xrpl_common_stdlib::types::amount::Amount;
use xrpl_stdlib_test_utils::EscrowScenario;

#[test]
fn releases_above_ten_xrp() {
    let _guard = EscrowScenario::builder()
        .with_amount(Amount::XRP { num_drops: 20_000_000 })
        .install();

    assert!(my_contract_logic().is_success());
}
```

Builder methods:

| Method                          | Sets                                                            |
| ------------------------------- | --------------------------------------------------------------- |
| `with_account(AccountID)`       | The escrow's account                                            |
| `with_amount(Amount)`           | The escrowed amount                                             |
| `with_set_data_returns(Result)` | What a `set_data` call returns, for exercising the failure path |

Anything left unset falls back to `apply_default_expectations`. Use `build()` instead of `install()`
to get the `MockHostBindings` and add expectations by hand, and `mock_common::MockHostBindings`
directly when you want no scenario defaults at all.

## Versioning

Released in lockstep with `xrpl-common-stdlib`, `xrpl-escrow-stdlib`, and `xrpl-macros` — always use
matching versions. Because the mocks are generated from `xrpl-common-stdlib`'s `HostBindings` trait,
a mismatched pair will not compile.
