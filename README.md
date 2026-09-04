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

| Crate                | Provides                                                                  |
| -------------------- | ------------------------------------------------------------------------- |
| `xrpl-common-stdlib` | Host bindings, transaction and ledger-object field access, and XRPL types |
| `xrpl-escrow-stdlib` | `EscrowFinishContext`, `FinishResult`, and the escrow-only host functions |

Do not add `xrpl-macros`. It is internal, and `xrpl-common-stdlib` re-exports everything from it: `#[smart_escrow]`, `#[smart_contract]`, `r_address!`, `hash256!`, `pubkey!`, `currency!`, and `blob!`.

Contracts target `wasm32v1-none` and set `crate-type = ["cdylib"]`. See [hello_world](https://github.com/ripple/xrpl-wasm-stdlib/tree/main/examples/smart-escrows/hello_world/) for a full manifest, and the [Complete Developer Guide](https://ripple.github.io/xrpl-wasm-stdlib/xrpl_common_stdlib/guide/index.html) for the release profile.

### Testing a contract

`xrpl-stdlib-test-utils` mocks the XRPL host, so you can test contract logic with `cargo test` — no node, no WASM. It depends on `mockall`, which is not `no_std`, so gate it to non-WASM targets:

```shell
cargo add --dev --target 'cfg(not(target_arch = "wasm32"))' xrpl-stdlib-test-utils
```

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

Anything you leave unset gets a default. See [the crate's README](https://github.com/ripple/xrpl-wasm-stdlib/tree/main/xrpl-stdlib-test-utils) for the full builder.

### Running the examples' integration tests

Each example under `examples/smart-escrows/` has an integration test that submits real transactions to a rippled node:

```shell
./scripts/run-tests.sh                                     # all examples + e2e contracts
./scripts/run-tests.sh examples/smart-escrows/hello_world   # a single example
```

If rippled is already running on `ws://localhost:6006` (your own instance, or a container left over from a previous run), that's reused automatically — no Docker needed. Otherwise this starts a local rippled in Docker, pinned to the exact image [CI uses](.github/workflows/test.yml), so your results match CI without any manual node setup. Override with:

| Variable         | Effect                                                                                    |
| ---------------- | ----------------------------------------------------------------------------------------- |
| `DEVNET=true`    | Test against WASM Devnet (`wss://wasm.devnet.rippletest.net:51233`) instead               |
| `NO_DOCKER=true` | Force-skip Docker; use a rippled you're already running yourself on `ws://localhost:6006` |

The Docker node is left running across test runs for speed; stop it with `./scripts/docker-rippled.sh stop` when you're done. See [`scripts/README.md`](./scripts/README.md) and [`CONTRIBUTING.md`](./CONTRIBUTING.md) for more on the local dev/test scripts.

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
