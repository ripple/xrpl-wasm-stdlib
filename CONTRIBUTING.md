# Contributing to XRPL WebAssembly Standard Library

## Quick Start

### Prerequisites

- **Rust toolchain** - [Install here](https://rust-lang.org/tools/install/)
- **Node.js** (for testing tools)
- **Basic Git/GitHub knowledge** - [Git Handbook](https://guides.github.com/introduction/git-handbook/)

### Setup

```shell
# Clone your fork and set up development environment
./scripts/setup.sh

# Verify installation
./scripts/run-tests.sh examples/smart-escrows/hello_world
```

## Development Workflow

### Running Formatting Checks

```shell
./scripts/fmt.sh && ./scripts/clippy.sh
```

### Test your changes

```
./scripts/run-all.sh
```

### Pull Request Requirements

**All PRs must:**

- Pass all existing tests (`./scripts/run-all.sh` and in CI)
- Follow general code style guidelines (enforced by CI) and [naming conventions](./docs/NAMING_CONVENTIONS.md).
- Include tests for new functionality
- Update documentation as needed
- Use a Conventional Commits PR title (see below) and a non-empty description filled in from the PR template

### Conventional Commits

PR titles are checked in CI and must follow this format:

```
<type>: <Description>
```

- `<type>` is one of the allowed types in the table below, all lowercase.
- The description must start with a capital letter.
- Keep the title short and imperative (e.g. "Add typed AMM accessor", not "Added typed AMM accessor").

Allowed types:

| Type       | Use for                                                          |
| ---------- | ---------------------------------------------------------------- |
| `feat`     | A new feature (host function, helper, public API addition, etc.) |
| `fix`      | A bug fix                                                        |
| `docs`     | Documentation-only changes                                       |
| `style`    | Formatting, missing semicolons, etc.; no code behavior change    |
| `refactor` | Code change that neither fixes a bug nor adds a feature          |
| `perf`     | Performance improvement                                          |
| `test`     | Adding or correcting tests                                       |
| `build`    | Build system, Cargo, toolchain, or dependency changes            |
| `ci`       | CI configuration and workflow changes                            |
| `chore`    | Maintenance that doesn't fit the categories above                |
| `release`  | Release-related changes (version bumps, changelog updates, etc.) |
| `example`  | Adding or changing a sample contract under `examples/`           |

Examples:

- `feat: Add typed accessor for AMM ledger object`
- `fix: Correct return code for missing keylet`
- `docs: Document hello_world build steps`
- `ci: Enforce conventional commit PR titles`
- `example: Add freelancer escrow sample`

When a PR is merged with squash-and-merge, the PR title becomes the commit message — keeping titles in this format keeps `git log` on `main` clean and machine-readable.

When merging without squashing, individual commits are also checked; commits that do not follow the format will be flagged by the `Check PR commits` workflow.

**For new examples:**

- Include comprehensive README with functionality description, build/test instructions, and code walkthrough
- Add integration test (`runTest.js`)
- Test on WASM devnet
- Add to main README examples list

**For library changes:**

- Consider backward compatibility
- Update API documentation and comprehensive guide
- Add unit tests where applicable
- Include performance considerations

## Testing

### Test Networks

| Network         | Endpoint                                 | Purpose             |
| --------------- | ---------------------------------------- | ------------------- |
| **WASM Devnet** | `wss://wasm.devnet.rippletest.net:51233` | Integration testing |
| **Local Node**  | `ws://localhost:6006`                    | Development         |

### Debugging and Development

**Web UI for manual testing:**

```shell
# Build your WASM contract
cargo build --target wasm32v1-none --release

# Upload to deployed testing interface
# Open: https://ripple.github.io/xrpl-wasm-stdlib/ui/
# Click "Choose File" and select your .wasm file
```

**Using trace statements for debugging:**

These debugging statements will show up in the `debug.log` for rippled.

```rust
use xrpl_wasm_stdlib::core::current_tx::traits::TransactionCommonFields;
use xrpl_wasm_stdlib::ctx::SmartFeatureContext;
use xrpl_wasm_stdlib::host::trace::{trace, trace_data, DataRepr};
use xrpl_wasm_stdlib::smart_escrow;
use xrpl_escrow_stdlib::{EscrowFinishContext, FinishResult};

#[smart_escrow]
fn finish_impl(ctx: EscrowFinishContext) -> FinishResult {
    trace("Contract starting").ok();

    let account = match ctx.tx().get_account() {
        Ok(acc) => {
            trace_data("Account", &acc.0, DataRepr::AsHex).ok();
            acc
        },
        Err(e) => {
            return e.code().into();
        }
    };

    // Rest of logic...
    FinishResult::succeed()
}
```

The `#[smart_escrow]` macro generates the `extern "C" fn finish() -> i32` export; your annotated function can be named anything except `finish` (that name is reserved for the generated export).

**Integration test template (`runTest.js`):**

```javascript
const CONFIG = {
  wasmPath: "./target/wasm32v1-none/release/my_example.wasm",
  rippledHost: process.env.RIPPLED_HOST || "wasm.devnet.rippletest.net",
  testAccount: "rN7n7otQDd6FczFgLdSqtcsAUxDkw6fzRH",
}

async function runTest() {
  // Set up test scenario
  // Execute contract with test data
  // Verify expected results
  console.log("Test passed!")
}

runTest().catch(console.error)
```

## Project Structure

```
xrpl-wasm-stdlib/
├── src/                    # Library source code
├── examples/smart-escrows/ # Example smart contracts
├── scripts/                # Development and CI scripts
├── ui/                     # Testing web interface
├── e2e-tests/              # Integration tests
└── docs/                   # Documentation
```

## Adding New Examples

1. **Create directory:** `examples/smart-escrows/my-example/`

2. **Set up project structure:**

   ```shell
   # Use existing example as template
   cp -r examples/smart-escrows/hello_world examples/smart-escrows/my-example
   cd examples/smart-escrows/my-example
   ```

3. **Essential files:**
   - `Cargo.toml` - Package configuration with proper WASM settings
   - `src/lib.rs` - Contract implementation with `#![no_std]` and `#![no_main]`
   - `README.md` - Comprehensive documentation (see other examples for a template)
   - `runTest.js` - Integration test

4. **Test and integrate:**

   ```shell
   # Test your example
   ./scripts/run-tests.sh examples/smart-escrows/my-example

   # Add to main README examples list
   # Update comprehensive guide if significant
   ```

## Release Process (Maintainers)

```shell
# Update version and changelog
vim Cargo.toml CHANGELOG.md

# Full test suite
./scripts/run-all.sh

# Tag and release
git tag v0.x.y
git push origin v0.x.y
```

## Getting Help

- Check [Complete Developer Guide](https://ripple.github.io/xrpl-wasm-stdlib/xrpl_wasm_stdlib/guide/index.html)
- Search existing GitHub issues
- Create new issue with "question" label
- Reference related issues in PRs

## Community Guidelines

- Be respectful and constructive
- Help newcomers learn
- Focus on technical discussions
- Provide clear reproduction steps for bugs

Thank you for contributing to the XRPL WebAssembly Standard Library!
