#!/bin/bash
set -e

# Coverage script for xrpl-wasm-stdlib
#
# This script runs the e2e-tests with coverage instrumentation to measure
# which parts of xrpl-wasm-stdlib are exercised by the integration tests.
#
# Requirements:
#   - cargo-llvm-cov: Install with `cargo install cargo-llvm-cov`
#
# Usage:
#   ./scripts/coverage.sh
#
# Output:
#   - HTML report: target/llvm-cov/html/index.html
#   - LCOV report: target/llvm-cov/lcov.info
#   - Console summary showing coverage percentages

echo "=== xrpl-wasm-stdlib Coverage Report ==="
echo ""

# Check if cargo-llvm-cov is installed
if ! command -v cargo-llvm-cov &> /dev/null; then
    echo "Error: cargo-llvm-cov is not installed"
    echo "Install it with: cargo install cargo-llvm-cov"
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
cargo llvm-cov \
    --no-report \
    --features xrpl-wasm-stdlib/test-host-bindings \
    --workspace \
    --tests \
    -- --nocapture

echo "=== DEBUG: inspecting instrumented binaries and profile data ==="
SYSROOT_BIN="$(dirname "$(rustc --print target-libdir)")/bin"
echo "SYSROOT_BIN=$SYSROOT_BIN"
ls -la target/llvm-cov-target/ | head -20
echo "--- profraw files ---"
ls -la target/llvm-cov-target/*.profraw 2>&1 | head -20
echo "--- keylet_exists binary section headers (looking for __llvm_covmap/__llvm_covfun) ---"
KEYLET_BIN=$(ls target/llvm-cov-target/debug/deps/keylet_exists-* 2>/dev/null | grep -v '\.' | head -1)
echo "KEYLET_BIN=$KEYLET_BIN"
objdump -h "$KEYLET_BIN" 2>&1 | grep -i "cov\|Idx Name" || echo "objdump not available or no cov sections found"
echo "--- llvm-profdata merge (manual) ---"
"$SYSROOT_BIN/llvm-profdata" merge -sparse target/llvm-cov-target/*.profraw -o /tmp/manual.profdata
"$SYSROOT_BIN/llvm-profdata" show --all-functions --counts /tmp/manual.profdata 2>&1 | head -5
echo "--- llvm-cov report on keylet_exists alone ---"
"$SYSROOT_BIN/llvm-cov" report -instr-profile=/tmp/manual.profdata -object "$KEYLET_BIN" 2>&1 | tail -10
echo "=== END DEBUG ==="

cargo llvm-cov report \
    --ignore-filename-regex "e2e-tests/.*"

echo ""
echo "=== Coverage Summary ==="
echo ""
echo "The above shows coverage for xrpl-wasm-stdlib exercised by e2e-tests."
echo ""
echo "To generate HTML report, run:"
echo "  cd e2e-tests && cargo llvm-cov --features xrpl-wasm-stdlib/test-host-bindings --workspace --tests --html --ignore-filename-regex 'e2e-tests/.*'"
echo ""
echo "To generate LCOV report, run:"
echo "  cd e2e-tests && cargo llvm-cov --features xrpl-wasm-stdlib/test-host-bindings --workspace --tests --lcov --output-path ../target/llvm-cov/lcov.info --ignore-filename-regex 'e2e-tests/.*'"
