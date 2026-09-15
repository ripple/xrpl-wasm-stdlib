use crate::fields::decoder::{FieldDecoder, FromLedger};
use crate::host::Result;
use crate::types::account_id::{ACCOUNT_ID_SIZE, AccountID};
use crate::types::currency::{CURRENCY_SIZE, Currency};
use crate::types::decode_error::DecodeError;
use crate::types::mpt_id::{MPT_ID_SIZE, MptId};

/// Serialized XRP issue length (20 zero bytes, same width as a currency code).
pub const XRP_ISSUE_SIZE: usize = CURRENCY_SIZE;

/// Serialized MPT issue length (same as [`MPT_ID_SIZE`]: sequence number + issuer).
pub const MPT_ISSUE_SIZE: usize = MPT_ID_SIZE;

/// Serialized IOU issue length: currency followed by issuer.
pub const IOU_ISSUE_SIZE: usize = CURRENCY_SIZE + ACCOUNT_ID_SIZE;

/// Struct to represent an Issue of type XRP. Exists so that other structs can restrict type
/// information to XRP in their declarations (this is not possible with just the `Issue` enum below).
///
/// ## Derived Traits
///
/// - `Copy`: Efficient for this zero-sized type
/// - `PartialEq, Eq`: Enable comparisons
/// - `Debug, Clone`: Standard traits for development and consistency
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct XrpIssue {}

/// Defines an issue for IOUs ([`IOU_ISSUE_SIZE`] bytes: currency then issuer).
///
/// ## Derived Traits
///
/// - `PartialEq, Eq`: Enable comparisons and use in collections
/// - `Debug, Clone`: Standard traits for development and consistency
///
/// Note: `Copy` is intentionally not derived due to the struct's size ([`IOU_ISSUE_SIZE`] bytes).
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct IouIssue {
    issuer: AccountID,
    currency: Currency,
    _bytes: [u8; IOU_ISSUE_SIZE],
}

impl IouIssue {
    pub fn new(issuer: AccountID, currency: Currency) -> Self {
        let mut bytes = [0u8; IOU_ISSUE_SIZE];
        bytes[..CURRENCY_SIZE].copy_from_slice(currency.as_bytes());
        bytes[CURRENCY_SIZE..].copy_from_slice(&issuer.0);
        Self {
            issuer,
            currency,
            _bytes: bytes,
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self._bytes
    }
}

/// Struct to represent an Issue of type MPT. Exists so that other structs can restrict type
/// information to MPT in their declarations (this is not possible with just the `Issue` enum below).
///
/// ## Derived Traits
///
/// - `Copy`: Efficient for this [`MPT_ISSUE_SIZE`]-byte struct, enabling implicit copying
/// - `PartialEq, Eq`: Enable comparisons
/// - `Debug, Clone`: Standard traits for development and consistency
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct MptIssue {
    mpt_id: MptId,
}

impl MptIssue {
    pub fn new(mpt_id: MptId) -> Self {
        Self { mpt_id }
    }

    pub fn mpt_id(&self) -> MptId {
        self.mpt_id
    }
}

/// Represents an issue without a value, such as reading `Asset1` and `Asset2` in AMM ledger
/// objects.
///
/// ## Derived Traits
///
/// - `PartialEq, Eq`: Enable comparisons and use in collections
/// - `Debug, Clone`: Standard traits for development and consistency
///
/// Note: `Copy` is intentionally not derived because the `IOU` variant is [`IOU_ISSUE_SIZE`] bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub enum Issue {
    XRP(XrpIssue),
    IOU(IouIssue),
    MPT(MptIssue),
}

impl Issue {
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Issue::XRP(_) => {
                static XRP_BUF: [u8; XRP_ISSUE_SIZE] = [0; XRP_ISSUE_SIZE];
                &XRP_BUF
            }
            Issue::IOU(iou) => iou.as_bytes(),
            Issue::MPT(mpt) => mpt.mpt_id.as_bytes(),
        }
    }

    /// Creates an Issue from a buffer and length, detecting the type based on the byte count.
    ///
    /// # Arguments
    ///
    /// * `buffer` - An [`IOU_ISSUE_SIZE`]-byte buffer containing the issue data
    /// * `len` - The actual number of bytes written to the buffer
    ///
    /// # Returns
    ///
    /// Returns `Result<Issue>` where:
    /// * `Ok(Issue::XRP(...))` - If len is [`XRP_ISSUE_SIZE`]
    /// * `Ok(Issue::MPT(...))` - If len is [`MPT_ISSUE_SIZE`]
    /// * `Ok(Issue::IOU(...))` - If len is [`IOU_ISSUE_SIZE`]
    /// * `Err(Error)` - If len is not one of the expected values
    #[inline]
    pub fn from_buffer(buffer: [u8; IOU_ISSUE_SIZE], len: usize) -> Result<Self> {
        match len {
            XRP_ISSUE_SIZE => Result::Ok(Issue::XRP(XrpIssue {})),
            MPT_ISSUE_SIZE => {
                let mpt_bytes: [u8; MPT_ISSUE_SIZE] = buffer[..MPT_ISSUE_SIZE]
                    .try_into()
                    .unwrap_or([0u8; MPT_ISSUE_SIZE]);
                let mpt_id = MptId::from(mpt_bytes);
                Result::Ok(Issue::MPT(MptIssue::new(mpt_id)))
            }
            IOU_ISSUE_SIZE => {
                let currency_bytes: [u8; CURRENCY_SIZE] = buffer[..CURRENCY_SIZE]
                    .try_into()
                    .unwrap_or([0u8; CURRENCY_SIZE]);
                let issuer_bytes: [u8; ACCOUNT_ID_SIZE] = buffer[CURRENCY_SIZE..IOU_ISSUE_SIZE]
                    .try_into()
                    .unwrap_or([0u8; ACCOUNT_ID_SIZE]);
                let currency = Currency::from(currency_bytes);
                let issuer = AccountID::from(issuer_bytes);
                Result::Ok(Issue::IOU(IouIssue::new(issuer, currency)))
            }
            _ => Result::Err(crate::host::Error::from_code(len as i32)),
        }
    }
}

/// `FieldDecoder` for XRPL issues. The host writes a variable number of bytes into the fixed
/// [`IOU_ISSUE_SIZE`]-byte buffer — [`XRP_ISSUE_SIZE`] for XRP, [`MPT_ISSUE_SIZE`] for MPT,
/// [`IOU_ISSUE_SIZE`] for IOU — and the variant is detected from `bytes_written` (see
/// [`Issue::from_buffer`]); a count that matches none of those is a decode error.
impl FieldDecoder for Issue {
    type Buffer = [u8; IOU_ISSUE_SIZE];

    #[inline]
    fn empty_buffer() -> Self::Buffer {
        [0u8; IOU_ISSUE_SIZE]
    }

    #[inline]
    fn decode(buf: Self::Buffer, bytes_written: usize) -> core::result::Result<Self, DecodeError> {
        match Issue::from_buffer(buf, bytes_written) {
            Result::Ok(issue) => core::result::Result::Ok(issue),
            Result::Err(_) => core::result::Result::Err(DecodeError),
        }
    }
}

impl FromLedger for Issue {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iou_issue_size_is_currency_plus_issuer() {
        assert_eq!(IOU_ISSUE_SIZE, CURRENCY_SIZE + ACCOUNT_ID_SIZE);
        assert_eq!(IOU_ISSUE_SIZE, 40);
        assert_eq!(XRP_ISSUE_SIZE, CURRENCY_SIZE);
        assert_eq!(XRP_ISSUE_SIZE, 20);
        assert_eq!(MPT_ISSUE_SIZE, MPT_ID_SIZE);
        assert_eq!(MPT_ISSUE_SIZE, 24);
    }

    // Test IouIssue byte layout
    #[test]
    fn test_iou_issue_creation() {
        let issuer = AccountID::from([1u8; ACCOUNT_ID_SIZE]);
        let currency = Currency::from([2u8; CURRENCY_SIZE]);
        let iou = IouIssue::new(issuer, currency);

        // Verify bytes structure (currency first, then issuer)
        let bytes = iou.as_bytes();
        assert_eq!(bytes.len(), IOU_ISSUE_SIZE);
        assert_eq!(&bytes[..CURRENCY_SIZE], currency.as_bytes());
        assert_eq!(&bytes[CURRENCY_SIZE..], &issuer.0);
    }

    #[test]
    fn test_iou_issue_with_standard_currency() {
        let issuer = AccountID::from([0xAB; ACCOUNT_ID_SIZE]);
        let currency = Currency::from(*b"USD");
        let iou = IouIssue::new(issuer, currency);

        let bytes = iou.as_bytes();
        assert_eq!(&bytes[..CURRENCY_SIZE], currency.as_bytes());
        assert_eq!(&bytes[CURRENCY_SIZE..], &issuer.0);
    }

    #[test]
    fn test_iou_issue_different_issuers_not_equal() {
        let issuer1 = AccountID::from([1u8; ACCOUNT_ID_SIZE]);
        let issuer2 = AccountID::from([3u8; ACCOUNT_ID_SIZE]);
        let currency = Currency::from([2u8; CURRENCY_SIZE]);

        let iou1 = IouIssue::new(issuer1, currency);
        let iou2 = IouIssue::new(issuer2, currency);

        assert_ne!(iou1, iou2);
    }

    // Test MptIssue accessor
    #[test]
    fn test_mpt_issue_creation() {
        let issuer = AccountID::from([1u8; ACCOUNT_ID_SIZE]);
        let mpt_id = MptId::new(12345, issuer);
        let mpt = MptIssue::new(mpt_id);

        assert_eq!(mpt.mpt_id(), mpt_id);
    }

    // Test Issue::from_buffer parsing logic
    #[test]
    fn test_issue_from_buffer_xrp() {
        let buffer = [0u8; IOU_ISSUE_SIZE];
        let result = Issue::from_buffer(buffer, XRP_ISSUE_SIZE);
        assert!(matches!(result, Result::Ok(Issue::XRP(_))));
    }

    #[test]
    fn test_issue_from_buffer_mpt() {
        let mut buffer = [0u8; IOU_ISSUE_SIZE];
        buffer[0..4].copy_from_slice(&12345u32.to_be_bytes());
        buffer[4..MPT_ISSUE_SIZE].copy_from_slice(&[0xAB; ACCOUNT_ID_SIZE]);

        let result = Issue::from_buffer(buffer, MPT_ISSUE_SIZE);
        match result {
            Result::Ok(Issue::MPT(mpt)) => {
                assert_eq!(mpt.mpt_id().get_sequence_num(), 12345);
                assert_eq!(
                    mpt.mpt_id().get_issuer(),
                    AccountID::from([0xAB; ACCOUNT_ID_SIZE])
                );
            }
            _ => panic!("Expected MPT issue"),
        }
    }

    #[test]
    fn test_issue_from_buffer_iou() {
        let mut buffer = [0u8; IOU_ISSUE_SIZE];
        buffer[..CURRENCY_SIZE].copy_from_slice(&[0xCC; CURRENCY_SIZE]);
        buffer[CURRENCY_SIZE..IOU_ISSUE_SIZE].copy_from_slice(&[0xDD; ACCOUNT_ID_SIZE]);

        let result = Issue::from_buffer(buffer, IOU_ISSUE_SIZE);
        match result {
            Result::Ok(Issue::IOU(iou)) => {
                let bytes = iou.as_bytes();
                assert_eq!(&bytes[..CURRENCY_SIZE], &[0xCC; CURRENCY_SIZE]);
                assert_eq!(&bytes[CURRENCY_SIZE..], &[0xDD; ACCOUNT_ID_SIZE]);
            }
            _ => panic!("Expected IOU issue"),
        }
    }

    #[test]
    fn test_issue_as_bytes() {
        let xrp = Issue::XRP(XrpIssue {});
        assert_eq!(xrp.as_bytes(), &[0u8; XRP_ISSUE_SIZE]);

        let issuer = AccountID::from([0xAA; ACCOUNT_ID_SIZE]);
        let currency = Currency::from([0xBB; CURRENCY_SIZE]);
        let iou = Issue::IOU(IouIssue::new(issuer, currency));
        assert_eq!(iou.as_bytes().len(), IOU_ISSUE_SIZE);

        let mpt_id = MptId::new(1, AccountID::from([0xCC; ACCOUNT_ID_SIZE]));
        let mpt = Issue::MPT(MptIssue::new(mpt_id));
        assert_eq!(mpt.as_bytes(), mpt_id.as_bytes());
    }

    #[test]
    fn test_issue_from_buffer_invalid_length() {
        let buffer = [0u8; IOU_ISSUE_SIZE];
        let result = Issue::from_buffer(buffer, 10);
        assert!(matches!(result, Result::Err(_)));

        let result = Issue::from_buffer(buffer, 30);
        assert!(matches!(result, Result::Err(_)));

        let result = Issue::from_buffer(buffer, 0);
        assert!(matches!(result, Result::Err(_)));
    }
}
