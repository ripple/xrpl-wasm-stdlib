# File Naming Conventions

This document outlines the standardized file naming conventions for the xrpl-wasm-stdlib project.

## Overview

Consistent file naming improves code readability, maintainability, and reduces confusion when navigating the codebase.
Different file types follow different conventions based on their language ecosystem and community standards.

## Conventions by File Type

### Rust Files

**Source Files (`.rs`)**

- Use **snake_case** for all Rust source files
- Examples: `account_id.rs`, `hash_256.rs`, `error_codes.rs`, `host_bindings.rs`
- Rationale: This is the idiomatic Rust convention as specified in
  the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/naming.html)

**Crate/Package Names**

- Use **kebab-case** for crate names in `Cargo.toml` and top-level crate directories
- Examples: `xrpl-wasm-stdlib`, `xrpl-address-macro`
- Rationale: Cargo convention for package names

**Module Directories**

- Use **snake_case** for module directories that map to Rust modules
- Examples: `float_tests`, `gas_benchmark`, `host_functions_test`
- Rationale: Consistency with Rust module naming

### JavaScript/TypeScript Files

**JavaScript Files (`.js`, `.ts`)**

- Use **camelCase** for all JavaScript and TypeScript files
- Examples: `compareGasResults.js`, `gasBenchmark.js`, `deployWasmCode.js`, `runSingleTest.js`
- Rationale: This is the traditional Node.js convention and aligns with JavaScript naming patterns

**Configuration Files**

- Use the standard naming for configuration files: `package.json`, `package-lock.json`, `tsconfig.json`
- These follow their ecosystem conventions

### Shell Scripts

**Shell Scripts (`.sh`)**

- Use **kebab-case** for all shell scripts
- Examples: `benchmark-gas.sh`, `build-and-test.sh`, `check-wasm-exports.sh`, `run-tests.sh`
- Rationale: This is the Unix/Linux convention for command-line tools

### Documentation Files

**Markdown Files (`.md`)**

- Use **SCREAMING_SNAKE_CASE** for special documentation files: `README.md`, `CONTRIBUTING.md`, `LICENSE`
- Use **kebab-case** for other documentation: `comprehensive-guide.md`, `naming-conventions.md`
- Rationale: Common convention in open-source projects

### Other Files

**Configuration Files**

- Follow the standard naming for each tool: `Cargo.toml`, `rust-toolchain.toml`, `.gitignore`
- These follow their respective ecosystem conventions

**HTML/CSS Files**

- Use **kebab-case** for HTML and CSS files: `index.html`, `styles.css`
- Rationale: Web development convention

## Quick Reference

| File Type               | Convention           | Example                  |
| ----------------------- | -------------------- | ------------------------ |
| Rust source files       | snake_case           | `account_id.rs`          |
| Rust crate names        | kebab-case           | `xrpl-wasm-stdlib`       |
| Rust module directories | snake_case           | `float_tests`            |
| JavaScript files        | camelCase            | `compareGasResults.js`   |
| Shell scripts           | kebab-case           | `benchmark-gas.sh`       |
| Special docs            | SCREAMING_SNAKE_CASE | `README.md`              |
| Other docs              | kebab-case           | `comprehensive-guide.md` |
| HTML/CSS                | kebab-case           | `index.html`             |

## Enforcement

When adding new files to the project:

1. **Check the file type** and refer to this guide for the appropriate naming convention
2. **Be consistent** with existing files of the same type
3. **Update references** if renaming existing files
4. **Test thoroughly** after renaming to ensure all references are updated

## Exceptions

Some files may not follow these conventions due to:

- **Generated files**: Build artifacts and generated code may have their own naming schemes
- **Third-party tools**: Configuration files for external tools follow their conventions
- **Historical reasons**: Some files may retain their original names for backward compatibility

When in doubt, follow the convention that matches the majority of similar files in the codebase.
