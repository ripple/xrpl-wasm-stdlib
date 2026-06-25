use core::num::NonZeroI32;

/// Return type for Smart Escrow finish entry points.
///
/// Wraps the `i32` the host inspects after the contract returns:
/// - positive → escrow released
/// - `0`      → escrow not released, no error (hold)
/// - negative → finish rejected
///
/// Construct with [`FinishResult::succeed`] / [`FinishResult::hold`] /
/// [`FinishResult::reject`] for the common cases, or [`FinishResult::succeed_with`]
/// / [`FinishResult::reject_with`] for a custom code. `From<i32>` is also
/// implemented so error codes from host calls (e.g. `e.code()`) can be
/// propagated directly: `return e.code().into();`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FinishResult(i32);

impl FinishResult {
    /// Release the escrow (maps to `1`).
    pub const fn succeed() -> Self {
        Self(1)
    }

    /// Do not release the escrow; no error (maps to `0`).
    pub const fn hold() -> Self {
        Self(0)
    }

    /// Reject the escrow finish (maps to `-1`).
    pub const fn reject() -> Self {
        Self(-1)
    }

    /// Release the escrow with a custom positive code.
    ///
    /// Returns `None` if `code <= 0`, since a non-positive code would be
    /// interpreted by the host as hold or reject.
    pub fn succeed_with(code: NonZeroI32) -> Option<Self> {
        (code.get() > 0).then_some(Self(code.get()))
    }

    /// Reject the escrow finish with a custom negative code.
    ///
    /// Returns `None` if `code >= 0`, since a non-negative code would be
    /// interpreted by the host as release or hold.
    pub fn reject_with(code: NonZeroI32) -> Option<Self> {
        (code.get() < 0).then_some(Self(code.get()))
    }

    /// Returns `true` if this result will release the escrow.
    pub const fn is_succeed(self) -> bool {
        self.0 > 0
    }

    /// Returns `true` if this result holds the escrow (no release, no error).
    pub const fn is_hold(self) -> bool {
        self.0 == 0
    }

    /// Returns `true` if this result rejects the finish.
    pub const fn is_reject(self) -> bool {
        self.0 < 0
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
    fn hold_maps_to_zero() {
        assert_eq!(i32::from(FinishResult::hold()), 0);
    }

    #[test]
    fn reject_maps_to_negative_one() {
        assert_eq!(i32::from(FinishResult::reject()), -1);
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
        assert!(!FinishResult::succeed().is_hold());
        assert!(!FinishResult::succeed().is_reject());

        assert!(FinishResult::hold().is_hold());
        assert!(!FinishResult::hold().is_succeed());
        assert!(!FinishResult::hold().is_reject());

        assert!(FinishResult::reject().is_reject());
        assert!(!FinishResult::reject().is_succeed());
        assert!(!FinishResult::reject().is_hold());
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
