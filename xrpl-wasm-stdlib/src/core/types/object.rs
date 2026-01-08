/// Placeholder type for object SFields.
///
/// Object types in XRPL (like Memo, SignerEntry, etc.) cannot be directly retrieved
/// as complete values. Instead, they are used within `Location` to navigate to
/// specific object fields.
///
/// This type intentionally does NOT implement `LedgerObjectFieldGetter` to prevent compile-time
/// misuse. If you need to access object fields, use `Location` to navigate to
/// specific fields within the object.
///
/// TODO: explore using the Locator under the hood here
#[derive(Debug, Eq, PartialEq)]
pub struct Object;
