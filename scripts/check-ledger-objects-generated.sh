#!/bin/bash
# Ledger-object generated.rs drift check
# Regenerates the two generated ledger-object files from rippled source and
# verifies they match what's currently committed/on-disk. Fails if a source
# change (e.g. to ledger_entries.macro, sfields.macro, LedgerFormats.h,
# tools/generateLedgerObjects.js, tools/sfieldTypeMap.js, or tools/fieldDocs.json)
# would produce different output than what's checked in.

set -euo pipefail

# Change to the repository root directory (where this script's grandparent directory is located)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$REPO_ROOT"

echo "🔍 Checking generated ledger-object files are up to date..."

WASM_STDLIB_FILE="xrpl-common-stdlib/src/objects/generated.rs"
ESCROW_STDLIB_FILE="xrpl-escrow-stdlib/src/ledger_objects/generated.rs"

# Check if Node.js is available
if ! command -v node &> /dev/null; then
    echo "❌ Node.js is not installed. Please install Node.js to run this check."
    exit 1
fi

# Save the current contents of the generated files so we can restore them
# after regenerating (the generator writes in place, and this check must
# never leave the working tree dirty on success).
TMP_DIR=$(mktemp -d)
trap 'rm -rf "$TMP_DIR"' EXIT

cp "$WASM_STDLIB_FILE" "$TMP_DIR/wasm-stdlib.before.rs"
cp "$ESCROW_STDLIB_FILE" "$TMP_DIR/escrow-stdlib.before.rs"

echo "🔧 Regenerating ledger-object files from rippled..."
./scripts/generate-ledger-objects.sh

# generate-ledger-objects.sh already runs rustfmt on its output, so no extra
# formatting pass is needed here before comparing.

DRIFT=0

if ! diff -q "$TMP_DIR/wasm-stdlib.before.rs" "$WASM_STDLIB_FILE" > /dev/null; then
    echo "❌ $WASM_STDLIB_FILE is out of date"
    diff -u "$TMP_DIR/wasm-stdlib.before.rs" "$WASM_STDLIB_FILE" || true
    DRIFT=1
fi

if ! diff -q "$TMP_DIR/escrow-stdlib.before.rs" "$ESCROW_STDLIB_FILE" > /dev/null; then
    echo "❌ $ESCROW_STDLIB_FILE is out of date"
    diff -u "$TMP_DIR/escrow-stdlib.before.rs" "$ESCROW_STDLIB_FILE" || true
    DRIFT=1
fi

# Restore the original files regardless of outcome, so this check never
# leaves the working tree dirty.
cp "$TMP_DIR/wasm-stdlib.before.rs" "$WASM_STDLIB_FILE"
cp "$TMP_DIR/escrow-stdlib.before.rs" "$ESCROW_STDLIB_FILE"

if [[ "$DRIFT" -ne 0 ]]; then
    echo ""
    echo "❌ Generated ledger-object files are out of date."
    echo "💡 Run ./scripts/generate-ledger-objects.sh and commit the result."
    exit 1
fi

echo "✅ Generated ledger-object files are up to date!"
