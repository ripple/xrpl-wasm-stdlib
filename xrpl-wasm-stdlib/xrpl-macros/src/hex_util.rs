//! Shared hex-decoding helpers used by the typed-constant macros.

/// Decode an even-length hex string into bytes. Caller must verify length and
/// that every char is `is_ascii_hexdigit` before calling — non-hex input will
/// panic.
pub(crate) fn decode_hex(s: &str) -> Vec<u8> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
        .collect()
}
