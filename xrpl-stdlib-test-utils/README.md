# xrpl-stdlib-test-utils

Test harness for XRPL Smart Escrow contracts: mock host bindings plus scenario builders, so a test
states escrow facts instead of wiring up individual host-function expectations.

Contracts read the ledger through host functions that only exist inside `rippled`. This crate stands
in for the host, so you can test contract logic with `cargo test` — no node, no WASM.

## Installation

```shell
cargo add xrpl-common-stdlib xrpl-escrow-stdlib
cargo add --dev --target 'cfg(not(target_arch = "wasm32"))' xrpl-stdlib-test-utils
```

The `--target` flag matters: it puts this crate under
`[target.'cfg(not(target_arch = "wasm32"))'.dev-dependencies]` instead of plain
`[dev-dependencies]`. This crate depends on `mockall`, which is not `no_std` and cannot build for
`wasm32v1-none`, so without that gate `cargo build --target wasm32v1-none` fails.

## Usage

`EscrowScenario` builds a mock with defaults already wired up, so a test overrides only the facts it
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
