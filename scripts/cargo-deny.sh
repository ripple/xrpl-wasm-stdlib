#!/bin/bash
# cargo-deny advisory check
# Mirrors the cargo_deny job from GitHub Actions.
#
# Advisory-only experiment (issue #126): checks RustSec advisories on the
# published library workspace (root Cargo.toml). examples/ and e2e-tests/ only
# path-depend on those crates, so they are not checked separately.
# licenses/bans/sources in deny.toml are not enforced yet.
#
# Usage:
#   ./scripts/cargo-deny.sh            # fail if the library workspace has advisories
#   ./scripts/cargo-deny.sh --warn-only  # print findings, always exit 0

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$REPO_ROOT"

WARN_ONLY=0
if [[ "${1:-}" == "--warn-only" ]]; then
    WARN_ONLY=1
elif [[ "${1:-}" != "" ]]; then
    echo "Usage: $0 [--warn-only]"
    exit 2
fi

if ! command -v cargo-deny &> /dev/null; then
    echo "❌ cargo-deny is not installed."
    echo "   Install with: cargo install cargo-deny --locked"
    exit 1
fi

echo "🔧 Running cargo-deny advisory check on the library workspace..."
echo "   $(cargo deny --version 2>/dev/null || true)"

status=0
if cargo deny --config "$REPO_ROOT/deny.toml" check advisories; then
    echo "✅ cargo-deny advisory check passed"
else
    echo "❌ cargo-deny advisory check failed"
    status=1
fi

if [[ "$WARN_ONLY" -eq 1 && "$status" -ne 0 ]]; then
    echo "⚠️  cargo-deny reported advisories, but --warn-only is set (issue #126 experiment)."
    exit 0
fi

exit "$status"
