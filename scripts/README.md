# Local CI Scripts

This directory contains bash scripts that mirror the GitHub Actions workflow defined in `.github/workflows/test.yml`.
These scripts allow you to run the same tests locally that run in CI, ensuring consistency between local development and
the CI environment.

## Quick Start

1. **Setup your environment** (run once):

   ```shell
   ./scripts/setup.sh
   ```

2. **Run all tests** (equivalent to the full CI pipeline):
   ```shell
   ./scripts/run-all.sh
   ```

## Individual Scripts

You can also run individual test suites:

### Core Development Scripts

- **`setup.sh`** - Install dependencies and setup the development environment
- **`run-all.sh`** - Run the complete test suite (all scripts below in sequence)

### Main Scripts

- **`build-and-test.sh`** - Build and test the native workspace, build WASM examples, and run craft
- **`build.sh`** - Build all examples (accepts `release` argument for release builds)

### Helper Scripts

- **`clippy.sh`** - Run Clippy linting on both native and WASM workspaces
- **`fmt.sh`** - Check Rust code formatting
- **`run-markdown.sh`** - Execute bash code blocks in Markdown files
- **`run-tests.sh`** - Run integration tests for examples and end-to-end tests
- **`host-function-audit.sh`** - Audit host functions against XRPLd (requires Node.js)
- **`benchmark-gas.sh`** - Measure and compare gas costs of optimized helper functions
- **`generate-sfields.sh`** - Generate type-safe SField constants from rippled source (requires Node.js)

## Usage Examples

```shell
# Setup environment (run once)
./scripts/setup.sh

# Run quick checks before committing
./scripts/fmt.sh

# Run comprehensive tests
./scripts/run-all.sh

# Build in release mode
./scripts/build.sh release

# Run only clippy checks
./scripts/clippy.sh

# Run only integration tests
./scripts/run-tests.sh

# Run gas benchmarks (requires local rippled instance)
./scripts/benchmark-gas.sh

# Generate SField constants from rippled source
./scripts/generate-sfields.sh
```

## Environment Variables

All scripts respect the `RUSTFLAGS` environment variable. By default, they set:

```shell
export RUSTFLAGS="-Dwarnings"
```

This matches the CI environment and ensures warnings are treated as errors.

## Gas Benchmark Script

The `benchmark-gas.sh` script measures and compares gas costs of optimized helper functions:

```shell
# Run gas benchmarks (requires local rippled instance running on ws://127.0.0.1:6006)
./scripts/benchmark-gas.sh
```

This script:

1. Builds the gas_benchmark contract
2. Deploys it to a local rippled instance
3. Executes it multiple times to measure gas consumption
4. Generates a comparison report

**Requirements for gas benchmarking:**

- Local rippled instance running on `ws://127.0.0.1:6006`
- Node.js (for the benchmark runner scripts)

**Output files:**

- `gas_benchmark_results.json` - Raw measurement data
- `GAS_BENCHMARK_REPORT.md` - Formatted comparison report

## Generate SFields Script

The `generate-sfields.sh` script generates type-safe SField constants from the rippled source code:

```shell
# Generate from default rippled repository (smart-escrow branch)
./scripts/generate-sfields.sh

# Generate from a specific rippled repository or branch
./scripts/generate-sfields.sh https://github.com/XRPLF/rippled/tree/develop

# Generate to a custom output file
./scripts/generate-sfields.sh https://github.com/XRPLF/rippled/tree/ripple/smart-escrow custom_output.rs
```

This script:

1. Fetches SField definitions from rippled source (local path or GitHub URL)
2. Generates type-safe Rust constants with proper type mappings
3. Applies custom type overrides for special fields (TransactionType, Condition, Fulfillment)
4. Outputs to `xrpl-wasm-stdlib/src/sfield.rs` by default

**Custom type mappings:**

The script automatically applies custom type mappings for fields that need specialized types:

- `TransactionType` → `TransactionType` enum (not `u16`)
- `Condition` → `ConditionBlob` (not `StandardBlob`)
- `Fulfillment` → `FulfillmentBlob` (not `StandardBlob`)

To add more custom mappings, edit the `customFieldTypes` object in `tools/generateSFields.js`.

## Requirements

- **Rust**: Stable toolchain (installed automatically by `setup.sh`)
- **Pre-commit**: For running pre-commit hooks (installed by `setup.sh`)
- **Node.js**: Required for host function audit and gas benchmark scripts

## GitHub Actions Compatibility

These scripts are designed to work both locally and in GitHub Actions. The same scripts can be called from GitHub
Actions workflows, ensuring perfect consistency between local and CI environments.

## Troubleshooting

- **Permission denied**: Run `chmod +x scripts/*.sh` to make scripts executable
- **Pre-commit not found**: Run `./scripts/setup.sh` to install dependencies
- **Node.js required**: Install Node.js for the host function audit, or skip that script
- **WASM target missing**: The scripts automatically install the `wasm32v1-none` target

## Script Dependencies

```
setup.sh (run first)
├── build-and-test.sh
├── build.sh
├── check-wasm-exports.sh
├── clippy.sh
├── run-tests.sh
    └── ../build.sh (dependency)
├── fmt.sh
├── host-function-audit.sh (requires Node.js)
└── run-markdown.sh
```

**Note**: Pre-commit checks are handled by the GitHub Actions workflow using `pre-commit/action@v3.0.1` rather than a
local script.

The `run-all.sh` script runs all individual scripts in the correct order.
