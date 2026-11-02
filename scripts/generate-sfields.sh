#!/bin/bash
# Generate SField constants from rippled source
# This script generates type-safe SField constants for the XRPL WASM Standard Library.
# It also regenerates the STI_* type-code constants in
# xrpl-common-stdlib/src/type_codes.rs from the contract rippled source.
#
# Fields are sourced from two rippled branches: escrow-side fields are always
# taken from the escrow branch, and the contract branch only contributes
# fields escrow doesn't have at all (e.g. sfContractCode, sfInstanceParameter*).
# Any disagreement -- same-name field with a different (type, ordinal) OR two
# different fields whose (type, ordinal) collide on the wire code -- is
# resolved in favor of escrow with a warning; the contract-side entry is
# dropped. See tools/generateSFields.js for the merge logic.
# STI_* type codes are contract-only additions on top of escrow's set (no
# escrow-side losses), so type_codes.rs is generated from the contract
# source alone.
#
# Usage:
#   ./scripts/generate-sfields.sh [escrow-source] [contract-source] [output-file]
#   ./scripts/generate-sfields.sh --check [escrow-source] [contract-source]
#
# Both sources default to the rippled commits CI pins (XRPLD_DOCKER_IMAGE's tag
# and XRPLD_CONTRACT_COMMIT in .github/workflows/test.yml, resolved by
# scripts/lib/xrpld-pins.sh). Override them positionally, or with
# XRPLD_ESCROW_REF / XRPLD_CONTRACT_REF (any branch, tag, or sha).
#
# With --check, sfield.rs and type_codes.rs are regenerated in place, diffed
# against the current working-tree contents (not git), and restored; the script
# fails on any drift and leaves the working tree unchanged. This is the CI drift gate (run by
# run-all.sh and the check_generated job). It has to regenerate in place
# because tools/generateSFields.js reads the existing sfield.rs to preserve
# its hand-written header, and always writes type_codes.rs at a fixed path.

set -euo pipefail

# Change to the repository root directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$REPO_ROOT"

source "$SCRIPT_DIR/lib/xrpld-pins.sh"
source "$SCRIPT_DIR/lib/check-generated.sh"

CHECK=0
if [[ "${1:-}" == "--check" ]]; then
    CHECK=1
    shift
fi

# Default rippled sources: the commits CI tests against (see lib/xrpld-pins.sh).
# Override with the first two arguments, or XRPLD_ESCROW_REF / XRPLD_CONTRACT_REF.
ESCROW_SOURCE="${1:-$(xrpld_escrow_source)}"
CONTRACT_SOURCE="${2:-$(xrpld_contract_source)}"

# Output file (can be overridden with the third argument). type_codes.rs is
# always written alongside, at a fixed path inside tools/generateSFields.js.
OUTPUT_FILE="${3:-xrpl-common-stdlib/src/sfield.rs}"
TYPE_CODES_FILE="xrpl-common-stdlib/src/type_codes.rs"

# Check if Node.js is available
if ! command -v node &> /dev/null; then
    echo "❌ Node.js is not installed. Please install Node.js to run this script."
    exit 1
fi

regenerate() {
    node tools/generateSFields.js "$ESCROW_SOURCE" "$CONTRACT_SOURCE" "$OUTPUT_FILE" \
        && rustfmt --edition 2024 "$OUTPUT_FILE" "$TYPE_CODES_FILE"
}

if [[ "$CHECK" -eq 1 ]]; then
    echo "🔍 Checking $OUTPUT_FILE and $TYPE_CODES_FILE are up to date..."
    echo "📦 Escrow source:   $ESCROW_SOURCE"
    echo "📦 Contract source: $CONTRACT_SOURCE"
    check_generated regenerate "./scripts/generate-sfields.sh" "$OUTPUT_FILE" "$TYPE_CODES_FILE"
    exit $?
fi

echo "🔧 Generating SField constants..."
echo "📦 Escrow source:   $ESCROW_SOURCE"
echo "📦 Contract source: $CONTRACT_SOURCE"
echo "📝 Output:          $OUTPUT_FILE (+ $TYPE_CODES_FILE)"
echo ""

echo "🔍 Fetching field definitions from rippled and formatting the output..."
regenerate

echo ""
echo "✅ SField constants and STI_* type codes generated successfully!"
echo ""
echo "💡 To add more custom type mappings, edit the customFieldTypes object in tools/generateSFields.js"
