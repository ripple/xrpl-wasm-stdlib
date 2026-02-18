use crate::host::Result;
use crate::host::error_codes::{
    match_result_code, match_result_code_optional, match_result_code_with_expected_bytes,
    match_result_code_with_expected_bytes_optional,
};

/// Helper function for retrieving fixed-size fields with exact byte validation.
///
/// This function encapsulates the common pattern of:
/// 1. Allocating a buffer of fixed size
/// 2. Calling a host function to retrieve the field
/// 3. Validating that exactly the expected number of bytes were returned
/// 4. Returning the initialized buffer
///
/// # Type Parameters
///
/// * `N` - The size of the buffer (compile-time constant)
/// * `F` - The type of the host function closure
///
/// # Arguments
///
/// * `field_code` - The field code identifying which field to retrieve
/// * `host_fn` - A closure that calls the appropriate host function
///   - Takes: (field_code: i32, buffer_ptr: *mut u8, buffer_size: usize) -> i32
///   - Returns: result code (number of bytes written or error code)
///
/// # Returns
///
/// Returns `Result<[u8; N]>` containing the initialized buffer if successful
///
/// # Example
///
/// ```ignore
/// let buffer = get_fixed_size_field_with_expected_bytes::<20>(
///     field_code,
///     |fc, buf, size| unsafe { get_current_ledger_obj_field(fc, buf, size) },
/// )?;
/// ```
#[inline]
pub fn get_fixed_size_field_with_expected_bytes<const N: usize, F>(
    field_code: i32,
    host_fn: F,
) -> Result<[u8; N]>
where
    F: FnOnce(i32, *mut u8, usize) -> i32,
{
    let mut buffer = core::mem::MaybeUninit::<[u8; N]>::uninit();
    let result_code = host_fn(field_code, buffer.as_mut_ptr().cast(), N);
    match_result_code_with_expected_bytes(result_code, N, || unsafe { buffer.assume_init() })
}

/// Optional variant of `get_fixed_size_field_with_expected_bytes`.
///
/// Returns `None` if the field is not found, otherwise behaves identically to the required variant.
///
/// # Type Parameters
///
/// * `N` - The size of the buffer (compile-time constant)
/// * `F` - The type of the host function closure
///
/// # Arguments
///
/// * `field_code` - The field code identifying which field to retrieve
/// * `host_fn` - A closure that calls the appropriate host function
///
/// # Returns
///
/// Returns `Result<Option<[u8; N]>>` where:
/// * `Ok(Some(buffer))` - If the field is present and has the expected size
/// * `Ok(None)` - If the field is not found
/// * `Err(Error)` - If there's an error retrieving the field
///
/// # Example
///
/// ```ignore
/// let buffer = get_fixed_size_field_with_expected_bytes_optional::<20>(
///     field_code,
///     |fc, buf, size| unsafe { get_current_ledger_obj_field(fc, buf, size) },
/// )?;
/// ```
#[inline]
pub fn get_fixed_size_field_with_expected_bytes_optional<const N: usize, F>(
    field_code: i32,
    host_fn: F,
) -> Result<Option<[u8; N]>>
where
    F: FnOnce(i32, *mut u8, usize) -> i32,
{
    let mut buffer = core::mem::MaybeUninit::<[u8; N]>::uninit();
    let result_code = host_fn(field_code, buffer.as_mut_ptr().cast(), N);
    match_result_code_with_expected_bytes_optional(result_code, N, || {
        Some(unsafe { buffer.assume_init() })
    })
}

/// Helper function for retrieving variable-size fields.
///
/// This function encapsulates the common pattern for variable-size fields where:
/// 1. A buffer of maximum size is allocated
/// 2. A host function is called to retrieve the field
/// 3. The actual number of bytes written is returned (not validated for exact match)
/// 4. Both the buffer and the actual length are returned
///
/// This is used for fields like Amount and Blob where the actual size can vary.
///
/// # Type Parameters
///
/// * `N` - The maximum size of the buffer (compile-time constant)
/// * `F` - The type of the host function closure
///
/// # Arguments
///
/// * `field_code` - The field code identifying which field to retrieve
/// * `host_fn` - A closure that calls the appropriate host function
///   - Takes: (field_code: i32, buffer_ptr: *mut u8, buffer_size: usize) -> i32
///   - Returns: result code (number of bytes written or error code)
///
/// # Returns
///
/// Returns `Result<([u8; N], usize)>` containing the buffer and actual length if successful
///
/// # Example
///
/// ```ignore
/// let (buffer, len) = get_variable_size_field::<48>(
///     field_code,
///     |fc, buf, size| unsafe { get_current_ledger_obj_field(fc, buf, size) },
/// )?;
/// ```
#[inline]
pub fn get_variable_size_field<const N: usize, F>(
    field_code: i32,
    host_fn: F,
) -> Result<([u8; N], usize)>
where
    F: FnOnce(i32, *mut u8, usize) -> i32,
{
    let mut buffer = core::mem::MaybeUninit::<[u8; N]>::uninit();
    let result_code = host_fn(field_code, buffer.as_mut_ptr().cast(), N);
    match_result_code(result_code, || {
        let len = result_code as usize;
        (unsafe { buffer.assume_init() }, len)
    })
}

/// Optional variant of `get_variable_size_field`.
///
/// Returns `None` if the field is not found, otherwise behaves identically to the required variant.
///
/// # Type Parameters
///
/// * `N` - The maximum size of the buffer (compile-time constant)
/// * `F` - The type of the host function closure
///
/// # Arguments
///
/// * `field_code` - The field code identifying which field to retrieve
/// * `host_fn` - A closure that calls the appropriate host function
///
/// # Returns
///
/// Returns `Result<Option<([u8; N], usize)>>` where:
/// * `Ok(Some((buffer, len)))` - If the field is present
/// * `Ok(None)` - If the field is not found
/// * `Err(Error)` - If there's an error retrieving the field
///
/// # Example
///
/// ```ignore
/// let result = get_variable_size_field_optional::<48>(
///     field_code,
///     |fc, buf, size| unsafe { get_current_ledger_obj_field(fc, buf, size) },
/// )?;
/// ```
#[inline]
pub fn get_variable_size_field_optional<const N: usize, F>(
    field_code: i32,
    host_fn: F,
) -> Result<Option<([u8; N], usize)>>
where
    F: FnOnce(i32, *mut u8, usize) -> i32,
{
    let mut buffer = core::mem::MaybeUninit::<[u8; N]>::uninit();
    let result_code = host_fn(field_code, buffer.as_mut_ptr().cast(), N);
    match_result_code_optional(result_code, || {
        let len = result_code as usize;
        Some((unsafe { buffer.assume_init() }, len))
    })
}
