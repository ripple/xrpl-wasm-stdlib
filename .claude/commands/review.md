---
description: Review the current branch diff for issues before merging
---

## Changes to Review

!`git diff --name-only main...HEAD`

## Detailed Diff

!`git diff main...HEAD`

Review the above changes for:

1. **Code Quality**
   - Rust: Follow API guidelines, proper error handling with custom Result types
   - JavaScript: camelCase naming, no unused imports
   - Shell: kebab-case naming, proper error handling with `set -euo pipefail`

2. **WASM Contract Safety**
   - Verify `#![no_std]` and `#![no_main]` attributes
   - Check for heap allocations (Vec, String, Box, HashMap)
   - Ensure wasm32v1-none target usage
   - Verify finish() returns i32 (1 for success, 0 for failure)
   - Confirm proper error handling with library's Result type

3. **Testing**
   - Unit tests for new functionality
   - Integration tests (runTest.js) for new examples
   - All tests pass with `./scripts/run-all.sh`

4. **Documentation**
   - README updates for new examples
   - API documentation for public functions
   - Inline comments for complex logic

5. **Performance & Size**
   - WASM binaries < 64KB
   - Proper release profile settings (opt-level = "s", lto = true)
   - No unnecessary dependencies

6. **Naming Conventions**
   - Rust files: snake_case
   - JavaScript files: camelCase
   - Shell scripts: kebab-case
   - Crates: kebab-case

Give specific, actionable feedback per file with line numbers where applicable.
