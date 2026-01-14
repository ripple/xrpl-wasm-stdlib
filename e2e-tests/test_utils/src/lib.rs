//! Test utilities for XRPL WASM e2e tests.
//!
//! This crate provides testing utilities that are useful for e2e tests,
//! but should not be included in the production xrpl-wasm-stdlib.
//!
//! ## Features
//!
//! - Assertion macros with trace output for debugging in WASM environments
//!
//! ## Usage
//!
//! Add this to your test contract's `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! test_utils = { workspace = true }
//! ```
//!
//! Then use the assertion macros in your test code:
//!
//! ```rust
//! #[cfg(target_arch = "wasm32")]
//! test_utils::assert_eq!(value, expected, "Values should match");
//! ```

#![no_std]

pub mod assert;
