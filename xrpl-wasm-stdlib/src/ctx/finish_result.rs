use core::num::NonZeroI32;

/// Return type for Smart Feature entry points.
///
/// Wraps the `i32` the host inspects after a Smart Feature's WASM function
/// returns:
/// - positive (`> 0`) → the Smart Feature allows the native transaction to proceed
/// - `0`              → the Smart Feature blocks the transaction (no specific error)
/// - negative (`< 0`) → the Smart Feature blocks and propagates a host error code
///
/// Both `0` and negative values are rejections (per XLS-100: a return value
/// greater than zero allows the operation, otherwise it is blocked). The
/// distinction is purely for diagnostics: `0` is a clean rejection, while a
/// negative value carries an error code from a failed host call.
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
    /// Returns `None` if `code <= 0`, since a non-positive code would be
    /// interpreted by the host as a rejection.
    pub fn succeed_with(code: NonZeroI32) -> Option<Self> {
        (code.get() > 0).then_some(Self(code.get()))
    }

    /// Block the Smart Feature with a custom negative error code.
    ///
    /// Returns `None` if `code >= 0`, since a non-negative code would not
    /// carry an error signal.
    pub fn reject_with(code: NonZeroI32) -> Option<Self> {
        (code.get() < 0).then_some(Self(code.get()))
    }

    /// Returns `true` if this result allows the transaction to proceed.
    pub const fn is_succeed(self) -> bool {
        self.0 > 0
    }

    /// Returns `true` if this result blocks the transaction.
    pub const fn is_reject(self) -> bool {
        self.0 <= 0
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

    // --- succeed_with / reject_with validation ---

    #[test]
    fn succeed_with_positive_is_some() {
        let r = FinishResult::succeed_with(NonZeroI32::new(42).unwrap()).unwrap();
        assert_eq!(i32::from(r), 42);
    }

    #[test]
    fn succeed_with_negative_is_none() {
        assert!(FinishResult::succeed_with(NonZeroI32::new(-1).unwrap()).is_none());
    }

    #[test]
    fn succeed_with_i32_max_is_some() {
        let r = FinishResult::succeed_with(NonZeroI32::new(i32::MAX).unwrap()).unwrap();
        assert_eq!(i32::from(r), i32::MAX);
    }

    #[test]
    fn reject_with_negative_is_some() {
        let r = FinishResult::reject_with(NonZeroI32::new(-5).unwrap()).unwrap();
        assert_eq!(i32::from(r), -5);
    }

    #[test]
    fn reject_with_positive_is_none() {
        assert!(FinishResult::reject_with(NonZeroI32::new(1).unwrap()).is_none());
    }

    // --- predicate accessors ---

    #[test]
    fn predicates_match_sign() {
        assert!(FinishResult::succeed().is_succeed());
        assert!(!FinishResult::succeed().is_reject());

        assert!(FinishResult::reject().is_reject());
        assert!(!FinishResult::reject().is_succeed());

        // A negative host error code is also a rejection.
        let err = FinishResult::from(-15);
        assert!(err.is_reject());
        assert!(!err.is_succeed());
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
    fn from_i32_preserves_host_error_code_exactly() {
        // Simulates: `return e.code().into();` where e.code() is a negative host error code.
        let host_error_code: i32 = -15; // INVALID_PARAMS
        assert_eq!(
            i32::from(FinishResult::from(host_error_code)),
            host_error_code
        );
    }
}
