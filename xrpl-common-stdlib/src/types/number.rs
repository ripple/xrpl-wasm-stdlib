use crate::host;
use crate::host::error_codes::match_result_code_with_expected_bytes;
use crate::host::{Error, Result, RoundingMode};

/// The number of bytes in the serialized STNumber (float) representation.
const NUMBER_SIZE: usize = 12;

/// An opaque XRPL `STNumber` value: a decimal float represented as `mantissa × 10^exponent`.
///
/// The wire format is 12 bytes — an 8-byte big-endian `i64` mantissa followed by a 4-byte
/// big-endian `i32` exponent. This is the representation every `float_*` host function consumes
/// and produces, and it is distinct from the 8-byte
/// [`IOUNumber`](crate::types::iou_number::IOUNumber) value carried inside an IOU `STAmount`.
///
/// # Important
///
/// This type is intentionally opaque: arithmetic and conversions MUST go through the host, which
/// delegates to rippled's `Number` class so results stay exactly consensus-compatible. The bytes
/// are only ever produced by the host, so callers cannot construct an out-of-range value.
///
/// `PartialEq`/`Eq` compare the raw bytes. The host canonicalizes every value it emits, so bytewise
/// equality matches semantic equality for host-produced values.
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct Number([u8; NUMBER_SIZE]);

impl Number {
    /// The value `0`.
    pub const ZERO: Number = Number([0u8; NUMBER_SIZE]);

    /// The value `1` (mantissa = 1,000,000,000,000,000, exponent = -15).
    pub const ONE: Number = Number([
        0x0D, 0xE0, 0xB6, 0xB3, 0xA7, 0x64, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xEE,
    ]);

    /// The value `-1` (mantissa = -1,000,000,000,000,000, exponent = -15).
    pub const NEGATIVE_ONE: Number = Number([
        0xF2, 0x1F, 0x49, 0x4C, 0x58, 0x9C, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xEE,
    ]);

    /// Converts a signed integer to a `Number`.
    pub fn from_int(value: i64) -> Result<Number> {
        let mut out = [0u8; NUMBER_SIZE];
        let rescode = unsafe {
            host::float_from_int(
                value,
                out.as_mut_ptr(),
                NUMBER_SIZE,
                RoundingMode::ToNearest.into(),
            )
        };
        match_result_code_with_expected_bytes(rescode, NUMBER_SIZE, || Number(out))
    }

    /// Converts an unsigned integer to a `Number`.
    pub fn from_uint(value: u64) -> Result<Number> {
        // The host reads the unsigned value from a little-endian byte buffer (native order on the
        // little-endian WASM target).
        let value_bytes = value.to_le_bytes();
        let mut out = [0u8; NUMBER_SIZE];
        let rescode = unsafe {
            host::float_from_uint(
                value_bytes.as_ptr(),
                value_bytes.len(),
                out.as_mut_ptr(),
                NUMBER_SIZE,
                RoundingMode::ToNearest.into(),
            )
        };
        match_result_code_with_expected_bytes(rescode, NUMBER_SIZE, || Number(out))
    }

    /// Constructs a `Number` from an explicit mantissa and exponent (`mantissa × 10^exponent`).
    pub fn from_mant_exp(mantissa: i64, exponent: i32) -> Result<Number> {
        let mut out = [0u8; NUMBER_SIZE];
        let rescode = unsafe {
            host::float_from_mant_exp(
                mantissa,
                exponent,
                out.as_mut_ptr(),
                NUMBER_SIZE,
                RoundingMode::ToNearest.into(),
            )
        };
        match_result_code_with_expected_bytes(rescode, NUMBER_SIZE, || Number(out))
    }

    /// Converts a serialized `STAmount` (e.g. from an amount field) to a `Number`.
    pub fn from_stamount(bytes: &[u8]) -> Result<Number> {
        let mut out = [0u8; NUMBER_SIZE];
        let rescode = unsafe {
            host::float_from_stamount(
                bytes.as_ptr(),
                bytes.len(),
                out.as_mut_ptr(),
                NUMBER_SIZE,
                RoundingMode::ToNearest.into(),
            )
        };
        match_result_code_with_expected_bytes(rescode, NUMBER_SIZE, || Number(out))
    }

    /// Converts a serialized `STNumber` (12-byte) to a `Number`.
    pub fn from_stnumber(bytes: &[u8]) -> Result<Number> {
        let mut out = [0u8; NUMBER_SIZE];
        let rescode = unsafe {
            host::float_from_stnumber(
                bytes.as_ptr(),
                bytes.len(),
                out.as_mut_ptr(),
                NUMBER_SIZE,
                RoundingMode::ToNearest.into(),
            )
        };
        match_result_code_with_expected_bytes(rescode, NUMBER_SIZE, || Number(out))
    }

    /// Converts this `Number` to a signed integer, rounding to nearest.
    pub fn to_int(&self) -> Result<i64> {
        let mut int_bytes = [0u8; 8];
        let rescode = unsafe {
            host::float_to_int(
                self.0.as_ptr(),
                self.0.len(),
                int_bytes.as_mut_ptr(),
                int_bytes.len(),
                RoundingMode::ToNearest.into(),
            )
        };
        match_result_code_with_expected_bytes(rescode, 8, || i64::from_le_bytes(int_bytes))
    }

    /// Decomposes this `Number` into its `(mantissa, exponent)` components.
    ///
    /// No rounding is applied — the value is already rounded/canonical.
    pub fn to_mant_exp(&self) -> Result<(i64, i32)> {
        let mut mant_bytes = [0u8; 8];
        let mut exp_bytes = [0u8; 4];
        let rescode = unsafe {
            host::float_to_mant_exp(
                self.0.as_ptr(),
                self.0.len(),
                mant_bytes.as_mut_ptr(),
                mant_bytes.len(),
                exp_bytes.as_mut_ptr(),
                exp_bytes.len(),
            )
        };
        match_result_code_with_expected_bytes(rescode, 8, || {
            (
                i64::from_le_bytes(mant_bytes),
                i32::from_le_bytes(exp_bytes),
            )
        })
    }

    /// Compares this `Number` to another via the host, backing the [`Ord`]/[`PartialOrd`] impls.
    fn compare_via_host(&self, other: &Number) -> Result<core::cmp::Ordering> {
        let rescode = unsafe {
            host::float_cmp(
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

    /// Returns `self + other`, rounding per `rounding`.
    pub fn add(&self, other: &Number, rounding: RoundingMode) -> Result<Number> {
        let mut out = [0u8; NUMBER_SIZE];
        let rescode = unsafe {
            host::float_add(
                self.0.as_ptr(),
                self.0.len(),
                other.0.as_ptr(),
                other.0.len(),
                out.as_mut_ptr(),
                NUMBER_SIZE,
                rounding.into(),
            )
        };
        match_result_code_with_expected_bytes(rescode, NUMBER_SIZE, || Number(out))
    }

    /// Returns `self - other`, rounding per `rounding`.
    pub fn subtract(&self, other: &Number, rounding: RoundingMode) -> Result<Number> {
        let mut out = [0u8; NUMBER_SIZE];
        let rescode = unsafe {
            host::float_sub(
                self.0.as_ptr(),
                self.0.len(),
                other.0.as_ptr(),
                other.0.len(),
                out.as_mut_ptr(),
                NUMBER_SIZE,
                rounding.into(),
            )
        };
        match_result_code_with_expected_bytes(rescode, NUMBER_SIZE, || Number(out))
    }

    /// Returns `self * other`, rounding per `rounding`.
    pub fn multiply(&self, other: &Number, rounding: RoundingMode) -> Result<Number> {
        let mut out = [0u8; NUMBER_SIZE];
        let rescode = unsafe {
            host::float_mult(
                self.0.as_ptr(),
                self.0.len(),
                other.0.as_ptr(),
                other.0.len(),
                out.as_mut_ptr(),
                NUMBER_SIZE,
                rounding.into(),
            )
        };
        match_result_code_with_expected_bytes(rescode, NUMBER_SIZE, || Number(out))
    }

    /// Returns `self / other`, rounding per `rounding`.
    pub fn divide(&self, other: &Number, rounding: RoundingMode) -> Result<Number> {
        let mut out = [0u8; NUMBER_SIZE];
        let rescode = unsafe {
            host::float_div(
                self.0.as_ptr(),
                self.0.len(),
                other.0.as_ptr(),
                other.0.len(),
                out.as_mut_ptr(),
                NUMBER_SIZE,
                rounding.into(),
            )
        };
        match_result_code_with_expected_bytes(rescode, NUMBER_SIZE, || Number(out))
    }

    /// Returns `self` raised to the integer power `n`, rounding per `rounding`.
    pub fn pow(&self, n: i32, rounding: RoundingMode) -> Result<Number> {
        let mut out = [0u8; NUMBER_SIZE];
        let rescode = unsafe {
            host::float_pow(
                self.0.as_ptr(),
                self.0.len(),
                n,
                out.as_mut_ptr(),
                NUMBER_SIZE,
                rounding.into(),
            )
        };
        match_result_code_with_expected_bytes(rescode, NUMBER_SIZE, || Number(out))
    }

    /// Returns the `n`th root of `self` (e.g. `n = 2` for the square root), rounding per `rounding`.
    pub fn root(&self, n: i32, rounding: RoundingMode) -> Result<Number> {
        let mut out = [0u8; NUMBER_SIZE];
        let rescode = unsafe {
            host::float_root(
                self.0.as_ptr(),
                self.0.len(),
                n,
                out.as_mut_ptr(),
                NUMBER_SIZE,
                rounding.into(),
            )
        };
        match_result_code_with_expected_bytes(rescode, NUMBER_SIZE, || Number(out))
    }
}

impl From<[u8; NUMBER_SIZE]> for Number {
    fn from(value: [u8; NUMBER_SIZE]) -> Self {
        Number(value)
    }
}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Number {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        // Comparison delegates to rippled's `Number` via the host. A well-formed `Number` (the only
        // kind the host produces) always compares cleanly, so this cannot fail in practice.
        self.compare_via_host(other).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::host::host_bindings_trait::MockHostBindings;
    use crate::host::setup_mock;

    // An arbitrary well-formed 12-byte value used to stand in for a host-produced float.
    const SAMPLE: [u8; NUMBER_SIZE] = [
        0xD4, 0x91, 0xC3, 0x79, 0x37, 0xE0, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];

    #[test]
    fn test_from_int_success() {
        let mut mock = MockHostBindings::new();
        mock.expect_float_from_int()
            .times(1)
            .returning(|_, out, out_len, _| {
                unsafe { out.copy_from_nonoverlapping(SAMPLE.as_ptr(), NUMBER_SIZE) }
                out_len as i32
            });
        let _guard = setup_mock(mock);

        assert_eq!(Number::from_int(42).unwrap(), Number(SAMPLE));
    }

    #[test]
    fn test_from_int_host_error() {
        let mut mock = MockHostBindings::new();
        mock.expect_float_from_int()
            .times(1)
            .returning(|_, _, _, _| -19); // INVALID_FLOAT_INPUT
        let _guard = setup_mock(mock);

        assert!(Number::from_int(0).is_err());
    }

    #[test]
    fn test_from_uint_success() {
        let mut mock = MockHostBindings::new();
        mock.expect_float_from_uint()
            .times(1)
            .returning(|_, _, out, out_len, _| {
                unsafe { out.copy_from_nonoverlapping(SAMPLE.as_ptr(), NUMBER_SIZE) }
                out_len as i32
            });
        let _guard = setup_mock(mock);

        assert_eq!(Number::from_uint(42).unwrap(), Number(SAMPLE));
    }

    #[test]
    fn test_from_mant_exp_success() {
        let mut mock = MockHostBindings::new();
        mock.expect_float_from_mant_exp()
            .times(1)
            .returning(|_, _, out, out_len, _| {
                unsafe { out.copy_from_nonoverlapping(SAMPLE.as_ptr(), NUMBER_SIZE) }
                out_len as i32
            });
        let _guard = setup_mock(mock);

        assert_eq!(
            Number::from_mant_exp(5_000_000_000_000_000, -15).unwrap(),
            Number(SAMPLE)
        );
    }

    #[test]
    fn test_float_from_stamount_success() {
        let mut mock = MockHostBindings::new();
        mock.expect_float_from_stamount()
            .times(1)
            .returning(|_, _, out, out_len, _| {
                unsafe { out.copy_from_nonoverlapping(SAMPLE.as_ptr(), NUMBER_SIZE) }
                out_len as i32
            });
        let _guard = setup_mock(mock);

        assert_eq!(Number::from_stamount(&[0u8; 48]).unwrap(), Number(SAMPLE));
    }

    #[test]
    fn test_float_from_stnumber_success() {
        let mut mock = MockHostBindings::new();
        mock.expect_float_from_stnumber()
            .times(1)
            .returning(|_, _, out, out_len, _| {
                unsafe { out.copy_from_nonoverlapping(SAMPLE.as_ptr(), NUMBER_SIZE) }
                out_len as i32
            });
        let _guard = setup_mock(mock);

        assert_eq!(
            Number::from_stnumber(&[0u8; NUMBER_SIZE]).unwrap(),
            Number(SAMPLE)
        );
    }

    #[test]
    fn test_float_to_int_success() {
        let mut mock = MockHostBindings::new();
        mock.expect_float_to_int()
            .times(1)
            .returning(|_, _, out, out_len, _| {
                unsafe { out.copy_from_nonoverlapping(42i64.to_le_bytes().as_ptr(), 8) }
                out_len as i32
            });
        let _guard = setup_mock(mock);

        assert_eq!(Number(SAMPLE).to_int().unwrap(), 42);
    }

    #[test]
    fn test_float_to_mant_exp_success() {
        let mut mock = MockHostBindings::new();
        mock.expect_float_to_mant_exp()
            .times(1)
            .returning(|_, _, mant, mant_len, exp, _| {
                unsafe {
                    mant.copy_from_nonoverlapping(123i64.to_le_bytes().as_ptr(), 8);
                    exp.copy_from_nonoverlapping(5i32.to_le_bytes().as_ptr(), 4);
                }
                mant_len as i32
            });
        let _guard = setup_mock(mock);

        assert_eq!(Number(SAMPLE).to_mant_exp().unwrap(), (123, 5));
    }

    #[test]
    fn test_compare_orderings() {
        for (code, expected) in [
            (0, core::cmp::Ordering::Equal),
            (1, core::cmp::Ordering::Greater),
            (2, core::cmp::Ordering::Less),
        ] {
            let mut mock = MockHostBindings::new();
            mock.expect_float_cmp()
                .times(1)
                .returning(move |_, _, _, _| code);
            let _guard = setup_mock(mock);

            assert_eq!(Number(SAMPLE).cmp(&Number(SAMPLE)), expected);
        }
    }

    #[test]
    fn test_compare_via_host_error() {
        let mut mock = MockHostBindings::new();
        mock.expect_float_cmp().times(1).returning(|_, _, _, _| -19);
        let _guard = setup_mock(mock);

        assert!(Number(SAMPLE).compare_via_host(&Number(SAMPLE)).is_err());
    }

    #[test]
    fn test_zero_is_all_zeros() {
        assert_eq!(Number::ZERO, Number([0u8; NUMBER_SIZE]));
    }

    // A second distinct 12-byte value, for the two-operand arithmetic mocks.
    const OTHER: [u8; NUMBER_SIZE] = [
        0xD4, 0x83, 0x8D, 0x7E, 0xA4, 0xC6, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];

    #[test]
    fn test_float_add_success() {
        let mut mock = MockHostBindings::new();
        mock.expect_float_add()
            .times(1)
            .returning(|_, _, _, _, out, out_len, _| {
                unsafe { out.copy_from_nonoverlapping(SAMPLE.as_ptr(), NUMBER_SIZE) }
                out_len as i32
            });
        let _guard = setup_mock(mock);

        assert_eq!(
            Number(SAMPLE)
                .add(&Number(OTHER), RoundingMode::ToNearest)
                .unwrap(),
            Number(SAMPLE)
        );
    }

    #[test]
    fn test_float_sub_success() {
        let mut mock = MockHostBindings::new();
        mock.expect_float_sub()
            .times(1)
            .returning(|_, _, _, _, out, out_len, _| {
                unsafe { out.copy_from_nonoverlapping(SAMPLE.as_ptr(), NUMBER_SIZE) }
                out_len as i32
            });
        let _guard = setup_mock(mock);

        assert_eq!(
            Number(SAMPLE)
                .subtract(&Number(OTHER), RoundingMode::ToNearest)
                .unwrap(),
            Number(SAMPLE)
        );
    }

    #[test]
    fn test_float_mult_success() {
        let mut mock = MockHostBindings::new();
        mock.expect_float_mult()
            .times(1)
            .returning(|_, _, _, _, out, out_len, _| {
                unsafe { out.copy_from_nonoverlapping(SAMPLE.as_ptr(), NUMBER_SIZE) }
                out_len as i32
            });
        let _guard = setup_mock(mock);

        assert_eq!(
            Number(SAMPLE)
                .multiply(&Number(OTHER), RoundingMode::ToNearest)
                .unwrap(),
            Number(SAMPLE)
        );
    }

    #[test]
    fn test_float_div_success() {
        let mut mock = MockHostBindings::new();
        mock.expect_float_div()
            .times(1)
            .returning(|_, _, _, _, out, out_len, _| {
                unsafe { out.copy_from_nonoverlapping(SAMPLE.as_ptr(), NUMBER_SIZE) }
                out_len as i32
            });
        let _guard = setup_mock(mock);

        assert_eq!(
            Number(SAMPLE)
                .divide(&Number(OTHER), RoundingMode::ToNearest)
                .unwrap(),
            Number(SAMPLE)
        );
    }

    #[test]
    fn test_float_div_host_error() {
        let mut mock = MockHostBindings::new();
        mock.expect_float_div()
            .times(1)
            .returning(|_, _, _, _, _, _, _| -20); // INVALID_FLOAT_COMPUTATION (e.g. divide by zero)
        let _guard = setup_mock(mock);

        assert!(
            Number(SAMPLE)
                .divide(&Number(OTHER), RoundingMode::ToNearest)
                .is_err()
        );
    }

    #[test]
    fn test_float_pow_success() {
        let mut mock = MockHostBindings::new();
        mock.expect_float_pow()
            .times(1)
            .returning(|_, _, _, out, out_len, _| {
                unsafe { out.copy_from_nonoverlapping(SAMPLE.as_ptr(), NUMBER_SIZE) }
                out_len as i32
            });
        let _guard = setup_mock(mock);

        assert_eq!(
            Number(SAMPLE).pow(2, RoundingMode::ToNearest).unwrap(),
            Number(SAMPLE)
        );
    }

    #[test]
    fn test_float_root_success() {
        let mut mock = MockHostBindings::new();
        mock.expect_float_root()
            .times(1)
            .returning(|_, _, _, out, out_len, _| {
                unsafe { out.copy_from_nonoverlapping(SAMPLE.as_ptr(), NUMBER_SIZE) }
                out_len as i32
            });
        let _guard = setup_mock(mock);

        assert_eq!(
            Number(SAMPLE).root(2, RoundingMode::ToNearest).unwrap(),
            Number(SAMPLE)
        );
    }
}
