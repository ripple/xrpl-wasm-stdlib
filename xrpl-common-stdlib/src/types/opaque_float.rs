/// Opaque 64-bit representation of an XRPL fungible token (IOU) amount.
///
/// This struct encapsulates the XRPL's custom floating-point format used for fungible tokens.
/// The format is: `[Type:1][Sign:1][Exponent:8][Mantissa:54]` bits.
///
/// # Important
///
/// This type is intentionally opaque - arithmetic operations MUST be performed through
/// host functions (float_add, float_multiply, etc.) which use rippled's Number class
/// to ensure exact compatibility with XRPL consensus rules.
///
/// # Format Details
///
/// - **Type bit** (bit 63): Always 1 for fungible tokens
/// - **Sign bit** (bit 62): 1 = positive, 0 = negative
/// - **Exponent** (bits 61-54): 8 bits, biased by 97 (range -96 to +80)
/// - **Mantissa** (bits 53-0): 54 bits providing ~16 decimal digits precision
///
/// # Special Values
///
/// - Zero: `0x8000000000000000`
/// - Maximum: ~9.999999999999999 × 10^80
/// - Minimum positive: ~1.0 × 10^-81
///
/// ## Derived Traits
///
/// - `Copy`: Efficient for this 8-byte struct, enabling implicit copying
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
/// # use xrpl_common_stdlib::types::opaque_float::OpaqueFloat;
/// // Create from host function
/// let mut float_bytes = [0u8; 8];
/// // float_from_int(100, float_bytes.as_mut_ptr(), 8, 0);
/// let amount = OpaqueFloat(float_bytes);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct OpaqueFloat(pub [u8; 8]);

impl OpaqueFloat {
    // /// Accessor for the 8-bit `exponent` component of an `OpaqueFloat`.
    // ///
    // /// WARNING: Use with caution. In general, `OpaqueFloat` should not be deconstructed; prefer
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
    // /// Accessor for the 54-bit `mantissa` of an `OpaqueFloat`.
    // ///
    // /// WARNING: Use with caution. In general, `OpaqueFloat` should not be deconstructed; prefer
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
    //     /// Create a new `OpaqueFloat` from an exponent and mantissa.
    //     ///
    //     /// WARNING: Use with caution. In general, `OpaqueFloat` should be constructed using
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
    //         // Create a new OpaqueFloat from the combined bytes
    //         // ALWAYS use `from_be_bytes` because that's how things are assembled directly above.
    //         // This will manifest a u64 which will be correct.
    //         OpaqueFloat(u64::from_be_bytes(bytes))
    //     }
}

impl From<[u8; 8]> for OpaqueFloat {
    fn from(value: [u8; 8]) -> Self {
        OpaqueFloat(value)
    }
}

/// The number `1` in XRPL's custom float format.
pub const FLOAT_ONE: [u8; 8] = [0xD4, 0x83, 0x8D, 0x7E, 0xA4, 0xC6, 0x80, 0x00];

/// The number `0` in XRPL's custom float format.
pub const FLOAT_NEGATIVE_ONE: [u8; 8] = [0x94, 0x83, 0x8D, 0x7E, 0xA4, 0xC6, 0x80, 0x00];

#[cfg(test)]
mod tests {
    // use super::*;
    // #[test]
    // fn test_exponent_mantissa_roundtrip() {
    //     // Test with various exponent and mantissa values
    //     let test_cases = [
    //         (0u8, 0u64),
    //         (42u8, 123456789u64),
    //         // (255u8, 0x003FFFFFFFFFFFFu64), // Max exponent, max mantissa
    //     ];
    //
    //     for (exponent, mantissa) in test_cases {
    //         // Create an OpaqueFloat from exponent and mantissa
    //         let float = OpaqueFloat(exponent, mantissa);
    //
    //         // Extract the exponent and mantissa
    //         let extracted_exponent = float.get_exponent();
    //         let extracted_mantissa = float.get_mantissa();
    //
    //         // Verify they match the original values
    //         assert_eq!(
    //             extracted_exponent, exponent,
    //             "Exponent mismatch: expected {}, got {}",
    //             exponent, extracted_exponent
    //         );
    //         assert_eq!(
    //             extracted_mantissa, mantissa,
    //             "Mantissa mismatch: expected {}, got {}",
    //             mantissa, extracted_mantissa
    //         );
    //     }
    // }
    // #[test]
    // #[should_panic(expected = "Mantissa exceeds 54 bits")]
    // fn test_mantissa_too_large() {
    //     // This should panic because the mantissa is too large (55 bits set)
    //     OpaqueFloat::from_exponent_and_mantissa(0, 0x007FFFFFFFFFFFFF);
    // }
    // #[test]
    // fn test_from_bytes() {
    //     let one_as_opaque_float = 18014398509481984u64;
    //     let opaque_float = OpaqueFloat(one_as_opaque_float);
    //     let exponent_be = opaque_float.get_exponent();
    //     assert_eq!(exponent_be, 1);
    //
    //     let exponent_le = opaque_float.get_exponent_le();
    //     assert_eq!(exponent_le, 1);
    //
    //     // Test with zeros
    //     let zero_bytes: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
    //     let zero_float = OpaqueFloat::from(zero_bytes);
    //     assert_eq!(zero_float.0, 0);
    //
    //     // Test with max values
    //     let max_bytes: [u8; 8] = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
    //     let max_float = OpaqueFloat::from(max_bytes);
    //     assert_eq!(max_float.0, u64::MAX);
    // }
    // #[test]
    // fn test_edge_cases() {
    //     // Test with minimum exponent and mantissa
    //     let min_float = OpaqueFloat::from_exponent_and_mantissa(0, 0);
    //     assert_eq!(min_float.get_exponent(), 0);
    //     assert_eq!(min_float.get_mantissa(), 0);
    //
    //     // Test with maximum exponent and mantissa
    //     let max_exponent: u8 = 255;
    //     let max_mantissa: u64 = 0x003FFFFFFFFFFFFF; // 54 bits all set to 1
    //     let max_float = OpaqueFloat::from_exponent_and_mantissa(max_exponent, max_mantissa);
    //     assert_eq!(max_float.get_exponent(), max_exponent);
    //     assert_eq!(max_float.get_mantissa(), max_mantissa);
    //
    //     // Test with maximum exponent and zero mantissa
    //     let max_exp_zero_mantissa = OpaqueFloat::from_exponent_and_mantissa(max_exponent, 0);
    //     assert_eq!(max_exp_zero_mantissa.get_exponent(), max_exponent);
    //     assert_eq!(max_exp_zero_mantissa.get_mantissa(), 0);
    //
    //     // Test with zero exponent and maximum mantissa
    //     let zero_exp_max_mantissa = OpaqueFloat::from_exponent_and_mantissa(0, max_mantissa);
    //     assert_eq!(zero_exp_max_mantissa.get_exponent(), 0);
    //     assert_eq!(zero_exp_max_mantissa.get_mantissa(), max_mantissa);
    // }
    // #[test]
    // fn test_bit_patterns() {
    //     // Test with specific bit patterns to ensure correct extraction
    //     // (use big-endian when constructing bit patterns, for human understanding)
    //
    //     // Set only the first 6 bits of exponent (in first byte)
    //     let exponent_first_part: u8 = 0x3F; // 0b00111111
    //     // Set only the last 2 bits of exponent (in second byte)
    //     let exponent_second_part: u8 = 0x03; // 0b00000011
    //
    //     // We don't need a mantissa for this test
    //     // let mantissa: u64 = 0x0000000000000001; // Just the lowest bit set
    //
    //     // Manually construct the bytes
    //     let mut bytes = [0u8; 8];
    //     bytes[0] = exponent_first_part;
    //     bytes[1] = exponent_second_part << 6; // Shift to the top 2 bits
    //
    //     // Create the OpaqueFloat (use big-endian for human understanding)
    //     let float = OpaqueFloat(u64::from_be_bytes(bytes));
    //
    //     // The expected exponent is 0b11111111 = 255
    //     assert_eq!(float.get_exponent(), 255);
    //     // The expected mantissa should be 0
    //     assert_eq!(float.get_mantissa(), 0);
    //
    //     // Now test with a mantissa
    //     bytes[1] |= 0x3F; // Set the lower 6 bits of the second byte
    //     bytes[7] = 0x01; // Set the lowest bit in the last byte
    //
    //     let float_with_mantissa = OpaqueFloat(u64::from_be_bytes(bytes));
    //
    //     // The exponent should still be 255
    //     assert_eq!(float_with_mantissa.get_exponent(), 255);
    //     // The mantissa should now have bits set
    //     assert_ne!(float_with_mantissa.get_mantissa(), 0);
    // }
}
