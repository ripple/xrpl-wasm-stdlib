#!/bin/bash
# Generate ledger-object field accessor traits from rippled source.
#
# This regenerates xrpl-common-stdlib/src/objects/generated/ (one file per ledger
# entry, plus mod.rs) from rippled's include/xrpl/protocol/detail/ledger_entries.macro
# (field lists) and include/xrpl/protocol/detail/sfields.macro (field Rust types, via
# the shared tools/sfieldTypeMap.js). Doc comments are fetched live from the
# xrpl-dev-portal ledger-entry-types docs. See tools/generateLedgerObjects.js for the
# parsing and emission logic.
#
# Escrow is excluded -- it stays hand-written in xrpl-escrow-stdlib because its `Data`
# field has bespoke, host-mutable ContractData semantics the generator doesn't model.
#
# Only a single rippled source is read (unlike generate-sfields.sh / generate-tx-flags.sh,
# which merge an escrow branch and a contract branch -- ledger-entry definitions don't
# currently have that same base/contract split).

set -euo pipefail

# Change to the repository root directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$REPO_ROOT"

# Default rippled source (can be overridden with the first argument, or the
# RIPPLED_BRANCH env var for just swapping the branch/tag/commit of the
# default XRPLF/rippled repo).
DEFAULT_SOURCE="https://github.com/XRPLF/rippled/tree/${RIPPLED_BRANCH:-xrplf/smart-contracts}"
RIPPLED_SOURCE="${1:-$DEFAULT_SOURCE}"

# Output root (can be overridden with the second argument). Per-entry files are
# written under <OUTPUT_ROOT>/xrpl-common-stdlib/src/objects/generated/.
OUTPUT_ROOT="${2:-$REPO_ROOT}"

echo "🔧 Generating ledger-object field accessor traits..."
echo "📦 rippled source: $RIPPLED_SOURCE"
echo "📝 Output root:    $OUTPUT_ROOT"
echo ""

# Check if Node.js is available
if ! command -v node &> /dev/null; then
    echo "❌ Node.js is not installed. Please install Node.js to run this script."
    exit 1
fi

# Run the generator. It writes xrpl-common-stdlib/src/objects/generated/*.rs and
# runs `rustfmt --edition 2024` on its own output.
node tools/generateLedgerObjects.js "$RIPPLED_SOURCE" "$OUTPUT_ROOT"

echo ""
echo "✅ Ledger-object generated/ files written successfully!"
echo ""
echo "💡 These files are consumed via 'pub mod generated;' in"
echo "   xrpl-common-stdlib/src/objects/mod.rs."
