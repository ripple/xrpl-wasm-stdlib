/// Return type for Smart Feature entry points.
///
/// Wraps the `i32` the host inspects after a Smart Feature's WASM function
/// returns:
/// - positive (`> 0`) → the Smart Feature allows the native transaction to proceed
/// - `0`              → the Smart Feature blocks the transaction (no specific error)
/// - negative (`< 0`) → the Smart Feature blocks and propagates an error code
///
/// Both `0` and negative values are rejections (per XLS-100: a return value
/// greater than zero allows the operation, otherwise it is blocked). The
/// distinction is purely for diagnostic purposes: `0` is a clean rejection, while a
/// negative value carries an error code whose meaning is defined by the contract author.
/// For example, `-6` might represent the result of a failed host call, but the same
/// value could carry a different contract-defined meaning.
///
/// Construct with [`FinishResult::succeed`] / [`FinishResult::reject`] for the
/// common cases, or [`FinishResult::succeed_with`] / [`FinishResult::reject_with`]
/// for a custom code. `From<i32>` is also implemented so error codes from host
/// calls (e.g. `e.code()`) can be propagated directly:
/// `return e.code().into();`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FinishResult(i32);

impl FinishResult {
    /// Allow the Smart Feature to proceed (maps to `1`).
    pub const fn succeed() -> Self {
        Self(1)
    }

    /// Block the Smart Feature with no specific error code (maps to `0`).
    pub const fn reject() -> Self {
        Self(0)
    }

    /// Allow the Smart Feature with a custom positive code.
    ///
    /// `N` must be a positive compile-time constant; passing a non-positive value
    /// is a compile-time error. The code is recorded in the `WasmReturnCode`
    /// ledger metadata field and can be used for diagnostics.
    ///
    /// ```
    /// # use xrpl_escrow_stdlib::FinishResult;
    /// let result = FinishResult::succeed_with::<42>();
    /// assert_eq!(i32::from(result), 42);
    /// ```
    pub fn succeed_with<const N: i32>() -> Self {
        const { assert!(N > 0, "succeed_with requires a positive code") };
        Self(N)
    }

    /// Block the Smart Feature with a custom negative error code.
    ///
    /// `N` must be a negative compile-time constant; passing a non-negative value
    /// is a compile-time error. The code is recorded in the `WasmReturnCode`
    /// ledger metadata field and can be used for diagnostics.
    ///
    /// ```
    /// # use xrpl_wasm_stdlib::FinishResult;
    /// let result = FinishResult::reject_with::<-5>();
    /// assert_eq!(i32::from(result), -5);
    /// ```
    pub fn reject_with<const N: i32>() -> Self {
        const { assert!(N <= 0, "reject_with requires a negative code") };
        Self(N)
    }
}

impl From<FinishResult> for i32 {
    fn from(result: FinishResult) -> i32 {
        result.0
    }
}

impl From<i32> for FinishResult {
    fn from(code: i32) -> Self {
        Self(code)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- constructors ---

    #[test]
    fn succeed_maps_to_one() {
        assert_eq!(i32::from(FinishResult::succeed()), 1);
    }

    #[test]
    fn reject_maps_to_zero() {
        assert_eq!(i32::from(FinishResult::reject()), 0);
    }

    // --- succeed_with / reject_with ---

    #[test]
    fn succeed_with_positive_uses_code() {
        assert_eq!(i32::from(FinishResult::succeed_with::<42>()), 42);
    }

    #[test]
    fn succeed_with_i32_max_uses_code() {
        assert_eq!(
            i32::from(FinishResult::succeed_with::<{ i32::MAX }>()),
            i32::MAX
        );
    }

    #[test]
    fn reject_with_negative_uses_code() {
        assert_eq!(i32::from(FinishResult::reject_with::<-5>()), -5);
    }

    #[test]
    fn reject_with_i32_min_uses_code() {
        assert_eq!(
            i32::from(FinishResult::reject_with::<{ i32::MIN }>()),
            i32::MIN
        );
    }

    #[test]
    fn reject_with_zero_uses_code() {
        assert_eq!(i32::from(FinishResult::reject_with::<0>()), 0);
    }

    // --- From<i32> (used for error-code propagation: `e.code().into()`) ---

    #[test]
    fn from_positive_i32_roundtrips() {
        assert_eq!(i32::from(FinishResult::from(7)), 7);
    }

    #[test]
    fn from_zero_roundtrips() {
        assert_eq!(i32::from(FinishResult::from(0)), 0);
    }

    #[test]
    fn from_negative_i32_roundtrips() {
        assert_eq!(i32::from(FinishResult::from(-3)), -3);
    }

    #[test]
    fn from_i32_preserves_error_code_exactly() {
        // Simulates: `return e.code().into();` where e.code() is a negative error code.
        let host_error_code: i32 = -15; // INVALID_PARAMS
        assert_eq!(
            i32::from(FinishResult::from(host_error_code)),
            host_error_code
        );
    }
}
