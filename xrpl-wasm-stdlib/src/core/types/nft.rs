//! NFToken (Non-Fungible Token) type for XRPL.
//!
//! Provides a high-level interface for working with NFTokens on the XRP Ledger.
//!
//! ## NFTokenID Structure
//!
//! An NFTokenID is a 32-byte identifier with the following structure:
//!
//! ```text
//! 000B 0539 C35B55AA096BA6D87A6E6C965A6534150DC56E5E 12C5D09E 0000000C
//! +--- +--- +--------------------------------------- +------- +-------
//! |    |    |                                        |        |
//! |    |    |                                        |        └─> Sequence (32 bits)
//! |    |    |                                        └─> Scrambled Taxon (32 bits)
//! |    |    └─> Issuer Address (160 bits / 20 bytes)
//! |    └─> Transfer Fee (16 bits)
//! └─> Flags (16 bits)
//! ```

use crate::core::types::account_id::{ACCOUNT_ID_SIZE, AccountID};
use crate::core::types::blob::{URI_BLOB_SIZE, UriBlob};
use crate::host;
use crate::host::{Error, Result};

/// Size of an NFTokenID in bytes (256 bits)
pub const NFT_ID_SIZE: usize = 32;

/// NFToken flags - see [NFToken documentation](https://xrpl.org/docs/references/protocol/data-types/nftoken)
pub mod flags {
    /// The issuer (or an entity authorized by the issuer) may destroy the object.
    /// If this flag is set, the object may be burned by the issuer even if the issuer
    /// does not currently hold the object. The object's owner can always burn it.
    pub const BURNABLE: u16 = 0x0001;

    /// If set, indicates that the minted token may only be bought or sold for XRP.
    /// This can be useful for compliance purposes if the issuer wants to avoid
    /// other tokens.
    pub const ONLY_XRP: u16 = 0x0002;

    /// If set, automatically create trust lines to hold transfer fees as specified
    /// in the TransferFee field.
    pub const TRUST_LINE: u16 = 0x0004;

    /// If set, indicates that the minted token may be transferred to others.
    /// If not set, the token can only be transferred back to the issuer.
    pub const TRANSFERABLE: u16 = 0x0008;
}

/// A wrapper around NFToken flags that provides efficient helper methods.
///
/// ## Derived Traits
///
/// - `Copy`: Efficient for this 2-byte struct, enabling implicit copying
/// - `PartialEq, Eq`: Enable comparisons
/// - `Debug, Clone`: Standard traits for development and consistency
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NftFlags(u16);

impl NftFlags {
    /// Creates a new NftFlags from a raw flags value.
    #[inline]
    pub const fn new(flags: u16) -> Self {
        NftFlags(flags)
    }

    /// Returns the raw flags value.
    #[inline]
    pub const fn as_u16(&self) -> u16 {
        self.0
    }

    /// Checks if the NFToken has the `BURNABLE` flag set.
    ///
    /// If this flag is set, the issuer (or an entity authorized by the issuer)
    /// may destroy the token even if they don't currently hold it.
    #[inline]
    pub const fn is_burnable(&self) -> bool {
        self.0 & flags::BURNABLE != 0
    }

    /// Checks if the NFToken has the `ONLY_XRP` flag set.
    ///
    /// If this flag is set, the token may only be bought or sold for XRP.
    #[inline]
    pub const fn is_only_xrp(&self) -> bool {
        self.0 & flags::ONLY_XRP != 0
    }

    /// Checks if the NFToken has the `TRUST_LINE` flag set.
    ///
    /// If this flag is set, trust lines are automatically created to hold
    /// transfer fees.
    #[inline]
    pub const fn is_trust_line(&self) -> bool {
        self.0 & flags::TRUST_LINE != 0
    }

    /// Checks if the NFToken has the `TRANSFERABLE` flag set.
    ///
    /// If this flag is set, the token may be transferred to others.
    /// If not set, the token can only be transferred back to the issuer.
    #[inline]
    pub const fn is_transferable(&self) -> bool {
        self.0 & flags::TRANSFERABLE != 0
    }
}

impl From<u16> for NftFlags {
    fn from(value: u16) -> Self {
        NftFlags(value)
    }
}

impl From<NftFlags> for u16 {
    fn from(value: NftFlags) -> Self {
        value.0
    }
}

/// Represents an NFToken (Non-Fungible Token) on the XRP Ledger.
///
/// The `NFToken` type wraps a 32-byte NFTokenID and provides methods to extract
/// all fields encoded within the identifier, as well as retrieve associated
/// metadata like the NFT's URI.
///
/// # NFTokenID Encoding
///
/// The 32-byte identifier contains:
/// - **Bytes 0-1**: Flags (16 bits, big-endian)
/// - **Bytes 2-3**: Transfer fee (16 bits, big-endian, in 1/100,000 units)
/// - **Bytes 4-23**: Issuer account address (160 bits)
/// - **Bytes 24-27**: Scrambled taxon (32 bits, big-endian)
/// - **Bytes 28-31**: Sequence number (32 bits, big-endian)
///
/// ## Derived Traits
///
/// - `Copy`: Efficient for this 32-byte struct, enabling implicit copying
/// - `PartialEq, Eq`: Enable comparisons and use in collections
/// - `Debug, Clone`: Standard traits for development and consistency
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct NFToken(pub [u8; NFT_ID_SIZE]);

impl NFToken {
    /// Creates a new NFToken from a 32-byte identifier.
    ///
    /// # Arguments
    ///
    /// * `id` - The 32-byte NFTokenID
    ///
    #[inline]
    pub const fn new(id: [u8; NFT_ID_SIZE]) -> Self {
        NFToken(id)
    }

    /// Returns the raw NFTokenID as a byte array.
    ///
    #[inline]
    pub const fn as_bytes(&self) -> &[u8; NFT_ID_SIZE] {
        &self.0
    }

    /// Returns a pointer to the NFTokenID data.
    ///
    /// This is primarily used internally for FFI calls to host functions.
    #[inline]
    pub const fn as_ptr(&self) -> *const u8 {
        self.0.as_ptr()
    }

    /// Returns the length of the NFTokenID (always 32 bytes).
    #[inline]
    #[allow(clippy::len_without_is_empty)]
    pub const fn len(&self) -> usize {
        NFT_ID_SIZE
    }

    /// Retrieves the flags associated with this NFToken.
    ///
    /// Flags are stored in the first 2 bytes of the NFTokenID (big-endian).
    ///
    /// # Returns
    ///
    /// * `Ok(NftFlags)` - A flags wrapper with helper methods
    /// * `Err(Error)` - If the host function fails
    ///
    pub fn flags(&self) -> Result<NftFlags> {
        let result = unsafe { host::get_nft_flags(self.as_ptr(), self.len()) };

        match result {
            code if code >= 0 => Result::Ok(NftFlags::new(code as u16)),
            code => Result::Err(Error::from_code(code)),
        }
    }

    /// Retrieves the transfer fee for this NFToken.
    ///
    /// The transfer fee is expressed in 1/100,000 units, meaning:
    /// - A value of 1 represents 0.001% (1/10 of a basis point)
    /// - A value of 100 represents 0.1% (10 basis points)
    /// - A value of 1000 represents 1% (100 basis points)
    /// - Maximum allowed value is 50,000 (representing 50%)
    ///
    /// # Returns
    ///
    /// * `Ok(u16)` - The transfer fee (0-50,000)
    /// * `Err(Error)` - If the host function fails
    ///
    pub fn transfer_fee(&self) -> Result<u16> {
        let result = unsafe { host::get_nft_transfer_fee(self.as_ptr(), self.len()) };

        match result {
            code if code >= 0 => Result::Ok(code as u16),
            code => Result::Err(Error::from_code(code)),
        }
    }

    /// Retrieves the issuer account of this NFToken.
    ///
    /// The issuer is encoded in bytes 4-23 of the NFTokenID (160 bits / 20 bytes).
    ///
    /// # Returns
    ///
    /// * `Ok(AccountID)` - The issuer's account identifier
    /// * `Err(Error)` - If the host function fails
    ///
    pub fn issuer(&self) -> Result<AccountID> {
        let mut account_buf = [0u8; ACCOUNT_ID_SIZE];
        let result = unsafe {
            host::get_nft_issuer(
                self.as_ptr(),
                self.len(),
                account_buf.as_mut_ptr(),
                account_buf.len(),
            )
        };

        match result {
            code if code > 0 => Result::Ok(AccountID(account_buf)),
            code => Result::Err(Error::from_code(code)),
        }
    }

    /// Retrieves the taxon of this NFToken.
    ///
    /// The taxon is an issuer-defined value that groups related NFTs together.
    /// # Returns
    ///
    /// * `Ok(u32)` - The taxon value
    /// * `Err(Error)` - If the host function fails
    ///
    pub fn taxon(&self) -> Result<u32> {
        let mut taxon_buf = [0u8; 4];
        let result = unsafe {
            host::get_nft_taxon(
                self.as_ptr(),
                self.len(),
                taxon_buf.as_mut_ptr(),
                taxon_buf.len(),
            )
        };

        match result {
            code if code > 0 => {
                // Convert big-endian bytes to u32
                let taxon = u32::from_be_bytes(taxon_buf);
                Result::Ok(taxon)
            }
            code => Result::Err(Error::from_code(code)),
        }
    }

    /// Retrieves the token sequence number of this NFToken.
    ///
    /// The token sequence number is automatically incremented for each NFToken minted
    /// by the issuer, based on the `MintedNFTokens` field of the issuer's account.
    /// This ensures each NFToken has a unique identifier.
    ///
    /// # Returns
    ///
    /// * `Ok(u32)` - The token sequence number
    /// * `Err(Error)` - If the host function fails
    ///
    pub fn token_sequence(&self) -> Result<u32> {
        let mut serial_buf = [0u8; 4];
        let result = unsafe {
            host::get_nft_serial(
                self.as_ptr(),
                self.len(),
                serial_buf.as_mut_ptr(),
                serial_buf.len(),
            )
        };

        match result {
            code if code > 0 => {
                // Convert big-endian bytes to u32
                let serial = u32::from_be_bytes(serial_buf);
                Result::Ok(serial)
            }
            code => Result::Err(Error::from_code(code)),
        }
    }

    /// Retrieves the URI of this NFToken for a given owner.
    ///
    /// # Arguments
    ///
    /// * `owner` - The account that owns this NFToken
    ///
    /// # Returns
    ///
    /// * `Ok(UriBlob)` - The URI data (variable length, up to 256 bytes)
    /// * `Err(Error)` - If the NFT is not found or the host function fails
    ///
    ///
    pub fn uri(&self, owner: &AccountID) -> Result<UriBlob> {
        let mut uri_buf = [0u8; URI_BLOB_SIZE];
        let result = unsafe {
            host::get_nft(
                owner.0.as_ptr(),
                owner.0.len(),
                self.as_ptr(),
                self.len(),
                uri_buf.as_mut_ptr(),
                uri_buf.len(),
            )
        };

        match result {
            code if code > 0 => Result::Ok(UriBlob::from(uri_buf)),
            code => Result::Err(Error::from_code(code)),
        }
    }
}

impl From<[u8; NFT_ID_SIZE]> for NFToken {
    fn from(value: [u8; NFT_ID_SIZE]) -> Self {
        NFToken(value)
    }
}

impl AsRef<[u8]> for NFToken {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::host::host_bindings_trait::MockHostBindings;
    use crate::host::setup_mock;
    use mockall::predicate::{always, eq};

    #[test]
    fn test_nft_creation() {
        let nft_id = [0u8; 32];
        let nft = NFToken::new(nft_id);
        assert_eq!(nft.as_bytes(), &nft_id);
        assert_eq!(nft.len(), 32);
    }

    #[test]
    fn test_nft_from_array() {
        let nft_id = [0u8; 32];
        let nft: NFToken = nft_id.into();
        assert_eq!(nft.as_bytes(), &nft_id);
    }

    // NftFlags tests
    #[test]
    fn test_nft_flags_no_flags_set() {
        let nft_flags = NftFlags::new(0);
        assert!(!nft_flags.is_burnable());
        assert!(!nft_flags.is_only_xrp());
        assert!(!nft_flags.is_trust_line());
        assert!(!nft_flags.is_transferable());
        assert_eq!(nft_flags.as_u16(), 0);
    }

    #[test]
    fn test_nft_flags_burnable() {
        let nft_flags = NftFlags::new(flags::BURNABLE);
        assert!(nft_flags.is_burnable());
        assert!(!nft_flags.is_only_xrp());
        assert!(!nft_flags.is_trust_line());
        assert!(!nft_flags.is_transferable());
        assert_eq!(nft_flags.as_u16(), flags::BURNABLE);
    }

    #[test]
    fn test_nft_flags_only_xrp() {
        let nft_flags = NftFlags::new(flags::ONLY_XRP);
        assert!(!nft_flags.is_burnable());
        assert!(nft_flags.is_only_xrp());
        assert!(!nft_flags.is_trust_line());
        assert!(!nft_flags.is_transferable());
        assert_eq!(nft_flags.as_u16(), flags::ONLY_XRP);
    }

    #[test]
    fn test_nft_flags_trust_line() {
        let nft_flags = NftFlags::new(flags::TRUST_LINE);
        assert!(!nft_flags.is_burnable());
        assert!(!nft_flags.is_only_xrp());
        assert!(nft_flags.is_trust_line());
        assert!(!nft_flags.is_transferable());
        assert_eq!(nft_flags.as_u16(), flags::TRUST_LINE);
    }

    #[test]
    fn test_nft_flags_transferable() {
        let nft_flags = NftFlags::new(flags::TRANSFERABLE);
        assert!(!nft_flags.is_burnable());
        assert!(!nft_flags.is_only_xrp());
        assert!(!nft_flags.is_trust_line());
        assert!(nft_flags.is_transferable());
        assert_eq!(nft_flags.as_u16(), flags::TRANSFERABLE);
    }

    #[test]
    fn test_nft_flags_multiple_flags() {
        let nft_flags = NftFlags::new(flags::BURNABLE | flags::TRANSFERABLE);
        assert!(nft_flags.is_burnable());
        assert!(!nft_flags.is_only_xrp());
        assert!(!nft_flags.is_trust_line());
        assert!(nft_flags.is_transferable());
        assert_eq!(nft_flags.as_u16(), flags::BURNABLE | flags::TRANSFERABLE);
    }

    #[test]
    fn test_nft_flags_all_flags_set() {
        let all_flags = flags::BURNABLE | flags::ONLY_XRP | flags::TRUST_LINE | flags::TRANSFERABLE;
        let nft_flags = NftFlags::new(all_flags);
        assert!(nft_flags.is_burnable());
        assert!(nft_flags.is_only_xrp());
        assert!(nft_flags.is_trust_line());
        assert!(nft_flags.is_transferable());
        assert_eq!(nft_flags.as_u16(), all_flags);
    }

    #[test]
    fn test_nft_flags_from_u16() {
        let flags_value: u16 = flags::BURNABLE | flags::ONLY_XRP;
        let nft_flags: NftFlags = flags_value.into();
        assert!(nft_flags.is_burnable());
        assert!(nft_flags.is_only_xrp());
        assert_eq!(nft_flags.as_u16(), flags_value);
    }

    #[test]
    fn test_nft_flags_into_u16() {
        let nft_flags = NftFlags::new(flags::TRANSFERABLE | flags::TRUST_LINE);
        let flags_value: u16 = nft_flags.into();
        assert_eq!(flags_value, flags::TRANSFERABLE | flags::TRUST_LINE);
    }

    #[test]
    fn test_nft_flags_equality() {
        let nft_flags1 = NftFlags::new(flags::BURNABLE);
        let nft_flags2 = NftFlags::new(flags::BURNABLE);
        let nft_flags3 = NftFlags::new(flags::ONLY_XRP);

        assert_eq!(nft_flags1, nft_flags2);
        assert_ne!(nft_flags1, nft_flags3);
    }

    #[test]
    fn test_nft_flags_clone() {
        let nft_flags1 = NftFlags::new(flags::TRANSFERABLE);
        let nft_flags2 = nft_flags1;

        assert_eq!(nft_flags1, nft_flags2);
        assert!(nft_flags2.is_transferable());
    }

    // NFToken additional tests
    #[test]
    fn test_nft_as_ptr() {
        let nft_id = [42u8; 32];
        let nft = NFToken::new(nft_id);

        let ptr = nft.as_ptr();
        assert!(!ptr.is_null());

        // Verify the pointer points to the correct data
        unsafe {
            assert_eq!(*ptr, 42u8);
        }
    }

    #[test]
    fn test_nft_as_ref() {
        let nft_id = [7u8; 32];
        let nft = NFToken::new(nft_id);

        let slice: &[u8] = nft.as_ref();
        assert_eq!(slice.len(), 32);
        assert_eq!(slice, &nft_id);
    }

    #[test]
    fn test_nft_equality() {
        let nft_id1 = [5u8; 32];
        let nft_id2 = [5u8; 32];
        let nft_id3 = [6u8; 32];

        let nft1 = NFToken::new(nft_id1);
        let nft2 = NFToken::new(nft_id2);
        let nft3 = NFToken::new(nft_id3);

        assert_eq!(nft1, nft2);
        assert_ne!(nft1, nft3);
    }

    #[test]
    fn test_nft_clone() {
        let nft_id = [9u8; 32];
        let nft1 = NFToken::new(nft_id);
        let nft2 = nft1;

        assert_eq!(nft1, nft2);
        assert_eq!(nft1.as_bytes(), nft2.as_bytes());
    }

    // NFToken method tests
    #[test]
    fn test_nft_flags_method() {
        let mut mock = MockHostBindings::new();
        let nft_id = [0u8; 32];
        let expected_flags = 0x0001u16; // BURNABLE flag

        // Set up expectations
        mock.expect_get_nft_flags()
            .with(always(), eq(NFT_ID_SIZE))
            .returning(move |_, _| expected_flags as i32);

        let _guard = setup_mock(mock);

        let nft = NFToken::new(nft_id);
        let result = nft.flags();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_u16(), expected_flags);
    }

    #[test]
    fn test_nft_transfer_fee_method() {
        let mut mock = MockHostBindings::new();
        let nft_id = [0u8; 32];
        let expected_fee = 1000u16;

        // Set up expectations
        mock.expect_get_nft_transfer_fee()
            .with(always(), eq(NFT_ID_SIZE))
            .returning(move |_, _| expected_fee as i32);

        let _guard = setup_mock(mock);

        let nft = NFToken::new(nft_id);
        let result = nft.transfer_fee();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_fee);
    }

    #[test]
    fn test_nft_issuer_method() {
        let mut mock = MockHostBindings::new();
        let nft_id = [0u8; 32];

        // Set up expectations
        mock.expect_get_nft_issuer()
            .with(always(), eq(NFT_ID_SIZE), always(), eq(ACCOUNT_ID_SIZE))
            .returning(|_, _, _, _| ACCOUNT_ID_SIZE as i32);

        let _guard = setup_mock(mock);

        let nft = NFToken::new(nft_id);
        let result = nft.issuer();
        assert!(result.is_ok());
        let issuer = result.unwrap();
        assert_eq!(issuer.0.len(), ACCOUNT_ID_SIZE);
    }

    #[test]
    fn test_nft_taxon_method() {
        let mut mock = MockHostBindings::new();
        let nft_id = [0u8; 32];

        // Set up expectations - taxon is a u32 (4 bytes)
        mock.expect_get_nft_taxon()
            .with(always(), eq(NFT_ID_SIZE), always(), eq(4))
            .returning(|_, _, _, _| 4);

        let _guard = setup_mock(mock);

        let nft = NFToken::new(nft_id);
        let result = nft.taxon();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn test_nft_token_sequence_method() {
        let mut mock = MockHostBindings::new();
        let nft_id = [0u8; 32];

        // Set up expectations - serial is a u32 (4 bytes)
        mock.expect_get_nft_serial()
            .with(always(), eq(NFT_ID_SIZE), always(), eq(4))
            .returning(|_, _, _, _| 4);

        let _guard = setup_mock(mock);

        let nft = NFToken::new(nft_id);
        let result = nft.token_sequence();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn test_nft_uri_method() {
        let mut mock = MockHostBindings::new();
        let nft_id = [0u8; 32];
        let owner = AccountID([0u8; ACCOUNT_ID_SIZE]);
        let expected_uri_len = 10;

        // Set up expectations
        mock.expect_get_nft()
            .with(
                always(),
                eq(ACCOUNT_ID_SIZE),
                always(),
                eq(NFT_ID_SIZE),
                always(),
                eq(URI_BLOB_SIZE),
            )
            .returning(move |_, _, _, _, _, _| expected_uri_len);

        let _guard = setup_mock(mock);

        let nft = NFToken::new(nft_id);
        let result = nft.uri(&owner);
        assert!(result.is_ok());
        let uri = result.unwrap();
        assert!(uri.len <= URI_BLOB_SIZE);
    }
}
