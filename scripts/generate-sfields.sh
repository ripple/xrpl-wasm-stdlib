#!/bin/bash
# Generate SField constants from rippled source
# This script generates type-safe SField constants for the XRPL WASM Standard Library.
# It also regenerates the STI_* type-code constants in
# xrpl-wasm-stdlib/src/core/type_codes.rs from the same rippled source.

set -euo pipefail

# Change to the repository root directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$REPO_ROOT"

# Default rippled source (can be overridden with first argument)
# Using xrplf/smart-contracts, which is a superset of ripple/smart-escrow (all
# Smart Escrow fields plus the Smart Contract (XLS-101) fields and STI_* type
# codes this stdlib now also needs, e.g. sfContractCode, sfInstanceParameter*,
# STI_INT32/STI_INT64/STI_DATA/STI_DATATYPE/STI_JSON).
RIPPLED_SOURCE="${1:-https://github.com/XRPLF/rippled/tree/xrplf/smart-contracts}"

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
echo "🎨 Formatting generated output..."
cargo fmt -p xrpl-wasm-stdlib

echo ""
echo "✅ SField constants and STI_* type codes generated successfully!"
echo ""
echo "💡 To add more custom type mappings, edit the customFieldTypes object in tools/generateSFields.js"
