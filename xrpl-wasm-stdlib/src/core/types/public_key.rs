use crate::core::current_tx::CurrentTxFieldGetter;
use crate::host::field_helpers::{
    get_fixed_size_field_with_expected_bytes, get_fixed_size_field_with_expected_bytes_optional,
};
use crate::host::{Result, get_tx_field};
use crate::sfield::SField;

pub const PUBLIC_KEY_BUFFER_SIZE: usize = 33;

/// A 33-byte public key for secp256k1 and ed25519 DSA types.
///
/// Public keys on the XRP Ledger are 33 bytes and can be either:
/// - **secp256k1**: Compressed ECDSA public key (0x02 or 0x03 prefix)
/// - **ed25519**: EdDSA public key (0xED prefix)
///
/// ## Derived Traits
///
/// - `PartialEq, Eq`: Enable comparisons and use in collections
/// - `Debug, Clone`: Standard traits for development and consistency
///
/// Note: `Copy` is intentionally not derived due to the struct's size (33 bytes).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicKey(pub [u8; PUBLIC_KEY_BUFFER_SIZE]);

impl From<[u8; PUBLIC_KEY_BUFFER_SIZE]> for PublicKey {
    fn from(bytes: [u8; PUBLIC_KEY_BUFFER_SIZE]) -> Self {
        Self(bytes) // Access private field legally here
    }
}

impl From<[u8; 64]> for PublicKey {
    fn from(bytes: [u8; 64]) -> Self {
        // Take the first PUBLIC_KEY_BUFFER_SIZE bytes from the 64-byte array
        let mut key_bytes = [0u8; PUBLIC_KEY_BUFFER_SIZE];
        key_bytes.copy_from_slice(&bytes[..PUBLIC_KEY_BUFFER_SIZE]);
        PublicKey(key_bytes)
    }
}

impl TryFrom<&[u8]> for PublicKey {
    type Error = &'static str;

    /// Attempts to create a `PublicKey` from a byte slice.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the slice length is not exactly `PUBLIC_KEY_BUFFER_SIZE` (33) bytes.
    fn try_from(bytes: &[u8]) -> core::result::Result<Self, Self::Error> {
        if bytes.len() != PUBLIC_KEY_BUFFER_SIZE {
            return Err("slice must be exactly 33 bytes to construct a PublicKey");
        }
        let mut key_bytes = [0u8; PUBLIC_KEY_BUFFER_SIZE];
        key_bytes.copy_from_slice(bytes);
        Ok(PublicKey(key_bytes))
    }
}

/// Implementation of `CurrentTxFieldGetter` for XRPL public keys.
///
/// This implementation handles 33-byte compressed public key fields in XRPL transactions.
/// Public keys are used for cryptographic signature verification and are commonly found
/// in the SigningPubKey field and various other cryptographic contexts.
///
/// # Buffer Management
///
/// Uses a 33-byte buffer and validates that exactly 33 bytes are returned
/// from the host function. The buffer is converted to a PublicKey using
/// the `From<[u8; 33]>` implementation.
impl CurrentTxFieldGetter for PublicKey {
    #[inline]
    fn get_from_current_tx<const CODE: i32>(field: SField<Self, CODE>) -> Result<Self> {
        get_fixed_size_field_with_expected_bytes::<33, _>(
            i32::from(field),
            |fc, buf, size| unsafe { get_tx_field(fc, buf, size) },
        )
        .map(|buffer| buffer.into())
    }

    #[inline]
    fn get_from_current_tx_optional<const CODE: i32>(
        field: SField<Self, CODE>,
    ) -> Result<Option<Self>> {
        get_fixed_size_field_with_expected_bytes_optional::<33, _>(
            i32::from(field),
            |fc, buf, size| unsafe { get_tx_field(fc, buf, size) },
        )
        .map(|buffer| buffer.map(|b| b.into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // secp256k1 compressed public key (0x02 prefix)
    const PUBKEY_SECP256K1: [u8; PUBLIC_KEY_BUFFER_SIZE] = [
        0x02, 0xC7, 0x38, 0x7F, 0xFC, 0x25, 0xC1, 0x56, 0xCA, 0x7F, 0x8A, 0x6D, 0x76, 0x0C, 0x8D,
        0x01, 0xEF, 0x64, 0x2C, 0xEE, 0x9C, 0xE4, 0x68, 0x0C, 0x33, 0xFF, 0xB3, 0xFF, 0x39, 0xAF,
        0xEC, 0xFE, 0x70,
    ];

    // ed25519 public key (0xED prefix)
    const PUBKEY_ED25519: [u8; PUBLIC_KEY_BUFFER_SIZE] = [
        0xED, 0xD9, 0xB3, 0x59, 0x98, 0x02, 0xB2, 0x14, 0xA9, 0x9D, 0x75, 0x77, 0x12, 0xD6, 0xAB,
        0xDF, 0x72, 0xF8, 0x3C, 0x63, 0xBB, 0xD5, 0x38, 0x61, 0x41, 0x17, 0x90, 0xB1, 0x3D, 0x04,
        0xB2, 0xC5, 0xC9,
    ];

    #[test]
    fn test_from_33_bytes_secp256k1() {
        // Test From<[u8; 33]> with a secp256k1 key (0x02 prefix)
        let pk = PublicKey::from(PUBKEY_SECP256K1);
        assert_eq!(pk.0, PUBKEY_SECP256K1);
    }

    #[test]
    fn test_from_33_bytes_ed25519() {
        // Test From<[u8; 33]> with an ed25519 key (0xED prefix)
        let pk = PublicKey::from(PUBKEY_ED25519);
        assert_eq!(pk.0, PUBKEY_ED25519);
    }

    #[test]
    fn test_from_64_byte_array_takes_first_33() {
        // Test From<[u8; 64]> - should take first 33 bytes
        let mut bytes_64 = [0xAAu8; 64];
        for (i, byte) in bytes_64.iter_mut().enumerate().take(33) {
            *byte = i as u8;
        }
        let pubkey = PublicKey::from(bytes_64);
        for i in 0..33 {
            assert_eq!(pubkey.0[i], i as u8);
        }
    }

    #[test]
    fn test_try_from_slice_exact_33_bytes_secp256k1() {
        // TryFrom<&[u8]> succeeds when slice is exactly 33 bytes
        let pk = PublicKey::try_from(PUBKEY_SECP256K1.as_slice()).unwrap();
        assert_eq!(pk.0, PUBKEY_SECP256K1);
    }

    #[test]
    fn test_try_from_slice_exact_33_bytes_ed25519() {
        // TryFrom<&[u8]> succeeds when slice is exactly 33 bytes
        let pk = PublicKey::try_from(PUBKEY_ED25519.as_slice()).unwrap();
        assert_eq!(pk.0, PUBKEY_ED25519);
    }

    #[test]
    fn test_try_from_slice_shorter_than_33_bytes_errors() {
        // TryFrom<&[u8]> must reject slices shorter than 33 bytes
        let short_slice: &[u8] = &[0x02, 0xAA, 0xBB, 0xCC, 0xDD];
        assert!(PublicKey::try_from(short_slice).is_err());
    }

    #[test]
    fn test_try_from_slice_longer_than_33_bytes_errors() {
        // TryFrom<&[u8]> must reject slices longer than 33 bytes
        let long_slice: &[u8] = &[0xFFu8; 50];
        assert!(PublicKey::try_from(long_slice).is_err());
    }

    #[test]
    fn test_try_from_slice_empty_errors() {
        // TryFrom<&[u8]> must reject empty slices
        let empty: &[u8] = &[];
        assert!(PublicKey::try_from(empty).is_err());
    }
}
