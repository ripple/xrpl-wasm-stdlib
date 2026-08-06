use crate::fields::decoder::{FieldDecoder, FromLedger, decode_exact};
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
    fn decode(buf: Self::Buffer, bytes_written: usize) -> core::result::Result<Self, DecodeError> {
        decode_exact(buf, bytes_written)
    }
}

impl FromLedger for Currency {}

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

    #[test]
    fn test_currency_new_and_from_20_bytes() {
        // Non-standard 20-byte currency code
        let original: [u8; CURRENCY_SIZE] = [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
            0x0F, 0x10, 0x11, 0x12, 0x13, 0x14,
        ];

        // Exercise Currency::new
        let currency_new = Currency::new(original);
        assert_eq!(currency_new.as_bytes(), &original);

        // Exercise From<[u8; 20]> for Currency
        let currency_from = Currency::from(original);
        assert_eq!(currency_from.as_bytes(), &original);

        // Both constructors should produce the same result
        assert_eq!(currency_new, currency_from);
    }
}
