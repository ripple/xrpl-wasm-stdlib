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

# Change to e2e-tests directory
cd e2e-tests

# Clean previous coverage data
cargo llvm-cov clean --workspace

# Run tests with coverage instrumentation and display summary in terminal
# --workspace: Include all workspace members
# --tests: Run in test mode
# --features test-host-bindings: Enable test host bindings for xrpl-wasm-stdlib
#   This is equivalent to cfg(test) but works with cargo llvm-cov
# --ignore-filename-regex: Exclude test files from coverage report
echo "Running tests with coverage instrumentation..."
echo ""
cargo llvm-cov \
    --features xrpl-wasm-stdlib/test-host-bindings \
    --workspace \
    --tests \
    --ignore-filename-regex "e2e-tests/.*" \
    -- --nocapture

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
