#!/bin/bash
# WASM contract exports checking script
# Checks that all WASM examples export the required finish function

set -euo pipefail

# Change to the repository root directory (where this script's grandparent directory is located)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$REPO_ROOT"

echo "🔍 Checking WASM contract exports..."
# Check that all WASM examples export the required finish function
check_src_dir_exports() {
    local src_dir="$1"
    local dir=$(dirname "$src_dir")
    [[ "$dir" == *"test_utils"* ]] && return 0
    echo "🔧 Checking exports in $dir"
    if [[ -f "$src_dir/lib.rs" ]]; then
        grep -q "finish() -> i32" "$src_dir/lib.rs" || {
            echo "❌ Missing required finish() -> i32 export in $dir"
            exit 1
        }
    else
        echo "❌ Missing lib.rs in $src_dir"
        exit 1
    fi
}

find examples -type d -name "src" | while read -r src_dir; do
    check_src_dir_exports "$src_dir"
done

find e2e-tests -type d -name "src" | while read -r src_dir; do
    check_src_dir_exports "$src_dir"
done

echo "✅ WASM contract exports check passed!"
