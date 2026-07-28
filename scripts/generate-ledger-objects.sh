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
# With --check, the files are regenerated into a temp copy and diffed against what's
# committed; the script fails on any drift and leaves the working tree unchanged. This
# is the CI drift gate (run by run-all.sh and the GitHub Actions workflow).

set -euo pipefail

# Change to the repository root directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$REPO_ROOT"

CHECK=0
if [[ "${1:-}" == "--check" ]]; then
    CHECK=1
    shift
fi

# Default rippled source (can be overridden with the first argument, or the
# RIPPLED_BRANCH env var for just swapping the branch/tag/commit of the
# default XRPLF/rippled repo).
DEFAULT_SOURCE="https://github.com/XRPLF/rippled/tree/${RIPPLED_BRANCH:-xrplf/smart-contracts}"
RIPPLED_SOURCE="${1:-$DEFAULT_SOURCE}"

# Output root (can be overridden with the second argument in generate mode). Per-entry
# files are written under <OUTPUT_ROOT>/xrpl-common-stdlib/src/objects/generated/.
OUTPUT_ROOT="${2:-$REPO_ROOT}"

GENERATED_DIR="xrpl-common-stdlib/src/objects/generated"

# Check if Node.js is available
if ! command -v node &> /dev/null; then
    echo "❌ Node.js is not installed. Please install Node.js to run this script."
    exit 1
fi

if [[ "$CHECK" -eq 1 ]]; then
    echo "🔍 Checking generated ledger-object files are up to date..."
    echo "📦 rippled source: $RIPPLED_SOURCE"

    # Save the current contents so we can restore them after regenerating (the
    # generator writes in place; a passing check must never leave the tree dirty).
    TMP_DIR=$(mktemp -d)
    trap 'rm -rf "$TMP_DIR"' EXIT
    cp -R "$GENERATED_DIR" "$TMP_DIR/generated.before"

    echo "🔧 Regenerating ledger-object files from rippled..."
    node tools/generateLedgerObjects.js "$RIPPLED_SOURCE" "$REPO_ROOT"

    DRIFT=0
    if ! diff -rq "$TMP_DIR/generated.before" "$GENERATED_DIR" > /dev/null; then
        echo "❌ $GENERATED_DIR is out of date"
        diff -ru "$TMP_DIR/generated.before" "$GENERATED_DIR" || true
        DRIFT=1
    fi

    # Restore the original directory regardless of outcome.
    rm -rf "$GENERATED_DIR"
    cp -R "$TMP_DIR/generated.before" "$GENERATED_DIR"

    if [[ "$DRIFT" -ne 0 ]]; then
        echo ""
        echo "❌ Generated ledger-object files are out of date."
        echo "💡 Run ./scripts/generate-ledger-objects.sh and commit the result."
        exit 1
    fi

    echo "✅ Generated ledger-object files are up to date!"
    exit 0
fi

echo "🔧 Generating ledger-object field accessor traits..."
echo "📦 rippled source: $RIPPLED_SOURCE"
echo "📝 Output root:    $OUTPUT_ROOT"
echo ""

# Run the generator. It writes xrpl-common-stdlib/src/objects/generated/*.rs and
# runs `rustfmt --edition 2024` on its own output.
node tools/generateLedgerObjects.js "$RIPPLED_SOURCE" "$OUTPUT_ROOT"

echo ""
echo "✅ Ledger-object generated/ files written successfully!"
echo ""
echo "💡 These files are consumed via 'pub mod generated;' in"
echo "   xrpl-common-stdlib/src/objects/mod.rs."
