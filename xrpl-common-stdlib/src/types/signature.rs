//! A signature being verified against a message and public key.

/// A signature checked by [`check_sig`](crate::crypto::check_sig).
///
/// This is a thin, zero-copy newtype around a byte slice. Its only purpose is to give the
/// signature argument a distinct type from the message argument (see [`Message`]), so a
/// caller who swaps the two is caught at compile time rather than silently getting a wrong
/// answer at runtime.
///
/// [`Message`]: crate::types::message::Message
///
/// ## Derived Traits
///
/// - `Copy`: the newtype only borrows a slice, so copying is trivial
/// - `PartialEq, Eq`: enable comparisons
/// - `Debug, Clone`: standard traits for development and consistency
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Signature<'a>(pub &'a [u8]);

impl<'a> From<&'a [u8]> for Signature<'a> {
    fn from(bytes: &'a [u8]) -> Self {
        Self(bytes)
    }
}
