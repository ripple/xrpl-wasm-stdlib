use crate::core::current_tx::CurrentTxFieldGetter;
use crate::core::ledger_objects::LedgerObjectFieldGetter;
use crate::host::field_helpers::{get_variable_size_field, get_variable_size_field_optional};
use crate::host::{Result, get_current_ledger_obj_field, get_ledger_obj_field, get_tx_field};

/// Default blob size for general use (memos, etc.)
pub const DEFAULT_BLOB_SIZE: usize = 1024;

// Declared here because there is no Memo struct.
pub const MEMO_BLOB_SIZE: usize = DEFAULT_BLOB_SIZE;
pub const DOMAIN_BLOB_SIZE: usize = 256;

/// The maximum number of bytes in a Condition. Xrpld currently caps this value at 128 bytes
/// (see `maxSerializedCondition` in xrpld source code), so we do the same here.
pub const CONDITION_BLOB_SIZE: usize = 128;

/// The maximum number of bytes in a Fulfillment. Theoretically, the crypto-condition format allows for much larger
/// fulfillments, but xrpld currently caps this value at 256 bytes (see `maxSerializedFulfillment` in xrpld source
/// code), so we do the same here.
pub const FULFILLMENT_BLOB_SIZE: usize = 256;

/// Maximum size of a signature in bytes.
///
/// ECDSA signatures can be up to 72 bytes, which is the maximum signature size in XRPL.
/// EdDSA signatures are always 64 bytes.
pub const SIGNATURE_BLOB_SIZE: usize = 72;

/// Maximum size of a URI in bytes (applies to DIDs, Oracles, Credentials, NFTs, etc.)
pub const URI_BLOB_SIZE: usize = 256;

/// A variable-length binary data container with a fixed maximum size.
///
/// The `Blob` type is generic over its maximum capacity `N`, allowing you to
/// create blobs of different sizes for different use cases. The actual data
/// length is tracked separately in the `len` field.
///
/// # Type Parameters
///
/// * `N` - The maximum capacity of the blob in bytes
///
/// # Examples
///
/// ```
/// use xrpl_wasm_stdlib::core::types::blob::{Blob, StandardBlob, UriBlob, DEFAULT_BLOB_SIZE};
///
/// // Create a standard 1024-byte blob
/// let standard_blob: Blob<DEFAULT_BLOB_SIZE> = Blob::new();
///
/// // Create a standard 1024-byte blob
/// let standard_blob_typed: StandardBlob = StandardBlob::new();
///
/// // Create a smaller 256-byte blob for URIs
/// let uri_blob: UriBlob = UriBlob::new();
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub struct Blob<const N: usize> {
    pub data: [u8; N],

    /// The actual length of this blob, if less than data.len()
    pub len: usize,
}

impl<const N: usize> Blob<N> {
    /// Creates a new empty blob with the specified capacity.
    #[inline]
    pub const fn new() -> Self {
        Self {
            data: [0u8; N],
            len: 0,
        }
    }

    /// Creates a blob from a byte slice, copying up to N bytes.
    #[inline]
    pub fn from_slice(slice: &[u8]) -> Self {
        let mut data = [0u8; N];
        let len = slice.len().min(N);
        data[..len].copy_from_slice(&slice[..len]);
        Self { data, len }
    }

    /// Returns the actual length of the data in the blob.
    #[inline]
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Returns true if the blob contains no data.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the maximum capacity of the blob.
    #[inline]
    pub const fn capacity(&self) -> usize {
        N
    }

    /// Returns a slice of the actual data (not including unused capacity).
    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        &self.data[..self.len]
    }
}

impl<const N: usize> From<[u8; N]> for Blob<N> {
    fn from(bytes: [u8; N]) -> Self {
        Self {
            data: bytes,
            len: N,
        }
    }
}

impl<const N: usize> Default for Blob<N> {
    fn default() -> Self {
        Self::new()
    }
}

/// Type alias for the standard 1024-byte blob.
pub type StandardBlob = Blob<DEFAULT_BLOB_SIZE>;

/// Type alias for 128-byte blob (for Condition fields)
pub type ConditionBlob = Blob<CONDITION_BLOB_SIZE>;

/// Type alias for 256-byte blob (for Fulfillment fields)
pub type FulfillmentBlob = Blob<FULFILLMENT_BLOB_SIZE>;

/// Type alias for 1024-byte blob (for Memo fields).
pub type MemoBlob = Blob<MEMO_BLOB_SIZE>;

/// Type alias for 72-byte blob (for Signature fields).
pub type SignatureBlob = Blob<SIGNATURE_BLOB_SIZE>;

/// Type alias for 256-byte blob (applies to DIDs, Oracles, Credentials, NFTs, etc.)
pub type UriBlob = Blob<URI_BLOB_SIZE>;

pub type EmptyBlob = Blob<0>;

/// Empty blob constant.
pub const EMPTY_BLOB: EmptyBlob = Blob {
    data: [0u8; 0],
    len: 0usize,
};

/// Implementation of `LedgerObjectFieldGetter` for variable-length binary data.
///
/// This implementation handles blob fields in XRPL ledger objects, which can contain
/// arbitrary binary data such as memos, signatures, public keys, and other
/// variable-length content.
///
/// # Buffer Management
///
/// Uses a buffer of size `N` to accommodate blob field data. The actual
/// length of the data is determined by the return value from the host function
/// and stored in the Blob's `len` field. No strict byte count validation is
/// performed since blobs can vary significantly in size.
///
/// # Type Parameters
///
/// * `N` - The maximum capacity of the blob buffer in bytes
impl<const N: usize> LedgerObjectFieldGetter for Blob<N> {
    #[inline]
    fn get_from_current_ledger_obj(field_code: i32) -> Result<Self> {
        match get_variable_size_field::<N, _>(field_code, |fc, buf, size| unsafe {
            get_current_ledger_obj_field(fc, buf, size)
        }) {
            Result::Ok((data, len)) => Result::Ok(Blob { data, len }),
            Result::Err(e) => Result::Err(e),
        }
    }

    #[inline]
    fn get_from_current_ledger_obj_optional(field_code: i32) -> Result<Option<Self>> {
        match get_variable_size_field_optional::<N, _>(field_code, |fc, buf, size| unsafe {
            get_current_ledger_obj_field(fc, buf, size)
        }) {
            Result::Ok(opt) => Result::Ok(opt.map(|(data, len)| Blob { data, len })),
            Result::Err(e) => Result::Err(e),
        }
    }

    #[inline]
    fn get_from_ledger_obj(register_num: i32, field_code: i32) -> Result<Self> {
        match get_variable_size_field::<N, _>(field_code, |fc, buf, size| unsafe {
            get_ledger_obj_field(register_num, fc, buf, size)
        }) {
            Result::Ok((data, len)) => Result::Ok(Blob { data, len }),
            Result::Err(e) => Result::Err(e),
        }
    }

    #[inline]
    fn get_from_ledger_obj_optional(register_num: i32, field_code: i32) -> Result<Option<Self>> {
        match get_variable_size_field_optional::<N, _>(field_code, |fc, buf, size| unsafe {
            get_ledger_obj_field(register_num, fc, buf, size)
        }) {
            Result::Ok(opt) => Result::Ok(opt.map(|(data, len)| Blob { data, len })),
            Result::Err(e) => Result::Err(e),
        }
    }
}

/// Implementation of `CurrentTxFieldGetter` for variable-length binary data.
///
/// This implementation handles blob fields in XRPL transactions, which can contain
/// arbitrary binary data such as transaction signatures, memos, fulfillment data,
/// and other variable-length content that doesn't fit into fixed-size types.
///
/// # Buffer Management
///
/// Uses a buffer of size `N` to accommodate blob field data. The actual
/// length of the data is determined by the return value from the host function
/// and stored in the Blob's `len` field. No strict byte count validation is
/// performed since blobs can vary significantly in size.
///
/// # Type Parameters
///
/// * `N` - The maximum capacity of the blob buffer in bytes
impl<const N: usize> CurrentTxFieldGetter for Blob<N> {
    #[inline]
    fn get_from_current_tx(field_code: i32) -> Result<Self> {
        match get_variable_size_field::<N, _>(field_code, |fc, buf, size| unsafe {
            get_tx_field(fc, buf, size)
        }) {
            Result::Ok((data, len)) => Result::Ok(Blob { data, len }),
            Result::Err(e) => Result::Err(e),
        }
    }

    #[inline]
    fn get_from_current_tx_optional(field_code: i32) -> Result<Option<Self>> {
        match get_variable_size_field_optional::<N, _>(field_code, |fc, buf, size| unsafe {
            get_tx_field(fc, buf, size)
        }) {
            Result::Ok(opt) => Result::Ok(opt.map(|(data, len)| Blob { data, len })),
            Result::Err(e) => Result::Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_creates_empty_blob() {
        let blob: Blob<32> = Blob::new();
        assert_eq!(blob.len(), 0);
        assert!(blob.is_empty());
        assert_eq!(blob.capacity(), 32);
        assert_eq!(blob.as_slice(), &[]);
    }

    #[test]
    fn test_from_slice_with_exact_capacity() {
        let data = [1, 2, 3, 4, 5];
        let blob: Blob<5> = Blob::from_slice(&data);

        assert_eq!(blob.len(), 5);
        assert!(!blob.is_empty());
        assert_eq!(blob.as_slice(), &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_from_slice_with_excess_capacity() {
        let data = [1, 2, 3];
        let blob: Blob<10> = Blob::from_slice(&data);

        assert_eq!(blob.len(), 3);
        assert_eq!(blob.capacity(), 10);
        assert_eq!(blob.as_slice(), &[1, 2, 3]);
        // Verify unused capacity is zeroed
        assert_eq!(blob.data[3], 0);
        assert_eq!(blob.data[9], 0);
    }

    #[test]
    fn test_from_slice_truncates_when_too_large() {
        let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let blob: Blob<5> = Blob::from_slice(&data);

        // Should only copy first 5 bytes
        assert_eq!(blob.len(), 5);
        assert_eq!(blob.as_slice(), &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_from_slice_with_empty_slice() {
        let data: &[u8] = &[];
        let blob: Blob<10> = Blob::from_slice(data);

        assert_eq!(blob.len(), 0);
        assert!(blob.is_empty());
        assert_eq!(blob.as_slice(), &[]);
    }

    #[test]
    fn test_from_array_sets_full_length() {
        let data = [42u8; 8];
        let blob: Blob<8> = Blob::from(data);

        assert_eq!(blob.len(), 8);
        assert_eq!(blob.capacity(), 8);
        assert_eq!(blob.as_slice(), &[42, 42, 42, 42, 42, 42, 42, 42]);
    }

    #[test]
    fn test_default_creates_empty_blob() {
        let blob: Blob<16> = Blob::default();

        assert_eq!(blob.len(), 0);
        assert!(blob.is_empty());
        assert_eq!(blob.capacity(), 16);
    }

    #[test]
    fn test_as_slice_respects_length() {
        let mut blob: Blob<10> = Blob::new();
        blob.data[0] = 1;
        blob.data[1] = 2;
        blob.data[2] = 3;
        blob.len = 2; // Only first 2 bytes are "valid"

        assert_eq!(blob.as_slice(), &[1, 2]);
        assert_ne!(blob.as_slice(), &[1, 2, 3]);
    }

    #[test]
    fn test_equality_compares_full_struct() {
        let blob1: Blob<5> = Blob::from_slice(&[1, 2, 3]);
        let blob2: Blob<5> = Blob::from_slice(&[1, 2, 3]);
        let blob3: Blob<5> = Blob::from_slice(&[1, 2, 3, 4]);

        assert_eq!(blob1, blob2);
        assert_ne!(blob1, blob3);
    }

    #[test]
    fn test_clone_creates_independent_copy() {
        let blob1: Blob<5> = Blob::from_slice(&[1, 2, 3]);
        let mut blob2 = blob1;

        // Modify blob2
        blob2.data[0] = 99;

        // blob1 should be unchanged (Copy trait means independent copy)
        assert_eq!(blob1.data[0], 1);
        assert_eq!(blob2.data[0], 99);
    }

    #[test]
    fn test_standard_blob_type_alias() {
        let blob: StandardBlob = StandardBlob::new();
        assert_eq!(blob.capacity(), DEFAULT_BLOB_SIZE);
        assert_eq!(blob.capacity(), 1024);
    }

    #[test]
    fn test_condition_blob_type_alias() {
        let blob: ConditionBlob = ConditionBlob::new();
        assert_eq!(blob.capacity(), CONDITION_BLOB_SIZE);
        assert_eq!(blob.capacity(), 128);
    }

    #[test]
    fn test_fulfillment_blob_type_alias() {
        let blob: FulfillmentBlob = FulfillmentBlob::new();
        assert_eq!(blob.capacity(), FULFILLMENT_BLOB_SIZE);
        assert_eq!(blob.capacity(), 256);
    }

    #[test]
    fn test_signature_blob_type_alias() {
        let blob: SignatureBlob = SignatureBlob::new();
        assert_eq!(blob.capacity(), SIGNATURE_BLOB_SIZE);
        assert_eq!(blob.capacity(), 72);
    }

    #[test]
    fn test_uri_blob_type_alias() {
        let blob: UriBlob = UriBlob::new();
        assert_eq!(blob.capacity(), URI_BLOB_SIZE);
        assert_eq!(blob.capacity(), 256);
    }

    #[test]
    fn test_empty_blob_constant() {
        assert_eq!(EMPTY_BLOB.len(), 0);
        assert_eq!(EMPTY_BLOB.capacity(), 0);
        assert!(EMPTY_BLOB.is_empty());
        assert_eq!(EMPTY_BLOB.as_slice(), &[]);
    }

    #[test]
    fn test_zero_capacity_blob() {
        let blob: Blob<0> = Blob::new();
        assert_eq!(blob.capacity(), 0);
        assert_eq!(blob.len(), 0);
        assert!(blob.is_empty());
    }

    #[test]
    fn test_from_slice_with_zero_capacity_truncates_all() {
        let data = [1, 2, 3];
        let blob: Blob<0> = Blob::from_slice(&data);

        assert_eq!(blob.len(), 0);
        assert_eq!(blob.as_slice(), &[]);
    }

    #[test]
    fn test_legacy_signature_blob_size() {
        // This test verifies that a 32-byte blob works (the old SIGNATURE_BLOB_SIZE value)
        // Note: The actual signature type now uses 72 bytes (see signature module)
        let blob: Blob<32> = Blob::new();
        assert_eq!(blob.capacity(), 32);
    }

    #[test]
    fn test_large_blob_from_slice() {
        let data = [42u8; 2048];
        let blob: Blob<1024> = Blob::from_slice(&data);

        // Should truncate to capacity
        assert_eq!(blob.len(), 1024);
        assert_eq!(blob.as_slice().len(), 1024);
        assert!(blob.as_slice().iter().all(|&b| b == 42));
    }

    #[test]
    fn test_blob_with_binary_data() {
        let data = [0xFF, 0x00, 0xAB, 0xCD, 0xEF];
        let blob: Blob<10> = Blob::from_slice(&data);

        assert_eq!(blob.len(), 5);
        assert_eq!(blob.as_slice(), &[0xFF, 0x00, 0xAB, 0xCD, 0xEF]);
    }

    #[test]
    fn test_capacity_is_const() {
        let blob1: Blob<10> = Blob::new();
        let blob2: Blob<10> = Blob::from_slice(&[1, 2, 3, 4, 5]);

        // Capacity should always be N regardless of actual data
        assert_eq!(blob1.capacity(), 10);
        assert_eq!(blob2.capacity(), 10);
    }
}
