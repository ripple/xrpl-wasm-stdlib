---
description: Build and test a specific smart escrow example
argument-hint: [example-name]
---

## Testing Smart Escrow Example: $ARGUMENTS

### Build the Example

!`./scripts/build.sh release`

### Run Integration Test

!`./scripts/run-tests.sh examples/smart-escrows/$ARGUMENTS`

### Check WASM Size

!`ls -lh examples/target/wasm32v1-none/release/$ARGUMENTS.wasm`

### Verify Exports

!`./scripts/check-wasm-exports.sh examples/target/wasm32v1-none/release/$ARGUMENTS.wasm`

---

Analyze the test results:

- Did the integration test pass?
- Is the WASM binary size reasonable (< 64KB)?
- Are the exports correct (should have `finish` function)?
- Any warnings or errors during build?

If there are issues, provide specific recommendations for fixes.
