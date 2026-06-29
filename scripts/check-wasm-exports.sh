#!/bin/bash
# WASM contract exports checking script
# Checks that all WASM crates export at least one callable entry point.
# Smart escrows export finish() -> i32; smart contracts export named functions
# via #[unsafe(no_mangle)].

set -euo pipefail

# Change to the repository root directory (where this script's grandparent directory is located)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$REPO_ROOT"

echo "🔍 Checking WASM contract exports..."
check_src_dir_exports() {
    local src_dir="$1"
    local dir=$(dirname "$src_dir")
    [[ "$dir" == *"test_utils"* ]] && return 0
    echo "🔧 Checking exports in $dir"
    if [[ -f "$src_dir/lib.rs" ]]; then
        # Accept escrow-style (finish() -> i32), contract-style (#[unsafe(no_mangle)]),
        # or macro-style (#[wasm_export(...)]) entry points
        grep -q 'finish() -> i32\|#\[unsafe(no_mangle)\]\|#\[wasm_export' "$src_dir/lib.rs" || {
            echo "❌ Missing required export in $dir (expected finish() -> i32, #[unsafe(no_mangle)], or #[wasm_export])"
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
