use crate::core::ledger_objects::LedgerObjectFieldGetter;
use crate::host::field_helpers::{
    get_fixed_size_field_with_expected_bytes, get_fixed_size_field_with_expected_bytes_optional,
};
use crate::host::{Result, get_current_ledger_obj_field, get_ledger_obj_field};

pub const CURRENCY_SIZE: usize = 20;
pub const STANDARD_CURRENCY_SIZE: usize = 3; // For standard currencies like USD, EUR, etc.

/// Represents a currency code in the XRPL, which is a 20-byte identifier.
///
/// Currency codes in XRPL can be either:
/// - **Standard currencies**: 3-character ASCII codes (e.g., "USD", "EUR") stored in bytes 12-14
/// - **Non-standard currencies**: Full 20-byte hex values for custom tokens
///
/// ## Derived Traits
///
/// - `Copy`: Efficient for this 20-byte struct, enabling implicit copying
/// - `PartialEq, Eq`: Enable comparisons and use in hash-based collections
/// - `Debug, Clone`: Standard traits for development and consistency
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Currency(pub [u8; CURRENCY_SIZE]);

impl Currency {
    /// Creates a new Currency from a 20-byte array.
    pub fn new(code: [u8; CURRENCY_SIZE]) -> Self {
        Currency(code)
    }

    /// Gets the raw bytes of the Currency.
    pub fn as_bytes(&self) -> &[u8; CURRENCY_SIZE] {
        &self.0
    }
}

impl From<[u8; CURRENCY_SIZE]> for Currency {
    fn from(value: [u8; CURRENCY_SIZE]) -> Self {
        Currency(value)
    }
}

// Implement From<[u8; 3]> to create Currency from the standard currency array type
impl From<[u8; STANDARD_CURRENCY_SIZE]> for Currency {
    fn from(bytes: [u8; STANDARD_CURRENCY_SIZE]) -> Self {
        let mut arr = [0u8; CURRENCY_SIZE];
        arr[12..15].copy_from_slice(&bytes);
        Self(arr)
    }
}

/// Implementation of `LedgerObjectFieldGetter` for XRPL currency codes.
///
/// This implementation handles 20-byte currency code fields in XRPL ledger objects.
/// Currency codes uniquely identify different currencies and assets on the XRPL.
///
/// # Buffer Management
///
/// Uses a 20-byte buffer and validates that exactly 20 bytes are returned
/// from the host function to ensure data integrity.
impl LedgerObjectFieldGetter for Currency {
    #[inline]
    fn get_from_current_ledger_obj(field_code: i32) -> Result<Self> {
        get_fixed_size_field_with_expected_bytes::<CURRENCY_SIZE, _>(
            field_code,
            |fc, buf, size| unsafe { get_current_ledger_obj_field(fc, buf, size) },
        )
        .map(|buffer| buffer.into())
    }

    #[inline]
    fn get_from_current_ledger_obj_optional(field_code: i32) -> Result<Option<Self>> {
        get_fixed_size_field_with_expected_bytes_optional::<CURRENCY_SIZE, _>(
            field_code,
            |fc, buf, size| unsafe { get_current_ledger_obj_field(fc, buf, size) },
        )
        .map(|buffer| buffer.map(|b| b.into()))
    }

    #[inline]
    fn get_from_ledger_obj(register_num: i32, field_code: i32) -> Result<Self> {
        get_fixed_size_field_with_expected_bytes::<CURRENCY_SIZE, _>(
            field_code,
            |fc, buf, size| unsafe { get_ledger_obj_field(register_num, fc, buf, size) },
        )
        .map(|buffer| buffer.into())
    }

    #[inline]
    fn get_from_ledger_obj_optional(register_num: i32, field_code: i32) -> Result<Option<Self>> {
        get_fixed_size_field_with_expected_bytes_optional::<CURRENCY_SIZE, _>(
            field_code,
            |fc, buf, size| unsafe { get_ledger_obj_field(register_num, fc, buf, size) },
        )
        .map(|buffer| buffer.map(|b| b.into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_currency_from_standard_bytes() {
        // Test From<[u8; 3]> - places 3-byte code at bytes 12-14
        let standard_bytes = *b"USD";
        let currency = Currency::from(standard_bytes);

        let mut expected = [0u8; CURRENCY_SIZE];
        expected[12..15].copy_from_slice(&standard_bytes);

        assert_eq!(currency.as_bytes(), &expected);
    }

    #[test]
    fn test_standard_currency_byte_layout() {
        // Standard currencies are placed at bytes 12-14 with zeros elsewhere
        let eur = Currency::from(*b"EUR");
        let bytes = eur.as_bytes();

        // Bytes 0-11 should be zero
        assert_eq!(&bytes[0..12], &[0u8; 12]);
        // Bytes 12-14 should be "EUR"
        assert_eq!(&bytes[12..15], b"EUR");
        // Bytes 15-19 should be zero
        assert_eq!(&bytes[15..20], &[0u8; 5]);
    }
}
