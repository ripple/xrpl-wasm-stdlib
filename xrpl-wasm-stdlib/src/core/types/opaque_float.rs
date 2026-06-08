use crate::host::Result;
use crate::host::error_codes::match_result_code_with_expected_bytes;
use crate::host::{
    Error, float_add, float_compare, float_divide, float_multiply, float_pow, float_root,
    float_subtract,
};
use crate::host::{
    FLOAT_ROUNDING_MODES_DOWNWARD, FLOAT_ROUNDING_MODES_TO_NEAREST,
    FLOAT_ROUNDING_MODES_TOWARDS_ZERO, FLOAT_ROUNDING_MODES_UPWARD,
};
use crate::host::{
    float_from_int, float_from_mant_exp, float_from_stamount, float_from_stnumber, float_from_uint,
    float_to_int, float_to_mant_exp,
};
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
/// # use xrpl_wasm_stdlib::core::types::opaque_float::{OpaqueFloat, RoundingMode};
/// let amount = OpaqueFloat::from_int(100, RoundingMode::ToNearest).unwrap();
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct OpaqueFloat(pub [u8; 12]);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoundingMode {
    ToNearest = FLOAT_ROUNDING_MODES_TO_NEAREST as isize,
    ToZero = FLOAT_ROUNDING_MODES_TOWARDS_ZERO as isize,
    Downward = FLOAT_ROUNDING_MODES_DOWNWARD as isize,
    Upward = FLOAT_ROUNDING_MODES_UPWARD as isize,
}

impl OpaqueFloat {
    /// Converts a signed integer to an `OpaqueFloat` using the given rounding mode.
    pub fn from_int(i: i64, mode: RoundingMode) -> Result<OpaqueFloat> {
        let mut float_bytes = [0u8; 12];
        let rescode = unsafe { float_from_int(i, float_bytes.as_mut_ptr(), 12, mode as i32) };
        match_result_code_with_expected_bytes(rescode, 12, || OpaqueFloat(float_bytes))
    }

    /// Converts this `OpaqueFloat` to a signed integer using the given rounding mode.
    pub fn to_int(&self, mode: RoundingMode) -> Result<i64> {
        let mut int_bytes = [0u8; 8];
        let rescode = unsafe {
            float_to_int(
                self.0.as_ptr(),
                core::mem::size_of::<OpaqueFloat>(),
                int_bytes.as_mut_ptr(),
                8,
                mode as i32,
            )
        };
        match_result_code_with_expected_bytes(rescode, 8, || i64::from_le_bytes(int_bytes))
    }

    /// Converts an unsigned integer to an `OpaqueFloat` using the given rounding mode.
    pub fn from_uint(u: u64, mode: RoundingMode) -> Result<OpaqueFloat> {
        let mut float_bytes = [0u8; 12];
        let rescode = unsafe {
            float_from_uint(
                (&u as *const u64).cast::<u8>(),
                core::mem::size_of_val(&u),
                float_bytes.as_mut_ptr(),
                12,
                mode as i32,
            )
        };
        match_result_code_with_expected_bytes(rescode, 12, || OpaqueFloat(float_bytes))
    }

    /// Constructs an `OpaqueFloat` from a mantissa and exponent using the given rounding mode.
    pub fn from_mantissa_exponent(m: i64, e: i32, mode: RoundingMode) -> Result<OpaqueFloat> {
        let mut float_bytes = [0u8; 12];
        let rescode =
            unsafe { float_from_mant_exp(m, e, float_bytes.as_mut_ptr(), 12, mode as i32) };
        match_result_code_with_expected_bytes(rescode, 12, || OpaqueFloat(float_bytes))
    }

    /// Extracts the mantissa and exponent from this `OpaqueFloat`.
    pub fn to_mantissa_exponent(&self) -> Result<(i64, i32)> {
        let mut mant_bytes = [0u8; 8];
        let mut exp_bytes = [0u8; 4];
        let rescode = unsafe {
            float_to_mant_exp(
                self.0.as_ptr(),
                core::mem::size_of::<OpaqueFloat>(),
                mant_bytes.as_mut_ptr(),
                8,
                exp_bytes.as_mut_ptr(),
                4,
            )
        };
        match_result_code_with_expected_bytes(rescode, 8, || {
            (
                i64::from_le_bytes(mant_bytes),
                i32::from_le_bytes(exp_bytes),
            )
        })
    }

    /// Converts a serialized STAmount (8-byte) to an `OpaqueFloat` using the given rounding mode.
    pub fn from_stamount(amount: &[u8], mode: RoundingMode) -> Result<OpaqueFloat> {
        let mut float_bytes = [0u8; 12];
        let rescode = unsafe {
            float_from_stamount(
                amount.as_ptr(),
                amount.len(),
                float_bytes.as_mut_ptr(),
                12,
                mode as i32,
            )
        };
        match_result_code_with_expected_bytes(rescode, 12, || OpaqueFloat(float_bytes))
    }

    /// Converts a serialized STNumber (12-byte) to an `OpaqueFloat` using the given rounding mode.
    pub fn from_stnumber(bytes: &[u8], mode: RoundingMode) -> Result<OpaqueFloat> {
        let mut float_bytes = [0u8; 12];
        let rescode = unsafe {
            float_from_stnumber(
                bytes.as_ptr(),
                bytes.len(),
                float_bytes.as_mut_ptr(),
                12,
                mode as i32,
            )
        };
        match_result_code_with_expected_bytes(rescode, 12, || OpaqueFloat(float_bytes))
    }

    pub fn compare(&self, other: &OpaqueFloat) -> Result<core::cmp::Ordering> {
        let rescode = unsafe {
            float_compare(
                self.0.as_ptr(),
                self.0.len(),
                other.0.as_ptr(),
                other.0.len(),
            )
        };
        match rescode {
            0 => Result::Ok(core::cmp::Ordering::Equal),
            1 => Result::Ok(core::cmp::Ordering::Greater),
            2 => Result::Ok(core::cmp::Ordering::Less),
            _ => Result::Err(Error::from_code(rescode)),
        }
    }

    pub fn add(&self, other: &OpaqueFloat) -> Result<OpaqueFloat> {
        let mut float_bytes = [0u8; 12];
        let rescode = unsafe {
            float_add(
                self.0.as_ptr(),
                self.0.len(),
                other.0.as_ptr(),
                other.0.len(),
                float_bytes.as_mut_ptr(),
                12,
                0,
            )
        };
        match_result_code_with_expected_bytes(rescode, 12, || OpaqueFloat(float_bytes))
    }

    pub fn subtract(&self, other: &OpaqueFloat) -> Result<OpaqueFloat> {
        let mut float_bytes = [0u8; 12];
        let rescode = unsafe {
            float_subtract(
                self.0.as_ptr(),
                self.0.len(),
                other.0.as_ptr(),
                other.0.len(),
                float_bytes.as_mut_ptr(),
                12,
                0,
            )
        };
        match_result_code_with_expected_bytes(rescode, 12, || OpaqueFloat(float_bytes))
    }

    pub fn multiply(&self, other: &OpaqueFloat) -> Result<OpaqueFloat> {
        let mut float_bytes = [0u8; 12];
        let rescode = unsafe {
            float_multiply(
                self.0.as_ptr(),
                self.0.len(),
                other.0.as_ptr(),
                other.0.len(),
                float_bytes.as_mut_ptr(),
                12,
                0,
            )
        };
        match_result_code_with_expected_bytes(rescode, 12, || OpaqueFloat(float_bytes))
    }

    pub fn divide(&self, other: &OpaqueFloat) -> Result<OpaqueFloat> {
        let mut float_bytes = [0u8; 12];
        let rescode = unsafe {
            float_divide(
                self.0.as_ptr(),
                self.0.len(),
                other.0.as_ptr(),
                other.0.len(),
                float_bytes.as_mut_ptr(),
                12,
                0,
            )
        };
        match_result_code_with_expected_bytes(rescode, 12, || OpaqueFloat(float_bytes))
    }

    pub fn pow(&self, n: i32) -> Result<OpaqueFloat> {
        let mut float_bytes = [0u8; 12];
        let rescode = unsafe {
            float_pow(
                self.0.as_ptr(),
                self.0.len(),
                n,
                float_bytes.as_mut_ptr(),
                12,
                0,
            )
        };
        match_result_code_with_expected_bytes(rescode, 12, || OpaqueFloat(float_bytes))
    }

    pub fn root(&self, n: i32) -> Result<OpaqueFloat> {
        let mut float_bytes = [0u8; 12];
        let rescode = unsafe {
            float_root(
                self.0.as_ptr(),
                self.0.len(),
                n,
                float_bytes.as_mut_ptr(),
                12,
                0,
            )
        };
        match_result_code_with_expected_bytes(rescode, 12, || OpaqueFloat(float_bytes))
    }
}

impl From<[u8; 12]> for OpaqueFloat {
    fn from(value: [u8; 12]) -> Self {
        OpaqueFloat(value)
    }
}

/// The number `1` in STNumber format (mantissa = 10^18, exponent = -18).
pub const FLOAT_ONE: [u8; 12] = [
    0x0D, 0xE0, 0xB6, 0xB3, 0xA7, 0x64, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xEE,
];

/// The number `-1` in STNumber format (mantissa = -10^18, exponent = -18).
pub const FLOAT_NEGATIVE_ONE: [u8; 12] = [
    0xF2, 0x1F, 0x49, 0x4C, 0x58, 0x9C, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xEE,
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::host::error_codes::INTERNAL_ERROR;
    use crate::host::host_bindings_trait::MockHostBindings;
    use crate::host::setup_mock;

    use core::cmp::Ordering;

    const EXPECTED_FLOAT: OpaqueFloat = OpaqueFloat([0xCC; 12]);
    const F: OpaqueFloat = OpaqueFloat([0u8; 12]);

    fn write_float(out_buff: *mut u8, out_buff_len: usize) -> i32 {
        assert_eq!(out_buff_len, 12);
        unsafe {
            out_buff.copy_from_nonoverlapping([0xCCu8; 12].as_ptr(), 12);
        }
        12
    }

    // from_int

    #[test]
    fn test_from_int_success() {
        let mut mock = MockHostBindings::new();
        mock.expect_float_from_int()
            .returning(|_, out_buff, out_buff_len, _| write_float(out_buff, out_buff_len));
        let _guard = setup_mock(mock);
        let result = OpaqueFloat::from_int(100, RoundingMode::ToNearest);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), EXPECTED_FLOAT);
    }

    #[test]
    fn test_from_int_error() {
        let mut mock = MockHostBindings::new();
        mock.expect_float_from_int()
            .returning(|_, _, _, _| INTERNAL_ERROR);
        let _guard = setup_mock(mock);
        assert!(OpaqueFloat::from_int(100, RoundingMode::ToNearest).is_err());
    }

    // to_int

    #[test]
    fn test_to_int_success() {
        let mut mock = MockHostBindings::new();
        mock.expect_float_to_int()
            .returning(|_, _, out_buff, out_buff_len, _| {
                assert_eq!(out_buff_len, 8);
                let bytes = 42i64.to_le_bytes();
                unsafe {
                    out_buff.copy_from_nonoverlapping(bytes.as_ptr(), 8);
                }
                8
            });
        let _guard = setup_mock(mock);
        let f = OpaqueFloat([0u8; 12]);
        let result = f.to_int(RoundingMode::ToNearest);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42i64);
    }

    #[test]
    fn test_to_int_error() {
        let mut mock = MockHostBindings::new();
        mock.expect_float_to_int()
            .returning(|_, _, _, _, _| INTERNAL_ERROR);
        let _guard = setup_mock(mock);
        assert!(
            OpaqueFloat([0u8; 12])
                .to_int(RoundingMode::ToNearest)
                .is_err()
        );
    }

    // from_uint

    #[test]
    fn test_from_uint_success() {
        let mut mock = MockHostBindings::new();
        mock.expect_float_from_uint()
            .returning(|_, _, out_buff, out_buff_len, _| write_float(out_buff, out_buff_len));
        let _guard = setup_mock(mock);
        let result = OpaqueFloat::from_uint(100, RoundingMode::ToNearest);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), EXPECTED_FLOAT);
    }

    #[test]
    fn test_from_uint_error() {
        let mut mock = MockHostBindings::new();
        mock.expect_float_from_uint()
            .returning(|_, _, _, _, _| INTERNAL_ERROR);
        let _guard = setup_mock(mock);
        assert!(OpaqueFloat::from_uint(100, RoundingMode::ToNearest).is_err());
    }

    // from_mantissa_exponent

    #[test]
    fn test_from_mantissa_exponent_success() {
        let mut mock = MockHostBindings::new();
        mock.expect_float_from_mant_exp()
            .returning(|_, _, out_buff, out_buff_len, _| write_float(out_buff, out_buff_len));
        let _guard = setup_mock(mock);
        let result = OpaqueFloat::from_mantissa_exponent(1234567890, -5, RoundingMode::ToNearest);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), EXPECTED_FLOAT);
    }

    #[test]
    fn test_from_mantissa_exponent_error() {
        let mut mock = MockHostBindings::new();
        mock.expect_float_from_mant_exp()
            .returning(|_, _, _, _, _| INTERNAL_ERROR);
        let _guard = setup_mock(mock);
        assert!(OpaqueFloat::from_mantissa_exponent(1, 0, RoundingMode::ToNearest).is_err());
    }

    // to_mantissa_exponent

    #[test]
    fn test_to_mantissa_exponent_success() {
        let mut mock = MockHostBindings::new();
        mock.expect_float_to_mant_exp().returning(
            |_, _, mant_buff, mant_buff_len, exp_buff, exp_buff_len| {
                assert_eq!(mant_buff_len, 8);
                assert_eq!(exp_buff_len, 4);
                let mant_bytes = 999i64.to_le_bytes();
                let exp_bytes = (-3i32).to_le_bytes();
                unsafe {
                    mant_buff.copy_from_nonoverlapping(mant_bytes.as_ptr(), 8);
                    exp_buff.copy_from_nonoverlapping(exp_bytes.as_ptr(), 4);
                }
                8
            },
        );
        let _guard = setup_mock(mock);
        let f = OpaqueFloat([0u8; 12]);
        let result = f.to_mantissa_exponent();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), (999i64, -3i32));
    }

    #[test]
    fn test_to_mantissa_exponent_error() {
        let mut mock = MockHostBindings::new();
        mock.expect_float_to_mant_exp()
            .returning(|_, _, _, _, _, _| INTERNAL_ERROR);
        let _guard = setup_mock(mock);
        assert!(OpaqueFloat([0u8; 12]).to_mantissa_exponent().is_err());
    }

    // from_stamount

    #[test]
    fn test_from_stamount_success() {
        let mut mock = MockHostBindings::new();
        mock.expect_float_from_stamount()
            .returning(|_, _, out_buff, out_buff_len, _| write_float(out_buff, out_buff_len));
        let _guard = setup_mock(mock);
        let result = OpaqueFloat::from_stamount(&[0u8; 8], RoundingMode::ToNearest);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), EXPECTED_FLOAT);
    }

    #[test]
    fn test_from_stamount_error() {
        let mut mock = MockHostBindings::new();
        mock.expect_float_from_stamount()
            .returning(|_, _, _, _, _| INTERNAL_ERROR);
        let _guard = setup_mock(mock);
        assert!(OpaqueFloat::from_stamount(&[0u8; 8], RoundingMode::ToNearest).is_err());
    }

    // from_stnumber

    #[test]
    fn test_from_stnumber_success() {
        let mut mock = MockHostBindings::new();
        mock.expect_float_from_stnumber()
            .returning(|_, _, out_buff, out_buff_len, _| write_float(out_buff, out_buff_len));
        let _guard = setup_mock(mock);
        let result = OpaqueFloat::from_stnumber(&[0u8; 8], RoundingMode::ToNearest);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), EXPECTED_FLOAT);
    }

    #[test]
    fn test_from_stnumber_error() {
        let mut mock = MockHostBindings::new();
        mock.expect_float_from_stnumber()
            .returning(|_, _, _, _, _| INTERNAL_ERROR);
        let _guard = setup_mock(mock);
        assert!(OpaqueFloat::from_stnumber(&[0u8; 8], RoundingMode::ToNearest).is_err());
    }
<<<<<<< HEAD
=======

    // compare

    #[test]
    fn test_compare_equal() {
        let mut mock = MockHostBindings::new();
        mock.expect_float_compare().returning(|_, _, _, _| 0);
        let _guard = setup_mock(mock);
        assert_eq!(F.compare(&F).unwrap(), Ordering::Equal);
    }

    #[test]
    fn test_compare_greater() {
        let mut mock = MockHostBindings::new();
        mock.expect_float_compare().returning(|_, _, _, _| 1);
        let _guard = setup_mock(mock);
        assert_eq!(F.compare(&F).unwrap(), Ordering::Greater);
    }

    #[test]
    fn test_compare_less() {
        let mut mock = MockHostBindings::new();
        mock.expect_float_compare().returning(|_, _, _, _| 2);
        let _guard = setup_mock(mock);
        assert_eq!(F.compare(&F).unwrap(), Ordering::Less);
    }

    #[test]
    fn test_compare_error() {
        let mut mock = MockHostBindings::new();
        mock.expect_float_compare()
            .returning(|_, _, _, _| INTERNAL_ERROR);
        let _guard = setup_mock(mock);
        assert!(F.compare(&F).is_err());
    }

    // add

    #[test]
    fn test_add_success() {
        let mut mock = MockHostBindings::new();
        mock.expect_float_add()
            .returning(|_, _, _, _, out_buff, out_buff_len, _| write_float(out_buff, out_buff_len));
        let _guard = setup_mock(mock);
        assert_eq!(F.add(&F).unwrap(), EXPECTED_FLOAT);
    }

    #[test]
    fn test_add_error() {
        let mut mock = MockHostBindings::new();
        mock.expect_float_add()
            .returning(|_, _, _, _, _, _, _| INTERNAL_ERROR);
        let _guard = setup_mock(mock);
        assert!(F.add(&F).is_err());
    }

    // subtract

    #[test]
    fn test_subtract_success() {
        let mut mock = MockHostBindings::new();
        mock.expect_float_subtract()
            .returning(|_, _, _, _, out_buff, out_buff_len, _| write_float(out_buff, out_buff_len));
        let _guard = setup_mock(mock);
        assert_eq!(F.subtract(&F).unwrap(), EXPECTED_FLOAT);
    }

    #[test]
    fn test_subtract_error() {
        let mut mock = MockHostBindings::new();
        mock.expect_float_subtract()
            .returning(|_, _, _, _, _, _, _| INTERNAL_ERROR);
        let _guard = setup_mock(mock);
        assert!(F.subtract(&F).is_err());
    }

    // multiply

    #[test]
    fn test_multiply_success() {
        let mut mock = MockHostBindings::new();
        mock.expect_float_multiply()
            .returning(|_, _, _, _, out_buff, out_buff_len, _| write_float(out_buff, out_buff_len));
        let _guard = setup_mock(mock);
        assert_eq!(F.multiply(&F).unwrap(), EXPECTED_FLOAT);
    }

    #[test]
    fn test_multiply_error() {
        let mut mock = MockHostBindings::new();
        mock.expect_float_multiply()
            .returning(|_, _, _, _, _, _, _| INTERNAL_ERROR);
        let _guard = setup_mock(mock);
        assert!(F.multiply(&F).is_err());
    }

    // divide

    #[test]
    fn test_divide_success() {
        let mut mock = MockHostBindings::new();
        mock.expect_float_divide()
            .returning(|_, _, _, _, out_buff, out_buff_len, _| write_float(out_buff, out_buff_len));
        let _guard = setup_mock(mock);
        assert_eq!(F.divide(&F).unwrap(), EXPECTED_FLOAT);
    }

    #[test]
    fn test_divide_error() {
        let mut mock = MockHostBindings::new();
        mock.expect_float_divide()
            .returning(|_, _, _, _, _, _, _| INTERNAL_ERROR);
        let _guard = setup_mock(mock);
        assert!(F.divide(&F).is_err());
    }

    // pow

    #[test]
    fn test_pow_success() {
        let mut mock = MockHostBindings::new();
        mock.expect_float_pow()
            .returning(|_, _, _, out_buff, out_buff_len, _| write_float(out_buff, out_buff_len));
        let _guard = setup_mock(mock);
        assert_eq!(F.pow(2).unwrap(), EXPECTED_FLOAT);
    }

    #[test]
    fn test_pow_error() {
        let mut mock = MockHostBindings::new();
        mock.expect_float_pow()
            .returning(|_, _, _, _, _, _| INTERNAL_ERROR);
        let _guard = setup_mock(mock);
        assert!(F.pow(2).is_err());
    }

    // root

    #[test]
    fn test_root_success() {
        let mut mock = MockHostBindings::new();
        mock.expect_float_root()
            .returning(|_, _, _, out_buff, out_buff_len, _| write_float(out_buff, out_buff_len));
        let _guard = setup_mock(mock);
        assert_eq!(F.root(2).unwrap(), EXPECTED_FLOAT);
    }

    #[test]
    fn test_root_error() {
        let mut mock = MockHostBindings::new();
        mock.expect_float_root()
            .returning(|_, _, _, _, _, _| INTERNAL_ERROR);
        let _guard = setup_mock(mock);
        assert!(F.root(2).is_err());
    }

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
>>>>>>> 13f8e0c (wrappers + tests)
}
