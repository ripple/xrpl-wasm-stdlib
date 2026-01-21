/// Placeholder type for array SFields.
///
/// Array types in XRPL (like Signers, Memos, etc.) cannot be directly retrieved
/// as complete values. Instead, they are used within `Location` to navigate to
/// specific array elements.
///
/// This type intentionally does NOT implement `LedgerObjectFieldGetter` to prevent compile-time
/// misuse. If you need to access array elements, use `Locator` to navigate to
/// specific fields within the array.
///
/// TODO: explore using the Locator under the hood here
#[derive(Debug, Eq, PartialEq)]
pub struct Array;
