use crate::host;
use crate::host::Result;
use crate::host::RoundingMode;

/// Opaque 96-bit (12-byte) float in rippled's STNumber wire format.
///
/// Layout: 8-byte big-endian signed int64 mantissa, followed by 4-byte big-endian signed int32
/// exponent. The value is `mantissa × 10^exponent`. The host normalizes results to the canonical
/// form (mantissa in the range [10^15, 10^16), or zero for the zero value).
///
/// # Important
///
/// This type is intentionally opaque — arithmetic operations **must** be performed through
/// host functions (`float_add`, `float_multiply`, etc.) which delegate to rippled's `Number`
/// class to ensure exact compatibility with XRPL consensus rules. `PartialEq`/`Eq` perform
/// bitwise comparison only; use `float_compare` for semantic equality (e.g. different
/// representations of zero).
///
/// # Example
///
/// ```no_run
/// # use xrpl_wasm_stdlib::core::types::xfloat::XFloat;
/// # use xrpl_wasm_stdlib::host::RoundingMode;
/// // Create the value 5 (= 5 × 10^0) via the host
/// let five = XFloat::from_mant_exp(5_000_000_000_000_000, -15, RoundingMode::ToNearest);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct XFloat(pub [u8; 12]);

impl XFloat {
    /// Creates an `XFloat` from a mantissa and exponent via the host's `float_from_mant_exp`.
    ///
    /// Creates an `XFloat` from a mantissa and exponent via the host's `float_from_mant_exp`.
    ///
    /// `rounding_mode` controls how the host rounds the result.
    pub fn from_mant_exp(
        mantissa: i64,
        exponent: i32,
        rounding_mode: RoundingMode,
    ) -> Result<Self> {
        let mut float_out = [0u8; 12];
        let rescode = unsafe {
            host::float_from_mant_exp(
                mantissa,
                exponent,
                float_out.as_mut_ptr(),
                12,
                rounding_mode.into(),
            )
        };
        host::error_codes::match_result_code_with_expected_bytes(rescode, 12, || XFloat(float_out))
    }
}

impl From<[u8; 12]> for XFloat {
    fn from(value: [u8; 12]) -> Self {
        XFloat(value)
    }
}

/// The value `0` in STNumber format: all bytes zero.
pub const FLOAT_ZERO: XFloat = XFloat([0u8; 12]);

/// The value `1` in STNumber format (mantissa=1,000,000,000,000,000, exponent=-15).
/// Verified against rippled via the e2e float_compare test.
pub const FLOAT_ONE: [u8; 12] = [
    0x0D, 0xE0, 0xB6, 0xB3, 0xA7, 0x64, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xEE,
];

/// The value `-1` in STNumber format (mantissa=-1,000,000,000,000,000, exponent=-15).
/// Verified against rippled via the e2e float_compare test.
pub const FLOAT_NEGATIVE_ONE: [u8; 12] = [
    0xF2, 0x1F, 0x49, 0x4C, 0x58, 0x9C, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xEE,
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::host::RoundingMode;
    use crate::host::host_bindings_trait::MockHostBindings;
    use crate::host::setup_mock;

    #[test]
    fn test_from_mant_exp_success() {
        const XFLOAT_BYTES: [u8; 12] = [
            0xD4, 0x91, 0xC3, 0x79, 0x37, 0xE0, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        let mut mock = MockHostBindings::new();
        mock.expect_float_from_mant_exp()
            .times(1)
            .returning(|_, _, out_buff, out_buff_len, _| {
                unsafe { out_buff.copy_from_nonoverlapping(XFLOAT_BYTES.as_ptr(), 12) }
                out_buff_len as i32
            });
        let _guard = setup_mock(mock);

        let result =
            XFloat::from_mant_exp(5_000_000_000_000_000, -15, RoundingMode::ToNearest).unwrap();
        assert_eq!(result, XFloat(XFLOAT_BYTES));
    }

    #[test]
    fn test_from_mant_exp_host_error() {
        let mut mock = MockHostBindings::new();
        mock.expect_float_from_mant_exp()
            .times(1)
            .returning(|_, _, _, _, _| -19); // INVALID_FLOAT_INPUT
        let _guard = setup_mock(mock);

        assert!(XFloat::from_mant_exp(0, 0, RoundingMode::ToNearest).is_err());
    }

    #[test]
    fn test_float_zero_is_all_zeros() {
        assert_eq!(FLOAT_ZERO, XFloat([0u8; 12]));
    }
}
