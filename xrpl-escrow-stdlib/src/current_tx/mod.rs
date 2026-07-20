//! ## Example
//!
//! Get sender Account and optional flags:
//!
//! ```no_run
//! use xrpl_escrow_stdlib::current_tx::escrow_finish::EscrowFinish;
//! use xrpl_wasm_stdlib::core::current_tx::traits::TransactionCommonFields;
//! let tx = EscrowFinish;
//! let account = tx.get_account().unwrap_or_panic();
//! let _flags = tx.get_flags().unwrap_or_panic();
//! ```

pub mod escrow_finish;
pub mod traits;
