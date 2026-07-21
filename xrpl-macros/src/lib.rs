//! Procedural-macro entry points for `xrpl-common-stdlib`.
//!
//! Each macro here is a thin shim that delegates to its module's `expand`.
//! Logic, helpers, and unit tests live in the per-macro files.
//!
//! - **Typed-constant macros** (`r_address!`, `hash256!`, `pubkey!`,
//!   `currency!`, `blob!`): validate at compile time and emit a typed XRPL
//!   value. `hex_util` holds decode helpers shared across these macros.
//! - **Entry-point macros** (`#[smart_escrow]`, `#[smart_contract]`): wrap
//!   user functions in the `extern "C"` symbols the XRPL host calls. All
//!   three stages — parse, validate, codegen — live in `entry_point/` and are
//!   shared between the two macros so adding a third follows the same pattern.

use proc_macro::TokenStream;

mod blob;
mod currency;
mod entry_point;
mod hash256;
mod hex_util;
mod pubkey;
mod r_address;

/// Converts an XRPL classic address (r-address) to a 20-byte [`AccountID`] at compile time.
///
/// Accepts a Base58Check-encoded string starting with `'r'`. Full checksum
/// verification happens at compile time — a bad address is a compile error, not a
/// runtime panic. Only string literals are accepted; runtime `&str` values are not.
///
/// # Example
///
/// ```rust,ignore
/// use xrpl_common_stdlib::r_address;
/// use xrpl_common_stdlib::core::types::account_id::AccountID;
///
/// const ACCOUNT: AccountID = r_address!("rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh");
/// ```
#[proc_macro]
pub fn r_address(input: TokenStream) -> TokenStream {
    match r_address::expand(input.into()) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

/// Converts a 64-character hex string to a 32-byte [`Hash256`] (`UInt<32>`) at compile time.
///
/// Accepts uppercase, lowercase, and mixed-case hex. Wrong length or non-hex
/// characters are compile errors.
///
/// # Example
///
/// ```rust,ignore
/// use xrpl_common_stdlib::hash256;
/// use xrpl_common_stdlib::core::types::uint::Hash256;
///
/// const H: Hash256 =
///     hash256!("0000000000000000000000000000000000000000000000000000000000000001");
/// ```
#[proc_macro]
pub fn hash256(input: TokenStream) -> TokenStream {
    match hash256::expand(input.into()) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

/// Converts a 66-character hex string to a 33-byte [`PublicKey`] at compile time.
///
/// The first two hex characters must be a valid XRPL public key prefix:
/// `02` or `03` (secp256k1) or `ED` (Ed25519). The prefix check is
/// case-insensitive — `ed` is accepted and normalised. Any other prefix, wrong
/// length, or non-hex characters are compile errors.
///
/// # Example
///
/// ```rust,ignore
/// use xrpl_common_stdlib::pubkey;
/// use xrpl_common_stdlib::core::types::public_key::PublicKey;
///
/// const KEY: PublicKey =
///     pubkey!("02C7387FFC25C156CA7F8A6D760C8D01EF642CEE9CE4680C33FFB3FF39AFECFE70");
/// ```
#[proc_macro]
pub fn pubkey(input: TokenStream) -> TokenStream {
    match pubkey::expand(input.into()) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

/// Converts an XRPL currency code to a 20-byte [`Currency`] at compile time.
///
/// Two forms are accepted:
///
/// - **Standard (3 ASCII alphanumeric chars)** — stored verbatim in bytes 12–14;
///   bytes 0–11 and 15–19 are zero. `"XRP"` is reserved and rejected. Standard
///   codes are case-sensitive (`"USD"` and `"usd"` are distinct on-ledger
///   identifiers); use uppercase by convention.
/// - **Non-standard (40 hex chars)** — interpreted as a raw 20-byte value. Must
///   not start with `00` (that would alias the standard-code format).
///
/// # Example
///
/// ```rust,ignore
/// use xrpl_common_stdlib::currency;
/// use xrpl_common_stdlib::core::types::currency::Currency;
///
/// const USD: Currency = currency!("USD");
/// const CUSTOM: Currency = currency!("0158415500000000C1F76FF6ECB0BAC600000000");
/// ```
#[proc_macro]
pub fn currency(input: TokenStream) -> TokenStream {
    match currency::expand(input.into()) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

/// Converts a hex string to a compile-time [`Blob<N>`].
///
/// Two forms:
///
/// - `blob!("DEADBEEF")` — exact-fit `Blob<4>` where `N` equals the decoded byte count.
/// - `blob!("DEADBEEF", 128)` — `Blob<128>` zero-padded to the given capacity;
///   the decoded byte count must not exceed the capacity.
///
/// In both forms `len` is set to the decoded byte count. The hex string must
/// have an even number of characters.
///
/// # Example
///
/// ```rust,ignore
/// use xrpl_common_stdlib::blob;
/// use xrpl_common_stdlib::core::types::blob::Blob;
///
/// const EXACT: Blob<4> = blob!("DEADBEEF");
/// const PADDED: Blob<32> = blob!("DEADBEEF", 32);
/// ```
#[proc_macro]
pub fn blob(input: TokenStream) -> TokenStream {
    match blob::expand(input.into()) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

/// Wraps a Smart Escrow finish function in the `extern "C" fn finish()` entry point
/// the XRPL host calls when an `EscrowFinish` transaction invokes the feature.
///
/// The annotated function must:
/// - Take `EscrowFinishContext` as its first (and only) argument.
/// - Return `FinishResult` or `i32`.
/// - The attribute takes no arguments — `#[smart_escrow(anything)]` is a compile error.
///
/// Any other signature is a compile error pointing at the offending token.
///
/// # Usage
///
/// Import this attribute from `xrpl_escrow_stdlib`, which re-exports it alongside
/// `EscrowFinishContext`. Importing directly from `xrpl_macros` is unsupported
/// because the generated code references types in `xrpl_escrow_stdlib`.
///
/// `FinishResult` and `i32` convert into each other (`From<i32>` /
/// `From<FinishResult>`), so a host error code can be propagated with
/// `.into()` either way:
///
/// ```rust,ignore
/// // Import from the feature crate, not from xrpl_macros directly.
/// use xrpl_escrow_stdlib::{smart_escrow, EscrowFinishContext, FinishResult};
/// use xrpl_escrow_stdlib::core::current_tx::traits::TransactionCommonFields;
/// use xrpl_escrow_stdlib::core::types::amount::Amount;
///
/// #[smart_escrow]
/// fn finish(ctx: EscrowFinishContext) -> FinishResult {
///     let fee = match ctx.tx().get_fee() {
///         Ok(f) => f,
///         Err(e) => return e.code().into(), // i32 -> FinishResult
///     };
///
///     match fee {
///         Amount::XRP { num_drops } if num_drops > 1000 => FinishResult::succeed(),
///         _ => FinishResult::reject(),
///     }
/// }
/// ```
///
/// Returning `i32` directly skips the macro's `FinishResult` handling:
///
/// ```rust,ignore
/// use xrpl_escrow_stdlib::{smart_escrow, EscrowFinishContext, FinishResult};
/// use xrpl_escrow_stdlib::core::current_tx::traits::TransactionCommonFields;
/// use xrpl_escrow_stdlib::core::types::amount::Amount;
///
/// #[smart_escrow]
/// fn finish(ctx: EscrowFinishContext) -> i32 {
///     let fee = match ctx.tx().get_fee() {
///         Ok(f) => f,
///         Err(e) => return e.code(),
///     };
///
///     let result = match fee {
///         Amount::XRP { num_drops } if num_drops > 1000 => FinishResult::succeed(),
///         _ => FinishResult::reject(),
///     };
///
///     result.into()
/// }
/// ```
#[proc_macro_attribute]
pub fn smart_escrow(attr: TokenStream, item: TokenStream) -> TokenStream {
    entry_point::smart_escrow::expand(attr, item)
}

/// Wraps a Smart Contract entry function in the appropriate `extern "C"` export.
#[proc_macro_attribute]
pub fn smart_contract(attr: TokenStream, item: TokenStream) -> TokenStream {
    entry_point::smart_contract::expand(attr, item)
}
