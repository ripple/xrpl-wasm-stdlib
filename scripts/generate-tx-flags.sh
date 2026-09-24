#!/bin/bash
# Generate type-safe transaction flag constants (tf*/asf*/tmf*) for the XRPL WASM Standard Library from rippled source.
#
# Flags are sourced from two rippled branches: the base branch is authoritative,
# and the contract branch only contributes flags that don't exist on the base
# branch at all (e.g. tfImmutable, tfSendAmount). Only individual flags are
# emitted; rippled's validity masks are omitted. See tools/generateTxFlags.js
# for the merge logic.
#
# Usage:
#   ./scripts/generate-tx-flags.sh [base-source] [contract-source] [output-file]
#   ./scripts/generate-tx-flags.sh --check [base-source] [contract-source]
#
# Both sources default to the rippled commits CI pins (XRPLD_DOCKER_IMAGE's tag
# and XRPLD_CONTRACT_COMMIT in .github/workflows/test.yml, resolved by
# scripts/lib/xrpld-pins.sh). Override them positionally, or with
# XRPLD_ESCROW_REF / XRPLD_CONTRACT_REF (any branch, tag, or sha).
#
# With --check, the file is regenerated in place, diffed against the current working-tree contents (not git),
# and restored; the script fails on any drift and leaves the working tree unchanged.
# This is the CI drift gate (run by run-all.sh and the check_generated job).

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
BASE_SOURCE="${1:-$(xrpld_escrow_source)}"
CONTRACT_SOURCE="${2:-$(xrpld_contract_source)}"

# Output file (can be overridden with the third argument)
OUTPUT_FILE="${3:-xrpl-common-stdlib/src/tx_flags.rs}"

# Check if Node.js is available
if ! command -v node &> /dev/null; then
    echo "❌ Node.js is not installed. Please install Node.js to run this script."
    exit 1
fi

regenerate() {
    node tools/generateTxFlags.js "$BASE_SOURCE" "$CONTRACT_SOURCE" "$OUTPUT_FILE" \
        && rustfmt --edition 2024 "$OUTPUT_FILE"
}

if [[ "$CHECK" -eq 1 ]]; then
    echo "🔍 Checking $OUTPUT_FILE is up to date..."
    echo "📦 Base source:     $BASE_SOURCE"
    echo "📦 Contract source: $CONTRACT_SOURCE"
    check_generated regenerate "./scripts/generate-tx-flags.sh" "$OUTPUT_FILE"
    exit $?
fi

echo "🔧 Generating transaction flag constants..."
echo "📦 Base source:     $BASE_SOURCE"
echo "📦 Contract source: $CONTRACT_SOURCE"
echo "📝 Output:          $OUTPUT_FILE"
echo ""

echo "🔍 Fetching flag definitions from rippled and formatting the output..."
regenerate

echo ""
echo "✅ Flag constants generated successfully!"
