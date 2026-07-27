#!/bin/bash
# Run all tests script
# Runs all the test jobs in sequence, mirroring the complete GitHub Actions workflow

set -euo pipefail

# Change to the repository root directory (where this script's parent directory is located)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$REPO_ROOT"

echo "🚀 Running complete Build and Test suite..."
echo ""

# Track start time
start_time=$(date +%s)

# Function to run a script and report results
run_script() {
    local script_name="$1"
    local script_path="./scripts/$script_name"
    echo "=================================================="
    echo "🔧 Running $script_name..."
    echo "=================================================="
    if [[ -x "$script_path" ]]; then
        if "$script_path"; then
            echo "✅ $script_name completed successfully"
        else
            echo "❌ $script_name failed"
            exit 1
        fi
    else
        echo "❌ Script $script_path not found or not executable"
        exit 1
    fi
    echo ""
}

# Run all test scripts in order
# Note: pre-commit checks are handled by GitHub Actions, not locally
run_script "/clippy.sh"
run_script "/fmt.sh"
run_script "/host-function-audit.sh"
run_script "/check-wasm-exports.sh"
run_script "/check-ledger-objects-generated.sh"
run_script "/build-and-test.sh"
run_script "/run-markdown.sh"
run_script "/run-tests.sh"

# Calculate and display total time
end_time=$(date +%s)
duration=$((end_time - start_time))
minutes=$((duration / 60))
seconds=$((duration % 60))

echo "=================================================="
echo "🎉 All tests completed successfully!"
echo "⏱️  Total time: ${minutes}m ${seconds}s"
echo "=================================================="
