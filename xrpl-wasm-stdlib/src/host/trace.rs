use crate::host::error_codes::match_result_code;

use crate::core::types::account_id::AccountID;
use crate::core::types::amount::Amount;
use crate::host;
use crate::host::Result;

/// Data representation
#[derive(Clone, Copy)]
pub enum DataRepr {
    /// As UTF-8
    AsUTF8 = 0,
    /// As hexadecimal
    AsHex = 1,
}

/// Write the contents of a message to the xrpld trace log.
///
/// # Parameters
/// * `msg`: A str ref pointing to an array of bytes containing UTF-8 characters.
///
/// # Returns
///
/// Returns an integer representing the result of the operation. A value of `0` or higher signifies
/// the number of message bytes that were written to the trace function. Non-zero values indicate
/// an error (e.g., incorrect buffer sizes).
#[inline(always)] // <-- Inline because this function is very small
pub fn trace(msg: &str) -> Result<i32> {
    // Use an empty slice's pointer instead of null to satisfy Rust's safety requirements
    // Even for zero-length slices, `slice::from_raw_parts` requires a non-null, aligned pointer
    let empty_data: &[u8] = &[];

    let result_code = unsafe {
        host::trace(
            msg.as_ptr(),
            msg.len(),
            empty_data.as_ptr(),
            0usize,
            DataRepr::AsUTF8 as _,
        )
    };

    match_result_code(result_code, || result_code)
}

/// Write the contents of a message to the xrpld trace log.
///
/// # Parameters
/// * `msg`: A str ref pointing to an array of bytes containing UTF-8 characters.
///
/// # Returns
///
/// Returns an integer representing the result of the operation. A value of `0` or higher signifies
/// the number of message bytes that were written to the trace function. Non-zero values indicate
/// an error (e.g., incorrect buffer sizes).
#[inline(always)] // <-- Inline because this function is very small
pub fn trace_data(msg: &str, data: &[u8], data_repr: DataRepr) -> Result<i32> {
    let result_code = unsafe {
        let data_ptr = data.as_ptr();
        let data_len = data.len();
        host::trace(msg.as_ptr(), msg.len(), data_ptr, data_len, data_repr as _)
    };

    match_result_code(result_code, || result_code)
}

/// Write the contents of a message, and a number, to the xrpld trace log.
///
/// # Parameters
/// * `msg`: A str ref pointing to an array of bytes containing UTF-8 characters.
/// * `number`: A number to emit into the trace logs.
///
/// # Returns
///
/// Returns an integer representing the result of the operation. A value of `0` or higher signifies
/// the number of message bytes that were written to the trace function. Non-zero values indicate
/// an error (e.g., incorrect buffer sizes).
#[inline(always)]
pub fn trace_num(msg: &str, number: i64) -> Result<i32> {
    let result_code = unsafe { host::trace_num(msg.as_ptr(), msg.len(), number) };
    match_result_code(result_code, || result_code)
}

#[inline(always)]
pub fn trace_account_buf(msg: &str, account_id: &[u8; 20]) -> Result<i32> {
    let result_code = unsafe {
        host::trace_account(
            msg.as_ptr(),
            msg.len(),
            account_id.as_ptr(),
            account_id.len(),
        )
    };
    match_result_code(result_code, || result_code)
}

#[inline(always)]
pub fn trace_account(msg: &str, account_id: &AccountID) -> Result<i32> {
    let result_code = unsafe {
        host::trace_account(
            msg.as_ptr(),
            msg.len(),
            account_id.0.as_ptr(),
            account_id.0.len(),
        )
    };
    match_result_code(result_code, || result_code)
}

#[inline(always)]
pub fn trace_amount(msg: &str, amount: &Amount) -> Result<i32> {
    // Convert Amount to the STAmount format expected by the host trace function
    let (amount_bytes, len) = amount.to_stamount_bytes();

    let result_code =
        unsafe { host::trace_amount(msg.as_ptr(), msg.len(), amount_bytes.as_ptr(), len) };

    match_result_code(result_code, || result_code)
}

/// Write a float to the XRPLD trace log
#[inline(always)]
pub fn trace_float(msg: &str, f: &[u8; 8]) -> Result<i32> {
    let result_code = unsafe { host::trace_opaque_float(msg.as_ptr(), msg.len(), f.as_ptr(), 8) };
    match_result_code(result_code, || result_code)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::amount::Amount;

    #[test]
    fn test_trace_amount_xrp() {
        // Create a test XRP Amount
        let amount = Amount::XRP {
            num_drops: 1_000_000,
        };
        let message = "Test XRP amount";

        // Call trace_amount function
        let result = trace_amount(message, &amount);

        // Should return Ok
        assert!(result.is_ok());
    }

    #[test]
    fn test_trace_amount_mpt() {
        // Create a test MPT Amount
        use crate::core::types::account_id::AccountID;
        use crate::core::types::mpt_id::MptId;

        const VALUE: u64 = 500_000;
        const SEQUENCE_NUM: u32 = 12345;
        const ISSUER_BYTES: [u8; 20] = [1u8; 20];

        let issuer = AccountID::from(ISSUER_BYTES);
        let mpt_id = MptId::new(SEQUENCE_NUM, issuer);
        let amount = Amount::MPT {
            num_units: VALUE,
            is_positive: true,
            mpt_id,
        };

        let message = "Test MPT amount";

        // Call trace_amount function
        let result = trace_amount(message, &amount);

        // Should return Ok
        assert!(result.is_ok());
    }

    #[test]
    fn test_trace_amount_iou() {
        // Create a test IOU Amount
        use crate::core::types::account_id::AccountID;
        use crate::core::types::currency::Currency;
        use crate::core::types::opaque_float::OpaqueFloat;

        let currency_bytes = [2u8; 20];
        let issuer_bytes = [3u8; 20];
        let amount_bytes = [0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x30, 0x39]; // Simple test float

        let currency = Currency::from(currency_bytes);
        let issuer = AccountID::from(issuer_bytes);
        let amount = OpaqueFloat(amount_bytes);

        let amount = Amount::IOU {
            amount,
            issuer,
            currency,
        };

        let message = "Test IOU amount";

        // Call trace_amount function
        let result = trace_amount(message, &amount);

        // Should return Ok
        assert!(result.is_ok());
    }

    #[test]
    fn test_trace_amount_negative_xrp() {
        // Create a test negative XRP Amount
        let amount = Amount::XRP {
            num_drops: -1_000_000,
        };
        let message = "Test negative XRP amount";

        // Call trace_amount function
        let result = trace_amount(message, &amount);

        // Should return Ok
        assert!(result.is_ok());
    }

    #[test]
    fn test_trace_bytes_format() {
        // Test XRP format
        let xrp_amount = Amount::XRP {
            num_drops: 1_000_000,
        };
        let (_bytes, len) = xrp_amount.to_stamount_bytes();
        assert_eq!(len, 48); // All Amount types should return 48 bytes

        // Test specific fee amount (10 drops)
        let fee_amount = Amount::XRP { num_drops: 10 };
        let (bytes, len) = fee_amount.to_stamount_bytes();
        assert_eq!(len, 48); // All Amount types should return 48 bytes

        // Check the actual bytes for 10 drops
        // Expected: just the raw drop amount (10)
        let expected_bytes = [64, 0, 0, 0, 0, 0, 0, 10];
        assert_eq!(&bytes[0..8], &expected_bytes);

        // Test IOU format
        use crate::core::types::account_id::AccountID;
        use crate::core::types::currency::Currency;
        use crate::core::types::opaque_float::OpaqueFloat;

        let currency_bytes = [2u8; 20];
        let issuer_bytes = [3u8; 20];
        let amount_bytes = [0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x30, 0x39];

        let iou_amount = Amount::IOU {
            amount: OpaqueFloat(amount_bytes),
            issuer: AccountID::from(issuer_bytes),
            currency: Currency::from(currency_bytes),
        };
        let (bytes, len) = iou_amount.to_stamount_bytes();
        assert_eq!(len, 48); // All Amount types should return 48 bytes
        assert_eq!(&bytes[0..8], &amount_bytes); // Should match the opaque float bytes

        // Test MPT format
        use crate::core::types::mpt_id::MptId;

        const VALUE: u64 = 500_000;
        const SEQUENCE_NUM: u32 = 12345;
        const ISSUER_BYTES: [u8; 20] = [1u8; 20];

        let issuer = AccountID::from(ISSUER_BYTES);
        let mpt_id = MptId::new(SEQUENCE_NUM, issuer);
        let mpt_amount = Amount::MPT {
            num_units: VALUE,
            is_positive: true,
            mpt_id,
        };
        let (bytes, len) = mpt_amount.to_stamount_bytes();
        assert_eq!(len, 48); // All Amount types should return 48 bytes
        assert_eq!(bytes[0], 0b_0110_0000); // Positive MPT prefix
        assert_eq!(&bytes[1..9], &VALUE.to_be_bytes()); // Amount bytes
    }
}
