use crate::fields::decoder::{FieldDecoder, FromLedger};
use crate::types::decode_error::DecodeError;

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

/// `FieldDecoder` for XRPL currency codes: decodes a 20-byte buffer into a `Currency`, failing if
/// the host wrote a different number of bytes.
impl FieldDecoder for Currency {
    type Buffer = [u8; CURRENCY_SIZE];

    #[inline]
    fn empty_buffer() -> Self::Buffer {
        [0u8; CURRENCY_SIZE]
    }

    #[inline]
    fn decode(bytes: &[u8]) -> core::result::Result<Self, DecodeError> {
        let array: [u8; CURRENCY_SIZE] = bytes.try_into().map_err(|_| DecodeError)?;
        Ok(array.into())
    }
}

impl FromLedger for Currency {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_currency_creation() {
        // Create a test currency code
        let code_bytes = [1u8; CURRENCY_SIZE];
        let currency = Currency::new(code_bytes);

        // Verify the bytes
        assert_eq!(currency.as_bytes(), &code_bytes);
    }

    #[test]
    fn test_currency_from_bytes() {
        // Create a test byte array
        let bytes = [2u8; CURRENCY_SIZE];

        // Create a Currency from bytes
        let currency = Currency::from(bytes);

        // Verify the bytes
        assert_eq!(currency.as_bytes(), &bytes);
    }

    #[test]
    fn test_currency_equality() {
        // Create two identical currency codes
        let code1 = Currency::new([3u8; CURRENCY_SIZE]);
        let code2 = Currency::new([3u8; CURRENCY_SIZE]);

        // Create a different currency code
        let code3 = Currency::new([4u8; CURRENCY_SIZE]);

        // Test equality
        assert_eq!(code1, code2);
        assert_ne!(code1, code3);
    }

    #[test]
    fn test_currency_from_standard_bytes() {
        // Create a 3-byte array representing "USD"
        let standard_bytes = *b"USD";

        // Convert to Currency
        let currency = Currency::from(standard_bytes);

        // Create the expected 20-byte array (zeros with "USD" at positions 12-14)
        let mut expected = [0u8; CURRENCY_SIZE];
        expected[12..15].copy_from_slice(&standard_bytes);

        // Verify the bytes
        assert_eq!(currency.as_bytes(), &expected);
    }
}
