#!/bin/bash
# Ledger-object generated/ drift check
# Regenerates xrpl-common-stdlib/src/objects/generated/ from rippled source and
# verifies it matches what's currently committed/on-disk. Fails if a source change
# (e.g. to ledger_entries.macro, sfields.macro, tools/generateLedgerObjects.js, or
# tools/sfieldTypeMap.js) -- or a change to the xrpl-dev-portal doc pages --  would
# produce different output than what's checked in.

set -euo pipefail

# Change to the repository root directory (where this script's grandparent directory is located)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$REPO_ROOT"

echo "🔍 Checking generated ledger-object files are up to date..."

GENERATED_DIR="xrpl-common-stdlib/src/objects/generated"

# Check if Node.js is available
if ! command -v node &> /dev/null; then
    echo "❌ Node.js is not installed. Please install Node.js to run this check."
    exit 1
fi

# Save the current contents of the generated directory so we can restore it after
# regenerating (the generator writes in place, and this check must never leave the
# working tree dirty on success).
TMP_DIR=$(mktemp -d)
trap 'rm -rf "$TMP_DIR"' EXIT

cp -R "$GENERATED_DIR" "$TMP_DIR/generated.before"

echo "🔧 Regenerating ledger-object files from rippled..."
./scripts/generate-ledger-objects.sh

# generate-ledger-objects.sh already runs rustfmt on its output, so no extra
# formatting pass is needed here before comparing.

DRIFT=0

if ! diff -rq "$TMP_DIR/generated.before" "$GENERATED_DIR" > /dev/null; then
    echo "❌ $GENERATED_DIR is out of date"
    diff -ru "$TMP_DIR/generated.before" "$GENERATED_DIR" || true
    DRIFT=1
fi

# Restore the original directory regardless of outcome, so this check never leaves
# the working tree dirty.
rm -rf "$GENERATED_DIR"
cp -R "$TMP_DIR/generated.before" "$GENERATED_DIR"

if [[ "$DRIFT" -ne 0 ]]; then
    echo ""
    echo "❌ Generated ledger-object files are out of date."
    echo "💡 Run ./scripts/generate-ledger-objects.sh and commit the result."
    exit 1
fi

echo "✅ Generated ledger-object files are up to date!"
