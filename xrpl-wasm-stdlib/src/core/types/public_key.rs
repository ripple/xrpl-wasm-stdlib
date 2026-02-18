use crate::core::current_tx::CurrentTxFieldGetter;
use crate::host::field_helpers::{
    get_fixed_size_field_with_expected_bytes, get_fixed_size_field_with_expected_bytes_optional,
};
use crate::host::{Result, get_tx_field};

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

impl From<&[u8]> for PublicKey {
    fn from(bytes: &[u8]) -> Self {
        let mut key_bytes = [0u8; PUBLIC_KEY_BUFFER_SIZE];
        key_bytes[..bytes.len().min(PUBLIC_KEY_BUFFER_SIZE)]
            .copy_from_slice(&bytes[..bytes.len().min(PUBLIC_KEY_BUFFER_SIZE)]);
        PublicKey(key_bytes)
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
    fn get_from_current_tx(field_code: i32) -> Result<Self> {
        get_fixed_size_field_with_expected_bytes::<33, _>(field_code, |fc, buf, size| unsafe {
            get_tx_field(fc, buf, size)
        })
        .map(|buffer| buffer.into())
    }

    #[inline]
    fn get_from_current_tx_optional(field_code: i32) -> Result<Option<Self>> {
        get_fixed_size_field_with_expected_bytes_optional::<33, _>(
            field_code,
            |fc, buf, size| unsafe { get_tx_field(fc, buf, size) },
        )
        .map(|buffer| buffer.map(|b| b.into()))
    }
}

#[cfg(test)]
mod test_public_key {
    use super::*;

    #[test]
    fn test_from_64_byte_array_truncates() {
        // Test From<[u8; 64]> - should take first 33 bytes
        let mut bytes_64 = [0xAAu8; 64];
        // Put distinct values in first 33 bytes
        for (i, byte) in bytes_64.iter_mut().enumerate().take(33) {
            *byte = i as u8;
        }

        let pubkey = PublicKey::from(bytes_64);

        // Verify first 33 bytes are preserved
        for i in 0..33 {
            assert_eq!(pubkey.0[i], i as u8);
        }
    }

    #[test]
    fn test_from_slice_shorter_than_33_bytes() {
        // Test From<&[u8]> with fewer than 33 bytes - should zero-pad
        let short_slice: &[u8] = &[0x02, 0xAA, 0xBB, 0xCC, 0xDD];
        let pubkey = PublicKey::from(short_slice);

        // First 5 bytes should match input
        assert_eq!(&pubkey.0[..5], short_slice);
        // Remaining bytes should be zero
        assert_eq!(&pubkey.0[5..], &[0u8; 28]);
    }

    #[test]
    fn test_from_slice_longer_than_33_bytes_truncates() {
        // Test From<&[u8]> with more than 33 bytes - should truncate
        let long_slice: &[u8] = &[0xFFu8; 50];
        let pubkey = PublicKey::from(long_slice);

        // Should only contain first 33 bytes
        assert_eq!(pubkey.0, [0xFFu8; 33]);
    }

    #[test]
    fn test_from_empty_slice() {
        // Test From<&[u8]> with empty slice - should be all zeros
        let empty_slice: &[u8] = &[];
        let pubkey = PublicKey::from(empty_slice);
        assert_eq!(pubkey.0, [0u8; 33]);
    }
}
