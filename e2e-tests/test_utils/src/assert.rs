//! Assertion macros for WASM environments.
//!
//! This module provides assertion macros that work in WASM environments by using
//! the trace functions to emit readable error messages when assertions fail.
//!
//! **Note**: These assertions are only active on `wasm32` targets. On non-wasm32
//! targets, the expressions are evaluated (preserving side effects) but the
//! assertions themselves are skipped.

use xrpl_wasm_stdlib::core::types::transaction_type::TransactionType;
use xrpl_wasm_stdlib::host::trace::{DataRepr, trace_data, trace_num};

/// Trait for types that can be traced in assertions.
///
/// This trait provides a way to trace values in a type-appropriate manner:
/// - Numeric types use trace_num to display as decimal
/// - Other types use trace_data with hex representation
pub trait TraceValue {
    fn trace_value(msg: &str, value: &Self);
}

// Macro to implement TraceValue for numeric types that safely fit in i64
macro_rules! impl_trace_numeric {
    ($($t:ty),*) => {
        $(
            impl TraceValue for $t {
                #[inline]
                fn trace_value(msg: &str, value: &$t) {
                    let _ = trace_num(msg, *value as i64);
                }
            }
        )*
    }
}

// Implement for types that safely cast to i64
// - All signed types fit (i8, i16, i32, i64, isize)
// - Small unsigned types fit (u8, u16, u32)
impl_trace_numeric!(i8, i16, i32, i64, isize, u8, u16, u32);

// Special handling for u64 and usize - use decimal display via trace_num
// For values that fit in i64, display as decimal; otherwise show as hex
impl TraceValue for u64 {
    #[inline]
    fn trace_value(msg: &str, value: &u64) {
        if *value <= i64::MAX as u64 {
            let _ = trace_num(msg, *value as i64);
        } else {
            // Value too large for i64, use hex representation
            let bytes = value.to_be_bytes();
            let _ = trace_data(msg, &bytes, DataRepr::AsHex);
        }
    }
}

impl TraceValue for usize {
    #[inline]
    fn trace_value(msg: &str, value: &usize) {
        if *value <= i64::MAX as usize {
            let _ = trace_num(msg, *value as i64);
        } else {
            // Value too large for i64, use hex representation
            let bytes = value.to_be_bytes();
            let _ = trace_data(msg, &bytes, DataRepr::AsHex);
        }
    }
}

// Implement for array types (byte arrays, etc.) - uses hex representation
impl<const N: usize> TraceValue for [u8; N] {
    #[inline]
    fn trace_value(msg: &str, value: &[u8; N]) {
        let _ = trace_data(msg, value, DataRepr::AsHex);
    }
}

// Implement for byte slices - uses hex representation
impl TraceValue for &[u8] {
    #[inline]
    fn trace_value(msg: &str, value: &&[u8]) {
        let _ = trace_data(msg, value, DataRepr::AsHex);
    }
}

// Implement for byte array references - uses hex representation
// This handles cases like &[u8; N] which are common with byte string literals
impl<const N: usize> TraceValue for &[u8; N] {
    #[inline]
    fn trace_value(msg: &str, value: &&[u8; N]) {
        let _ = trace_data(msg, *value, DataRepr::AsHex);
    }
}

// Implement for TransactionType enum - display as its i16 value
impl TraceValue for TransactionType {
    #[inline]
    fn trace_value(msg: &str, value: &TransactionType) {
        let _ = trace_num(msg, *value as i64);
    }
}

// Generic fallback implementation for any type - uses hex representation of raw bytes
// This will be used for enums and other types that don't have a specific implementation
pub fn trace_value_generic<T>(msg: &str, value: &T) {
    let data_ptr = value as *const T as *const u8;
    let data_len = core::mem::size_of::<T>();
    let data_slice = unsafe { core::slice::from_raw_parts(data_ptr, data_len) };
    let _ = trace_data(msg, data_slice, DataRepr::AsHex);
}

/// Asserts that two expressions are equal.
///
/// On `wasm32` targets, if the assertion fails, a trace message is emitted with
/// the values of both expressions, and then the program panics.
///
/// On non-wasm32 targets, the expressions are evaluated but the assertion is skipped.
///
/// # Examples
///
/// ```
/// // This will pass
/// assert_eq!(1, 1);
///
/// // This would fail and emit a trace message on wasm32
/// // assert_eq!(1, 2);
/// ```
#[macro_export]
macro_rules! assert_eq {
    ($left:expr, $right:expr) => {
        {
            let left_val = $left;
            let right_val = $right;
            #[cfg(target_arch = "wasm32")]
            {
                if left_val != right_val {
                    let _ = xrpl_wasm_stdlib::host::trace::trace(concat!("Assertion failed: ", stringify!($left), " != ", stringify!($right)));
                    <_ as $crate::assert::TraceValue>::trace_value("  left: ", &left_val);
                    <_ as $crate::assert::TraceValue>::trace_value("  right: ", &right_val);
                    panic!("assertion failed: {} != {}", stringify!($left), stringify!($right));
                }
            }
        }
    };
    ($left:expr, $right:expr, $($arg:tt)+) => {
        {
            let left_val = $left;
            let right_val = $right;
            #[cfg(target_arch = "wasm32")]
            {
                if left_val != right_val {
                    let _ = xrpl_wasm_stdlib::host::trace::trace(concat!("Assertion failed: ", stringify!($left), " != ", stringify!($right)));
                    <_ as $crate::assert::TraceValue>::trace_value("  left: ", &left_val);
                    <_ as $crate::assert::TraceValue>::trace_value("  right: ", &right_val);
                    let _ = xrpl_wasm_stdlib::host::trace::trace("  message: (see panic message for details)");
                    panic!("assertion failed: {} != {}: {}", stringify!($left), stringify!($right), format_args!($($arg)+));
                }
            }
        }
    };
}

/// Asserts that a condition is true.
///
/// On `wasm32` targets, if the assertion fails, a trace message is emitted with
/// the condition, and then the program panics.
///
/// On non-wasm32 targets, the condition is evaluated but the assertion is skipped.
///
/// # Examples
///
/// ```
/// // This will pass
/// assert!(true);
///
/// // This would fail and emit a trace message on wasm32
/// // assert!(false);
/// ```
#[macro_export]
macro_rules! assert {
    ($cond:expr) => {
        {
            let cond_val = $cond;
            #[cfg(target_arch = "wasm32")]
            {
                if !cond_val {
                    let _ = xrpl_wasm_stdlib::host::trace::trace(concat!("Assertion failed: ", stringify!($cond)));
                    panic!("assertion failed: {}", stringify!($cond));
                }
            }
        }
    };
    ($cond:expr, $($arg:tt)+) => {
        {
            let cond_val = $cond;
            #[cfg(target_arch = "wasm32")]
            {
                if !cond_val {
                    let _ = xrpl_wasm_stdlib::host::trace::trace(concat!("Assertion failed: ", stringify!($cond)));
                    let _ = xrpl_wasm_stdlib::host::trace::trace("  message: (see panic message for details)");
                    panic!("assertion failed: {}: {}", stringify!($cond), format_args!($($arg)+));
                }
            }
        }
    };
}

/// Asserts that two expressions are not equal.
///
/// On `wasm32` targets, if the assertion fails, a trace message is emitted with
/// the values of both expressions, and then the program panics.
///
/// On non-wasm32 targets, the expressions are evaluated but the assertion is skipped.
///
/// # Examples
///
/// ```
/// // This will pass
/// assert_ne!(1, 2);
///
/// // This would fail and emit a trace message on wasm32
/// // assert_ne!(1, 1);
/// ```
#[macro_export]
macro_rules! assert_ne {
    ($left:expr, $right:expr) => {
        {
            let left_val = $left;
            let right_val = $right;
            #[cfg(target_arch = "wasm32")]
            {
                if left_val == right_val {
                    let _ = xrpl_wasm_stdlib::host::trace::trace(concat!("Assertion failed: ", stringify!($left), " == ", stringify!($right)));
                    <_ as $crate::assert::TraceValue>::trace_value("  value: ", &left_val);
                    panic!("assertion failed: {} == {}", stringify!($left), stringify!($right));
                }
            }
        }
    };
    ($left:expr, $right:expr, $($arg:tt)+) => {
        {
            let left_val = $left;
            let right_val = $right;
            #[cfg(target_arch = "wasm32")]
            {
                if left_val == right_val {
                    let _ = xrpl_wasm_stdlib::host::trace::trace(concat!("Assertion failed: ", stringify!($left), " == ", stringify!($right)));
                    <_ as $crate::assert::TraceValue>::trace_value("  value: ", &left_val);
                    let _ = xrpl_wasm_stdlib::host::trace::trace("  message: (see panic message for details)");
                    panic!("assertion failed: {} == {}: {}", stringify!($left), stringify!($right), format_args!($($arg)+));
                }
            }
        }
    };

}
