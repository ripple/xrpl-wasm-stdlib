#!/bin/bash
# Generate type-safe transaction flag constants (tf*/asf*/tmf*) for the XRPL WASM Standard Library from rippled source.
#
# Flags are sourced from two rippled branches, both read live so upstream
# renames are picked up automatically next time this runs: the base branch is
# authoritative, and the contract branch only contributes flags/masks that
# don't exist on the base branch at all (e.g. tfImmutable,
# tfContractParameterMask). See tools/generateTxFlags.js for the merge logic.

set -euo pipefail

# Check if Node.js is available
if ! command -v node &> /dev/null; then
    echo "❌ Node.js is not installed. Please install Node.js to run this script."
    exit 1
fi

# Change to the repository root directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$REPO_ROOT"

# Default rippled sources (can be overridden with the first two arguments)
BASE_SOURCE="${1:-https://github.com/XRPLF/rippled/tree/ripple/se/supported}"
CONTRACT_SOURCE="${2:-https://github.com/XRPLF/rippled/tree/xrplf/smart-contracts}"

# Output file (can be overridden with the third argument)
OUTPUT_FILE="${3:-xrpl-wasm-stdlib/src/tx_flags.rs}"

echo "🔧 Generating transaction flag constants..."
echo "📦 Base source:     $BASE_SOURCE"
echo "📦 Contract source: $CONTRACT_SOURCE"
echo "📝 Output: $OUTPUT_FILE"
echo ""

# Run the generator
echo "🔍 Fetching flag definitions from rippled..."
node tools/generateTxFlags.js "$BASE_SOURCE" "$CONTRACT_SOURCE" "$OUTPUT_FILE"

echo ""
echo "🎨 Formatting generated output..."
cargo fmt -p xrpl-wasm-stdlib

echo ""
echo "✅ Flag constants generated successfully!"
