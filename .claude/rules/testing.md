# Testing Standards

## Test Types

### 1. Unit Tests (Rust)

Located in the same file as the code being tested:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::host::host_bindings_trait::MockHostBindings;
    use crate::host::setup_mock;
    use mockall::predicate::always;

    #[test]
    fn test_function_name() {
        // Test implementation
    }
}
```

Run with:

```bash
cargo test --workspace
```

### 2. Integration Tests (JavaScript)

Every example MUST have `runTest.js`:

```javascript
const CONFIG = {
  wasmPath: "./target/wasm32v1-none/release/example_name.wasm",
  rippledHost: process.env.RIPPLED_HOST || "wasm.devnet.rippletest.net",
  testAccount: "rN7n7otQDd6FczFgLdSqtcsAUxDkw6fzRH",
}

async function runTest() {
  // Set up test scenario
  // Execute contract with test data
  // Verify expected results
  console.log("Test passed!")
}

runTest().catch(console.error)
```

Run with:

```bash
./scripts/run-tests.sh examples/smart-escrows/example_name
```

### 3. End-to-End Tests

Located in `e2e-tests/` directory for comprehensive validation.

## Test Networks

### Local Node (Default)

- Endpoint: `ws://localhost:6006`
- Use for development and CI
- Fast, deterministic

### WASM Devnet

- Endpoint: `wss://wasm.devnet.rippletest.net:51233`
- Use for final validation before PR
- Run with: `DEVNET=true ./scripts/run-tests.sh`

## Testing Checklist

Before submitting a PR:

- [ ] All unit tests pass: `cargo test --workspace`
- [ ] All integration tests pass: `./scripts/run-tests.sh`
- [ ] Clippy passes: `./scripts/clippy.sh`
- [ ] Formatting passes: `./scripts/fmt.sh`
- [ ] Full test suite passes: `./scripts/run-all.sh`
- [ ] Tested on WASM devnet: `DEVNET=true ./scripts/run-tests.sh`

## Debugging Tests

### Using trace() in Contracts

```rust
use xrpl_wasm_stdlib::host::trace::{trace, trace_num, trace_data, DataRepr};

trace("Debug message").ok();
trace_num("Value: ", 42).ok();
trace_data("Bytes", &data, DataRepr::AsHex).ok();
```

Trace output appears in rippled's `debug.log`.

### Manual Testing with UI

1. Build contract: `cargo build --target wasm32v1-none --release`
2. Open: https://ripple.github.io/xrpl-wasm-stdlib/ui/
3. Upload WASM file
4. Test interactively

## Test Coverage

Use coverage tool:

```bash
./scripts/coverage.sh
```

Coverage requirements:

- New features should have >80% coverage
- Critical paths should have 100% coverage
- Edge cases and error paths must be tested

## Common Test Patterns

### Testing Error Conditions

```rust
#[test]
fn test_error_handling() {
    let result = function_that_fails();
    assert!(result.is_err());
    assert_eq!(result.err().unwrap().code(), EXPECTED_ERROR_CODE);
}
```

### Testing Success Cases

```rust
#[test]
fn test_success_case() {
    let result = function_that_succeeds();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected_value);
}
```

### Using Mocks

```rust
#[test]
fn test_with_mock() {
    let mut mock = MockHostBindings::new();
    mock.expect_some_function()
        .with(always())
        .returning(|_| 0);
    setup_mock(mock);

    // Test code using mocked function
}
```

## Watch Out For

- Integration tests use **real rippled node**, not mocks
- Tests must be **deterministic** - no random values
- Clean up test data between runs
- Don't commit test artifacts
- Ensure tests work on both local and devnet
