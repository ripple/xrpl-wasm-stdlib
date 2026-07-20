#!/bin/bash
# Generate SField constants from rippled source
# This script generates type-safe SField constants for the XRPL WASM Standard Library.
# It also regenerates the STI_* type-code constants in
# xrpl-wasm-stdlib/src/core/type_codes.rs from the contract rippled source.
#
# Fields are sourced from two rippled branches: escrow-side fields are always
# taken from the escrow branch (so a rename there is picked up automatically
# next time this runs), and the contract branch only contributes fields that
# don't exist on the escrow branch at all (e.g. sfContractCode,
# sfInstanceParameter*). See tools/generateSFields.js for the merge logic.
# STI_* type codes are contract-only additions on top of escrow's set (no
# escrow-side losses), so type_codes.rs is generated from the contract
# source alone.

set -euo pipefail

# Change to the repository root directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$REPO_ROOT"

# Default rippled sources (can be overridden with the first two arguments)
ESCROW_SOURCE="${1:-https://github.com/XRPLF/rippled/tree/ripple/se/supported}"
CONTRACT_SOURCE="${2:-https://github.com/XRPLF/rippled/tree/xrplf/smart-contracts}"

# Output file (can be overridden with the third argument)
OUTPUT_FILE="${3:-xrpl-common-stdlib/src/sfield.rs}"

echo "🔧 Generating SField constants..."
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
echo "🔍 Fetching field definitions from rippled..."
node tools/generateSFields.js "$ESCROW_SOURCE" "$CONTRACT_SOURCE" "$OUTPUT_FILE"

echo ""
echo "🎨 Formatting generated output..."
cargo fmt -p xrpl-wasm-stdlib

echo ""
echo "✅ SField constants and STI_* type codes generated successfully!"
echo ""
echo "💡 To add more custom type mappings, edit the customFieldTypes object in tools/generateSFields.js"
