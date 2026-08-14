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
- `fix: Correct return code for missing id`
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

### API design: distinct types for order-sensitive parameters

When a public function takes **two or more adjacent parameters of the same raw type** and their
order matters, give them **distinct types** with a newtype wrapper — following the existing
`PublicKey` / `AccountID` / `Hash256` pattern — so a caller who swaps them gets a **compile
error** instead of a silently wrong result at runtime.

This applies to **untyped raw parameters only** — the ones with nothing but a parameter name
distinguishing them. `crypto::check_sig` takes its message and signature as the distinct
`Message` and `Signature` newtypes (both wrapping `&[u8]`), so `check_sig(msg, sig, ...)`
cannot be called with the two swapped.

Do **not** add role newtypes on top of a type that is already a meaningful domain type. Where
two parameters are both `&AccountID` playing different roles (`credential_id`'s subject and
issuer, `paychan_id`'s account and destination), leave them as `&AccountID`: the extra wrapper
costs every caller a construction step for a swap the type name already documents. Likewise,
leave symmetric or interchangeable parameters alone — `amm_id`'s issue pair and `trustline_id`'s
canonically-ordered account pair produce the same result either way.

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
use xrpl_common_stdlib::host::trace::{trace, trace_hex};

#[smart_escrow]
fn finish_impl(ctx: EscrowFinishContext) -> FinishResult {
    trace("Contract starting");

    let account = match ctx.tx().get_account() {
        Ok(acc) => {
            trace_hex("Account", &acc.0);
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

The `#[smart_escrow]` macro generates the `extern "C" fn escrow_finish() -> i32` export; your annotated function can be named anything except `escrow_finish` (that name is reserved for the generated export).

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
xrpl-common-stdlib/
├── src/                    # Library source code
├── examples/smart-escrows/ # Example smart contracts
├── skills/                 # Claude Code skills (e.g. xrpl-smart-escrows)
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

The library publishes **four crates to crates.io**:

| Crate                    | Published | Notes                                                        |
| ------------------------ | --------- | ------------------------------------------------------------ |
| `xrpl-macros`            | yes       | Publishes first — the others depend on it.                   |
| `xrpl-common-stdlib`     | yes       | The general layer.                                           |
| `xrpl-stdlib-test-utils` | yes       | Contract test harness. Depends on `xrpl-common-stdlib` only. |
| `xrpl-escrow-stdlib`     | yes       | Smart Escrow layer. Publishes last.                          |

Publishing is **entirely manual**. There is no release workflow: neither a merge to `main` nor a `v*`
tag uploads anything. An owner runs the `cargo publish` commands by hand. Nothing in CI checks the
pre-publish gate, so work through these steps in order.

### Cutting a release

1. In a PR, bump the `version` field of each crate being released and merge it to `main`. Bump the
   `version` key on any in-workspace dependency on a bumped crate too, or the release ships a range
   pointing at the previous version.

   ```shell
   vim xrpl-macros/Cargo.toml xrpl-common-stdlib/Cargo.toml \
       xrpl-stdlib-test-utils/Cargo.toml xrpl-escrow-stdlib/Cargo.toml
   ```

2. From the merged commit, run the full gate. Nothing downstream re-runs it.

   ```shell
   git checkout main && git pull
   ./scripts/run-all.sh
   ```

3. Publish in dependency order, one at a time. Let each command finish before starting the next: a
   crate must be in the index before the following one can resolve it.

   ```shell
   cargo login          # once per machine, with a crates.io token scoped to publish-update
   cargo publish -p xrpl-macros
   cargo publish -p xrpl-common-stdlib
   cargo publish -p xrpl-stdlib-test-utils
   cargo publish -p xrpl-escrow-stdlib
   ```

   `--dry-run` packages and verifies without uploading. Under the 1.89 toolchain pin a dependent's
   dry run fails until its siblings are on the real index; to rehearse the whole set at once, use
   cargo 1.90 or newer: `cargo +stable publish --workspace --dry-run`.

4. Tag the published commit and cut a GitHub Release. The tag records what shipped; pushing it
   publishes nothing.

   ```shell
   git tag v0.9.x && git push origin v0.9.x
   gh release create v0.9.x --generate-notes --verify-tag
   ```

There is no hand-maintained changelog while the library is pre-1.0. `--generate-notes` builds the
notes from the merged PRs since the previous tag, and the Conventional-Commits PR titles keep them
readable. Revisit a curated changelog at 1.0.

**A published version is permanent** — it can be yanked but never replaced. A bad release burns the
number; the fix is the next patch (`0.9.x+1`), never a re-push of the same version.

### The first publish of a new crate name

Publishing a name for the first time creates the crate and makes that account its sole owner. So the
initial `0.9.0` of the three new names (`xrpl-common-stdlib`, `xrpl-escrow-stdlib`,
`xrpl-stdlib-test-utils`) should come from an account that should hold ownership. Add the other
maintainers afterwards, or releases stay blocked on one person:

```shell
cargo owner --add <crates-io-user-or-team> xrpl-common-stdlib
```

### The deprecated `xrpl-wasm-stdlib` name

`xrpl-wasm-stdlib` is the library's old pre-split name (published through `0.8.0`). It is not part
of the crate set above and is not bumped with it: a one-time doc-only `0.9.0` signpost is published
from `deprecated/xrpl-wasm-stdlib` to point users at the new crates. Never yank `0.7.x`/`0.8.0` (an
external crate depends on `^0.8`), and never publish a `0.8.x` signpost (it would land inside that
range and break them).

## Getting Help

- Check [Complete Developer Guide](https://ripple.github.io/xrpl-wasm-stdlib/xrpl_common_stdlib/guide/index.html)
- Search existing GitHub issues
- Create new issue with "question" label
- Reference related issues in PRs

## Community Guidelines

- Be respectful and constructive
- Help newcomers learn
- Focus on technical discussions
- Provide clear reproduction steps for bugs

Thank you for contributing to the XRPL WebAssembly Standard Library!
