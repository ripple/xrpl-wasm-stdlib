//! Builder for nested field access locators.
//!
//! Locators encode a path to a nested field (sfields and array indices) in a compact
//! binary format understood by the host. Use it to access fields like `Memos[0].MemoType`.
//!
//! Example
//! ```no_run
//! use xrpl_wasm_stdlib::core::locator::Locator;
//! use xrpl_wasm_stdlib::sfield;
//! let mut l = Locator::new();
//! l.pack(sfield::Memos);
//! l.pack(0);
//! l.pack(sfield::MemoType);
//! # let _ = (l.len() >= 3);
//! ```

/// The size of the buffer, in bytes, to use for any new locator
const LOCATOR_BUFFER_SIZE: usize = 64; // max depth: 64/4 = 16

/// A Locator encodes a path to a nested field as a sequence of 4-byte packed values
/// (sfield codes or array indices) in a compact binary format understood by the host.
///
/// ## Derived Traits
///
/// - `Debug`: Useful for development and debugging
/// - `Clone`: Reasonable for this 72-byte struct when explicit copying is needed
/// - `Eq, PartialEq`: Enable comparisons between locators
///
/// Note: `Copy` is intentionally not derived due to the struct's size (72 bytes).
/// Large `Copy` types can lead to accidental expensive copies and poor performance.
/// Use `.clone()` when you need to duplicate a locator.
#[derive(Clone, PartialEq, Eq, Debug)]
#[repr(C)]
pub struct Locator {
    buffer: [u8; LOCATOR_BUFFER_SIZE],

    /// An index into `buffer` where the next packing operation can be stored.
    cur_buffer_index: usize,
}

impl Default for Locator {
    fn default() -> Self {
        Self::new()
    }
}

impl Locator {
    /// Create a new empty Locator.
    pub fn new() -> Locator {
        Self {
            buffer: [0; LOCATOR_BUFFER_SIZE],
            cur_buffer_index: 0,
        }
    }

    pub fn pack(&mut self, sfield_or_index: impl Into<i32>) -> bool {
        if self.cur_buffer_index + 4 > LOCATOR_BUFFER_SIZE {
            return false;
        }

        let value_bytes: [u8; 4] = sfield_or_index.into().to_le_bytes();
        self.buffer[self.cur_buffer_index..self.cur_buffer_index + 4].copy_from_slice(&value_bytes);
        self.cur_buffer_index += 4;

        true
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.buffer.as_ptr()
    }

    pub fn num_packed_bytes(&self) -> usize {
        self.cur_buffer_index
    }

    pub fn len(&self) -> usize {
        self.cur_buffer_index
    }

    pub fn is_empty(&self) -> bool {
        self.cur_buffer_index == 0
    }

    pub fn repack_last(&mut self, sfield_or_index: impl Into<i32>) -> bool {
        if self.cur_buffer_index < 4 {
            return false;
        }

        self.cur_buffer_index -= 4;

        let value_bytes: [u8; 4] = sfield_or_index.into().to_le_bytes();
        self.buffer[self.cur_buffer_index..self.cur_buffer_index + 4].copy_from_slice(&value_bytes);
        self.cur_buffer_index += 4;

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sfield;

    #[test]
    fn test_pack_with_sfield_no_into_needed() {
        // This test demonstrates that .into() is no longer needed when using SField constants
        let mut locator = Locator::new();

        // Pack SField constants directly without .into()
        assert!(locator.pack(sfield::Memos));
        assert!(locator.pack(0));
        assert!(locator.pack(sfield::MemoData));

        assert_eq!(locator.len(), 12); // 3 packed values * 4 bytes each
    }

    #[test]
    fn test_pack_with_i32_still_works() {
        // This test verifies that i32 values still work as before
        let mut locator = Locator::new();

        assert!(locator.pack(123i32));
        assert!(locator.pack(456i32));

        assert_eq!(locator.len(), 8); // 2 packed values * 4 bytes each
    }

    #[test]
    fn test_repack_last_with_sfield() {
        let mut locator = Locator::new();

        locator.pack(sfield::Memos);
        locator.pack(0);

        // Repack the last value with a different SField
        assert!(locator.repack_last(sfield::MemoData));

        assert_eq!(locator.len(), 8); // Still 2 packed values
    }

    #[test]
    fn test_new_starts_empty() {
        let locator = Locator::new();
        assert_eq!(locator.len(), 0);
        assert!(locator.is_empty());
    }

    #[test]
    fn test_default_same_as_new() {
        assert_eq!(Locator::default(), Locator::new());
    }

    #[test]
    fn test_pack_writes_correct_bytes() {
        let mut locator = Locator::new();
        assert!(locator.pack(0x12345678i32));
        assert_eq!(locator.len(), 4);

        let bytes = unsafe { core::slice::from_raw_parts(locator.as_ptr(), 4) };
        assert_eq!(bytes, &0x12345678i32.to_le_bytes());
    }

    #[test]
    fn test_pack_returns_false_when_buffer_full() {
        let mut locator = Locator::new();

        // Fill all 16 slots (64 bytes / 4 bytes per pack)
        for i in 0..16 {
            assert!(locator.pack(i));
        }
        assert_eq!(locator.len(), 64);

        // 17th pack should fail
        assert!(!locator.pack(999i32));
        assert_eq!(locator.len(), 64);
    }

    #[test]
    fn test_is_empty_false_after_pack() {
        let mut locator = Locator::new();
        assert!(locator.is_empty());

        locator.pack(sfield::Memos);
        assert!(!locator.is_empty());
        assert_eq!(locator.len(), 4);
    }

    #[test]
    fn test_num_packed_bytes_equals_len() {
        let mut locator = Locator::new();
        assert_eq!(locator.num_packed_bytes(), locator.len());

        locator.pack(sfield::Memos);
        assert_eq!(locator.num_packed_bytes(), locator.len());
        assert_eq!(locator.num_packed_bytes(), 4);

        locator.pack(0);
        assert_eq!(locator.num_packed_bytes(), locator.len());
        assert_eq!(locator.num_packed_bytes(), 8);
    }

    #[test]
    fn test_repack_last_on_empty_returns_false() {
        let mut locator = Locator::new();
        assert!(!locator.repack_last(sfield::Memos));
        assert_eq!(locator.len(), 0);
    }

    #[test]
    fn test_repack_last_overwrites_correct_bytes() {
        let mut locator = Locator::new();
        locator.pack(0x11111111i32);
        locator.pack(0x22222222i32);
        assert_eq!(locator.len(), 8);

        assert!(locator.repack_last(0x33333333i32));
        assert_eq!(locator.len(), 8);

        let bytes = unsafe { core::slice::from_raw_parts(locator.as_ptr(), 8) };
        // First value unchanged
        assert_eq!(&bytes[0..4], &0x11111111i32.to_le_bytes());
        // Second value replaced
        assert_eq!(&bytes[4..8], &0x33333333i32.to_le_bytes());
    }
}
