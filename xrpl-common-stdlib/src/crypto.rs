use crate::host;
use crate::host::error_codes::match_result_code_with_expected_bytes;
use crate::host::{Error, Result};
use crate::types::public_key::{PUBLIC_KEY_BUFFER_SIZE, PublicKey};

/// SHA-512Half: SHA-512 of `data`, truncated to the first 32 bytes.
///
/// `data` may be 0..=1024 bytes (`maxWasmParamLength`); the host returns
/// `DataFieldTooLarge` for larger input.
pub fn sha512_half(data: &[u8]) -> Result<[u8; 32]> {
    let mut out = [0u8; 32];
    let rescode =
        unsafe { host::compute_sha512_half(data.as_ptr(), data.len(), out.as_mut_ptr(), 32) };
    match_result_code_with_expected_bytes(rescode, 32, || out)
}

/// Verify `sig` over `msg` for public key `key`.
///
/// `&PublicKey` (33 bytes) enforces the size constraint at the call site.
/// - secp256k1 keys (0x02/0x03): the host pre-hashes `msg` with SHA-512Half before ECDSA verify.
/// - Ed25519 keys (0xED): the host verifies the raw `msg` directly (no pre-hash), stripping the
///   0xED prefix; a non-canonical signature returns `Ok(false)`.
///
/// An empty `msg` or empty `sig` is not an error — the host returns `Ok(false)`.
///
/// Errors: `InvalidParams` if the key is malformed (not 33 bytes / bad prefix);
/// `DataFieldTooLarge` if any parameter exceeds 1024 bytes.
pub fn check_sig(msg: &[u8], sig: &[u8], key: &PublicKey) -> Result<bool> {
    let rescode = unsafe {
        host::check_sig(
            msg.as_ptr(),
            msg.len(),
            sig.as_ptr(),
            sig.len(),
            key.0.as_ptr(),
            PUBLIC_KEY_BUFFER_SIZE,
        )
    };
    match rescode {
        0 => Result::Ok(false),
        1 => Result::Ok(true),
        code if code < 0 => Result::Err(Error::from_code(code)),
        code => panic!("internal invariant violated: host returned unexpected value {code}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::host::error_codes::{DATA_FIELD_TOO_LARGE, INVALID_PARAMS};
    use crate::host::host_bindings_trait::MockHostBindings;
    use crate::host::setup_mock;

    fn write_digest(ptr: *mut u8, fill: u8) {
        unsafe {
            for i in 0..32 {
                *ptr.add(i) = fill;
            }
        }
    }

    // ---- sha512_half ----

    #[test]
    fn test_sha512_half_success() {
        let mut mock = MockHostBindings::new();
        mock.expect_compute_sha512_half()
            .times(1)
            .returning(|_data, _dlen, out_ptr, _olen| {
                write_digest(out_ptr, 0xCD);
                32
            });
        let _guard = setup_mock(mock);

        let result = sha512_half(b"hello world");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), [0xCD; 32]);
    }

    #[test]
    #[should_panic(expected = "internal invariant violated")]
    fn test_sha512_half_wrong_byte_count() {
        let mut mock = MockHostBindings::new();
        mock.expect_compute_sha512_half()
            .times(1)
            .returning(|_, _, _, _| 16); // host returns wrong (non-32) byte count
        let _guard = setup_mock(mock);

        let _ = sha512_half(b"hello");
    }

    #[test]
    fn test_sha512_half_oversized() {
        let mut mock = MockHostBindings::new();
        mock.expect_compute_sha512_half()
            .times(1)
            .returning(|_, _, _, _| DATA_FIELD_TOO_LARGE);
        let _guard = setup_mock(mock);

        let result = sha512_half(&[0u8; 1025]);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().code(), DATA_FIELD_TOO_LARGE);
    }

    // ---- check_sig ----

    const PUBKEY_BYTES: [u8; PUBLIC_KEY_BUFFER_SIZE] = [
        0x02, 0xC7, 0x38, 0x7F, 0xFC, 0x25, 0xC1, 0x56, 0xCA, 0x7F, 0x8A, 0x6D, 0x76, 0x0C, 0x8D,
        0x01, 0xEF, 0x64, 0x2C, 0xEE, 0x9C, 0xE4, 0x68, 0x0C, 0x33, 0xFF, 0xB3, 0xFF, 0x39, 0xAF,
        0xEC, 0xFE, 0x70,
    ];

    #[test]
    fn test_check_sig_valid() {
        let mut mock = MockHostBindings::new();
        mock.expect_check_sig()
            .times(1)
            .returning(|_, _, _, _, _, _| 1);
        let _guard = setup_mock(mock);

        let key = PublicKey::from(PUBKEY_BYTES);
        let result = check_sig(b"message", b"signature", &key);
        assert!(result.unwrap());
    }

    #[test]
    fn test_check_sig_invalid() {
        let mut mock = MockHostBindings::new();
        mock.expect_check_sig()
            .times(1)
            .returning(|_, _, _, _, _, _| 0);
        let _guard = setup_mock(mock);

        let key = PublicKey::from(PUBKEY_BYTES);
        let result = check_sig(b"message", b"signature", &key);
        assert!(!result.unwrap());
    }

    #[test]
    fn test_check_sig_bad_pubkey() {
        let mut mock = MockHostBindings::new();
        mock.expect_check_sig()
            .times(1)
            .returning(|_, _, _, _, _, _| INVALID_PARAMS);
        let _guard = setup_mock(mock);

        let key = PublicKey::from(PUBKEY_BYTES);
        let result = check_sig(b"message", b"signature", &key);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().code(), INVALID_PARAMS);
    }

    #[test]
    fn test_check_sig_oversized() {
        let mut mock = MockHostBindings::new();
        mock.expect_check_sig()
            .times(1)
            .returning(|_, _, _, _, _, _| DATA_FIELD_TOO_LARGE);
        let _guard = setup_mock(mock);

        let key = PublicKey::from(PUBKEY_BYTES);
        let result = check_sig(&[0u8; 1025], b"signature", &key);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().code(), DATA_FIELD_TOO_LARGE);
    }

    #[test]
    #[should_panic(expected = "internal invariant violated")]
    fn test_check_sig_unexpected_positive() {
        let mut mock = MockHostBindings::new();
        mock.expect_check_sig()
            .times(1)
            .returning(|_, _, _, _, _, _| 2); // host returns 2 — only 0 and 1 are valid
        let _guard = setup_mock(mock);

        let key = PublicKey::from(PUBKEY_BYTES);
        let _ = check_sig(b"m", b"s", &key);
    }
}
