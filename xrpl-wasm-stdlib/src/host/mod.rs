//! Host bindings and utilities exposed to WASM smart contracts.
//!
//! This module exposes the low-level host ABI plus typed primitives (Result, Error, helpers).
//! Most users should prefer the safe, high-level APIs in [`crate::core`], which wrap these bindings.
//!
//! ## Float Operations for Fungible Tokens (IOUs)
//!
//! The host provides float arithmetic functions for XRPL's fungible token amounts.
//! These operations use rippled's Number class via FFI to ensure exact consensus compatibility:
//!
//! - `float_from_int` / `float_from_uint` - Convert integers to float format
//! - `float_set` - Create float from exponent and mantissa
//! - `float_add` / `float_subtract` / `float_multiply` / `float_divide` - Arithmetic
//! - `float_pow` / `float_root` / `float_log` - Mathematical functions
//! - `float_compare` - Comparison operations
//!
//! All operations support explicit rounding modes (0=ToNearest, 1=TowardsZero, 2=Downward, 3=Upward).
//!
//! See the host_bindings documentation for detailed function signatures.

pub mod error_codes;
pub mod field_helpers;
pub mod trace;

// Float rounding mode constants (same as in host_bindings.rs)
#[allow(unused)]
pub const FLOAT_ROUNDING_MODES_TO_NEAREST: i32 = 0;
#[allow(unused)]
pub const FLOAT_ROUNDING_MODES_TOWARDS_ZERO: i32 = 1;
#[allow(unused)]
pub const FLOAT_ROUNDING_MODES_DOWNWARD: i32 = 2;
#[allow(unused)]
pub const FLOAT_ROUNDING_MODES_UPWARD: i32 = 3;

// This setup allows us to keep all host functions in the `host::` namespace, but vary the implementation based on
// target and build profiles.
// 1) `host_bindings_trait.rs` defines the trait that specifies the host functions available to WASM smart contracts.
// 2a) When `cargo test` is executed, then `host_bindings_test.rs` is included, which provides mock implementations.
// 2b) When `cargo build` is executed, then `host_bindings_empty.rs` is included, which provides a no-op implementation
//     that simply allows the build to pass when the target is not Wasm32.
// 2c) When `cargo build --target wasm32v1-none` (or any Wasm target) is executed, then `host_bindings_wasm.rs` is
//     included, which provides the actual host function implementations.
pub mod host_bindings_trait;

#[cfg(all(not(test), not(target_arch = "wasm32")))] // <-- e.g., `cargo build`
include!("host_bindings_empty.rs");

#[cfg(all(test, not(target_arch = "wasm32")))] // <-- e.g., `cargo test`
include!("host_bindings_test.rs");

// host functions defined by the host.
#[cfg(target_arch = "wasm32")] // <-- e.g., `cargo build --target wasm32v1-none`
include!("host_bindings_wasm.rs");

/// `Result` is a type that represents either a success ([`Ok`]) or failure ([`Err`]) result from the host.
#[must_use]
pub enum Result<T> {
    /// Contains the success value
    Ok(T),
    /// Contains the error value
    Err(Error), // TODO: Test if the WASM size is expanded if we use an enum here instead of i32
}

impl<T> Result<T> {
    /// Returns `true` if the result is [`Ok`].
    #[inline]
    pub fn is_ok(&self) -> bool {
        matches!(*self, Result::Ok(_))
    }

    /// Returns `true` if the result is [`Err`].
    #[inline]
    pub fn is_err(&self) -> bool {
        !self.is_ok()
    }

    /// Converts from `Result<T>` to `Option<T>`.
    ///
    /// Converts `self` into an `Option<T>`, consuming `self`,
    /// and discarding the error, if any.
    #[inline]
    pub fn ok(self) -> Option<T> {
        match self {
            Result::Ok(x) => Some(x),
            Result::Err(_) => None,
        }
    }

    /// Converts from `Result<T>` to `Option<Error>`.
    ///
    /// Converts `self` into an `Option<Error>`, consuming `self`,
    /// and discarding the success value, if any.
    #[inline]
    pub fn err(self) -> Option<Error> {
        match self {
            Result::Ok(_) => None,
            Result::Err(x) => Some(x),
        }
    }

    /// Returns the contained [`Ok`] value, consuming the `self` value.
    ///
    /// # Panics
    ///
    /// Panics if the value is an [`Err`], with a panic message provided by the
    /// [`Err`]'s value.
    #[inline]
    pub fn unwrap(self) -> T {
        match self {
            Result::Ok(t) => t,
            Result::Err(error) => {
                let _ = trace::trace_num("error_code=", error.code() as i64);
                panic!(
                    "called `Result::unwrap()` on an `Err` with code: {}",
                    error.code()
                )
            }
        }
    }

    /// Returns the contained [`Ok`] value or a provided default.
    #[inline]
    pub fn unwrap_or(self, default: T) -> T {
        match self {
            Result::Ok(t) => t,
            Result::Err(_) => default,
        }
    }

    /// Returns the contained [`Ok`] value or computes it from a closure.
    #[inline]
    pub fn unwrap_or_else<F: FnOnce(Error) -> T>(self, op: F) -> T {
        match self {
            Result::Ok(t) => t,
            Result::Err(e) => op(e),
        }
    }

    #[inline]
    pub fn unwrap_or_panic(self) -> T {
        self.unwrap_or_else(|error| {
            let _ = trace::trace_num("error_code=", error.code() as i64);
            core::panic!(
                "Failed in {}: error_code={}",
                core::panic::Location::caller(),
                error.code()
            );
        })
    }
}

impl From<i64> for Result<u64> {
    #[inline(always)] // <-- Inline because this function is very small
    fn from(value: i64) -> Self {
        match value {
            res if res >= 0 => Result::Ok(value as _),
            _ => Result::Err(Error::from_code(value as _)),
        }
    }
}

/// Possible errors returned by XRPL Programmability APIs.
///
/// Errors are global across all Programmability APIs.
#[derive(Clone, Copy, Debug)]
#[repr(i32)]
pub enum Error {
    /// Reserved for internal invariant trips, generally unrelated to inputs.
    /// These should be reported with an issue.
    InternalError = error_codes::INTERNAL_ERROR,

    /// The requested serialized field could not be found in the specified object.
    /// This error is returned when attempting to access a field that doesn't exist
    /// in the current transaction or ledger object.
    FieldNotFound = error_codes::FIELD_NOT_FOUND,

    /// The provided buffer is too small to hold the requested data.
    /// Increase the buffer size and retry the operation.
    BufferTooSmall = error_codes::BUFFER_TOO_SMALL,

    /// The API was asked to assume the object under analysis is an STArray but it was not.
    /// This error occurs when trying to perform array operations on non-array objects.
    NoArray = error_codes::NO_ARRAY,

    /// The specified field is not a leaf field and cannot be accessed directly.
    /// Leaf fields are primitive types that contain actual data values.
    NotLeafField = error_codes::NOT_LEAF_FIELD,

    /// The provided locator string is malformed or invalid.
    /// Locators must follow the proper format for field identification.
    LocatorMalformed = error_codes::LOCATOR_MALFORMED,

    /// The specified slot number is outside the valid range.
    /// Slot numbers must be within the allowed bounds for the current context.
    SlotOutRange = error_codes::SLOT_OUT_RANGE,

    /// No free slots are available for allocation.
    /// All available slots are currently in use. Consider reusing existing slots.
    SlotsFull = error_codes::SLOTS_FULL,

    /// The specified slot did not contain any slotted data (i.e., is empty).
    /// This error occurs when trying to access a slot that hasn't been allocated
    /// or has been freed.
    EmptySlot = error_codes::EMPTY_SLOT,

    /// The requested ledger object could not be found.
    /// This may occur if the object doesn't exist or the keylet is invalid.
    LedgerObjNotFound = error_codes::LEDGER_OBJ_NOT_FOUND,

    /// An error occurred while decoding serialized data.
    /// This typically indicates corrupted or invalidly formatted data.
    InvalidDecoding = error_codes::INVALID_DECODING,

    /// The data field is too large to be processed.
    /// Consider reducing the size of the data or splitting it into smaller chunks.
    DataFieldTooLarge = error_codes::DATA_FIELD_TOO_LARGE,

    /// A pointer or buffer length provided as a parameter described memory outside the allowed memory region.
    /// This error indicates a memory access violation.
    PointerOutOfBounds = error_codes::POINTER_OUT_OF_BOUNDS,

    /// No memory has been exported by the WebAssembly module.
    /// The module must export its memory for host functions to access it.
    NoMemoryExported = error_codes::NO_MEM_EXPORTED,

    /// One or more of the parameters provided to the API are invalid.
    /// Check the API documentation for valid parameter ranges and formats.
    InvalidParams = error_codes::INVALID_PARAMS,

    /// The provided account identifier is invalid.
    /// Account IDs must be valid 20-byte addresses in the proper format.
    InvalidAccount = error_codes::INVALID_ACCOUNT,

    /// The specified field identifier is invalid or not recognized.
    /// Field IDs must correspond to valid XRPL serialization fields.
    InvalidField = error_codes::INVALID_FIELD,

    /// The specified index is outside the valid bounds of the array or collection.
    /// Ensure the index is within the valid range for the target object.
    IndexOutOfBounds = error_codes::INDEX_OUT_OF_BOUNDS,

    /// The input provided for floating-point parsing is malformed.
    /// Floating-point values must be in the correct format for XFL operations.
    InvalidFloatInput = error_codes::INVALID_FLOAT_INPUT,

    /// An error occurred during floating-point computation.
    /// This may indicate overflow, underflow, or other arithmetic errors.
    InvalidFloatComputation = error_codes::INVALID_FLOAT_COMPUTATION,
}

impl Error {
    // TODO: Use Trait instead?
    #[inline(always)] // <-- Inline because this function is very small
    pub fn from_code(code: i32) -> Self {
        unsafe { core::mem::transmute(code) }
    }

    /// Error code
    #[inline(always)] // <-- Inline because this function is very small
    pub fn code(self) -> i32 {
        self as _
    }
}

impl From<Error> for i64 {
    fn from(val: Error) -> Self {
        val as i64
    }
}
