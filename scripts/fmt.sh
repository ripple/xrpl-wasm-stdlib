#!/bin/bash
# Rust formatting check script
# Mirrors the rustfmt job from GitHub Actions

set -euo pipefail

# Change to the repository root directory (where this script's grandparent directory is located)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$REPO_ROOT"

echo "🔧 Running Rust formatting check..."

echo "📝 Checking formatting for entire workspace..."
cargo fmt --all -- --check
(cd examples && cargo fmt --all -- --check)
(cd e2e-tests && cargo fmt --all -- --check)
(cd tools/generate-host-bindings && cargo fmt --all -- --check)

echo "✅ Formatting check passed!"
