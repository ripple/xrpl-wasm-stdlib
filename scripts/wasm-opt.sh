#!/bin/bash
# Shrink built release WASM contracts in place with Binaryen's wasm-opt (~30%).
# Installs wasm-opt if missing or off the pin, and fails if it can't, but exits
# 0 when there's simply no release WASM to optimize.
#
# No result caching: cargo re-uplifts the profile-dir .wasm from deps/ on every
# build (our in-place write breaks its hardlink), so each build.sh run is
# already exactly one pass over pristine output.
#
# Usage:
#   ./scripts/wasm-opt.sh                      # optimize the default release dirs
#   ./scripts/wasm-opt.sh path/to/dir a.wasm   # optimize specific dirs/files
#   ./scripts/wasm-opt.sh --ensure-tool        # only resolve/install wasm-opt, optimize nothing
#
# Environment:
#   SKIP_WASM_OPT=true       explicit opt-out of the whole step
#   WASM_OPT=<path>          use a specific wasm-opt binary instead of resolving one
#   WASM_OPT_NO_INSTALL=true never auto-install; fail if wasm-opt isn't already present
#   WASM_OPT_FLAGS="..."     replace the default flag set

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$REPO_ROOT"

if [[ "${SKIP_WASM_OPT:-false}" == "true" ]]; then
    echo "⏭️  SKIP_WASM_OPT set - skipping wasm-opt."
    exit 0
fi

ENSURE_ONLY=false
if [[ "${1:-}" == "--ensure-tool" ]]; then
    ENSURE_ONLY=true
    shift
fi

# Pinned: a different Binaryen means different bytes, and different gas numbers
# out of scripts/benchmark-gas.sh. The crate is versioned 0.<binaryen>.<patch>.
WASM_OPT_CRATE_VERSION="0.116.1"
PINNED_BINARYEN_VERSION="$(echo "$WASM_OPT_CRATE_VERSION" | cut -d. -f2)"

WASM_OPT_USER_SET=false
[[ -n "${WASM_OPT:-}" ]] && WASM_OPT_USER_SET=true

CARGO_BIN="${CARGO_HOME:-$HOME/.cargo}/bin"
resolve_wasm_opt() {
    if [[ -n "${WASM_OPT:-}" ]]; then
        command -v "$WASM_OPT" &> /dev/null && return 0
        echo "❌ WASM_OPT is set to '$WASM_OPT', which is not executable."
        exit 1
    fi
    if command -v wasm-opt &> /dev/null; then
        WASM_OPT="$(command -v wasm-opt)"
        return 0
    fi
    if [[ -x "$CARGO_BIN/wasm-opt" ]]; then
        WASM_OPT="$CARGO_BIN/wasm-opt"
        return 0
    fi
    return 1
}

binaryen_version_of() {
    "$1" --version 2>/dev/null | grep -oE '[0-9]+' | head -1
}

install_pinned_wasm_opt() {
    if [[ "${WASM_OPT_NO_INSTALL:-false}" == "true" ]]; then
        echo "❌ Need wasm-opt v$WASM_OPT_CRATE_VERSION but WASM_OPT_NO_INSTALL=true."
        echo "   Install Binaryen (brew install binaryen | apt-get install binaryen),"
        echo "   or run: cargo install wasm-opt --version $WASM_OPT_CRATE_VERSION --locked"
        exit 1
    fi
    echo "   (compiles Binaryen from source, a few minutes; needs a C++17 toolchain."
    echo "    Set WASM_OPT_NO_INSTALL=true to opt out.)"
    # Cleared because the repo-wide -Dwarnings would fail on third-party crates.
    if ! RUSTFLAGS="" cargo install wasm-opt --version "$WASM_OPT_CRATE_VERSION" --locked; then
        echo "❌ Failed to install wasm-opt via cargo."
        echo "   Install Binaryen another way (brew install binaryen | apt-get install binaryen)"
        echo "   and re-run, or set SKIP_WASM_OPT=true to build without optimization."
        exit 1
    fi
}

if ! resolve_wasm_opt; then
    echo "📦 wasm-opt not found - installing wasm-opt v$WASM_OPT_CRATE_VERSION via cargo..."
    install_pinned_wasm_opt
    if ! resolve_wasm_opt; then
        echo "❌ cargo install succeeded but wasm-opt is still not on PATH or in $CARGO_BIN."
        exit 1
    fi
fi

# A cargo-managed wasm-opt must actually match the pin. CI caches ~/.cargo/bin
# under a key derived from Cargo.lock, which a pin bump here doesn't change, so
# without this the old binary would silently survive the bump. A system Binaryen
# (brew/apt) is deliberately left alone - see AGENTS.md.
if [[ "$WASM_OPT_USER_SET" == "false" && "$WASM_OPT" -ef "$CARGO_BIN/wasm-opt" ]]; then
    have="$(binaryen_version_of "$WASM_OPT")"
    if [[ "$have" != "$PINNED_BINARYEN_VERSION" ]]; then
        echo "♻️  cargo-installed wasm-opt is Binaryen $have, pin wants $PINNED_BINARYEN_VERSION - reinstalling..."
        install_pinned_wasm_opt
        have="$(binaryen_version_of "$WASM_OPT")"
        if [[ "$have" != "$PINNED_BINARYEN_VERSION" ]]; then
            echo "❌ Installed wasm-opt v$WASM_OPT_CRATE_VERSION but got Binaryen $have, expected $PINNED_BINARYEN_VERSION."
            echo "   If the crate no longer versions as 0.<binaryen>.<patch>, fix PINNED_BINARYEN_VERSION in this script."
            exit 1
        fi
    fi
fi

if [[ "$ENSURE_ONLY" == "true" ]]; then
    echo "✅ $($WASM_OPT --version) ready at $WASM_OPT"
    exit 0
fi

# MVP + sign-ext + mutable-globals is exactly wasm32v1-none. Don't widen it:
# post-MVP instructions (bulk memory, SIMD, reftypes) would be rejected by the
# target and by rippled's engine.
DEFAULT_WASM_OPT_FLAGS=(
    -Oz
    --mvp-features
    --enable-sign-ext
    --enable-mutable-globals
    --strip-debug
    --strip-producers
)
if [[ -n "${WASM_OPT_FLAGS:-}" ]]; then
    # shellcheck disable=SC2206 # intentional word splitting of a user-supplied flag string
    FLAGS=($WASM_OPT_FLAGS)
else
    FLAGS=("${DEFAULT_WASM_OPT_FLAGS[@]}")
fi

# Release only; debug builds stay unstripped for troubleshooting.
TARGETS=("$@")
EXPLICIT_TARGETS=true
if [[ ${#TARGETS[@]} -eq 0 ]]; then
    EXPLICIT_TARGETS=false
    TARGETS=(
        "examples/target/wasm32v1-none/release"
        "e2e-tests/target/wasm32v1-none/release"
    )
fi

# maxdepth 1: deps/ and build/ below the profile dir hold intermediates.
WASM_FILES=()
for target in "${TARGETS[@]}"; do
    if [[ -d "$target" ]]; then
        while IFS= read -r wasm; do
            WASM_FILES+=("$wasm")
        done < <(find "$target" -maxdepth 1 -name '*.wasm' | sort)
    elif [[ -f "$target" ]]; then
        WASM_FILES+=("$target")
    elif [[ "$EXPLICIT_TARGETS" == "true" ]]; then
        echo "❌ $target not found."
        exit 1
    fi
done

# Nothing to do is only an error when the caller named the targets; with the
# defaults it just means no release WASM was built (e.g. a debug-only build).
if [[ ${#WASM_FILES[@]} -eq 0 ]]; then
    if [[ "$EXPLICIT_TARGETS" == "true" ]]; then
        echo "❌ No .wasm files found in: ${TARGETS[*]}"
        exit 1
    fi
    echo "   No release WASM found - nothing to optimize."
    exit 0
fi

echo "🗜️  Optimizing WASM with $($WASM_OPT --version)..."

total_before=0
total_after=0

for wasm in "${WASM_FILES[@]}"; do
    name="$(basename "$wasm")"
    before=$(wc -c < "$wasm" | tr -d ' ')

    tmp="$wasm.opt.tmp"
    if ! "$WASM_OPT" "${FLAGS[@]}" "$wasm" -o "$tmp"; then
        rm -f "$tmp"
        echo "❌ wasm-opt failed on $wasm"
        exit 1
    fi
    mv "$tmp" "$wasm"

    after=$(wc -c < "$wasm" | tr -d ' ')
    total_before=$((total_before + before))
    total_after=$((total_after + after))
    echo "   ✅ $name: ${before} -> ${after} bytes ($(( (before - after) * 100 / (before > 0 ? before : 1) ))% smaller)"
done

echo "🗜️  wasm-opt: ${#WASM_FILES[@]} contracts, ${total_before} -> ${total_after} bytes total"
