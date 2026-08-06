#!/bin/bash
# UI validation script - checks HTML structure, WASM embedding, and basic integrity

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$REPO_ROOT"

echo "🔍 Validating UI..."

UI_FILE="ui/index.html"
ERRORS=0

# Check if UI file exists
if [[ ! -f "$UI_FILE" ]]; then
    echo "❌ UI file not found: $UI_FILE"
    exit 1
fi

echo "📄 Checking HTML structure..."

# Check for required elements
required_elements=(
    "id=\"connectionStatus\""
    "id=\"networkInfo\""
    "id=\"connectBtn\""
    "id=\"disconnectBtn\""
    "id=\"generateBtn\""
    "id=\"accountsList\""
    "id=\"escrowsList\""
    "id=\"logArea\""
    "id=\"currentWasm\""
    "id=\"deployWasmStatus\""
)

for element in "${required_elements[@]}"; do
    if ! grep -q "$element" "$UI_FILE"; then
        echo "❌ Missing required element: $element"
        ((ERRORS++))
    fi
done

echo "✅ HTML structure validated"

echo "🔧 Checking JavaScript functions..."

# Check for required JavaScript functions
required_functions=(
    "function connectToNetwork()"
    "function disconnectFromNetwork()"
    "function generateAccount()"
    "function deployCurrentWasm()"
    "function finishEscrow()"
    "function log("
)

for func in "${required_functions[@]}"; do
    if ! grep -q "$func" "$UI_FILE"; then
        echo "❌ Missing required function: $func"
        ((ERRORS++))
    fi
done

echo "✅ JavaScript functions validated"

echo "📦 Checking WASM embedding..."

# Check that EMBEDDED_WASM object exists and has examples
if ! grep -q "const EMBEDDED_WASM = {" "$UI_FILE"; then
    echo "❌ EMBEDDED_WASM object not found"
    ((ERRORS++))
else
    # Count embedded WASM examples
    wasm_count=$(grep -c "\"0061736d" "$UI_FILE" || true)
    if [[ $wasm_count -eq 0 ]]; then
        echo "❌ No WASM examples embedded (no valid WASM hex found)"
        ((ERRORS++))
    else
        echo "✅ Found $wasm_count embedded WASM examples"
    fi
fi

# Check that EXAMPLES_WITH_README exists
if ! grep -q "const EXAMPLES_WITH_README = {" "$UI_FILE"; then
    echo "❌ EXAMPLES_WITH_README object not found"
    ((ERRORS++))
else
    echo "✅ EXAMPLES_WITH_README object found"
fi

echo "🔗 Checking external dependencies..."

# Check that xrpl library is loaded
if ! grep -q "xrpl@5.1.0-smartescrow" "$UI_FILE"; then
    echo "⚠️  Warning: xrpl library version may have changed"
fi

# Check that styles.css is referenced
if ! grep -q 'href="styles.css"' "$UI_FILE"; then
    echo "❌ styles.css not referenced in HTML"
    ((ERRORS++))
fi

echo "✅ External dependencies checked"

if [[ $ERRORS -gt 0 ]]; then
    echo ""
    echo "❌ UI validation failed with $ERRORS error(s)"
    exit 1
else
    echo ""
    echo "✅ UI validation passed!"
    exit 0
fi
