#!/bin/bash
# Generate transaction/ledger flag constants (tf*/asf*/tmf*) from rippled source
# This script generates type-safe flag constants for the XRPL WASM Standard Library.

set -euo pipefail

# Change to the repository root directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$REPO_ROOT"

# Default rippled source (can be overridden with first argument)
# Using xrplf/smart-contracts, which is a superset of ripple/smart-escrow (see
# scripts/generate-sfields.sh for why) and also defines the Smart Contract
# (XLS-101) flags (tfImmutable, tfSendAmount, tfContractParameterMask, etc).
RIPPLED_SOURCE="${1:-https://github.com/XRPLF/rippled/tree/xrplf/smart-contracts}"

# Output file (can be overridden with second argument)
OUTPUT_FILE="${2:-xrpl-wasm-stdlib/src/sflags.rs}"

echo "🔧 Generating transaction/ledger flag constants..."
echo "📦 Source: $RIPPLED_SOURCE"
echo "📝 Output: $OUTPUT_FILE"
echo ""

# Check if Node.js is available
if ! command -v node &> /dev/null; then
    echo "❌ Node.js is not installed. Please install Node.js to run this script."
    exit 1
fi

# Run the generator
echo "🔍 Fetching flag definitions from rippled..."
node tools/generateSFlags.js "$RIPPLED_SOURCE" "$OUTPUT_FILE"

echo ""
echo "🎨 Formatting generated output..."
cargo fmt -p xrpl-wasm-stdlib

echo ""
echo "✅ Flag constants generated successfully!"
