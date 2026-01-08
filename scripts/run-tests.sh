#!/bin/bash
# End-to-end tests script
# Mirrors the e2e-tests job from GitHub Actions

set -euo pipefail

# Change to the repository root directory (where this script's grandparent directory is located)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$REPO_ROOT"

echo "🔧 Running end-to-end tests..."

# Ensure wasm32 target is available
echo "📦 Ensuring wasm32v1-none target is installed..."
rustup target add wasm32v1-none

echo "🏗️  Building examples..."
scripts/build.sh
scripts/build.sh release

echo "🧪 Running integration tests..."

set +e

run_integration_test() {
    local dir="$1"
    local contract_name="$2"
    local wasm_file_release="$3"

    if [[ ! -f "$dir/runTest.js" ]]; then
        echo "❌ Error: Test file runTest.js not found in $dir"
        exit 1
    fi
    echo "🔧 Running integration test for $contract_name in $dir"
    exit_code=0
    if [[ "${DEVNET:-}" == "true" || -n "${DEVNET:-}" ]]; then
        node tests/runSingleTest.js "$dir" "$wasm_file_release" "wss://wasm.devnet.rippletest.net:51233"
        exit_code=$?
    else
        node tests/runSingleTest.js "$dir" "$wasm_file_release"
        exit_code=$?
    fi
    exit $exit_code
}

if [[ $# -gt 0 ]]; then
    arg="$1"
    test_dir="$(realpath "$arg")"
    test_name=$(basename "$test_dir")
    if [[ "$test_dir" == *"/examples/"* ]]; then
        wasm_file_release="examples/target/wasm32v1-none/release/${test_name}.wasm"
    elif [[ "$test_dir" == *"/e2e-tests/"* ]]; then
        wasm_file_release="e2e-tests/target/wasm32v1-none/release/${test_name}.wasm"
    else
        echo "❌ Error: Unknown test directory: $test_dir"
        exit 1
    fi
    run_integration_test "$test_dir" "$test_name" "$wasm_file_release"
    exit $?
fi
all_tests_passed=true
failed_tests=()

while read -r cargo_file; do
    dir=$(dirname "$cargo_file")
    contract_name=$(basename "$dir")
    wasm_file_release="examples/target/wasm32v1-none/release/${contract_name}.wasm"
    (run_integration_test "$dir" "$contract_name" "$wasm_file_release")
    exit_code=$?
    if [[ $exit_code -ne 0 ]]; then
        all_tests_passed=false
        failed_tests+=("$contract_name")
    fi
done < <(find examples -mindepth 2 -name "Cargo.toml" -type f)

while read -r cargo_file; do
    dir=$(dirname "$cargo_file")
    contract_name=$(basename "$dir")
    wasm_file_release="e2e-tests/target/wasm32v1-none/release/${contract_name}.wasm"
    # TODO: remove this when tests are written for all the e2e-tests
    if [[ ! -f "$dir/runTest.js" ]]; then
        echo "⚠️  Skipping $contract_name: runTest.js not found in $dir"
        continue
    fi
    (run_integration_test "$dir" "$contract_name" "$wasm_file_release")
    exit_code=$?
    if [[ $exit_code -ne 0 ]]; then
        all_tests_passed=false
        failed_tests+=("$contract_name")
    fi
done < <(find e2e-tests -mindepth 2 -name "Cargo.toml" -type f)

if [[ "$all_tests_passed" == true ]]; then
    echo "✅ All end-to-end tests passed!"
else
    echo "❌ Some end-to-end tests failed."
    echo "Failed tests: ${failed_tests[*]}"
    exit 1
fi
