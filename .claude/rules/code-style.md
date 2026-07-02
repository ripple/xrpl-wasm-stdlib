# Code Style Guidelines

## File Naming Conventions

### Rust Files

- **Source files**: snake_case (e.g., `account_id.rs`, `error_codes.rs`)
- **Crate names**: kebab-case (e.g., `xrpl-wasm-stdlib`, `xrpl-macros`)
- **Module directories**: snake_case (e.g., `float_tests`, `gas_benchmark`)

### JavaScript Files

- **All JS files**: camelCase (e.g., `runTest.js`, `deployWasmCode.js`)
- **Config files**: Standard naming (`package.json`, `tsconfig.json`)

### Shell Scripts

- **All scripts**: kebab-case (e.g., `build-and-test.sh`, `run-tests.sh`)

### Documentation

- **Special docs**: SCREAMING_SNAKE_CASE (`README.md`, `CONTRIBUTING.md`)
- **Other docs**: kebab-case (`comprehensive-guide.md`, `naming-conventions.md`)

## Rust Style

Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

### Formatting

Enforced by `cargo fmt`:

- 4-space indentation
- 100-character line limit
- Trailing commas in multi-line items

### Linting

Enforced by `cargo clippy` with `-Dclippy::all`:

- All warnings are errors
- No unused imports or variables
- Proper error handling
- Idiomatic Rust patterns

### Documentation

````rust
/// Brief description of the function.
///
/// More detailed explanation if needed.
///
/// # Arguments
///
/// * `param1` - Description of param1
/// * `param2` - Description of param2
///
/// # Returns
///
/// Description of return value
///
/// # Errors
///
/// Description of possible errors
///
/// # Examples
///
/// ```
/// let result = function(arg1, arg2);
/// ```
pub fn function(param1: Type1, param2: Type2) -> Result<ReturnType> {
    // Implementation
}
````

### Error Handling

```rust
// GOOD: Explicit error handling
match operation() {
    Ok(value) => value,
    Err(e) => {
        trace(&format!("Error: {:?}", e)).ok();
        return 0;
    }
}

// BAD: Using unwrap() or expect()
let value = operation().unwrap(); // NEVER do this in WASM contracts
```

## JavaScript Style

### TypeScript Strict Mode

When using TypeScript:

- Strict mode is ON
- No unused imports or variables
- Explicit types for function parameters and returns

### Node.js Patterns

```javascript
// Use async/await
async function runTest() {
  try {
    const result = await someOperation()
    console.log("Success:", result)
  } catch (error) {
    console.error("Error:", error)
    throw error
  }
}

// Proper error handling
runTest().catch(console.error)
```

## Shell Script Style

### Error Handling

Every script should start with:

```bash
#!/bin/bash
set -euo pipefail
```

- `set -e`: Exit on error
- `set -u`: Exit on undefined variable
- `set -o pipefail`: Exit on pipe failure

### Script Structure

```bash
#!/bin/bash
set -euo pipefail

# Change to repo root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$REPO_ROOT"

# Script logic here
```

## Comments

### When to Comment

- Complex algorithms or business logic
- Non-obvious workarounds
- Performance-critical sections
- Public API documentation

### When NOT to Comment

- Obvious code (let the code speak)
- Redundant information
- Outdated comments (update or remove)

## Import Organization

### Rust

```rust
// Standard library (if not no_std)
use std::collections::HashMap;

// External crates
use xrpl_wasm_stdlib::host::{Result, Error};

// Internal modules
use crate::core::types::AccountID;

// Specific imports
use xrpl_wasm_stdlib::host::Result::{Ok, Err};
```

### JavaScript

```javascript
// Node.js built-ins
const fs = require("fs")
const path = require("path")

// External packages
const xrpl = require("xrpl")

// Local modules
const { deployWasm } = require("./deployWasmCode")
```

## Consistency

- Follow existing patterns in the file/module
- When in doubt, match surrounding code
- Use automated tools (`cargo fmt`, `clippy`) to enforce consistency
