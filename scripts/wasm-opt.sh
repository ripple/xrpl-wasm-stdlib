#!/bin/bash
# Shrink built release WASM contracts in place with Binaryen's wasm-opt (~30%).
# A sidecar `<name>.wasm.opt-stamp` holds the optimized hash so reruns skip
# modules cargo didn't relink. Installs wasm-opt if missing and fails if it
# can't, but exits 0 when there's simply no release WASM to optimize.
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
# out of scripts/benchmark-gas.sh.
WASM_OPT_CRATE_VERSION="0.116.1"

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

if ! resolve_wasm_opt; then
    if [[ "${WASM_OPT_NO_INSTALL:-false}" == "true" ]]; then
        echo "❌ wasm-opt not found and WASM_OPT_NO_INSTALL=true."
        echo "   Install Binaryen (brew install binaryen | apt-get install binaryen),"
        echo "   or run: cargo install wasm-opt --version $WASM_OPT_CRATE_VERSION --locked"
        exit 1
    fi

    echo "📦 wasm-opt not found - installing wasm-opt v$WASM_OPT_CRATE_VERSION via cargo..."
    echo "   (this compiles Binaryen from source and takes a few minutes the first time;"
    echo "    it needs a C++17 toolchain. Set WASM_OPT_NO_INSTALL=true to opt out.)"
    # Cleared because the repo-wide -Dwarnings would fail on third-party crates.
    if ! RUSTFLAGS="" cargo install wasm-opt --version "$WASM_OPT_CRATE_VERSION" --locked; then
        echo "❌ Failed to install wasm-opt via cargo."
        echo "   Install Binaryen another way (brew install binaryen | apt-get install binaryen)"
        echo "   and re-run, or set SKIP_WASM_OPT=true to build without optimization."
        exit 1
    fi
    if ! resolve_wasm_opt; then
        echo "❌ cargo install succeeded but wasm-opt is still not on PATH or in $CARGO_BIN."
        exit 1
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

hash_of() {
    if command -v shasum &> /dev/null; then
        shasum -a 256 "$1" | cut -d' ' -f1
    else
        sha256sum "$1" | cut -d' ' -f1
    fi
}

# The stamp covers the optimizer and its flags as well as the bytes, so that
# editing WASM_OPT_FLAGS or moving to a different wasm-opt re-optimizes a module
# cargo hasn't relinked.
OPT_SIG="$($WASM_OPT --version) | ${FLAGS[*]}"
stamp_value() {
    echo "$(hash_of "$1") | $OPT_SIG"
}

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
optimized=0
skipped=0

for wasm in "${WASM_FILES[@]}"; do
    name="$(basename "$wasm")"
    stamp="$wasm.opt-stamp"
    before=$(wc -c < "$wasm" | tr -d ' ')

    if [[ -f "$stamp" ]] && [[ "$(cat "$stamp")" == "$(stamp_value "$wasm")" ]]; then
        echo "   ⏭️  $name (already optimized, ${before} bytes)"
        skipped=$((skipped + 1))
        total_before=$((total_before + before))
        total_after=$((total_after + before))
        continue
    fi

    tmp="$wasm.opt.tmp"
    if ! "$WASM_OPT" "${FLAGS[@]}" "$wasm" -o "$tmp"; then
        rm -f "$tmp"
        echo "❌ wasm-opt failed on $wasm"
        exit 1
    fi
    mv "$tmp" "$wasm"

    after=$(wc -c < "$wasm" | tr -d ' ')
    stamp_value "$wasm" > "$stamp"
    optimized=$((optimized + 1))
    total_before=$((total_before + before))
    total_after=$((total_after + after))
    echo "   ✅ $name: ${before} -> ${after} bytes ($(( (before - after) * 100 / (before > 0 ? before : 1) ))% smaller)"
done

if [[ $total_before -gt 0 ]]; then
    echo "🗜️  wasm-opt: ${optimized} optimized, ${skipped} unchanged; ${total_before} -> ${total_after} bytes total"
fi
