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
# Escrow is generated slot-only: the slot-based `EscrowFields` trait + `Escrow` struct
# are emitted here, but its current-object accessors and the host-mutable `Data`
# (ContractData) field stay hand-written in xrpl-escrow-stdlib.
#
# Only a single rippled source is read (unlike generate-sfields.sh / generate-tx-flags.sh,
# which merge an escrow branch and a contract branch -- ledger-entry definitions don't
# currently have that same base/contract split).
#
# Usage:
#   ./scripts/generate-ledger-objects.sh [rippled-source] [output-root]
#   ./scripts/generate-ledger-objects.sh --check [rippled-source]
#
# The rippled source defaults to the commit CI pins (the XRPLD_DOCKER_IMAGE tag in
# .github/workflows/test.yml, resolved by scripts/lib/xrpld-pins.sh). Override it
# with the first argument, or XRPLD_ESCROW_REF=<branch|tag|sha>.
#
# With --check, the files are regenerated in place, diffed against the current working-tree contents (not git),
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

# Default rippled source: the commit CI tests against (see lib/xrpld-pins.sh).
# Override with the first argument, or XRPLD_ESCROW_REF=<branch|tag|sha>.
RIPPLED_SOURCE="${1:-$(xrpld_escrow_source)}"

# Output root (can be overridden with the second argument in generate mode). Per-entry
# files are written under <OUTPUT_ROOT>/xrpl-common-stdlib/src/objects/generated/.
OUTPUT_ROOT="${2:-$REPO_ROOT}"

GENERATED_DIR="xrpl-common-stdlib/src/objects/generated"

# Check if Node.js is available
if ! command -v node &> /dev/null; then
    echo "❌ Node.js is not installed. Please install Node.js to run this script."
    exit 1
fi

# The generator writes in place and runs `rustfmt --edition 2024` on its own output.
regenerate() {
    node tools/generateLedgerObjects.js "$RIPPLED_SOURCE" "$REPO_ROOT"
}

if [[ "$CHECK" -eq 1 ]]; then
    echo "🔍 Checking generated ledger-object files are up to date..."
    echo "📦 rippled source: $RIPPLED_SOURCE"
    check_generated regenerate "./scripts/generate-ledger-objects.sh" "$GENERATED_DIR"
    exit $?
fi

echo "🔧 Generating ledger-object field accessor traits..."
echo "📦 rippled source: $RIPPLED_SOURCE"
echo "📝 Output root:    $OUTPUT_ROOT"
echo ""

node tools/generateLedgerObjects.js "$RIPPLED_SOURCE" "$OUTPUT_ROOT"

echo ""
echo "✅ Ledger-object generated/ files written successfully!"
echo ""
echo "💡 These files are consumed via 'pub mod generated;' in"
echo "   xrpl-common-stdlib/src/objects/mod.rs."
