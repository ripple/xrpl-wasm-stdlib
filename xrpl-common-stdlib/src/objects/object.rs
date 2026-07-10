/// Placeholder type for object SFields.
///
/// Object types in XRPL (like Memo, SignerEntry, etc.) cannot be directly retrieved as complete
/// values, so `Object` implements neither [`crate::fields::decoder::FromCurrentTx`] nor
/// [`crate::fields::decoder::FromLedger`]: passing an `SField<Object, _>` to a field getter is a
/// compile-time error. To reach into an object, navigate with a `Locator`.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Object;
