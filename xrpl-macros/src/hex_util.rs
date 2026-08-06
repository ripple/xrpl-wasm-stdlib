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

#[cfg(test)]
mod tests {
    // Simple tests for decoding logic
    use super::decode_hex;

    #[test]
    fn decodes_lowercase() {
        assert_eq!(decode_hex("deadbeef"), vec![0xde, 0xad, 0xbe, 0xef]);
    }

    #[test]
    fn decodes_uppercase() {
        assert_eq!(decode_hex("DEADBEEF"), vec![0xde, 0xad, 0xbe, 0xef]);
    }

    #[test]
    fn decodes_mixed_case() {
        assert_eq!(decode_hex("DeAdBeEf"), vec![0xde, 0xad, 0xbe, 0xef]);
    }

    #[test]
    fn decodes_empty() {
        assert_eq!(decode_hex(""), Vec::<u8>::new());
    }

    #[test]
    fn decodes_single_byte() {
        assert_eq!(decode_hex("FF"), vec![0xff]);
        assert_eq!(decode_hex("00"), vec![0x00]);
    }
}
