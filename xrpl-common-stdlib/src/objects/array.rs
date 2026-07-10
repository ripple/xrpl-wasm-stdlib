/// Placeholder type for array SFields.
///
/// Array types in XRPL (like Signers, Memos, etc.) cannot be directly retrieved as complete
/// values, so `Array` implements neither [`crate::fields::decoder::FromCurrentTx`] nor
/// [`crate::fields::decoder::FromLedger`]: passing an `SField<Array, _>` to a field getter is a
/// compile-time error. To reach into an array, navigate with a `Locator`.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Array;
