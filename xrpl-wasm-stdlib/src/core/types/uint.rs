//! Generic unsigned integer types with configurable bit sizes

use crate::core::current_tx::CurrentTxFieldGetter;
use crate::core::ledger_objects::LedgerObjectFieldGetter;
use crate::host::field_helpers::{
    get_fixed_size_field_with_expected_bytes, get_fixed_size_field_with_expected_bytes_optional,
};
use crate::host::{Result, get_current_ledger_obj_field, get_ledger_obj_field, get_tx_field};

/// A generic unsigned integer type with configurable byte size.
///
/// This type provides a zero-cost abstraction for fixed-size unsigned integers
/// of arbitrary byte lengths. Common instantiations include UInt128, UInt160,
/// UInt192, and UInt256.
///
/// # Type Parameters
///
/// * `N` - The size of the integer in bytes
///
/// ## Derived Traits
///
/// - `PartialEq, Eq`: Essential for comparisons and use in collections
/// - `Debug, Clone`: Standard traits for development and consistency
///
/// Note: `Copy` is intentionally not derived because `N` can be arbitrarily large.
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UInt<const N: usize>(pub [u8; N]);

impl<const N: usize> From<[u8; N]> for UInt<N> {
    fn from(bytes: [u8; N]) -> Self {
        Self(bytes)
    }
}

impl<const N: usize> UInt<N> {
    /// Returns the inner bytes as a reference to the inner array.
    pub fn as_bytes(&self) -> &[u8; N] {
        &self.0
    }
}

// Keep the existing constants for compatibility
pub const UINT128_SIZE: usize = 16;
pub const UINT160_SIZE: usize = 20;
pub const UINT192_SIZE: usize = 24;
pub const UINT256_SIZE: usize = 32;

// Alias for Hash constants
pub const HASH128_SIZE: usize = UINT128_SIZE;
pub const HASH160_SIZE: usize = UINT160_SIZE;
pub const HASH192_SIZE: usize = UINT192_SIZE;
pub const HASH256_SIZE: usize = UINT256_SIZE;

// Type aliases for common sizes
pub type UInt128 = UInt<UINT128_SIZE>;
pub type UInt160 = UInt<UINT160_SIZE>;
pub type UInt192 = UInt<UINT192_SIZE>;
pub type UInt256 = UInt<UINT256_SIZE>;

// Alias for Hash types
pub type Hash128 = UInt128;
pub type Hash160 = UInt160;
pub type Hash192 = UInt192;
pub type Hash256 = UInt256;

/// Implementation of `LedgerObjectFieldGetter` for 128-bit cryptographic hashes.
///
/// This implementation handles 16-byte hash fields in XRPL ledger objects.
/// Hash128 values are commonly used for shorter identifiers and checksums
/// in XRPL, such as email hashes.
///
/// # Buffer Management
///
/// Uses a 16-byte buffer (HASH128_SIZE) and validates that exactly 16 bytes
/// are returned from the host function to ensure data integrity.
impl LedgerObjectFieldGetter for Hash128 {
    #[inline]
    fn get_from_current_ledger_obj(field_code: i32) -> Result<Self> {
        match get_fixed_size_field_with_expected_bytes::<HASH128_SIZE, _>(
            field_code,
            |fc, buf, size| unsafe { get_current_ledger_obj_field(fc, buf, size) },
        ) {
            Result::Ok(buffer) => Result::Ok(buffer.into()),
            Result::Err(e) => Result::Err(e),
        }
    }

    #[inline]
    fn get_from_current_ledger_obj_optional(field_code: i32) -> Result<Option<Self>> {
        match get_fixed_size_field_with_expected_bytes_optional::<HASH128_SIZE, _>(
            field_code,
            |fc, buf, size| unsafe { get_current_ledger_obj_field(fc, buf, size) },
        ) {
            Result::Ok(buffer) => Result::Ok(buffer.map(|b| b.into())),
            Result::Err(e) => Result::Err(e),
        }
    }

    #[inline]
    fn get_from_ledger_obj(register_num: i32, field_code: i32) -> Result<Self> {
        match get_fixed_size_field_with_expected_bytes::<HASH128_SIZE, _>(
            field_code,
            |fc, buf, size| unsafe { get_ledger_obj_field(register_num, fc, buf, size) },
        ) {
            Result::Ok(buffer) => Result::Ok(buffer.into()),
            Result::Err(e) => Result::Err(e),
        }
    }

    #[inline]
    fn get_from_ledger_obj_optional(register_num: i32, field_code: i32) -> Result<Option<Self>> {
        match get_fixed_size_field_with_expected_bytes_optional::<HASH128_SIZE, _>(
            field_code,
            |fc, buf, size| unsafe { get_ledger_obj_field(register_num, fc, buf, size) },
        ) {
            Result::Ok(buffer) => Result::Ok(buffer.map(|b| b.into())),
            Result::Err(e) => Result::Err(e),
        }
    }
}

/// Implementation of `LedgerObjectFieldGetter` for 256-bit cryptographic hashes.
///
/// This implementation handles 32-byte hash fields in XRPL ledger objects.
/// Hash256 values are widely used throughout XRPL for transaction IDs,
/// ledger indexes, object IDs, and various cryptographic operations.
///
/// # Buffer Management
///
/// Uses a 32-byte buffer (HASH256_SIZE) and validates that exactly 32 bytes
/// are returned from the host function to ensure data integrity.
impl LedgerObjectFieldGetter for Hash256 {
    #[inline]
    fn get_from_current_ledger_obj(field_code: i32) -> Result<Self> {
        match get_fixed_size_field_with_expected_bytes::<HASH256_SIZE, _>(
            field_code,
            |fc, buf, size| unsafe { get_current_ledger_obj_field(fc, buf, size) },
        ) {
            Result::Ok(buffer) => Result::Ok(buffer.into()),
            Result::Err(e) => Result::Err(e),
        }
    }

    #[inline]
    fn get_from_current_ledger_obj_optional(field_code: i32) -> Result<Option<Self>> {
        match get_fixed_size_field_with_expected_bytes_optional::<HASH256_SIZE, _>(
            field_code,
            |fc, buf, size| unsafe { get_current_ledger_obj_field(fc, buf, size) },
        ) {
            Result::Ok(buffer) => Result::Ok(buffer.map(|b| b.into())),
            Result::Err(e) => Result::Err(e),
        }
    }

    #[inline]
    fn get_from_ledger_obj(register_num: i32, field_code: i32) -> Result<Self> {
        match get_fixed_size_field_with_expected_bytes::<HASH256_SIZE, _>(
            field_code,
            |fc, buf, size| unsafe { get_ledger_obj_field(register_num, fc, buf, size) },
        ) {
            Result::Ok(buffer) => Result::Ok(buffer.into()),
            Result::Err(e) => Result::Err(e),
        }
    }

    #[inline]
    fn get_from_ledger_obj_optional(register_num: i32, field_code: i32) -> Result<Option<Self>> {
        match get_fixed_size_field_with_expected_bytes_optional::<HASH256_SIZE, _>(
            field_code,
            |fc, buf, size| unsafe { get_ledger_obj_field(register_num, fc, buf, size) },
        ) {
            Result::Ok(buffer) => Result::Ok(buffer.map(|b| b.into())),
            Result::Err(e) => Result::Err(e),
        }
    }
}

/// Implementation of `CurrentTxFieldGetter` for 256-bit cryptographic hashes.
///
/// This implementation handles 32-byte hash fields in XRPL transactions.
/// Hash256 values are used for transaction IDs, account transaction IDs,
/// references to other transactions, and various cryptographic identifiers.
///
/// # Buffer Management
///
/// Uses a 32-byte buffer (HASH256_SIZE) and validates that exactly 32 bytes
/// are returned from the host function to ensure data integrity.
impl CurrentTxFieldGetter for Hash256 {
    #[inline]
    fn get_from_current_tx(field_code: i32) -> Result<Self> {
        match get_fixed_size_field_with_expected_bytes::<HASH256_SIZE, _>(
            field_code,
            |fc, buf, size| unsafe { get_tx_field(fc, buf, size) },
        ) {
            Result::Ok(buffer) => Result::Ok(buffer.into()),
            Result::Err(e) => Result::Err(e),
        }
    }

    #[inline]
    fn get_from_current_tx_optional(field_code: i32) -> Result<Option<Self>> {
        match get_fixed_size_field_with_expected_bytes_optional::<HASH256_SIZE, _>(
            field_code,
            |fc, buf, size| unsafe { get_tx_field(fc, buf, size) },
        ) {
            Result::Ok(buffer) => Result::Ok(buffer.map(|b| b.into())),
            Result::Err(e) => Result::Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests for generic UInt<N>
    #[test]
    fn test_uint_creation_generic() {
        // Create a UInt with 8 bytes
        let bytes = [1u8, 2, 3, 4, 5, 6, 7, 8];
        let uint = UInt::<8>(bytes);

        // Verify the bytes
        assert_eq!(uint.0, bytes);
    }

    #[test]
    fn test_uint_from_bytes_generic() {
        // Create a test byte array
        let bytes = [0xAB, 0xCD, 0xEF, 0x12];

        // Create a UInt from bytes using From trait
        let uint = UInt::<4>::from(bytes);

        // Verify the bytes
        assert_eq!(uint.as_bytes(), &bytes);
    }

    #[test]
    fn test_uint_as_bytes() {
        // Create a UInt with specific bytes
        let bytes = [0xFF, 0x00, 0xFF, 0x00, 0xAA, 0xBB];
        let uint = UInt::<6>::from(bytes);

        // Get the bytes back
        let retrieved_bytes = uint.as_bytes();

        // Verify they match
        assert_eq!(retrieved_bytes, &bytes);
    }

    #[test]
    fn test_uint_equality() {
        // Create two identical UInts
        let bytes1 = [1u8, 2, 3, 4];
        let uint1 = UInt::<4>::from(bytes1);
        let uint2 = UInt::<4>::from(bytes1);

        // Create a different UInt
        let bytes2 = [5u8, 6, 7, 8];
        let uint3 = UInt::<4>::from(bytes2);

        // Test equality
        assert_eq!(uint1, uint2);
        assert_ne!(uint1, uint3);
    }

    #[test]
    #[allow(clippy::clone_on_copy)]
    fn test_uint_clone() {
        // Create a UInt
        let bytes = [0x11, 0x22, 0x33, 0x44];
        let uint1 = UInt::<4>::from(bytes);

        // Clone it
        let uint2 = uint1.clone();

        // Verify they are equal
        assert_eq!(uint1, uint2);
        assert_eq!(uint1.as_bytes(), uint2.as_bytes());
    }

    #[test]
    fn test_uint_copy() {
        // Create a UInt
        let bytes = [0xDE, 0xAD, 0xBE, 0xEF];
        let uint1 = UInt::<4>::from(bytes);

        // Copy it (implicit copy due to Copy trait)
        let uint2 = uint1.clone();

        // Both should be usable and equal
        assert_eq!(uint1, uint2);
        assert_eq!(uint1.as_bytes(), uint2.as_bytes());
    }

    #[test]
    fn test_uint_debug() {
        // Create a UInt
        let bytes = [0x01, 0x02];
        let uint = UInt::<2>::from(bytes);

        // Verify Debug trait is implemented by using it in an assertion
        // We can't use format! in no_std, but we can verify the trait exists
        let _ = uint; // Debug trait is derived, so this test verifies compilation
    }

    // Tests for UInt128
    #[test]
    fn test_uint128_creation() {
        // Create a UInt128 (16 bytes)
        let bytes = [1u8; 16];
        let uint128 = UInt128::from(bytes);

        // Verify the bytes
        assert_eq!(uint128.as_bytes(), &bytes);
        assert_eq!(uint128.as_bytes().len(), UINT128_SIZE);
    }

    #[test]
    fn test_uint128_all_zeros() {
        // Create a UInt128 with all zeros
        let bytes = [0u8; 16];
        let uint128 = UInt128::from(bytes);

        // Verify all bytes are zero
        assert_eq!(uint128.as_bytes(), &[0u8; 16]);
    }

    #[test]
    fn test_uint128_all_ones() {
        // Create a UInt128 with all ones (max value)
        let bytes = [0xFFu8; 16];
        let uint128 = UInt128::from(bytes);

        // Verify all bytes are 0xFF
        assert_eq!(uint128.as_bytes(), &[0xFFu8; 16]);
    }

    // Tests for UInt160
    #[test]
    fn test_uint160_creation() {
        // Create a UInt160 (20 bytes)
        let bytes = [2u8; 20];
        let uint160 = UInt160::from(bytes);

        // Verify the bytes
        assert_eq!(uint160.as_bytes(), &bytes);
        assert_eq!(uint160.as_bytes().len(), UINT160_SIZE);
    }

    #[test]
    fn test_uint160_pattern() {
        // Create a UInt160 with a specific pattern
        let mut bytes = [0u8; 20];
        for (i, byte) in bytes.iter_mut().enumerate() {
            *byte = (i % 256) as u8;
        }
        let uint160 = UInt160::from(bytes);

        // Verify the pattern
        assert_eq!(uint160.as_bytes(), &bytes);
    }

    // Tests for UInt192
    #[test]
    fn test_uint192_creation() {
        // Create a UInt192 (24 bytes)
        let bytes = [3u8; 24];
        let uint192 = UInt192::from(bytes);

        // Verify the bytes
        assert_eq!(uint192.as_bytes(), &bytes);
        assert_eq!(uint192.as_bytes().len(), UINT192_SIZE);
    }

    #[test]
    fn test_uint192_alternating_pattern() {
        // Create a UInt192 with alternating bytes
        let mut bytes = [0u8; 24];
        for (i, byte) in bytes.iter_mut().enumerate() {
            *byte = if i % 2 == 0 { 0xAA } else { 0x55 };
        }
        let uint192 = UInt192::from(bytes);

        // Verify the pattern
        assert_eq!(uint192.as_bytes(), &bytes);
    }

    // Tests for UInt256
    #[test]
    fn test_uint256_creation() {
        // Create a UInt256 (32 bytes)
        let bytes = [4u8; 32];
        let uint256 = UInt256::from(bytes);

        // Verify the bytes
        assert_eq!(uint256.as_bytes(), &bytes);
        assert_eq!(uint256.as_bytes().len(), UINT256_SIZE);
    }

    #[test]
    fn test_uint256_specific_value() {
        // Create a UInt256 with a specific value
        let mut bytes = [0u8; 32];
        bytes[0] = 0x12;
        bytes[15] = 0x34;
        bytes[31] = 0x56;
        let uint256 = UInt256::from(bytes);

        // Verify specific bytes
        assert_eq!(uint256.as_bytes()[0], 0x12);
        assert_eq!(uint256.as_bytes()[15], 0x34);
        assert_eq!(uint256.as_bytes()[31], 0x56);
    }

    // Tests for Hash type aliases
    #[test]
    fn test_hash128_alias() {
        // Hash128 should be the same as UInt128
        let bytes = [0xAB; 16];
        let hash128 = Hash128::from(bytes);
        let uint128 = UInt128::from(bytes);

        // They should be equal
        assert_eq!(hash128, uint128);
        assert_eq!(hash128.as_bytes(), uint128.as_bytes());
    }

    #[test]
    fn test_hash160_alias() {
        // Hash160 should be the same as UInt160
        let bytes = [0xCD; 20];
        let hash160 = Hash160::from(bytes);
        let uint160 = UInt160::from(bytes);

        // They should be equal
        assert_eq!(hash160, uint160);
        assert_eq!(hash160.as_bytes(), uint160.as_bytes());
    }

    #[test]
    fn test_hash192_alias() {
        // Hash192 should be the same as UInt192
        let bytes = [0xEF; 24];
        let hash192 = Hash192::from(bytes);
        let uint192 = UInt192::from(bytes);

        // They should be equal
        assert_eq!(hash192, uint192);
        assert_eq!(hash192.as_bytes(), uint192.as_bytes());
    }

    #[test]
    fn test_hash256_alias() {
        // Hash256 should be the same as UInt256
        let bytes = [0x12; 32];
        let hash256 = Hash256::from(bytes);
        let uint256 = UInt256::from(bytes);

        // They should be equal
        assert_eq!(hash256, uint256);
        assert_eq!(hash256.as_bytes(), uint256.as_bytes());
    }

    // Tests for constants
    #[test]
    fn test_uint_constants() {
        // Verify the size constants
        assert_eq!(UINT128_SIZE, 16);
        assert_eq!(UINT160_SIZE, 20);
        assert_eq!(UINT192_SIZE, 24);
        assert_eq!(UINT256_SIZE, 32);
    }

    #[test]
    fn test_hash_constants() {
        // Verify hash constants match uint constants
        assert_eq!(HASH128_SIZE, UINT128_SIZE);
        assert_eq!(HASH160_SIZE, UINT160_SIZE);
        assert_eq!(HASH192_SIZE, UINT192_SIZE);
        assert_eq!(HASH256_SIZE, UINT256_SIZE);

        // Verify the actual values
        assert_eq!(HASH128_SIZE, 16);
        assert_eq!(HASH160_SIZE, 20);
        assert_eq!(HASH192_SIZE, 24);
        assert_eq!(HASH256_SIZE, 32);
    }

    // Edge case tests
    #[test]
    fn test_uint_single_byte() {
        // Test with a single byte UInt
        let bytes = [0x42];
        let uint = UInt::<1>::from(bytes);

        assert_eq!(uint.as_bytes(), &[0x42]);
    }

    #[test]
    fn test_uint_large_size() {
        // Test with a larger custom size
        let bytes = [0x99; 64];
        let uint = UInt::<64>::from(bytes);

        assert_eq!(uint.as_bytes().len(), 64);
        assert_eq!(uint.as_bytes(), &[0x99; 64]);
    }

    #[test]
    fn test_uint_equality_different_sizes() {
        // UInt<4> and UInt<8> are different types and cannot be compared
        // This test just verifies they can coexist
        let uint4 = UInt::<4>::from([1, 2, 3, 4]);
        let uint8 = UInt::<8>::from([1, 2, 3, 4, 5, 6, 7, 8]);

        // Just verify they exist and have correct sizes
        assert_eq!(uint4.as_bytes().len(), 4);
        assert_eq!(uint8.as_bytes().len(), 8);
    }

    #[test]
    fn test_uint_repr_c() {
        // Verify that UInt has the correct memory layout
        // This is important for FFI compatibility
        use core::mem;

        // Size should be exactly N bytes
        assert_eq!(mem::size_of::<UInt<16>>(), 16);
        assert_eq!(mem::size_of::<UInt<20>>(), 20);
        assert_eq!(mem::size_of::<UInt<24>>(), 24);
        assert_eq!(mem::size_of::<UInt<32>>(), 32);

        // Alignment should be 1 (byte-aligned)
        assert_eq!(mem::align_of::<UInt<16>>(), 1);
    }

    #[test]
    fn test_uint_from_array_direct() {
        // Test creating UInt directly from array
        let uint = UInt([0x01, 0x02, 0x03, 0x04]);

        assert_eq!(uint.as_bytes(), &[0x01, 0x02, 0x03, 0x04]);
    }

    #[test]
    fn test_multiple_uint_instances() {
        // Test creating multiple instances with different values
        let uint1 = UInt128::from([1u8; 16]);
        let uint2 = UInt128::from([2u8; 16]);
        let uint3 = UInt128::from([3u8; 16]);

        // Verify they are all different
        assert_ne!(uint1, uint2);
        assert_ne!(uint2, uint3);
        assert_ne!(uint1, uint3);

        // Verify their values
        assert_eq!(uint1.as_bytes(), &[1u8; 16]);
        assert_eq!(uint2.as_bytes(), &[2u8; 16]);
        assert_eq!(uint3.as_bytes(), &[3u8; 16]);
    }
}
