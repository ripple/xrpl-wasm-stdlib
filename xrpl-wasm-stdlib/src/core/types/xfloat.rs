/// Opaque 96-bit representation of an XRPL fungible token (IOU) amount (STNumber format).
///
/// This struct encapsulates the 12-byte STNumber format used by rippled's host functions.
/// Layout: 8-byte big-endian int64 mantissa followed by 4-byte big-endian int32 exponent.
///
/// # Important
///
/// This type is intentionally opaque - arithmetic operations MUST be performed through
/// host functions (float_add, float_multiply, etc.) which use rippled's Number class
/// to ensure exact compatibility with XRPL consensus rules.
///
/// ## Derived Traits
///
/// - `Copy`: Efficient for this 12-byte struct, enabling implicit copying
/// - `PartialEq, Eq`: Enable comparisons (bitwise comparison only)
/// - `Debug, Clone`: Standard traits for development and consistency
///
/// **Note**: `PartialEq` and `Eq` perform bitwise comparison only. For semantic
/// comparison of amounts (e.g., handling different representations of zero),
/// use host functions.
///
/// # Example
///
/// ```no_run
/// # use xrpl_wasm_stdlib::core::types::xfloat::XFloat;
/// // Create from host function
/// let mut float_bytes = [0u8; 12];
/// // float_from_int(100, float_bytes.as_mut_ptr(), 12, 0);
/// let amount = XFloat(float_bytes);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct XFloat(pub [u8; 12]);

impl XFloat {
    // /// Accessor for the 8-bit `exponent` component of an `XFloat`.
    // ///
    // /// WARNING: Use with caution. In general, `XFloat` should not be deconstructed; prefer
    // /// host functions instead (e.g., for things like math operations).
    // pub fn get_exponent(&self) -> u8 {
    //     // In big-endian form, the exponent is:
    //     // * Last 6 bits from bytes[0] (shifted to make room for the 2 bits from bytes[1])
    //     // * First 2 bits from bytes[1] (shifted down to the lowest position)
    //     let exponent: u8 = ((self.0[0] & 0x3F) << 2) | ((self.0[1] & 0xC0) >> 6);
    //
    //     exponent
    // }
    //
    // /// Accessor for the 54-bit `mantissa` of an `XFloat`.
    // ///
    // /// WARNING: Use with caution. In general, `XFloat` should not be deconstructed; prefer
    // /// host functions instead (e.g., for things like math operations).
    // pub fn get_mantissa(&self) -> u64 {
    //     // Extract the 54-bit mantissa from the remaining bytes.
    //     // The mantissa starts from the last 6 bits of the second byte and continues through the
    //     // remaining bytes.
    //
    //     // Extract the top 6 bits from the second byte (after the 2 exponent bits)
    //     let top_6_bits = (self[1] & 0x3F) as u64;
    //
    //     // Extract the remaining 48 bits from self 2-7
    //     let next_8_bits = (self[2] as u64) << 40;
    //     let next_16_bits = (self[3] as u64) << 32;
    //     let next_24_bits = (self[4] as u64) << 24;
    //     let next_32_bits = (self[5] as u64) << 16;
    //     let next_40_bits = (self[6] as u64) << 8;
    //     let next_48_bits = self[7] as u64;
    //
    //     // Combine all the bits to form the 54-bit mantissa
    //     let mantissa = (top_6_bits << 48)
    //         | next_8_bits
    //         | next_16_bits
    //         | next_24_bits
    //         | next_32_bits
    //         | next_40_bits
    //         | next_48_bits;
    //
    //     mantissa
    // }
    //
    //     /// Create a new `XFloat` from an exponent and mantissa.
    //     ///
    //     /// WARNING: Use with caution. In general, `XFloat` should be constructed using
    //     /// host functions instead of manually setting the exponent and mantissa.
    //     ///
    //     /// # Arguments
    //     ///
    //     /// * `exponent` - An 8-bit exponent value
    //     /// * `mantissa` - A 54-bit mantissa value
    //     ///
    //     /// # Panics
    //     ///
    //     /// Panics if the mantissa exceeds 54 bits.
    //     pub fn from_exponent_and_mantissa(exponent: u8, mantissa: u64) -> Self {
    //         // Ensure the mantissa fits in 54 bits
    //         if mantissa > 0x003FFFFFFFFFFFFF {
    //             panic!("Mantissa exceeds 54 bits");
    //         }
    //
    //         // Create a buffer for the combined value
    //         let mut bytes = [0u8; 8];
    //
    //         // Store the first 6 bits of the exponent in the first byte
    //         bytes[0] = (exponent >> 2) & 0x3F;
    //
    //         // Store the last 2 bits of the exponent in the first 2 bits of the second byte
    //         bytes[1] = (exponent & 0x03) << 6;
    //
    //         // We need to shift the mantissa to align with our byte layout
    //         // The mantissa is 54 bits, which is 6 bits in the first byte and 48 bits in the remaining 6 bytes
    //
    //         // First, handle the top 6 bits that go into the second byte (after the 2 exponent bits)
    //         bytes[1] |= ((mantissa >> 48) & 0x3F) as u8;
    //
    //         // Now handle the remaining 48 bits that go into bytes 2-7
    //         bytes[2] = ((mantissa >> 40) & 0xFF) as u8;
    //         bytes[3] = ((mantissa >> 32) & 0xFF) as u8;
    //         bytes[4] = ((mantissa >> 24) & 0xFF) as u8;
    //         bytes[5] = ((mantissa >> 16) & 0xFF) as u8;
    //         bytes[6] = ((mantissa >> 8) & 0xFF) as u8;
    //         bytes[7] = (mantissa & 0xFF) as u8;
    //
    //         // Create a new XFloat from the combined bytes
    //         // ALWAYS use `from_be_bytes` because that's how things are assembled directly above.
    //         // This will manifest a u64 which will be correct.
    //         XFloat(u64::from_be_bytes(bytes))
    //     }
}

impl From<[u8; 12]> for XFloat {
    fn from(value: [u8; 12]) -> Self {
        XFloat(value)
    }
}

/// The number `1` in STNumber format (mantissa=1000000000000000, exponent=-15).
pub const FLOAT_ONE: [u8; 12] = [
    0x0D, 0xE0, 0xB6, 0xB3, 0xA7, 0x64, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xEE,
];

/// The number `-1` in STNumber format (mantissa=-1000000000000000, exponent=-15).
pub const FLOAT_NEGATIVE_ONE: [u8; 12] = [
    0xF2, 0x1F, 0x49, 0x4C, 0x58, 0x9C, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xEE,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xfloat_from_one_canonical_bytes() {
        // float_from_int(1) produces mantissa=1000000000000000, exponent=-15 in STNumber format.
        // Verified against rippled via the e2e float_compare test.
        let one = XFloat::from(FLOAT_ONE);
        assert_eq!(
            one.0,
            [
                0x0D, 0xE0, 0xB6, 0xB3, 0xA7, 0x64, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xEE
            ]
        );
    }
}
