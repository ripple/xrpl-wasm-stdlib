#!/bin/bash
# Host function audit script
# Mirrors the host_function_audit job from GitHub Actions

set -euo pipefail

# Change to the repository root directory (where this script's grandparent directory is located)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$REPO_ROOT"

echo "🔧 Running host function audit..."

# Check if Node.js is available
if ! command -v node &> /dev/null; then
    echo "❌ Node.js is not installed. Please install Node.js to run the host function audit."
    exit 1
fi

echo "🔍 Auditing host functions to ensure they match XRPLd host functions..."
node tools/compareHostFunctions.js https://github.com/XRPLF/rippled/tree/ripple/se/supported

echo "✅ Host function audit completed!"
echo "ℹ️  Note: This job should not be 'required' for PRs, as during development there may be temporary discrepancies between craft and rippled"
