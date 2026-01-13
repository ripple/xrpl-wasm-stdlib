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

## Documentation

| Section                                                                                                     | Description                                     |
| ----------------------------------------------------------------------------------------------------------- | ----------------------------------------------- |
| **[Complete Developer Guide](https://ripple.github.io/xrpl-wasm-stdlib/xrpl_wasm_stdlib/guide/index.html)** | Comprehensive guide with working internal links |
| **[Rust API Docs](https://ripple.github.io/xrpl-wasm-stdlib)**                                              | Generated API documentation (`cargo doc`)       |

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
