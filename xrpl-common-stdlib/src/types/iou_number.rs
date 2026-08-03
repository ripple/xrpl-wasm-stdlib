/// The 8-byte value field of an XRPL fungible token (IOU) amount.
///
/// This is the leading 8 bytes of a serialized IOU `STAmount` (the value, without the trailing
/// currency and issuer). It is **not** an `STAmount`, and it is a different encoding from the
/// 12-byte `STNumber` used for host arithmetic.
///
/// The format is `[Type:1][Sign:1][Exponent:8][Mantissa:54]` bits, big-endian.
///
/// # Important
///
/// This type is a read-only wire representation. Arithmetic MUST be performed through the host's
/// `Number` (STNumber) type, which uses rippled's `Number` class to stay consensus-exact. The
/// accessors below expose the raw encoded fields for inspection only.
///
/// # Format Details
///
/// - **Type bit** (bit 63): Always 1 for fungible tokens
/// - **Sign bit** (bit 62): 1 = positive, 0 = negative
/// - **Exponent** (bits 61-54): 8 bits, biased by 97 (real exponent range -96 to +80)
/// - **Mantissa** (bits 53-0): 54 bits providing ~16 decimal digits precision
///
/// # Special Values
///
/// - Zero: `0x8000000000000000` (mantissa is 0)
/// - Maximum: ~9.999999999999999 × 10^80
/// - Minimum positive: ~1.0 × 10^-81
///
/// ## Derived Traits
///
/// - `Copy`: Efficient for this 8-byte struct, enabling implicit copying
/// - `PartialEq, Eq`: Enable comparisons (bitwise comparison only)
/// - `Debug, Clone`: Standard traits for development and consistency
///
/// **Note**: `PartialEq` and `Eq` perform bitwise comparison only. For semantic comparison of
/// amounts (e.g., handling different representations of zero), convert to `Number` and use the
/// host `compare`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct IOUNumber(pub [u8; 8]);

/// The bias added to the real exponent to produce the 8-bit stored exponent.
const EXPONENT_BIAS: i32 = 97;

impl IOUNumber {
    /// Returns `true` if the sign bit marks this value as positive.
    ///
    /// Note the sign bit is meaningless for zero; prefer [`IOUNumber::is_zero`] to test for zero.
    pub fn is_positive(&self) -> bool {
        // Sign bit is bit 62 -> bit 6 of the first big-endian byte.
        self.0[0] & 0x40 == 0x40
    }

    /// Returns `true` if this value is zero (mantissa is 0).
    pub fn is_zero(&self) -> bool {
        self.mantissa() == 0
    }

    /// Returns the real (unbiased) base-10 exponent.
    ///
    /// WARNING: prefer the host `Number` type for arithmetic; this exposes the raw encoded field.
    pub fn exponent(&self) -> i32 {
        // The 8-bit stored exponent is the low 6 bits of byte 0 followed by the top 2 bits of
        // byte 1. It is biased by `EXPONENT_BIAS`; subtract to recover the real exponent.
        let stored = (((self.0[0] & 0x3F) as i32) << 2) | (((self.0[1] & 0xC0) as i32) >> 6);
        stored - EXPONENT_BIAS
    }

    /// Returns the 54-bit unsigned mantissa.
    ///
    /// WARNING: prefer the host `Number` type for arithmetic; this exposes the raw encoded field.
    pub fn mantissa(&self) -> u64 {
        // The 54-bit mantissa is the low 6 bits of byte 1 followed by bytes 2..=7.
        let top_6 = (self.0[1] & 0x3F) as u64;
        (top_6 << 48)
            | ((self.0[2] as u64) << 40)
            | ((self.0[3] as u64) << 32)
            | ((self.0[4] as u64) << 24)
            | ((self.0[5] as u64) << 16)
            | ((self.0[6] as u64) << 8)
            | (self.0[7] as u64)
    }
}

impl From<[u8; 8]> for IOUNumber {
    fn from(value: [u8; 8]) -> Self {
        IOUNumber(value)
    }
}

/// The number `1` in XRPL's custom IOU float format.
pub const FLOAT_ONE: [u8; 8] = [0xD4, 0x83, 0x8D, 0x7E, 0xA4, 0xC6, 0x80, 0x00];

/// The number `-1` in XRPL's custom IOU float format.
pub const FLOAT_NEGATIVE_ONE: [u8; 8] = [0x94, 0x83, 0x8D, 0x7E, 0xA4, 0xC6, 0x80, 0x00];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_decodes() {
        let one = IOUNumber(FLOAT_ONE);
        assert!(one.is_positive());
        assert!(!one.is_zero());
        // 1 == 1_000_000_000_000_000 × 10^-15
        assert_eq!(one.exponent(), -15);
        assert_eq!(one.mantissa(), 1_000_000_000_000_000);
    }

    #[test]
    fn test_negative_one_decodes() {
        let neg_one = IOUNumber(FLOAT_NEGATIVE_ONE);
        assert!(!neg_one.is_positive());
        assert!(!neg_one.is_zero());
        assert_eq!(neg_one.exponent(), -15);
        assert_eq!(neg_one.mantissa(), 1_000_000_000_000_000);
    }

    #[test]
    fn test_zero_is_zero() {
        // Canonical IOU zero: type bit set, everything else 0.
        let zero = IOUNumber([0x80, 0, 0, 0, 0, 0, 0, 0]);
        assert!(zero.is_zero());
        assert_eq!(zero.mantissa(), 0);
    }

    #[test]
    fn test_exponent_mantissa_extraction() {
        // Construct exponent = 5 (stored 102), mantissa = 12345, positive IOU.
        const EXPONENT: u8 = 5;
        const STORED: u8 = EXPONENT + EXPONENT_BIAS as u8; // 102
        const MANTISSA: u64 = 12345;

        let mut bytes = [0u8; 8];
        // Type bit (0x80) + sign bit (0x40) + high 6 bits of stored exponent.
        bytes[0] = 0xC0 | ((STORED >> 2) & 0x3F);
        // Low 2 bits of stored exponent occupy the top of byte 1.
        bytes[1] = (STORED & 0x03) << 6;

        // The 54-bit mantissa is stored big-endian in byte 1's low 6 bits followed by bytes 2..=7.
        bytes[1] |= ((MANTISSA >> 48) & 0x3F) as u8;
        bytes[2] = ((MANTISSA >> 40) & 0xFF) as u8;
        bytes[3] = ((MANTISSA >> 32) & 0xFF) as u8;
        bytes[4] = ((MANTISSA >> 24) & 0xFF) as u8;
        bytes[5] = ((MANTISSA >> 16) & 0xFF) as u8;
        bytes[6] = ((MANTISSA >> 8) & 0xFF) as u8;
        bytes[7] = (MANTISSA & 0xFF) as u8;

        let num = IOUNumber(bytes);
        assert!(num.is_positive());
        assert_eq!(num.exponent(), EXPONENT as i32);
        assert_eq!(num.mantissa(), MANTISSA);
    }
}
