#!/bin/bash
# WASM version metadata checking script
# Checks that every built contract carries a version custom section for each stdlib crate it
# depends on, and that the embedded version matches that crate's current Cargo.toml version.
#
# This guards the `#[used]` + `#[link_section = "<crate>-version"]` statics in
# xrpl-common-stdlib/src/lib.rs and xrpl-escrow-stdlib/src/lib.rs against being silently
# dropped by the linker under the release profile's `lto = true` / `codegen-units = 1`.
#
# Requires release WASM artifacts, so run this after ./scripts/build.sh (or build-and-test.sh).

set -euo pipefail

# Change to the repository root directory (where this script's parent directory is located)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$REPO_ROOT"

# Crates that embed a version section. The section name is "<crate>-version".
CRATES=("xrpl-common-stdlib" "xrpl-escrow-stdlib")

# First `version = "x.y.z"` under [package] in the crate's manifest.
crate_version() {
    grep -m1 '^version' "$1/Cargo.toml" | sed -E 's/.*"(.*)".*/\1/'
}

# examples/target/.../release/hello_world.wasm -> the manifest of the crate named hello_world
contract_manifest() {
    local stem root
    stem="$(basename "$1" .wasm)"
    root="${1%%/target/*}"
    grep -rl "^name *= *\"${stem}\"" "$root" --include=Cargo.toml | head -1
}

echo "🔍 Checking WASM version metadata sections..."

failed=0
found_any=0

for wasm in examples/target/wasm32v1-none/release/*.wasm \
    e2e-tests/target/wasm32v1-none/release/*.wasm; do
    [[ -f "$wasm" ]] || continue
    found_any=1
    echo "🔧 Checking $wasm"

    manifest="$(contract_manifest "$wasm")"
    if [[ -z "$manifest" ]]; then
        echo "   ⚠️  No manifest found for $(basename "$wasm"), skipping"
        continue
    fi

    for crate in "${CRATES[@]}"; do
        # Only require a section for crates this contract actually depends on.
        grep -q "^${crate} *=" "$manifest" || continue

        section="${crate}-version"
        expected="$(crate_version "$crate")"

        # In the WASM binary a custom section's name is immediately followed by its payload,
        # so one match asserts both "the section survived" and "the version is current".
        if strings -a "$wasm" | grep -q "${section}${expected}"; then
            echo "   ✅ ${section} = ${expected}"
        elif strings -a "$wasm" | grep -q "${section}"; then
            actual="$(strings -a "$wasm" | grep -o "${section}[0-9][0-9.]*" | head -1 | sed "s/${section}//")"
            echo "   ❌ ${section} is stale: found '${actual}', expected '${expected}'"
            failed=1
        else
            echo "   ❌ ${section} is missing — the version static was dropped from the module"
            failed=1
        fi
    done
done

if [[ "$found_any" -eq 0 ]]; then
    echo "❌ No release WASM artifacts found. Run ./scripts/build.sh release first."
    exit 1
fi

if [[ "$failed" -ne 0 ]]; then
    echo "❌ WASM version metadata check failed!"
    exit 1
fi

echo "✅ WASM version metadata check passed!"
