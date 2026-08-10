//! A message whose signature is being verified.

/// A message whose signature is checked by [`check_sig`](crate::crypto::check_sig).
///
/// This is a thin, zero-copy newtype around a byte slice. Its only purpose is to give the
/// message argument a distinct type from the signature argument (see [`Signature`]), so a
/// caller who swaps the two is caught at compile time rather than silently getting a wrong
/// answer at runtime.
///
/// [`Signature`]: crate::types::signature::Signature
///
/// ## Derived Traits
///
/// - `Copy`: the newtype only borrows a slice, so copying is trivial
/// - `PartialEq, Eq`: enable comparisons
/// - `Debug, Clone`: standard traits for development and consistency
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Message<'a>(pub &'a [u8]);
