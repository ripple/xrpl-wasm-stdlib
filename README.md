# xrpl-wasm-stdlib Library

The XRPL Standard Library provides safe, type-safe access to XRPL host functions for WebAssembly smart contract development. This `no_std` library offers zero-cost abstractions over raw host function calls and handles memory management, error handling, and type conversions.

## Quick Start

There is an interface available at <https://ripple.github.io/xrpl-wasm-stdlib/ui/> for local or Devnet testing.

### Examples Overview

- **[hello_world](https://github.com/ripple/xrpl-wasm-stdlib/tree/main/examples/smart-escrows/hello_world/)** - Basic escrow with logging
- **[oracle](https://github.com/ripple/xrpl-wasm-stdlib/tree/main/examples/smart-escrows/oracle/)** - Price-based release using oracle data
- **[kyc](https://github.com/ripple/xrpl-wasm-stdlib/tree/main/examples/smart-escrows/kyc/)** - Credential-based verification
- **[notary](https://github.com/ripple/xrpl-wasm-stdlib/tree/main/examples/smart-escrows/notary/)** - Multi-signature authorization
- **[nft_owner](https://github.com/ripple/xrpl-wasm-stdlib/tree/main/examples/smart-escrows/nft_owner/)** - NFT ownership verification
- **[ledger_sqn](https://github.com/ripple/xrpl-wasm-stdlib/tree/main/examples/smart-escrows/ledger_sqn/)** - Sequence-based release

## Installation

A Smart Escrow needs two crates:

```shell
cargo add xrpl-common-stdlib xrpl-escrow-stdlib
```

| Crate                | Provides                                                                                                              |
| -------------------- | --------------------------------------------------------------------------------------------------------------------- |
| `xrpl-common-stdlib` | Host bindings, transaction and ledger-object field access, XRPL types, and every macro re-exported from `xrpl-macros` |
| `xrpl-escrow-stdlib` | `EscrowFinishContext`, `FinishResult`, and the escrow-only host functions                                             |

Both are needed: `xrpl-escrow-stdlib` deliberately does not re-export `xrpl-common-stdlib`, so the
general layer stays usable on its own and the dependency direction stays visible in every contract's
manifest.

You never need to depend on `xrpl-macros` directly. It is an internal crate, published only because
Rust requires procedural macros to live in their own crate, and the code it generates refers to
`xrpl_common_stdlib` paths — so it is not usable on its own. `xrpl-common-stdlib` re-exports
`#[smart_escrow]`, `#[smart_contract]`, and the typed-constant macros (`r_address!`, `hash256!`,
`pubkey!`, `currency!`, `blob!`).

All four published crates are versioned in lockstep, so keep whichever ones you name on matching
versions — `cargo add` does that for you on a fresh contract.

Contracts target `wasm32v1-none` and set `crate-type = ["cdylib"]`; see
[hello_world](https://github.com/ripple/xrpl-wasm-stdlib/tree/main/examples/smart-escrows/hello_world/)
for a complete manifest, and the [Complete Developer Guide](https://ripple.github.io/xrpl-wasm-stdlib/xrpl_common_stdlib/guide/index.html)
for the release profile a size-constrained contract wants.

### Testing a contract

`xrpl-stdlib-test-utils` stands in for the XRPL host so contract logic can be unit-tested with plain
`cargo test` — no node, no WASM. Gate it to non-WASM targets: it pulls in `mockall`, which is not
`no_std` and will not build for `wasm32v1-none`.

```shell
cargo add --dev --target 'cfg(not(target_arch = "wasm32"))' xrpl-stdlib-test-utils
```

That lands it under `[target.'cfg(not(target_arch = "wasm32"))'.dev-dependencies]`.

```rust,ignore
use xrpl_common_stdlib::types::amount::Amount;
use xrpl_stdlib_test_utils::EscrowScenario;

#[test]
fn releases_above_ten_xrp() {
    let _guard = EscrowScenario::builder()
        .with_amount(Amount::XRP { num_drops: 20_000_000 })
        .install();
    // ... call your contract's logic and assert on the result
}
```

Unset scenario facts fall back to defaults; see
[the crate's README](https://github.com/ripple/xrpl-wasm-stdlib/tree/main/xrpl-stdlib-test-utils) for
the full builder. If you only want the raw mock, enable `xrpl-common-stdlib`'s `test-host-bindings`
feature directly under the same `cfg` gate.

## Documentation

| Section                                                                                                       | Description                                     |
| ------------------------------------------------------------------------------------------------------------- | ----------------------------------------------- |
| **[Complete Developer Guide](https://ripple.github.io/xrpl-wasm-stdlib/xrpl_common_stdlib/guide/index.html)** | Comprehensive guide with working internal links |
| **[Rust API Docs](https://ripple.github.io/xrpl-wasm-stdlib)**                                                | Generated API documentation (`cargo doc`)       |

The complete developer guide includes:

- Getting Started - Installation, first contract, core concepts
- API Reference - Complete API documentation and usage patterns
- Examples - Smart escrow examples and tutorials
- Development Guide - Building, testing, and CI setup

## Key Features

- **Type-safe access** to transaction and ledger data
- **Memory-safe operations** with no heap allocations
- **Deterministic execution** across all nodes/validators
- **Zero-cost abstractions** over host functions
- **Comprehensive error handling** with custom `Result` types

## Safety and Constraints

Smart escrows run in a constrained WebAssembly environment:

- **Read-only ledger access** (except escrow data updates)
- **Deterministic execution** required
- **Resource limits** enforced
- **No network/file system** access

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md) for detailed guidelines on:

- Development setup and workflow
- Code standards and style guidelines
- Pull request process
- Testing requirements
- Release procedures

We welcome contributions of all kinds!
