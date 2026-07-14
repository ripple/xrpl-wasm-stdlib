---
name: security-review
description: Comprehensive security audit for WASM smart contracts. Use when reviewing code for vulnerabilities, before deployments, or when the user mentions security.
allowed-tools: Read, Grep, Glob
---

# WASM Smart Contract Security Review

Perform a comprehensive security audit of the WASM smart contracts in this repository.

## 1. Memory Safety Violations

Search for heap allocations that could cause runtime panics:

- `Vec<` - Dynamic vectors
- `String` - Heap-allocated strings
- `Box<` - Heap-allocated pointers
- `HashMap` - Hash maps
- `BTreeMap` - B-tree maps
- `Rc<` or `Arc<` - Reference counting

**Action**: Flag any usage in WASM contract files (examples/smart-escrows/**/\*.rs, e2e-tests/**/\*.rs)

## 2. Standard Library Usage

Check for `std` usage in no_std contracts:

- Files should have `#![no_std]` attribute
- No `use std::` imports (except in tests with `#[cfg(test)]`)
- No `println!` or `print!` macros (use `trace` instead)

**Action**: Verify all WASM contracts are properly marked as no_std

## 3. Error Handling Issues

Look for unsafe error handling:

- `.unwrap()` calls (will panic on error)
- `.expect()` calls (will panic on error)
- Unhandled `Result` types
- Using `std::result::Result` instead of library's custom `Result`

**Action**: Ensure all errors are properly handled with match or `?` operator

## 4. Integer Overflow/Underflow

Check arithmetic operations:

- Unchecked addition, subtraction, multiplication
- Array indexing without bounds checks
- Type conversions that could truncate

**Action**: Recommend using checked arithmetic or proper validation

## 5. Determinism Violations

Smart contracts MUST be deterministic:

- No random number generation
- No system time usage
- No floating-point operations (unless using library's XFL types)
- No external I/O or network calls

**Action**: Flag any non-deterministic operations

## 6. Resource Exhaustion

Check for potential DoS vectors:

- Unbounded loops
- Recursive functions without depth limits
- Large fixed-size arrays that could overflow stack

**Action**: Verify all loops have clear termination conditions

## 7. Access Control

Verify proper authorization checks:

- Account verification before releasing escrow
- Signature validation
- Credential checks
- NFT ownership verification

**Action**: Ensure contracts validate all required conditions

## 8. Input Validation

Check for proper input validation:

- Transaction field validation
- Amount checks (non-zero, within bounds)
- Account ID validation
- Hash validation

**Action**: Verify all inputs are validated before use

## 9. Build Configuration

Review Cargo.toml for security issues:

- `panic = "abort"` is set (required for no_std)
- No unsafe dependencies
- Proper optimization settings

**Action**: Verify build configuration is secure

## 10. Test Coverage

Ensure security-critical paths are tested:

- Error conditions
- Edge cases
- Authorization failures
- Invalid inputs

**Action**: Recommend additional tests for uncovered security paths

## Report Format

Provide findings in this format:

### Critical Issues

- **Issue**: Description
- **Location**: File:Line
- **Impact**: Security impact
- **Recommendation**: How to fix

### Medium Issues

[Same format]

### Low Issues / Recommendations

[Same format]

### Summary

- Total issues found
- Critical: X
- Medium: Y
- Low: Z
- Overall security assessment
