#!/bin/bash
# Generate SField constants from rippled source
# This script generates type-safe SField constants for the XRPL WASM Standard Library

set -euo pipefail

# Change to the repository root directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$REPO_ROOT"

# Default rippled source (can be overridden with first argument)
# Using smart-escrow branch which has the latest smart escrow features
RIPPLED_SOURCE="${1:-https://github.com/XRPLF/rippled/tree/ripple/smart-escrow}"

# Output file (can be overridden with second argument)
OUTPUT_FILE="${2:-xrpl-wasm-stdlib/src/sfield.rs}"

echo "🔧 Generating SField constants..."
echo "📦 Source: $RIPPLED_SOURCE"
echo "📝 Output: $OUTPUT_FILE"
echo ""

# Check if Node.js is available
if ! command -v node &> /dev/null; then
    echo "❌ Node.js is not installed. Please install Node.js to run this script."
    exit 1
fi

# Run the generator
echo "🔍 Fetching field definitions from rippled..."
node tools/generateSFields.js "$RIPPLED_SOURCE" "$OUTPUT_FILE"

echo ""
echo "✅ SField constants generated successfully!"
echo "ℹ️  Custom type mappings applied:"
echo "   - TransactionType → TransactionType (not u16)"
echo "   - Condition → ConditionBlob (not StandardBlob)"
echo "   - Fulfillment → FulfillmentBlob (not StandardBlob)"
echo ""
echo "💡 To add more custom type mappings, edit tools/generateSFields.js"
