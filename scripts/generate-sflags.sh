#!/bin/bash
# Generate transaction/ledger flag constants (tf*/asf*/tmf*) from rippled source
# This script generates type-safe flag constants for the XRPL WASM Standard Library.
#
# Flags are sourced from two rippled branches: escrow-side flags are always
# taken from the escrow branch (so a rename there is picked up automatically
# next time this runs), and the contract branch only contributes flags/masks
# that don't exist on the escrow branch at all (e.g. tfImmutable,
# tfContractParameterMask). See tools/generateSFlags.js for the merge logic.

set -euo pipefail

# Change to the repository root directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$REPO_ROOT"

# Default rippled sources (can be overridden with the first two arguments)
ESCROW_SOURCE="${1:-https://github.com/XRPLF/rippled/tree/ripple/smart-escrow}"
CONTRACT_SOURCE="${2:-https://github.com/XRPLF/rippled/tree/xrplf/smart-contracts}"

# Output file (can be overridden with the third argument)
OUTPUT_FILE="${3:-xrpl-wasm-stdlib/src/tx_flags.rs}"

echo "🔧 Generating transaction/ledger flag constants..."
echo "📦 Escrow source:   $ESCROW_SOURCE"
echo "📦 Contract source: $CONTRACT_SOURCE"
echo "📝 Output: $OUTPUT_FILE"
echo ""

# Check if Node.js is available
if ! command -v node &> /dev/null; then
    echo "❌ Node.js is not installed. Please install Node.js to run this script."
    exit 1
fi

# Run the generator
echo "🔍 Fetching flag definitions from rippled..."
node tools/generateSFlags.js "$ESCROW_SOURCE" "$CONTRACT_SOURCE" "$OUTPUT_FILE"

echo ""
echo "🎨 Formatting generated output..."
cargo fmt -p xrpl-wasm-stdlib

echo ""
echo "✅ Flag constants generated successfully!"
