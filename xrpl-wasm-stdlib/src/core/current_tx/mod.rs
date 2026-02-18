//! # Current Transaction Retrieval Module
//!
//! This module provides utilities for retrieving typed fields from the current XRPL transaction
//! within the context of XRPL Programmability. It offers a safe, type-safe
//! interface over the low-level host functions for accessing transaction data, such as from an
//! `EscrowFinish` transaction.
//!
//! ## Overview
//!
//! When processing XRPL transactions in a permissionless programmability environment, you often
//! need to extract specific fields like account IDs, hashes, public keys, and other data. This
//! module provides convenient wrapper functions that handle the low-level buffer management
//! and error handling required to safely retrieve these fields.
//!
//! ## Field Types Supported
//!
//! - **AccountID**: 20-byte account identifiers
//! - **u32**: 32-bit unsigned integers
//! - **Hash256**: 256-bit cryptographic hashes
//! - **PublicKey**: 33-byte public keys
//! - **Blob**: Variable-length binary data
//!
//! ## Optional vs Required Fields
//!
//! The module provides both optional and required variants for field retrieval:
//!
//! - **Required variants** (e.g., `get_u32_field`): Return an error if the field is missing
//! - **Optional variants** (e.g., `get_optional_u32_field`): Return `None` if the field is missing
//!
//! ## Error Handling
//!
//! All functions return `Result<T>` or `Result<Option<T>>` types that encapsulate
//! the custom error handling required for the XRPL Programmability environment.
//!
//! ## Safety Considerations
//!
//! - All functions use fixed-size buffers appropriate for their data types
//! - Buffer sizes are validated against expected field sizes
//! - Unsafe operations are contained within the low-level host function calls
//! - Memory safety is ensured through proper buffer management
//! - Field codes are validated by the underlying host functions
//!
//! ## Performance Notes
//!
//! - All functions are marked `#[inline]` to minimize call overhead
//! - Buffer allocations are stack-based and have minimal cost
//! - Host function calls are the primary performance bottleneck
//!
//! ## Example
//!
//! Get sender Account and optional flags:
//!
//! ```no_run
//! use xrpl_wasm_stdlib::core::current_tx::escrow_finish::EscrowFinish;
//! use xrpl_wasm_stdlib::core::current_tx::traits::TransactionCommonFields;
//! let tx = EscrowFinish;
//! let account = tx.get_account().unwrap_or_panic();
//! let _flags = tx.get_flags().unwrap_or_panic();
//! ```

pub mod contract_call;
pub mod escrow_finish;
pub mod traits;

use crate::host::error_codes::{
    match_result_code_with_expected_bytes, match_result_code_with_expected_bytes_optional,
};
use crate::host::{Result, get_tx_field};

/// Trait for types that can be retrieved from current transaction fields.
///
/// This trait provides a unified interface for retrieving typed data from the current
/// XRPL transaction being processed, replacing the previous collection of type-specific
/// functions with a generic, type-safe approach.
///
/// ## Supported Types
///
/// The following types implement this trait:
/// - `u32` - 32-bit unsigned integers for sequence numbers, flags, timestamps
/// - `AccountID` - 20-byte account identifiers for transaction participants
/// - `Amount` - XRP amounts and token amounts for transaction values
/// - `Hash256` - 256-bit hashes for transaction IDs and references
/// - `PublicKey` - 33-byte compressed public keys for cryptographic operations
/// - `Blob<N>` - Variable-length binary data (generic over buffer size `N`)
///
/// ## Usage Patterns
///
/// ```rust,no_run
/// use xrpl_wasm_stdlib::core::current_tx::{get_field, get_field_optional};
/// use xrpl_wasm_stdlib::core::types::account_id::AccountID;
/// use xrpl_wasm_stdlib::core::types::amount::Amount;
/// use xrpl_wasm_stdlib::core::types::blob::{MemoBlob, MEMO_BLOB_SIZE};
/// use xrpl_wasm_stdlib::sfield;
/// # fn example() {
///   // Get required fields from the current transaction
///   let account: AccountID = get_field(sfield::Account).unwrap();
///   let sequence: u32 = get_field(sfield::Sequence).unwrap();
///   let fee: Amount = get_field(sfield::Fee).unwrap();
///
///   // Get optional fields from the current transaction
///   let flags: Option<u32> = get_field_optional(sfield::Flags).unwrap();
///   let memo: Option<MemoBlob> = get_field_optional(sfield::Memo).unwrap();
/// # }
/// ```
///
/// ## Error Handling
///
/// - Required field methods return `Result<T>` and error if the field is missing
/// - Optional field methods return `Result<Option<T>>` and return `None` if the field is missing
/// - All methods return appropriate errors for buffer size mismatches or other retrieval failures
///
/// ## Transaction Context
///
/// This trait operates on the "current transaction" - the transaction currently being
/// processed in the XRPL Programmability environment. The transaction context is
/// established by the XRPL host environment before calling into WASM code.
///
/// ## Safety Considerations
///
/// - All implementations use appropriately sized buffers for their data types
/// - Buffer sizes are validated against expected field sizes where applicable
/// - Unsafe operations are contained within the host function calls
/// - Transaction field access is validated by the host environment
pub trait CurrentTxFieldGetter: Sized {
    /// Get a required field from the current transaction.
    ///
    /// This method retrieves a field that must be present in the transaction.
    /// If the field is missing, an error is returned.
    ///
    /// # Arguments
    ///
    /// * `field_code` - The field code identifying which field to retrieve
    ///
    /// # Returns
    ///
    /// Returns a `Result<Self>` where:
    /// * `Ok(Self)` - The field value for the specified field
    /// * `Err(Error::FieldNotFound)` - If the field is not present in the transaction
    /// * `Err(Error)` - If the field cannot be retrieved or has unexpected size
    fn get_from_current_tx(field_code: i32) -> Result<Self>;

    /// Get an optional field from the current transaction.
    ///
    /// This method retrieves a field that may or may not be present in the transaction.
    /// If the field is missing, `None` is returned rather than an error.
    ///
    /// # Arguments
    ///
    /// * `field_code` - The field code identifying which field to retrieve
    ///
    /// # Returns
    ///
    /// Returns a `Result<Option<Self>>` where:
    /// * `Ok(Some(Self))` - The field value for the specified field
    /// * `Ok(None)` - If the field is not present in the transaction
    /// * `Err(Error)` - If the field cannot be retrieved or has unexpected size
    fn get_from_current_tx_optional(field_code: i32) -> Result<Option<Self>>;
}

/// Trait for types that can be retrieved as fixed-size fields from transactions.
///
/// This trait enables a generic implementation of `CurrentTxFieldGetter` for all fixed-size
/// unsigned integer types (u8, u16, u32, u64). Types implementing this trait must
/// have a known, constant size in bytes.
///
/// # Implementing Types
///
/// - `u8` - 1 byte
/// - `u16` - 2 bytes
/// - `u32` - 4 bytes
/// - `u64` - 8 bytes
trait FixedSizeFieldType: Sized {
    /// The size of this type in bytes
    const SIZE: usize;
}

impl FixedSizeFieldType for u8 {
    const SIZE: usize = 1;
}

impl FixedSizeFieldType for u16 {
    const SIZE: usize = 2;
}

impl FixedSizeFieldType for u32 {
    const SIZE: usize = 4;
}

impl FixedSizeFieldType for u64 {
    const SIZE: usize = 8;
}

/// Generic implementation of `CurrentTxFieldGetter` for all fixed-size unsigned integer types.
///
/// This single implementation handles u8, u16, u32, and u64 by leveraging the
/// `FixedSizeFieldType` trait. The implementation:
/// - Allocates a buffer of the appropriate size
/// - Calls the host function to retrieve the field
/// - Validates that the returned byte count matches the expected size
/// - Converts the buffer to the target type
///
/// # Buffer Management
///
/// Uses `MaybeUninit` for efficient stack allocation without initialization overhead.
/// The buffer size is determined at compile-time via the `SIZE` constant.
impl<T: FixedSizeFieldType> CurrentTxFieldGetter for T {
    #[inline]
    fn get_from_current_tx(field_code: i32) -> Result<Self> {
        let mut value = core::mem::MaybeUninit::<T>::uninit();
        let result_code = unsafe { get_tx_field(field_code, value.as_mut_ptr().cast(), T::SIZE) };
        match_result_code_with_expected_bytes(result_code, T::SIZE, || unsafe {
            value.assume_init()
        })
    }

    #[inline]
    fn get_from_current_tx_optional(field_code: i32) -> Result<Option<Self>> {
        let mut value = core::mem::MaybeUninit::<T>::uninit();
        let result_code = unsafe { get_tx_field(field_code, value.as_mut_ptr().cast(), T::SIZE) };
        match_result_code_with_expected_bytes_optional(result_code, T::SIZE, || {
            Some(unsafe { value.assume_init() })
        })
    }
}

/// Retrieves a field from the current transaction.
///
/// # Arguments
///
/// * `field_code` - The field code identifying which field to retrieve
///
/// # Returns
///
/// Returns a `Result<T>` where:
/// * `Ok(T)` - The field value for the specified field
/// * `Err(Error)` - If the field cannot be retrieved or has unexpected size
#[inline]
pub fn get_field<T: CurrentTxFieldGetter>(field_code: i32) -> Result<T> {
    T::get_from_current_tx(field_code)
}

/// Retrieves an optionally present field from the current transaction.
///
/// # Arguments
///
/// * `field_code` - The field code identifying which field to retrieve
///
/// # Returns
///
/// Returns a `Result<Option<T>>` where:
/// * `Ok(Some(T))` - The field value for the specified field
/// * `Ok(None)` - If the field is not present
/// * `Err(Error)` - If the field cannot be retrieved or has unexpected size
#[inline]
pub fn get_field_optional<T: CurrentTxFieldGetter>(field_code: i32) -> Result<Option<T>> {
    T::get_from_current_tx_optional(field_code)
}
