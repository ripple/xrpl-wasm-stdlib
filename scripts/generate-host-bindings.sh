#!/bin/bash
# Generate the host-binding trait and wire-signature list from rippled's host ABI.
#
# Regenerates xrpl-common-stdlib/src/host/host_bindings_trait.rs and host_bindings_list.rs
# from the `host_functions! { ... }` block in rippled's crates/xrpl-host-functions/src/lib.rs,
# via the Rust generator in tools/generate-host-bindings. The three HostBindings
# implementations (wasm / empty / test) are hand-written macros that expand from the list,
# so they need no regeneration.
#
# Usage:
#   ./scripts/generate-host-bindings.sh [rippled-source]
#   ./scripts/generate-host-bindings.sh --check [rippled-source]
#
# rippled-source is a local checkout directory or a GitHub tree URL. The default pins the
# same rippled commit as XRPLD_DOCKER_IMAGE in .github/workflows/test.yml, so the bindings,
# the e2e node and this drift gate all describe one commit. RIPPLED_REF=<branch|sha>
# overrides just the ref.
#
# With --check, the files are generated into a temp dir and diffed against what's committed;
# the script fails on any drift and leaves the working tree unchanged. This is the CI drift
# gate (run by run-all.sh and the GitHub Actions workflow).

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$REPO_ROOT"

CHECK=0
if [[ "${1:-}" == "--check" ]]; then
    CHECK=1
    shift
fi

WORKFLOW_FILE=".github/workflows/test.yml"
ABI_FILE="crates/xrpl-host-functions/src/lib.rs"
HOST_DIR="xrpl-common-stdlib/src/host"
GENERATED_FILES=("host_bindings_trait.rs" "host_bindings_list.rs")

pinned_commit() {
    # Same single source of truth scripts/docker-rippled.sh reads.
    grep -m1 'XRPLD_DOCKER_IMAGE:' "$WORKFLOW_FILE" | sed -E 's/^.*://'
}

DEFAULT_REF="${RIPPLED_REF:-$(pinned_commit)}"
if [[ -z "$DEFAULT_REF" ]]; then
    echo "❌ Could not read XRPLD_DOCKER_IMAGE from $WORKFLOW_FILE" >&2
    exit 1
fi
RIPPLED_SOURCE="${1:-https://github.com/XRPLF/rippled/tree/$DEFAULT_REF}"

TMP_DIR=$(mktemp -d)
trap 'rm -rf "$TMP_DIR"' EXIT

echo "📦 rippled source: $RIPPLED_SOURCE"
if [[ -d "$RIPPLED_SOURCE" ]]; then
    cp "$RIPPLED_SOURCE/$ABI_FILE" "$TMP_DIR/lib.rs"
else
    RAW_URL="${RIPPLED_SOURCE/github.com/raw.githubusercontent.com}"
    RAW_URL="$(echo "$RAW_URL" | sed 's#/tree/#/#')/$ABI_FILE"
    curl -fsSL "$RAW_URL" -o "$TMP_DIR/lib.rs"
fi

generate_into() {
    mkdir -p "$1"
    cargo run --quiet --manifest-path tools/generate-host-bindings/Cargo.toml -- \
        "$TMP_DIR/lib.rs" "$1" --source-label "$RIPPLED_SOURCE"
}

if [[ "$CHECK" -eq 1 ]]; then
    echo "🔍 Checking generated host-binding files are up to date..."
    generate_into "$TMP_DIR/out"

    DRIFT=0
    for f in "${GENERATED_FILES[@]}"; do
        if ! diff -u "$HOST_DIR/$f" "$TMP_DIR/out/$f"; then
            echo "❌ $HOST_DIR/$f is out of date"
            DRIFT=1
        fi
    done

    if [[ "$DRIFT" -ne 0 ]]; then
        echo ""
        echo "❌ Generated host-binding files are out of date."
        echo "💡 Run ./scripts/generate-host-bindings.sh and commit the result."
        exit 1
    fi
    echo "✅ Generated host-binding files are up to date!"
    exit 0
fi

echo "🔧 Generating host-binding files..."
generate_into "$HOST_DIR"
echo "✅ Wrote ${GENERATED_FILES[*]} to $HOST_DIR"
echo "💡 host_bindings_wasm.rs / host_bindings_empty.rs / host_bindings_test.rs expand from the list and need no changes,"
echo "   except apply_default_expectations in host_bindings_test.rs when a function was added."
