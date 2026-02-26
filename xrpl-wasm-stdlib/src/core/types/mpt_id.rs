use crate::core::types::account_id::AccountID;

pub const MPT_ID_SIZE: usize = 24;
pub const MPT_SEQUENCE_NUM_SIZE: usize = 4;

/// A 24-byte Multi-Purpose Token (MPT) identifier on the XRP Ledger.
///
/// An MPT ID uniquely identifies a multi-purpose token and consists of:
/// - **Bytes 0-3**: Sequence number (32 bits, big-endian)
/// - **Bytes 4-23**: Issuer account ID (160 bits)
///
/// ## Derived Traits
///
/// - `Copy`: Efficient for this 24-byte struct, enabling implicit copying
/// - `PartialEq, Eq`: Enable comparisons and use in hash-based collections
/// - `Debug, Clone`: Standard traits for development and consistency
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct MptId([u8; MPT_ID_SIZE]);

impl MptId {
    /// Creates a new MptId from a sequence number and an issuer account ID.
    pub fn new(sequence_num: u32, issuer: AccountID) -> Self {
        let mut bytes = [0u8; MPT_ID_SIZE];

        // Set the sequence number (first 4 bytes)
        bytes[0..4].copy_from_slice(&sequence_num.to_be_bytes());

        // Set the issuer account ID (last 20 bytes)
        bytes[4..MPT_ID_SIZE].copy_from_slice(&issuer.0);

        MptId(bytes)
    }

    /// Gets the sequence number part of the MptId.
    pub fn get_sequence_num(&self) -> u32 {
        // Transform the first 4 bytes of self.0 into a u32.
        u32::from_be_bytes([self.0[0], self.0[1], self.0[2], self.0[3]])
    }

    /// Gets the issuer account ID part of the MptId.
    pub fn get_issuer(&self) -> AccountID {
        // Transform the last 20 bytes of self.0 into an AccountID.
        let mut account_bytes = [0u8; 20]; // AccountID is 20 bytes
        account_bytes.copy_from_slice(&self.0[4..24]); // Extract bytes 4-23 (20 bytes total)
        AccountID::from(account_bytes)
    }

    /// Gets the raw bytes of the MptId.
    pub fn as_bytes(&self) -> &[u8; 24] {
        &self.0
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.0.as_ptr()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.iter().all(|&byte| byte == 0)
    }
}

impl From<[u8; 24]> for MptId {
    fn from(value: [u8; 24]) -> Self {
        MptId(value)
    }
}

impl From<(u32, AccountID)> for MptId {
    fn from(value: (u32, AccountID)) -> Self {
        MptId::new(value.0, value.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mpt_id_creation() {
        // Create a test account ID
        let account_bytes = [1u8; 20];
        let account_id = AccountID::from(account_bytes);

        // Create an MptId using the constructor
        let sequence_num = 12345u32;
        let mpt_id = MptId::new(sequence_num, account_id);

        // Verify the sequence number and issuer
        assert_eq!(mpt_id.get_sequence_num(), sequence_num);
        assert_eq!(mpt_id.get_issuer(), account_id);
    }

    #[test]
    fn test_mpt_id_byte_layout() {
        // Test that sequence number is stored as big-endian in first 4 bytes
        let sequence_num = 0x12345678u32;
        let account_id = AccountID::from([0xAA; 20]);
        let mpt_id = MptId::new(sequence_num, account_id);

        let bytes = mpt_id.as_bytes();
        // Sequence number should be big-endian
        assert_eq!(&bytes[0..4], &[0x12, 0x34, 0x56, 0x78]);
        // Issuer should follow
        assert_eq!(&bytes[4..24], &[0xAA; 20]);
    }

    #[test]
    fn test_mpt_id_from_bytes() {
        // Create a test byte array
        let mut bytes = [0u8; 24];
        // Set sequence number bytes (first 4 bytes, big-endian)
        bytes[0..4].copy_from_slice(&67890u32.to_be_bytes());
        // Set account ID bytes (last 20 bytes)
        for byte in bytes.iter_mut().skip(4).take(20) {
            *byte = 2;
        }

        // Create an MptId from bytes
        let mpt_id = MptId::from(bytes);

        // Verify the sequence number and issuer
        assert_eq!(mpt_id.get_sequence_num(), 67890u32);
        assert_eq!(mpt_id.get_issuer(), AccountID::from([2u8; 20]));
    }

    #[test]
    fn test_mpt_id_from_tuple() {
        // Create a test account ID
        let account_bytes = [3u8; 20];
        let account_id = AccountID::from(account_bytes);

        // Create an MptId from a tuple
        let sequence_num = 54321u32;
        let mpt_id = MptId::from((sequence_num, account_id));

        // Verify the sequence number and issuer
        assert_eq!(mpt_id.get_sequence_num(), sequence_num);
        assert_eq!(mpt_id.get_issuer(), account_id);
    }

    #[test]
    fn test_mpt_id_is_empty_when_all_zeros() {
        let mpt_id = MptId::from([0u8; 24]);
        assert!(mpt_id.is_empty());
    }

    #[test]
    fn test_mpt_id_is_not_empty_when_has_data() {
        // Any non-zero byte means not empty
        let mut bytes = [0u8; 24];
        bytes[0] = 1; // Set sequence number to 1
        let mpt_id = MptId::from(bytes);
        assert!(!mpt_id.is_empty());

        // Also test with non-zero issuer
        let mut bytes2 = [0u8; 24];
        bytes2[10] = 0xFF; // Non-zero byte in issuer portion
        let mpt_id2 = MptId::from(bytes2);
        assert!(!mpt_id2.is_empty());
    }

    #[test]
    fn test_mpt_id_sequence_num_zero() {
        // Edge case: sequence number 0
        let account_id = AccountID::from([0xBB; 20]);
        let mpt_id = MptId::new(0, account_id);

        assert_eq!(mpt_id.get_sequence_num(), 0);
        assert_eq!(&mpt_id.as_bytes()[0..4], &[0, 0, 0, 0]);
    }

    #[test]
    fn test_mpt_id_sequence_num_max() {
        // Edge case: max u32 sequence number
        let account_id = AccountID::from([0xCC; 20]);
        let mpt_id = MptId::new(u32::MAX, account_id);

        assert_eq!(mpt_id.get_sequence_num(), u32::MAX);
        assert_eq!(&mpt_id.as_bytes()[0..4], &[0xFF, 0xFF, 0xFF, 0xFF]);
    }
}
