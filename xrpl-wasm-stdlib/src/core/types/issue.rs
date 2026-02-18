use crate::core::ledger_objects::LedgerObjectFieldGetter;
use crate::core::types::account_id::AccountID;
use crate::core::types::currency::Currency;
use crate::core::types::mpt_id::MptId;
use crate::host::field_helpers::{get_variable_size_field, get_variable_size_field_optional};
use crate::host::{Result, get_current_ledger_obj_field, get_ledger_obj_field, transpose_option};

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

/// Defines an issue for IOUs (40 bytes: 20-byte currency + 20-byte issuer).
///
/// ## Derived Traits
///
/// - `PartialEq, Eq`: Enable comparisons and use in collections
/// - `Debug, Clone`: Standard traits for development and consistency
///
/// Note: `Copy` is intentionally not derived due to the struct's size (40 bytes).
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct IouIssue {
    issuer: AccountID,
    currency: Currency,
    _bytes: [u8; 40],
}

impl IouIssue {
    pub fn new(issuer: AccountID, currency: Currency) -> Self {
        let mut bytes = [0u8; 40];
        bytes[..20].copy_from_slice(currency.as_bytes());
        bytes[20..].copy_from_slice(&issuer.0);
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
/// - `Copy`: Efficient for this 24-byte struct, enabling implicit copying
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
/// Note: `Copy` is intentionally not derived because the `IOU` variant is 40 bytes.
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
                static XRP_BUF: [u8; 20] = [0; 20];
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
    /// * `buffer` - A 40-byte buffer containing the issue data
    /// * `len` - The actual number of bytes written to the buffer
    ///
    /// # Returns
    ///
    /// Returns `Result<Issue>` where:
    /// * `Ok(Issue::XRP(...))` - If len is 20 (XRP issue)
    /// * `Ok(Issue::MPT(...))` - If len is 24 (MPT issue)
    /// * `Ok(Issue::IOU(...))` - If len is 40 (IOU issue)
    /// * `Err(Error)` - If len is not one of the expected values
    #[inline]
    pub fn from_buffer(buffer: [u8; 40], len: usize) -> Result<Self> {
        match len {
            20 => Result::Ok(Issue::XRP(XrpIssue {})),
            24 => {
                let mpt_bytes: [u8; 24] = buffer[..24].try_into().unwrap_or([0u8; 24]);
                let mpt_id = MptId::from(mpt_bytes);
                Result::Ok(Issue::MPT(MptIssue::new(mpt_id)))
            }
            40 => {
                let currency_bytes: [u8; 20] = buffer[..20].try_into().unwrap_or([0u8; 20]);
                let issuer_bytes: [u8; 20] = buffer[20..40].try_into().unwrap_or([0u8; 20]);
                let currency = Currency::from(currency_bytes);
                let issuer = AccountID::from(issuer_bytes);
                Result::Ok(Issue::IOU(IouIssue::new(issuer, currency)))
            }
            _ => Result::Err(crate::host::Error::from_code(len as i32)),
        }
    }
}

/// Implementation of `LedgerObjectFieldGetter` for XRPL issues.
///
/// This implementation handles issue fields in XRPL ledger objects.
/// Supports all three Issue variants: XRP, IOU, and MPT.
///
/// # Buffer Management
///
/// Uses a 40-byte buffer to accommodate all Issue types:
/// - XRP: 20 bytes (all zeros)
/// - IOU: 40 bytes (20 bytes currency + 20 bytes issuer)
/// - MPT: 24 bytes (4 bytes sequence + 20 bytes issuer)
///
/// The implementation detects the Issue type based on the number of bytes returned
/// from the host function.
impl LedgerObjectFieldGetter for Issue {
    #[inline]
    fn get_from_current_ledger_obj(field_code: i32) -> Result<Self> {
        get_variable_size_field::<40, _>(field_code, |fc, buf, size| unsafe {
            get_current_ledger_obj_field(fc, buf, size)
        })
        .and_then(|(buffer, len)| Issue::from_buffer(buffer, len))
    }

    #[inline]
    fn get_from_current_ledger_obj_optional(field_code: i32) -> Result<Option<Self>> {
        get_variable_size_field_optional::<40, _>(field_code, |fc, buf, size| unsafe {
            get_current_ledger_obj_field(fc, buf, size)
        })
        .and_then(|opt| transpose_option(opt.map(|(buffer, len)| Issue::from_buffer(buffer, len))))
    }

    #[inline]
    fn get_from_ledger_obj(register_num: i32, field_code: i32) -> Result<Self> {
        get_variable_size_field::<40, _>(field_code, |fc, buf, size| unsafe {
            get_ledger_obj_field(register_num, fc, buf, size)
        })
        .and_then(|(buffer, len)| Issue::from_buffer(buffer, len))
    }

    #[inline]
    fn get_from_ledger_obj_optional(register_num: i32, field_code: i32) -> Result<Option<Self>> {
        get_variable_size_field_optional::<40, _>(field_code, |fc, buf, size| unsafe {
            get_ledger_obj_field(register_num, fc, buf, size)
        })
        .and_then(|opt| transpose_option(opt.map(|(buffer, len)| Issue::from_buffer(buffer, len))))
    }
}
