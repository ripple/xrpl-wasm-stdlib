# Project: XRPL WebAssembly Standard Library

## Overview

A Rust `no_std` library providing type-safe, zero-cost abstractions for XRPL WebAssembly smart contract development. Smart escrows are conditional payment contracts that run deterministically across all XRPL validators.

## Commands

### Build & Test

```bash
./scripts/setup.sh              # Initial setup (run once)
./scripts/run-all.sh            # Full test suite (clippy, fmt, build, tests)
./scripts/build.sh              # Build debug mode
./scripts/build.sh release      # Build release mode
./scripts/run-tests.sh          # Run integration tests
./scripts/fmt.sh                # Check formatting
./scripts/clippy.sh             # Run linter
cargo test --workspace          # Unit tests only
```

### Build Targets

```bash
# WASM contracts MUST use wasm32v1-none target
cargo build --target wasm32v1-none --release

# Native workspace (for tests/tools)
cargo build --workspace
```

### Testing

```bash
# Test specific example
./scripts/run-tests.sh examples/smart-escrows/hello_world

# Test on devnet instead of local
DEVNET=true ./scripts/run-tests.sh

# Manual testing UI
# https://ripple.github.io/xrpl-wasm-stdlib/ui/
```

## Architecture

### Project Structure

```
xrpl-wasm-stdlib/
├── xrpl-wasm-stdlib/src/       # Core library
│   ├── core/                   # Transaction & ledger access
│   ├── host/                   # Host function bindings
│   ├── sfield/                 # Serialization fields
│   └── types/                  # XRPL types (AccountID, Amount, etc.)
├── examples/smart-escrows/     # Example contracts
├── e2e-tests/                  # Integration tests
├── scripts/                    # Build & test scripts
├── ui/                         # Web testing interface
└── docs/                       # Documentation
```

### Key Concepts

- **Smart Escrows**: Conditional payment contracts with custom release logic
- **Host Functions**: 26 WASM host functions for ledger/transaction access
- **no_std**: No heap allocations, stack-only memory
- **Deterministic**: Must produce identical results across all validators
- **Read-only**: Cannot modify ledger (except escrow data updates)

## Conventions

### Rust Code Style

- **Files**: snake_case (e.g., `account_id.rs`, `error_codes.rs`)
- **Crates**: kebab-case (e.g., `xrpl-wasm-stdlib`, `xrpl-macros`)
- **Modules**: snake_case directories
- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Enforced by `cargo fmt` and `cargo clippy`

### JavaScript/Node.js

- **Files**: camelCase (e.g., `runTest.js`, `deployWasmCode.js`)
- Integration tests use `runTest.js` in each example directory

### Shell Scripts

- **Files**: kebab-case (e.g., `build-and-test.sh`, `run-tests.sh`)

### WASM Contract Structure

Every smart contract MUST have:

```rust
#![no_std]
#![no_main]

#[unsafe(no_mangle)]
pub extern "C" fn finish() -> i32 {
    // Contract logic here
    // Return 1 for success, 0 for failure
}
```

### Error Handling

- Use library's custom `Result` type, NOT std::result::Result
- Import: `use xrpl_wasm_stdlib::host::{Result, Error};`
- Import variants: `use xrpl_wasm_stdlib::host::Result::{Ok, Err};`
- Always handle errors explicitly
- Use `trace()` for debugging (appears in rippled debug.log)

### Memory Management

- NO heap allocations (no Vec, String, Box, etc.)
- Use fixed-size arrays: `[0u8; 32]`
- All data on stack
- Buffer sizes defined in `core::constants`

### Cargo.toml for Contracts

```toml
[lib]
crate-type = ["cdylib"]

[profile.release]
opt-level = "s"      # Optimize for size
lto = true           # Link-time optimization
codegen-units = 1    # Better optimization
panic = "abort"      # no_std requirement
```

## Watch Out For

### Critical Rules

- **NEVER use std library** in WASM contracts (use `#![no_std]`)
- **NEVER allocate on heap** (no Vec, String, HashMap, etc.)
- **ALWAYS use wasm32v1-none target** for contracts
- **ALWAYS return i32** from finish() function (1 = success, 0 = failure)
- **Tests use real local rippled node** - not mocks (except unit tests)

### Common Gotchas

- TypeScript strict mode ON - no unused imports/variables
- Clippy runs with `-Dclippy::all` (all warnings are errors)
- Integration tests expect `runTest.js` in example directories
- WASM binaries should be < 64KB (use release profile optimizations)
- Host functions return negative error codes on failure
- `trace()` statements only visible in rippled's debug.log

### Testing Requirements

- All PRs must pass `./scripts/run-all.sh`
- New features need unit tests AND integration tests
- New examples need comprehensive README + runTest.js
- Test on both local node and WASM devnet before PR

### Build Artifacts

- WASM output: `target/wasm32v1-none/release/*.wasm`
- Examples: `examples/target/wasm32v1-none/release/*.wasm`
- E2E tests: `e2e-tests/target/wasm32v1-none/release/*.wasm`

## Documentation

- API docs: `cargo doc --open`
- Comprehensive guide: https://ripple.github.io/xrpl-wasm-stdlib/xrpl_wasm_stdlib/guide/index.html
- See CONTRIBUTING.md for PR requirements
- See docs/NAMING_CONVENTIONS.md for file naming rules
