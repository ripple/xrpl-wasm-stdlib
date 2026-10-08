#!/bin/bash
# Host function audit script
# Mirrors the host_function_audit job from GitHub Actions

set -euo pipefail

# Change to the repository root directory (where this script's grandparent directory is located)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$REPO_ROOT"

echo "🔧 Running host function audit..."

# Check if cargo is available
if ! command -v cargo &> /dev/null; then
    echo "❌ cargo is not installed. Please install Rust to run the host function audit."
    exit 1
fi

echo "🔍 Auditing host functions to ensure they match XRPLd host functions..."
cargo run --release --quiet --bin compare-host-functions -- https://github.com/XRPLF/rippled/tree/ripple/se/supported

echo "✅ Host function audit completed!"
echo "ℹ️  Note: This job should not be 'required' for PRs, as during development there may be temporary discrepancies between craft and rippled"
