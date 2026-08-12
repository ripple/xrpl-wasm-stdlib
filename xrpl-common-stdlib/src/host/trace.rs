use crate::host;
use crate::types::account_id::AccountID;
use crate::types::amount::Amount;

/// How the host should read the data buffer. Mirrors xrpld's `TraceDataType`; the discriminants
/// are wire values, numbered from 1 so a zeroed `data_type` hits the host's invalid branch.
#[derive(Clone, Copy)]
#[repr(i32)]
pub enum TraceDataType {
    /// 8 little-endian bytes, printed as a signed decimal.
    Int64 = 1,
    /// 8 little-endian bytes, printed as an unsigned decimal.
    Uint64 = 2,
    /// 8 bytes holding an opaque float.
    Xfloat = 3,
    /// A 20-byte account ID, printed as base58.
    Account = 4,
    /// A serialized `STAmount`.
    Amount = 5,
    /// Raw bytes, hex-encoded by the host.
    AsHex = 6,
    /// Bytes printed verbatim as text.
    AsText = 7,
}

/// Fire-and-forget: the host checks its log level, swallows every error, and drops the call
/// silently when `msg.len() + data.len()` exceeds 1024 bytes.
#[inline(always)]
fn trace_impl(msg: &str, data_type: TraceDataType, data: &[u8]) {
    unsafe {
        host::trace(
            msg.as_ptr(),
            msg.len(),
            data_type as i32,
            data.as_ptr(),
            data.len(),
        )
    }
}

/// Write the contents of a message to the xrpld trace log.
///
/// # Parameters
/// * `msg`: A str ref pointing to an array of bytes containing UTF-8 characters.
#[inline(always)] // <-- Inline because this function is very small
pub fn trace(msg: &str) {
    trace_impl(msg, TraceDataType::AsText, &[]);
}

/// Write a message and a data buffer to the xrpld trace log, hex-encoded by the host.
///
/// # Parameters
/// * `msg`: A str ref pointing to an array of bytes containing UTF-8 characters.
/// * `data`: The bytes to emit alongside `msg`.
#[inline(always)] // <-- Inline because this function is very small
pub fn trace_hex(msg: &str, data: &[u8]) {
    trace_impl(msg, TraceDataType::AsHex, data);
}

/// Write a message and a data buffer to the xrpld trace log, printed verbatim as text.
///
/// # Parameters
/// * `msg`: A str ref pointing to an array of bytes containing UTF-8 characters.
/// * `data`: The bytes to emit alongside `msg`.
#[inline(always)] // <-- Inline because this function is very small
pub fn trace_text(msg: &str, data: &[u8]) {
    trace_impl(msg, TraceDataType::AsText, data);
}

/// Write the contents of a message, and a number, to the xrpld trace log.
///
/// # Parameters
/// * `msg`: A str ref pointing to an array of bytes containing UTF-8 characters.
/// * `number`: A number to emit into the trace logs.
#[inline(always)]
pub fn trace_num(msg: &str, number: i64) {
    trace_impl(msg, TraceDataType::Int64, &number.to_le_bytes());
}

/// Write the contents of a message, and an unsigned number, to the xrpld trace log. Use this
/// over [`trace_num`] for values above [`i64::MAX`].
///
/// # Parameters
/// * `msg`: A str ref pointing to an array of bytes containing UTF-8 characters.
/// * `number`: A number to emit into the trace logs.
#[inline(always)]
pub fn trace_num_unsigned(msg: &str, number: u64) {
    trace_impl(msg, TraceDataType::Uint64, &number.to_le_bytes());
}

#[inline(always)]
pub fn trace_acct_buf(msg: &str, account_id: &[u8; 20]) {
    trace_impl(msg, TraceDataType::Account, account_id);
}

#[inline(always)]
pub fn trace_acct(msg: &str, account_id: &AccountID) {
    trace_impl(msg, TraceDataType::Account, &account_id.0);
}

#[inline(always)]
pub fn trace_amt(msg: &str, amount: &Amount) {
    let (amount_bytes, len) = amount.to_stamount_bytes();

    trace_impl(msg, TraceDataType::Amount, &amount_bytes[..len]);
}

/// Write a float to the xrpld trace log.
#[inline(always)]
pub fn trace_float(msg: &str, f: &[u8; 8]) {
    trace_impl(msg, TraceDataType::Xfloat, f);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::host::host_bindings_trait::MockHostBindings;
    use crate::host::setup_mock;
    use crate::types::amount::Amount;

    #[test]
    fn test_trace_amt_xrp() {
        let mut mock = MockHostBindings::new();

        let message = "Test XRP amount";

        mock.expect_trace()
            .withf(|_, _, data_type, _, _| *data_type == TraceDataType::Amount as i32)
            .times(1)
            .returning(|_, _, _, _, _| ());

        let _guard = setup_mock(mock);

        // Create a test XRP Amount
        let amount = Amount::XRP {
            num_drops: 1_000_000,
        };

        trace_amt(message, &amount);
    }

    #[test]
    fn test_trace_amt_mpt() {
        let mut mock = MockHostBindings::new();

        let message = "Test MPT amount";

        mock.expect_trace()
            .withf(|_, _, data_type, _, _| *data_type == TraceDataType::Amount as i32)
            .times(1)
            .returning(|_, _, _, _, _| ());

        let _guard = setup_mock(mock);

        // Create a test MPT Amount
        use crate::types::account_id::AccountID;
        use crate::types::mpt_id::MptId;

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

        trace_amt(message, &amount);
    }

    #[test]
    fn test_trace_amt_iou() {
        let mut mock = MockHostBindings::new();

        let message = "Test IOU amount";

        mock.expect_trace()
            .withf(|_, _, data_type, _, _| *data_type == TraceDataType::Amount as i32)
            .times(1)
            .returning(|_, _, _, _, _| ());

        let _guard = setup_mock(mock);

        // Create a test IOU Amount
        use crate::types::account_id::AccountID;
        use crate::types::currency::Currency;
        use crate::types::iou_number::IOUNumber;

        let currency_bytes = [2u8; 20];
        let issuer_bytes = [3u8; 20];
        let amount_bytes = [0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x30, 0x39]; // Simple test float

        let currency = Currency::from(currency_bytes);
        let issuer = AccountID::from(issuer_bytes);
        let amount = IOUNumber(amount_bytes);

        let amount = Amount::IOU {
            amount,
            issuer,
            currency,
        };

        trace_amt(message, &amount);
    }

    #[test]
    fn test_trace_amt_negative_xrp() {
        let mut mock = MockHostBindings::new();

        let message = "Test negative XRP amount";

        mock.expect_trace()
            .withf(|_, _, data_type, _, _| *data_type == TraceDataType::Amount as i32)
            .times(1)
            .returning(|_, _, _, _, _| ());

        let _guard = setup_mock(mock);

        // Create a test negative XRP Amount
        let amount = Amount::XRP {
            num_drops: -1_000_000,
        };

        trace_amt(message, &amount);
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
        use crate::types::account_id::AccountID;
        use crate::types::currency::Currency;
        use crate::types::iou_number::IOUNumber;

        let currency_bytes = [2u8; 20];
        let issuer_bytes = [3u8; 20];
        let amount_bytes = [0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x30, 0x39];

        let iou_amount = Amount::IOU {
            amount: IOUNumber(amount_bytes),
            issuer: AccountID::from(issuer_bytes),
            currency: Currency::from(currency_bytes),
        };
        let (bytes, len) = iou_amount.to_stamount_bytes();
        assert_eq!(len, 48); // All Amount types should return 48 bytes
        assert_eq!(&bytes[0..8], &amount_bytes); // Should match the opaque float bytes

        // Test MPT format
        use crate::types::mpt_id::MptId;

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
