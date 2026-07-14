#!/bin/bash
set -e

# Coverage script for xrpl-wasm-stdlib
#
# This script runs the e2e-tests with coverage instrumentation to measure
# which parts of xrpl-wasm-stdlib are exercised by the integration tests.
#
# Requirements:
#   - cargo-llvm-cov: Install with `cargo install cargo-llvm-cov`
#   - lcov: Install with `apt-get install lcov` / `brew install lcov`
#
# Usage:
#   ./scripts/coverage.sh
#
# Output:
#   - LCOV report: e2e-tests/target/llvm-cov/lcov.info
#   - Console summary showing coverage percentages

echo "=== xrpl-wasm-stdlib Coverage Report ==="
echo ""

# Check if cargo-llvm-cov is installed
if ! command -v cargo-llvm-cov &> /dev/null; then
    echo "Error: cargo-llvm-cov is not installed"
    echo "Install it with: cargo install cargo-llvm-cov"
    exit 1
fi

# Check if lcov is installed (needed to merge per-crate e2e-tests coverage, see below)
if ! command -v lcov &> /dev/null; then
    echo "Error: lcov is not installed"
    echo "Install it with: apt-get install lcov / brew install lcov"
    exit 1
fi

echo "Running coverage for e2e-tests..."
echo ""

# Run tests with coverage instrumentation and display summary in terminal
# --workspace: Include all workspace members
# --tests: Run in test mode
# --features test-host-bindings: Enable test host bindings for xrpl-wasm-stdlib
#   This is equivalent to cfg(test) but works with cargo llvm-cov
# --ignore-filename-regex: Exclude test files from coverage report
echo "Running tests with coverage instrumentation..."
echo ""

# Run coverage on unit tests
cargo llvm-cov clean --workspace # Clean previous coverage data
cargo llvm-cov \
    --features xrpl-wasm-stdlib/test-host-bindings \
    --workspace \
    --tests \
    -- --nocapture

# Change to e2e-tests directory, run coverage on e2e tests
cd e2e-tests
cargo llvm-cov clean --workspace # Clean previous coverage data

# Run the tests to accumulate profile data, without generating a report yet.
# (--no-report skips cargo-llvm-cov's own report generation, which is broken
# below.)
cargo llvm-cov \
    --no-report \
    --features xrpl-wasm-stdlib/test-host-bindings \
    --workspace \
    --tests \
    -- --nocapture

# Every e2e-tests crate exports a wasm entry point; on native targets, each
# test binary still links against the same xrpl-wasm-stdlib. Asking
# cargo-llvm-cov to generate ONE report across all of these binaries at once
# (`cargo llvm-cov report --tests`) works fine on some LLVM versions but has
# been observed to silently produce an EMPTY report (no rows, no warning) on
# others, including the Linux toolchain used in CI -- passing a single
# `-object` at a time always works correctly, though.
#
# Work around this by exporting each e2e-tests crate's coverage separately
# (one object per invocation of the underlying `llvm-cov` tool) and merging
# the resulting lcov tracefiles with `lcov`, which sums per-line hit counts
# across files instead of relying on llvm-cov's own multi-binary merge.
SYSROOT_BIN="$(dirname "$(rustc --print target-libdir)")/bin"
PROFDATA="target/llvm-cov-target/e2e-tests-merged.profdata"
"$SYSROOT_BIN/llvm-profdata" merge -sparse target/llvm-cov-target/*.profraw -o "$PROFDATA"

PER_CRATE_DIR="target/llvm-cov/per-crate"
rm -rf "$PER_CRATE_DIR"
mkdir -p "$PER_CRATE_DIR"

E2E_CRATES=(
    float_tests
    gas_benchmark
    host_functions_test
    keylet_exists
    trace_escrow_account
    trace_escrow_finish
    trace_escrow_ledger_object
)

for crate in "${E2E_CRATES[@]}"; do
    bin=$(ls target/llvm-cov-target/debug/deps/"${crate}"-* 2>/dev/null | grep -v '\.' | head -1)
    if [ -z "$bin" ]; then
        echo "Error: could not find test binary for $crate"
        exit 1
    fi
    "$SYSROOT_BIN/llvm-cov" export -format=lcov \
        -instr-profile="$PROFDATA" \
        -object "$bin" \
        -ignore-filename-regex "e2e-tests/.*" \
        > "$PER_CRATE_DIR/${crate}.info"
done

mkdir -p target/llvm-cov
LCOV_ADD_ARGS=()
for f in "$PER_CRATE_DIR"/*.info; do
    LCOV_ADD_ARGS+=(-a "$f")
done
lcov "${LCOV_ADD_ARGS[@]}" -o target/llvm-cov/lcov.info

echo ""
lcov --list target/llvm-cov/lcov.info

echo ""
echo "=== Coverage Summary ==="
echo ""
echo "The above shows coverage for xrpl-wasm-stdlib exercised by e2e-tests."
echo ""
echo "LCOV report written to e2e-tests/target/llvm-cov/lcov.info"
echo ""
echo "To generate an HTML report from it, run:"
echo "  genhtml e2e-tests/target/llvm-cov/lcov.info -o e2e-tests/target/llvm-cov/html"
