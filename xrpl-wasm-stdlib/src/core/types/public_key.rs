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
    use crate::core::types::public_key::PUBLIC_KEY_BUFFER_SIZE;

    // secp256k1
    const PUBKEY_SECP256K1: [u8; PUBLIC_KEY_BUFFER_SIZE] = [
        0x02, 0xC7, 0x38, 0x7F, 0xFC, 0x25, 0xC1, 0x56, 0xCA, 0x7F, 0x8A, 0x6D, 0x76, 0x0C, 0x8D,
        0x01, 0xEF, 0x64, 0x2C, 0xEE, 0x9C, 0xE4, 0x68, 0x0C, 0x33, 0xFF, 0xB3, 0xFF, 0x39, 0xAF,
        0xEC, 0xFE, 0x70,
    ];

    // ed25519
    const PUBKEY_ED25519: [u8; PUBLIC_KEY_BUFFER_SIZE] = [
        0xED, 0xD9, 0xB3, 0x59, 0x98, 0x02, 0xB2, 0x14, 0xA9, 0x9D, 0x75, 0x77, 0x12, 0xD6, 0xAB,
        0xDF, 0x72, 0xF8, 0x3C, 0x63, 0xBB, 0xD5, 0x38, 0x61, 0x41, 0x17, 0x90, 0xB1, 0x3D, 0x04,
        0xB2, 0xC5, 0xC9,
    ];

    // uint8_t sig_ed[] =
    // {
    // 0x56,0x68,0x80,0x76,0x70,0xFE,0xCE,0x60,0x34,0xAF,
    // 0xD6,0xCD,0x1B,0xB4,0xC6,0x60,0xAE,0x08,0x39,0x6D,
    // 0x6D,0x8B,0x7D,0x22,0x71,0x3B,0xDA,0x26,0x43,0xC1,
    // 0xE1,0x91,0xC4,0xE4,0x4D,0x8E,0x02,0xE8,0x57,0x8B,
    // 0x20,0x45,0xDA,0xD4,0x8F,0x97,0xFC,0x16,0xF8,0x92,
    // 0x5B,0x6B,0x51,0xFB,0x3B,0xE5,0x0F,0xB0,0x4B,0x3A,
    // 0x20,0x4C,0x53,0x04U
    // };

    #[test]
    fn test_get_ref() {
        let pubkey_secp256k1_ref: &[u8] = PUBKEY_SECP256K1.as_slice();

        assert_eq!(pubkey_secp256k1_ref.len(), PUBLIC_KEY_BUFFER_SIZE);
        assert_eq!(pubkey_secp256k1_ref, PUBKEY_SECP256K1);
        assert_ne!(pubkey_secp256k1_ref, PUBKEY_ED25519);
    }
}
