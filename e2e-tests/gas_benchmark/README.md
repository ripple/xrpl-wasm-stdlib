# Gas Benchmark Contract

This is a WASM contract that benchmarks stdlib helper functions to measure gas consumption across different optimization branches.

## Overview

The contract exercises 12 different stdlib functions with 100 iterations each, measuring the total gas used. Results are compared between branches to identify performance improvements or regressions.

## Benchmarked Functions

### Transaction Field Access (2)

- `get_account()` - Retrieve account ID from transaction
- `get_fee()` - Retrieve fee amount from transaction

### Error Code Matching (4)

- `match_result_code()` - Basic result code validation
- `match_result_code_optional()` - Result code with optional semantics
- `match_result_code_with_expected_bytes()` - Exact byte count validation
- `match_result_code_with_expected_bytes_optional()` - Byte count with optional semantics

### Result Type Methods (4)

- `is_ok()` - Check if Result is Ok
- `is_err()` - Check if Result is Err
- `ok()` - Convert Result to Option
- `err()` - Extract error from Result

### Hex Decoding (2)

- `decode_hex_32()` - Decode 64-character hex to 32-byte array
- `decode_hex_20()` - Decode 40-character hex to 20-byte array

## Adding New Benchmarks

To add a new benchmark function:

### 1. Create the benchmark function

Add a new function in `src/lib.rs`:

```rust
/// Benchmark my_new_function
fn benchmark_my_new_function() -> u64 {
    const ITERATIONS: usize = 100;
    let mut count = 0u64;
    for _ in 0..ITERATIONS {
        // Call the function you want to benchmark
        if my_function().is_ok() {
            count += 1;
        }
    }
    count
}
```

**Important:** Always accumulate results into a counter and return it. This prevents the compiler from optimizing away the function calls.

### 2. Call it from finish()

Add a call in the `finish()` function:

```rust
trace("BENCHMARK: my_new_function");
accumulator = accumulator.wrapping_add(benchmark_my_new_function());
```

### 3. Rebuild and test

```bash
cd e2e-tests && cargo build -p gas_benchmark --target wasm32v1-none --release
./scripts/benchmark-gas.sh
```

## Design Principles

1. **Prevent Compiler Optimization** - Each benchmark accumulates results into a counter that's used in the return value, preventing the compiler from optimizing away function calls.

2. **Consistent Iterations** - All benchmarks run 100 iterations to ensure measurable gas differences.

3. **Trace Markers** - Each benchmark is marked with `trace()` calls for easy identification in transaction logs.

4. **Return Accumulator** - The final return value depends on the accumulator, ensuring all benchmarks are executed.

## Running Benchmarks

### Benchmark Specific Contract

```bash
# Benchmark gas_benchmark (default)
./scripts/benchmark-gas.sh

# Benchmark a specific contract
node tools/gasBenchmark.js my_contract

# Benchmark all contracts in e2e-tests
node tools/gasBenchmark.js all
```

### Generate Comparison Report

```bash
node tools/compareGasResults.js
```

### Compare Two Branches

```bash
# 1. Benchmark current branch
./scripts/benchmark-gas.sh

# 2. Switch to other branch (e.g., main)
git checkout main

# 3. Benchmark that branch
./scripts/benchmark-gas.sh

# 4. Generate comparison report
node tools/compareGasResults.js
```

### Benchmark Multiple Contracts

```bash
# Benchmark specific contracts
node tools/gasBenchmark.js gas_benchmark my_contract

# Benchmark all contracts
node tools/gasBenchmark.js all
```

## Output

Results are stored in `.benchmark/` (gitignored):

- `.benchmark/gas_benchmark_results.json` - Raw measurement data
- `.benchmark/GAS_BENCHMARK_REPORT.md` - Formatted comparison report

## Troubleshooting

### Gas readings are identical between branches

- Ensure the optimizations are actually applied to the stdlib
- Check that the benchmark functions are calling the optimized code
- Verify the WASM contract is being rebuilt with `cargo build --release`

### Binary size doesn't change

- Binary size is deterministic - it only changes if the code actually changes
- Verify optimizations are in the source code

### Connection errors

- Ensure local rippled instance is running on `ws://127.0.0.1:6006`
- Check that the instance is accepting transactions
