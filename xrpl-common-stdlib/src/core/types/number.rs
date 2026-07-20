// use crate::core::types::amount::opaque_float::OpaqueFloat;

/// Represents an Number value (mantissa * 10^exponent)
#[derive(Debug, Clone, Copy)]
pub struct Number {
    pub mantissa: i64,
    pub exponent: i32,
}

impl Number {
    /// Create from 12-byte serialized format (BIG-ENDIAN)
    pub fn from(bytes: &[u8]) -> Self {
        assert!(bytes.len() == 12, "Number::from expects a 12-byte slice");
        let mantissa_bytes: [u8; 8] = bytes[0..8].try_into().unwrap();
        let exponent_bytes: [u8; 4] = bytes[8..12].try_into().unwrap();

        Number {
            mantissa: i64::from_be_bytes(mantissa_bytes),
            exponent: i32::from_be_bytes(exponent_bytes),
        }
    }

    /// Convert to 12-byte serialized format (BIG-ENDIAN)
    pub fn as_bytes(&self) -> [u8; 12] {
        let mut bytes = [0u8; 12];
        bytes[0..8].copy_from_slice(&self.mantissa.to_be_bytes());
        bytes[8..12].copy_from_slice(&self.exponent.to_be_bytes());
        bytes
    }
}
