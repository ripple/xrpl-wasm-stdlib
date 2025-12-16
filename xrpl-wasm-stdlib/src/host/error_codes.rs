use crate::host::Error::{InternalError, PointerOutOfBounds};
use crate::host::trace::trace_num;
use crate::host::{Error, Result, Result::Err, Result::Ok};

/// Reserved for internal invariant trips, generally unrelated to inputs.
pub const INTERNAL_ERROR: i32 = -1;
/// The requested serialized field could not be found in the specified object.
pub const FIELD_NOT_FOUND: i32 = -2;
/// The provided buffer is too small to hold the requested data.
pub const BUFFER_TOO_SMALL: i32 = -3;
/// The API was asked to assume the object under analysis is an STArray but it was not.
pub const NO_ARRAY: i32 = -4;
/// The specified field is not a leaf field and cannot be accessed directly.
pub const NOT_LEAF_FIELD: i32 = -5;
/// The provided locator string is malformed or invalid.
pub const LOCATOR_MALFORMED: i32 = -6;
/// The specified slot number is outside the valid range.
pub const SLOT_OUT_RANGE: i32 = -7;
/// No free slots are available for allocation.
pub const SLOTS_FULL: i32 = -8;
/// The specified slot did not contain any slotted data (i.e., is empty).
pub const EMPTY_SLOT: i32 = -9;
/// The requested ledger object could not be found.
pub const LEDGER_OBJ_NOT_FOUND: i32 = -10;
/// An error occurred while decoding serialized data.
pub const INVALID_DECODING: i32 = -11;
/// The data field is too large to be processed.
pub const DATA_FIELD_TOO_LARGE: i32 = -12;
/// A pointer or buffer length provided as a parameter described memory outside the allowed memory region.
pub const POINTER_OUT_OF_BOUNDS: i32 = -13;
/// No memory has been exported by the WebAssembly module.
pub const NO_MEM_EXPORTED: i32 = -14;
/// One or more of the parameters provided to the API are invalid.
pub const INVALID_PARAMS: i32 = -15;
/// The provided account identifier is invalid.
pub const INVALID_ACCOUNT: i32 = -16;
/// The specified field identifier is invalid or not recognized.
pub const INVALID_FIELD: i32 = -17;
/// The specified index is outside the valid bounds of the array or collection.
pub const INDEX_OUT_OF_BOUNDS: i32 = -18;
/// The input provided for floating-point parsing is malformed.
pub const INVALID_FLOAT_INPUT: i32 = -19;
/// An error occurred during floating-point computation.
pub const INVALID_FLOAT_COMPUTATION: i32 = -20;

/// Evaluates a result code and executes a closure on success (result_code > 0).
///
/// # Arguments
///
/// * `result_code` - An integer representing the operation result code
/// * `on_success` - A closure that will be executed if result_code > 0
///
/// # Type Parameters
///
/// * `F` - The type of the closure
/// * `T` - The return type of the closure
///
/// # Returns
///
/// Returns a `Result<T>` where:
/// * `Ok(T)` - Contains the value returned by the closure if result_code > 0
/// * `Ok(None)` - If result_code == 0 (no data/empty result)
/// * `Err(Error)` - For negative result codes
///
/// # Note
///
/// This function treats 0 as a valid "no data" state and positive values as success.
#[inline(always)]
pub fn match_result_code<F, T>(result_code: i32, on_success: F) -> Result<T>
where
    F: FnOnce() -> T,
{
    match result_code {
        code if code >= 0 => Ok(on_success()),
        code => Err(Error::from_code(code)),
    }
}

/// Evaluates a result code and executes a closure on success, handling optional return values.
///
/// This function is similar to `match_result_code` but is designed to work with closures
/// that return `Option<T>` values, making it suitable for operations that may legitimately
/// return no data even on success.
///
/// # Arguments
///
/// * `result_code` - An integer representing the operation result code
/// * `on_success` - A closure that will be executed if result_code >= 0, returning `Option<T>`
///
/// # Type Parameters
///
/// * `F` - The type of the closure that returns `Option<T>`
/// * `T` - The inner type of the optional value returned by the closure
///
/// # Returns
///
/// Returns a `Result<Option<T>>` where:
/// * `Ok(Some(T))` - Contains the value returned by the closure if result_code >= 0 and closure returns Some
/// * `Ok(None)` - If result_code >= 0 but the closure returns None
/// * `Err(Error)` - For negative result codes
///
/// # Note
///
/// This function treats all non-negative result codes as success, allowing the closure
/// to determine whether data is present through its Option return type.
#[inline(always)]
pub fn match_result_code_optional<F, T>(result_code: i32, on_success: F) -> Result<Option<T>>
where
    F: FnOnce() -> Option<T>,
{
    match result_code {
        code if code >= 0 => Ok(on_success()),
        code => Err(Error::from_code(code)),
    }
}

/// Evaluates a result code against an expected number of bytes and executes a closure on exact match.
///
/// # Arguments
///
/// * `result_code` - An integer representing the operation result code
/// * `expected_num_bytes` - The exact number of bytes expected to have been written
/// * `on_success` - A closure that will be executed if the result code matches expected bytes
///
/// # Type Parameters
///
/// * `F` - The type of the closure
/// * `T` - The return type of the closure
///
/// # Returns
///
/// Returns a `Result<T>` where:
/// * `Ok(T)` - Contains the value returned by the closure if result_code matches expected_num_bytes
/// * `Err(InternalError)` - If result_code is non-negative but doesn't match expected bytes
/// * `Err(Error)` - For negative result codes
///
/// # Note
///
/// This function requires an exact match between the result code and expected byte count,
/// making it suitable for operations where the exact amount of data written is critical.
#[inline]
pub fn match_result_code_with_expected_bytes<F, T>(
    result_code: i32,
    expected_num_bytes: usize,
    on_success: F,
) -> Result<T>
where
    F: FnOnce() -> T,
{
    match result_code {
        code if code as usize == expected_num_bytes => Ok(on_success()),
        code if code >= 0 => Err(InternalError), // If here, this is a bug
        code => Err(Error::from_code(code)),
    }
}

/// Evaluates a result code against expected bytes with optional field handling.
///
/// This function combines exact byte count validation with optional field semantics,
/// making it suitable for operations that may encounter missing fields (which should
/// return `None`) while still validating exact byte counts for present fields.
///
/// # Arguments
///
/// * `result_code` - An integer representing the operation result code (typically bytes written)
/// * `expected_num_bytes` - The exact number of bytes expected for a successful operation
/// * `on_success` - A closure that will be executed on exact byte match, returning `Option<T>`
///
/// # Type Parameters
///
/// * `F` - The type of the closure that returns `Option<T>`
/// * `T` - The inner type of the optional value returned by the closure
///
/// # Returns
///
/// Returns a `Result<Option<T>>` where:
/// * `Ok(Some(T))` - If result_code matches expected_num_bytes and closure returns Some
/// * `Ok(None)` - If result_code matches expected_num_bytes and closure returns None, OR if result_code == FIELD_NOT_FOUND
/// * `Err(PointerOutOfBounds)` - If result_code is non-negative but doesn't match expected bytes (with debug tracing)
/// * `Err(Error)` - For other negative result codes (with debug tracing)
///
/// # Note
///
/// This function provides enhanced error handling with debug tracing for unexpected
/// byte counts and error codes, making it easier to diagnose issues during development.
/// The `FIELD_NOT_FOUND` error code is treated as a valid "no data" case.
#[inline]
pub fn match_result_code_with_expected_bytes_optional<F, T>(
    result_code: i32,
    expected_num_bytes: usize,
    on_success: F,
) -> Result<Option<T>>
where
    F: FnOnce() -> Option<T>,
{
    match result_code {
        code if code as usize == expected_num_bytes => Ok(on_success()),
        code if code == FIELD_NOT_FOUND => Ok(None),
        // Handle all positive, unexpected values as an internal error.
        code if code >= 0 => {
            let _ = trace_num(
                "Byte array was expected to have this many bytes: ",
                expected_num_bytes as i64,
            );
            let _ = trace_num("Byte array had this many bytes: ", code as i64);
            Err(PointerOutOfBounds)
        }
        // Handle all error values overtly.
        code => {
            let _ = trace_num("Encountered error_code:", code as i64);
            Err(Error::from_code(code))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::host::Error;

    // #[test]
    // fn test_match_result_code_with_expected_bytes_optional_byte_mismatch() {
    //     let mut mock = MockHostBindings::new();
    //
    //     // Set up expectations for trace_num calls (2 calls in the error path)
    //     mock.expect_trace_num()
    //       .with(always(), always(), always())
    //       .returning(|_, _, _| 0)
    //       .times(2);
    //
    //     let _guard = setup_mock(mock);
    //
    //     let expected_bytes = 20;
    //     let result = match_result_code_with_expected_bytes_optional(15, expected_bytes, || {
    //         Some("should_not_execute")
    //     });
    //     assert!(result.is_err());
    //     assert_eq!(result.err().unwrap().code(), POINTER_OUT_OF_BOUNDS);
    // }

    #[test]
    fn test_match_result_code_success_positive() {
        let result = match_result_code(5, || "success");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "success");
    }

    #[test]
    fn test_match_result_code_success_zero() {
        let result = match_result_code(0, || "zero_success");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "zero_success");
    }

    #[test]
    fn test_match_result_code_error_negative() {
        let result = match_result_code(INTERNAL_ERROR, || "should_not_execute");
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().code(), INTERNAL_ERROR);
    }

    #[test]
    fn test_match_result_code_error_field_not_found() {
        let result = match_result_code(FIELD_NOT_FOUND, || "should_not_execute");
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().code(), FIELD_NOT_FOUND);
    }

    #[test]
    fn test_match_result_code_closure_not_called_on_error() {
        let mut called = false;
        let _result = match_result_code(BUFFER_TOO_SMALL, || {
            called = true;
            "should_not_execute"
        });
        assert!(!called);
    }

    #[test]
    fn test_match_result_code_optional_success_some() {
        let result = match_result_code_optional(10, || Some("data"));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some("data"));
    }

    #[test]
    fn test_match_result_code_optional_success_none() {
        let result = match_result_code_optional(0, || None::<&str>);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_match_result_code_optional_error() {
        let result = match_result_code_optional(NO_ARRAY, || Some("should_not_execute"));
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().code(), NO_ARRAY);
    }

    #[test]
    fn test_match_result_code_with_expected_bytes_exact_match() {
        let expected_bytes = 32;
        let result = match_result_code_with_expected_bytes(32, expected_bytes, || "exact_match");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "exact_match");
    }

    #[test]
    fn test_match_result_code_with_expected_bytes_mismatch() {
        let expected_bytes = 32;
        let result =
            match_result_code_with_expected_bytes(16, expected_bytes, || "should_not_execute");
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().code(), INTERNAL_ERROR);
    }

    #[test]
    fn test_match_result_code_with_expected_bytes_negative_error() {
        let expected_bytes = 32;
        let result = match_result_code_with_expected_bytes(
            INVALID_PARAMS,
            expected_bytes,
            || "should_not_execute",
        );
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().code(), INVALID_PARAMS);
    }

    #[test]
    fn test_match_result_code_with_expected_bytes_zero_bytes() {
        let expected_bytes = 0;
        let result = match_result_code_with_expected_bytes(0, expected_bytes, || "zero_bytes");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "zero_bytes");
    }

    #[test]
    fn test_match_result_code_with_expected_bytes_optional_exact_match_some() {
        let expected_bytes = 20;
        let result =
            match_result_code_with_expected_bytes_optional(20, expected_bytes, || Some("data"));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some("data"));
    }

    #[test]
    fn test_match_result_code_with_expected_bytes_optional_exact_match_none() {
        let expected_bytes = 20;
        let result =
            match_result_code_with_expected_bytes_optional(20, expected_bytes, || None::<&str>);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_match_result_code_with_expected_bytes_optional_field_not_found() {
        let expected_bytes = 20;
        let result =
            match_result_code_with_expected_bytes_optional(FIELD_NOT_FOUND, expected_bytes, || {
                Some("should_not_execute")
            });
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_match_result_code_with_expected_bytes_optional_byte_mismatch() {
        let expected_bytes = 20;
        let result = match_result_code_with_expected_bytes_optional(15, expected_bytes, || {
            Some("should_not_execute")
        });
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().code(), POINTER_OUT_OF_BOUNDS);
    }

    #[test]
    fn test_match_result_code_with_expected_bytes_optional_other_error() {
        let expected_bytes = 20;
        let result =
            match_result_code_with_expected_bytes_optional(INVALID_ACCOUNT, expected_bytes, || {
                Some("should_not_execute")
            });
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().code(), INVALID_ACCOUNT);
    }

    #[test]
    fn test_match_result_code_with_expected_bytes_optional_zero_bytes() {
        let expected_bytes = 0;
        let result =
            match_result_code_with_expected_bytes_optional(0, expected_bytes, || Some("zero_data"));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some("zero_data"));
    }

    #[test]
    fn test_all_error_constants_are_negative() {
        let error_codes = [
            INTERNAL_ERROR,
            FIELD_NOT_FOUND,
            BUFFER_TOO_SMALL,
            NO_ARRAY,
            NOT_LEAF_FIELD,
            LOCATOR_MALFORMED,
            SLOT_OUT_RANGE,
            SLOTS_FULL,
            EMPTY_SLOT,
            LEDGER_OBJ_NOT_FOUND,
            INVALID_DECODING,
            DATA_FIELD_TOO_LARGE,
            POINTER_OUT_OF_BOUNDS,
            NO_MEM_EXPORTED,
            INVALID_PARAMS,
            INVALID_ACCOUNT,
            INVALID_FIELD,
            INDEX_OUT_OF_BOUNDS,
            INVALID_FLOAT_INPUT,
            INVALID_FLOAT_COMPUTATION,
        ];

        for &code in &error_codes {
            assert!(code < 0, "Error code {} should be negative", code);
        }
    }

    #[test]
    fn test_error_constants_are_unique() {
        let error_codes = [
            INTERNAL_ERROR,
            FIELD_NOT_FOUND,
            BUFFER_TOO_SMALL,
            NO_ARRAY,
            NOT_LEAF_FIELD,
            LOCATOR_MALFORMED,
            SLOT_OUT_RANGE,
            SLOTS_FULL,
            EMPTY_SLOT,
            LEDGER_OBJ_NOT_FOUND,
            INVALID_DECODING,
            DATA_FIELD_TOO_LARGE,
            POINTER_OUT_OF_BOUNDS,
            NO_MEM_EXPORTED,
            INVALID_PARAMS,
            INVALID_ACCOUNT,
            INVALID_FIELD,
            INDEX_OUT_OF_BOUNDS,
            INVALID_FLOAT_INPUT,
            INVALID_FLOAT_COMPUTATION,
        ];

        // Check that all error codes are unique by comparing each pair
        for (i, &code1) in error_codes.iter().enumerate() {
            for (j, &code2) in error_codes.iter().enumerate() {
                if i != j {
                    assert_ne!(
                        code1, code2,
                        "Error codes at indices {} and {} are not unique: {} == {}",
                        i, j, code1, code2
                    );
                }
            }
        }
    }

    #[test]
    fn test_error_from_code_roundtrip() {
        let test_codes = [
            INTERNAL_ERROR,
            FIELD_NOT_FOUND,
            BUFFER_TOO_SMALL,
            NO_ARRAY,
            NOT_LEAF_FIELD,
            LOCATOR_MALFORMED,
            SLOT_OUT_RANGE,
            SLOTS_FULL,
            EMPTY_SLOT,
            LEDGER_OBJ_NOT_FOUND,
            INVALID_DECODING,
            DATA_FIELD_TOO_LARGE,
            POINTER_OUT_OF_BOUNDS,
            NO_MEM_EXPORTED,
            INVALID_PARAMS,
            INVALID_ACCOUNT,
            INVALID_FIELD,
            INDEX_OUT_OF_BOUNDS,
            INVALID_FLOAT_INPUT,
            INVALID_FLOAT_COMPUTATION,
        ];

        for &code in &test_codes {
            let error = Error::from_code(code);
            assert_eq!(
                error.code(),
                code,
                "Error code roundtrip failed for code {}",
                code
            );
        }
    }

    #[test]
    fn test_closure_execution_count() {
        let mut execution_count = 0;
        let closure = || {
            execution_count += 1;
            "executed"
        };

        // Test that closure is executed exactly once on success
        let _result = match_result_code(1, closure);
        assert_eq!(execution_count, 1);

        // Reset counter and test that closure is not executed on error
        execution_count = 0;
        let closure = || {
            execution_count += 1;
            "should_not_execute"
        };
        let _result = match_result_code(INTERNAL_ERROR, closure);
        assert_eq!(execution_count, 0);
    }

    #[test]
    fn test_large_positive_result_codes() {
        // Test with large positive numbers that might be typical byte counts
        let large_positive = 1024;
        let result = match_result_code(large_positive, || "large_success");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "large_success");

        // Test with expected bytes matching
        let result = match_result_code_with_expected_bytes(
            large_positive,
            large_positive as usize,
            || "exact_large",
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "exact_large");
    }

    #[test]
    fn test_edge_case_usize_conversion() {
        // Test edge case where result_code as usize might have conversion issues
        let result_code = 255i32;
        let expected_bytes = 255usize;
        let result =
            match_result_code_with_expected_bytes(result_code, expected_bytes, || "converted");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "converted");
    }
}
