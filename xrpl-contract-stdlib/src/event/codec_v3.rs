use xrpl_wasm_stdlib::core::type_codes::{
    STI_ACCOUNT, STI_AMOUNT, STI_CURRENCY, STI_UINT8, STI_UINT16, STI_UINT32, STI_UINT64,
    STI_UINT128, STI_UINT160, STI_UINT192, STI_UINT256, STI_VL,
};
use xrpl_wasm_stdlib::core::types::account_id::AccountID;
use xrpl_wasm_stdlib::host::emit_event;

use core::mem::MaybeUninit;

// ============================================================================
// EventBuffer (unchanged)
// ============================================================================

pub struct EventBuffer {
    data: MaybeUninit<[u8; 1024]>,
    pos: usize,
    // start_pos: usize,
    vl_size: usize,
}

impl Default for EventBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl EventBuffer {
    #[inline]
    pub fn new() -> Self {
        let mut buf = EventBuffer {
            data: MaybeUninit::uninit(),
            pos: 1,
            // start_pos: 0,
            vl_size: 1,
        };
        buf.write_byte(0, 0);
        buf
    }

    #[inline(always)]
    fn write_byte(&mut self, index: usize, value: u8) {
        unsafe {
            (*self.data.as_mut_ptr())[index] = value;
        }
    }

    #[inline(always)]
    fn update_total_size(&mut self) {
        let content_size = self.pos - self.vl_size;

        let vl_size_needed = if content_size <= 192 {
            1
        } else if content_size <= 12480 {
            2
        } else {
            3
        };

        unsafe {
            let buffer_ptr = self.data.as_mut_ptr() as *mut u8;
            let buffer_slice = core::slice::from_raw_parts_mut(buffer_ptr, 1024);

            if vl_size_needed != self.vl_size {
                let shift = vl_size_needed - self.vl_size;

                if shift > 0 {
                    let mut i = self.pos;
                    while i > self.vl_size {
                        i -= 1;
                        buffer_slice[i + shift] = buffer_slice[i];
                    }
                    self.pos += shift;
                }

                self.vl_size = vl_size_needed;
            }

            let final_content_size = self.pos - self.vl_size;
            encode_vl_length(buffer_slice, 0, final_content_size);
        }
    }

    #[inline]
    pub fn emit(&mut self, event_type: &str) -> Result<(), i32> {
        self.update_total_size();

        unsafe {
            let ptr = self.data.as_ptr() as *const u8;
            emit_event(event_type.as_ptr(), event_type.len(), ptr, self.pos);
        }
        Ok(())
    }

    #[inline]
    pub fn get_buffer(&mut self) -> (*const u8, usize) {
        self.update_total_size();
        (self.data.as_ptr() as *const u8, self.pos)
    }
}

#[inline(never)]
pub fn emit_event_direct(buf: &mut EventBuffer, event_type: &str) -> Result<(), i32> {
    buf.update_total_size();
    unsafe {
        let ptr = buf.data.as_ptr() as *const u8;
        emit_event(event_type.as_ptr(), event_type.len(), ptr, buf.pos);
    }
    Ok(())
}

// ============================================================================
// Generic Trait for Event Emission
// ============================================================================

/// Trait for types that can be added to event buffers
pub trait ToEventBytes {
    /// Add this value to the event buffer with given key
    fn add_to_event(buf: &mut EventBuffer, key: &str, value: &Self) -> Result<(), i32>;
}

// ============================================================================
// Generic Wrapper Function
// ============================================================================

#[inline]
pub fn event_add<T: ToEventBytes>(buf: &mut EventBuffer, key: &str, value: &T) -> Result<(), i32> {
    T::add_to_event(buf, key, value)
}

// ============================================================================
// ToEventBytes Implementation for u8 (STI_UINT8)
// ============================================================================

impl ToEventBytes for u8 {
    #[inline]
    fn add_to_event(buf: &mut EventBuffer, key: &str, value: &Self) -> Result<(), i32> {
        let key_bytes = key.as_bytes();
        let key_len = key_bytes.len();

        // Space needed = key_size(1) + key_len + value_size(1) + type(1) + value(1)
        let space_needed = 1 + key_len + 1 + 1 + 1;

        if key_len > 127 || buf.pos + space_needed > 1024 {
            return Err(-1);
        }

        // Write key size
        buf.write_byte(buf.pos, key_len as u8);
        buf.pos += 1;

        // Write key
        let mut i = 0;
        while i < key_len {
            buf.write_byte(buf.pos + i, key_bytes[i]);
            i += 1;
        }
        buf.pos += key_len;

        // Write value size (type + data = 2 bytes)
        buf.write_byte(buf.pos, 2);
        buf.pos += 1;

        // Write type
        buf.write_byte(buf.pos, STI_UINT8);
        buf.pos += 1;

        // Write value
        buf.write_byte(buf.pos, *value);
        buf.pos += 1;

        Ok(())
    }
}

// ============================================================================
// ToEventBytes Implementation for u16 (STI_UINT16)
// ============================================================================

impl ToEventBytes for u16 {
    #[inline]
    fn add_to_event(buf: &mut EventBuffer, key: &str, value: &Self) -> Result<(), i32> {
        let key_bytes = key.as_bytes();
        let key_len = key_bytes.len();

        // Space needed = key_size(1) + key_len + value_size(1) + type(1) + value(2)
        let space_needed = 1 + key_len + 1 + 1 + 2;

        if key_len > 127 || buf.pos + space_needed > 1024 {
            return Err(-1);
        }

        // Write key size
        buf.write_byte(buf.pos, key_len as u8);
        buf.pos += 1;

        // Write key
        let mut i = 0;
        while i < key_len {
            buf.write_byte(buf.pos + i, key_bytes[i]);
            i += 1;
        }
        buf.pos += key_len;

        // Write value size (type + data = 3 bytes)
        buf.write_byte(buf.pos, 3);
        buf.pos += 1;

        // Write type
        buf.write_byte(buf.pos, STI_UINT16);
        buf.pos += 1;

        // Write value (big-endian)
        buf.write_byte(buf.pos, (*value >> 8) as u8);
        buf.write_byte(buf.pos + 1, *value as u8);
        buf.pos += 2;

        Ok(())
    }
}

// ============================================================================
// ToEventBytes Implementation for u32 (STI_UINT32)
// ============================================================================

impl ToEventBytes for u32 {
    #[inline]
    fn add_to_event(buf: &mut EventBuffer, key: &str, value: &Self) -> Result<(), i32> {
        let key_bytes = key.as_bytes();
        let key_len = key_bytes.len();

        // Space needed = key_size(1) + key_len + value_size(1) + type(1) + value(4)
        let space_needed = 1 + key_len + 1 + 1 + 4;

        if key_len > 127 || buf.pos + space_needed > 1024 {
            return Err(-1);
        }

        // Write key size
        buf.write_byte(buf.pos, key_len as u8);
        buf.pos += 1;

        // Write key
        let mut i = 0;
        while i < key_len {
            buf.write_byte(buf.pos + i, key_bytes[i]);
            i += 1;
        }
        buf.pos += key_len;

        // Write value size (type + data = 5 bytes)
        buf.write_byte(buf.pos, 5);
        buf.pos += 1;

        // Write type
        buf.write_byte(buf.pos, STI_UINT32);
        buf.pos += 1;

        // Write value (big-endian)
        buf.write_byte(buf.pos, (*value >> 24) as u8);
        buf.write_byte(buf.pos + 1, (*value >> 16) as u8);
        buf.write_byte(buf.pos + 2, (*value >> 8) as u8);
        buf.write_byte(buf.pos + 3, *value as u8);
        buf.pos += 4;

        Ok(())
    }
}

// ============================================================================
// ToEventBytes Implementation for u64 (STI_UINT64)
// ============================================================================

impl ToEventBytes for u64 {
    #[inline]
    fn add_to_event(buf: &mut EventBuffer, key: &str, value: &Self) -> Result<(), i32> {
        let key_bytes = key.as_bytes();
        let key_len = key_bytes.len();

        // Space needed = key_size(1) + key_len + value_size(1) + type(1) + value(8)
        let space_needed = 1 + key_len + 1 + 1 + 8;

        if key_len > 127 || buf.pos + space_needed > 1024 {
            return Err(-1);
        }

        // Write key size
        buf.write_byte(buf.pos, key_len as u8);
        buf.pos += 1;

        // Write key
        let mut i = 0;
        while i < key_len {
            buf.write_byte(buf.pos + i, key_bytes[i]);
            i += 1;
        }
        buf.pos += key_len;

        // Write value size (type + data = 9 bytes)
        buf.write_byte(buf.pos, 9);
        buf.pos += 1;

        // Write type
        buf.write_byte(buf.pos, STI_UINT64);
        buf.pos += 1;

        // Write value (big-endian)
        buf.write_byte(buf.pos, (*value >> 56) as u8);
        buf.write_byte(buf.pos + 1, (*value >> 48) as u8);
        buf.write_byte(buf.pos + 2, (*value >> 40) as u8);
        buf.write_byte(buf.pos + 3, (*value >> 32) as u8);
        buf.write_byte(buf.pos + 4, (*value >> 24) as u8);
        buf.write_byte(buf.pos + 5, (*value >> 16) as u8);
        buf.write_byte(buf.pos + 6, (*value >> 8) as u8);
        buf.write_byte(buf.pos + 7, *value as u8);
        buf.pos += 8;

        Ok(())
    }
}

// ============================================================================
// ToEventBytes Implementation for [u8; 16] (STI_UINT128)
// ============================================================================

impl ToEventBytes for [u8; 16] {
    #[inline]
    fn add_to_event(buf: &mut EventBuffer, key: &str, value: &Self) -> Result<(), i32> {
        let key_bytes = key.as_bytes();
        let key_len = key_bytes.len();

        // Space needed = key_size(1) + key_len + value_size(1) + type(1) + value(16)
        let space_needed = 1 + key_len + 1 + 1 + 16;

        if key_len > 127 || buf.pos + space_needed > 1024 {
            return Err(-1);
        }

        // Write key size
        buf.write_byte(buf.pos, key_len as u8);
        buf.pos += 1;

        // Write key
        let mut i = 0;
        while i < key_len {
            buf.write_byte(buf.pos + i, key_bytes[i]);
            i += 1;
        }
        buf.pos += key_len;

        // Write value size (type + data = 17 bytes)
        buf.write_byte(buf.pos, 17);
        buf.pos += 1;

        // Write type
        buf.write_byte(buf.pos, STI_UINT128);
        buf.pos += 1;

        // Write value
        let mut i = 0;
        while i < 16 {
            buf.write_byte(buf.pos + i, value[i]);
            i += 1;
        }
        buf.pos += 16;

        Ok(())
    }
}

// ============================================================================
// ToEventBytes Implementation for [u8; 20] (STI_UINT160)
// Note: This is for UINT160/Currency - AccountID has special handling below
// ============================================================================

impl ToEventBytes for [u8; 20] {
    #[inline]
    fn add_to_event(buf: &mut EventBuffer, key: &str, value: &Self) -> Result<(), i32> {
        let key_bytes = key.as_bytes();
        let key_len = key_bytes.len();

        // Space needed = key_size(1) + key_len + value_size(1) + type(1) + value(20)
        let space_needed = 1 + key_len + 1 + 1 + 20;

        if key_len > 127 || buf.pos + space_needed > 1024 {
            return Err(-1);
        }

        // Write key size
        buf.write_byte(buf.pos, key_len as u8);
        buf.pos += 1;

        // Write key
        let mut i = 0;
        while i < key_len {
            buf.write_byte(buf.pos + i, key_bytes[i]);
            i += 1;
        }
        buf.pos += key_len;

        // Write value size (type + data = 21 bytes)
        buf.write_byte(buf.pos, 21);
        buf.pos += 1;

        // Write type
        buf.write_byte(buf.pos, STI_UINT160);
        buf.pos += 1;

        // Write value
        let mut i = 0;
        while i < 20 {
            buf.write_byte(buf.pos + i, value[i]);
            i += 1;
        }
        buf.pos += 20;

        Ok(())
    }
}

// ============================================================================
// ToEventBytes Implementation for [u8; 24] (STI_UINT192)
// ============================================================================

impl ToEventBytes for [u8; 24] {
    #[inline]
    fn add_to_event(buf: &mut EventBuffer, key: &str, value: &Self) -> Result<(), i32> {
        let key_bytes = key.as_bytes();
        let key_len = key_bytes.len();

        // Space needed = key_size(1) + key_len + value_size(1) + type(1) + value(24)
        let space_needed = 1 + key_len + 1 + 1 + 24;

        if key_len > 127 || buf.pos + space_needed > 1024 {
            return Err(-1);
        }

        // Write key size
        buf.write_byte(buf.pos, key_len as u8);
        buf.pos += 1;

        // Write key
        let mut i = 0;
        while i < key_len {
            buf.write_byte(buf.pos + i, key_bytes[i]);
            i += 1;
        }
        buf.pos += key_len;

        // Write value size (type + data = 25 bytes)
        buf.write_byte(buf.pos, 25);
        buf.pos += 1;

        // Write type
        buf.write_byte(buf.pos, STI_UINT192);
        buf.pos += 1;

        // Write value
        let mut i = 0;
        while i < 24 {
            buf.write_byte(buf.pos + i, value[i]);
            i += 1;
        }
        buf.pos += 24;

        Ok(())
    }
}

// ============================================================================
// ToEventBytes Implementation for [u8; 32] (STI_UINT256)
// ============================================================================

impl ToEventBytes for [u8; 32] {
    #[inline]
    fn add_to_event(buf: &mut EventBuffer, key: &str, value: &Self) -> Result<(), i32> {
        let key_bytes = key.as_bytes();
        let key_len = key_bytes.len();

        // Space needed = key_size(1) + key_len + value_size(1) + type(1) + value(32)
        let space_needed = 1 + key_len + 1 + 1 + 32;

        if key_len > 127 || buf.pos + space_needed > 1024 {
            return Err(-1);
        }

        // Write key size
        buf.write_byte(buf.pos, key_len as u8);
        buf.pos += 1;

        // Write key
        let mut i = 0;
        while i < key_len {
            buf.write_byte(buf.pos + i, key_bytes[i]);
            i += 1;
        }
        buf.pos += key_len;

        // Write value size (type + data = 33 bytes)
        buf.write_byte(buf.pos, 33);
        buf.pos += 1;

        // Write type
        buf.write_byte(buf.pos, STI_UINT256);
        buf.pos += 1;

        // Write value
        let mut i = 0;
        while i < 32 {
            buf.write_byte(buf.pos + i, value[i]);
            i += 1;
        }
        buf.pos += 32;

        Ok(())
    }
}

// ============================================================================
// ToEventBytes Implementation for [u8; 8] (STI_AMOUNT)
// ============================================================================

impl ToEventBytes for [u8; 8] {
    #[inline]
    fn add_to_event(buf: &mut EventBuffer, key: &str, value: &Self) -> Result<(), i32> {
        let key_bytes = key.as_bytes();
        let key_len = key_bytes.len();

        // Space needed = key_size(1) + key_len + value_size(1) + type(1) + value(8)
        let space_needed = 1 + key_len + 1 + 1 + 8;

        if key_len > 127 || buf.pos + space_needed > 1024 {
            return Err(-1);
        }

        // Write key size
        buf.write_byte(buf.pos, key_len as u8);
        buf.pos += 1;

        // Write key
        let mut i = 0;
        while i < key_len {
            buf.write_byte(buf.pos + i, key_bytes[i]);
            i += 1;
        }
        buf.pos += key_len;

        // Write value size (type + data = 9 bytes)
        buf.write_byte(buf.pos, 9);
        buf.pos += 1;

        // Write type
        buf.write_byte(buf.pos, STI_AMOUNT);
        buf.pos += 1;

        // Write value
        let mut i = 0;
        while i < 8 {
            buf.write_byte(buf.pos + i, value[i]);
            i += 1;
        }
        buf.pos += 8;

        Ok(())
    }
}

// ============================================================================
// Helper: Newtype for Account (to distinguish from [u8; 20])
// ============================================================================

/// Wrapper type for Account IDs to distinguish from plain [u8; 20]
/// Use this when you want STI_ACCOUNT encoding with 0x14 prefix
impl ToEventBytes for AccountID {
    #[inline]
    fn add_to_event(buf: &mut EventBuffer, key: &str, value: &Self) -> Result<(), i32> {
        let key_bytes = key.as_bytes();
        let key_len = key_bytes.len();

        // Space needed = key_size(1) + key_len + value_size(1) + type(1) + prefix(1) + value(20)
        let space_needed = 1 + key_len + 1 + 1 + 1 + 20;

        if key_len > 127 || buf.pos + space_needed > 1024 {
            return Err(-1);
        }

        // Write key size
        buf.write_byte(buf.pos, key_len as u8);
        buf.pos += 1;

        // Write key
        let mut i = 0;
        while i < key_len {
            buf.write_byte(buf.pos + i, key_bytes[i]);
            i += 1;
        }
        buf.pos += key_len;

        // Write value size (type + prefix + data = 22 bytes)
        buf.write_byte(buf.pos, 22);
        buf.pos += 1;

        // Write type
        buf.write_byte(buf.pos, STI_ACCOUNT);
        buf.pos += 1;

        // Write account length prefix
        buf.write_byte(buf.pos, 0x14);
        buf.pos += 1;

        // Write value
        let mut i = 0;
        while i < 20 {
            buf.write_byte(buf.pos + i, value.0[i]);
            i += 1;
        }
        buf.pos += 20;

        Ok(())
    }
}

// ============================================================================
// Helper: Newtype for Currency (to distinguish from [u8; 20])
// ============================================================================

/// Wrapper type for Currency codes to distinguish from plain [u8; 20]
/// Use this when you want STI_CURRENCY encoding
pub struct Currency(pub [u8; 20]);

impl ToEventBytes for Currency {
    #[inline]
    fn add_to_event(buf: &mut EventBuffer, key: &str, value: &Self) -> Result<(), i32> {
        let key_bytes = key.as_bytes();
        let key_len = key_bytes.len();

        // Space needed = key_size(1) + key_len + value_size(1) + type(1) + value(20)
        let space_needed = 1 + key_len + 1 + 1 + 20;

        if key_len > 127 || buf.pos + space_needed > 1024 {
            return Err(-1);
        }

        // Write key size
        buf.write_byte(buf.pos, key_len as u8);
        buf.pos += 1;

        // Write key
        let mut i = 0;
        while i < key_len {
            buf.write_byte(buf.pos + i, key_bytes[i]);
            i += 1;
        }
        buf.pos += key_len;

        // Write value size (type + data = 21 bytes)
        buf.write_byte(buf.pos, 21);
        buf.pos += 1;

        // Write type
        buf.write_byte(buf.pos, STI_CURRENCY);
        buf.pos += 1;

        // Write value
        let mut i = 0;
        while i < 20 {
            buf.write_byte(buf.pos + i, value.0[i]);
            i += 1;
        }
        buf.pos += 20;

        Ok(())
    }
}

// ============================================================================
// VL Encoding Helper (unchanged)
// ============================================================================

#[inline]
pub fn encode_vl_length(buffer: &mut [u8], pos: usize, len: usize) -> usize {
    if len <= 192 {
        unsafe {
            *buffer.get_unchecked_mut(pos) = len as u8;
        }
        1
    } else if len <= 12480 {
        let encoded = len - 193;
        unsafe {
            *buffer.get_unchecked_mut(pos) = 193 + (encoded >> 8) as u8;
            *buffer.get_unchecked_mut(pos + 1) = (encoded & 0xff) as u8;
        }
        2
    } else if len <= 918744 {
        let encoded = len - 12481;
        unsafe {
            *buffer.get_unchecked_mut(pos) = 241 + (encoded >> 16) as u8;
            *buffer.get_unchecked_mut(pos + 1) = ((encoded >> 8) & 0xff) as u8;
            *buffer.get_unchecked_mut(pos + 2) = (encoded & 0xff) as u8;
        }
        3
    } else {
        0
    }
}

// ============================================================================
// String/Blob Helper (unchanged)
// ============================================================================

#[inline(always)]
pub fn event_add_str(buf: &mut EventBuffer, key: &str, value: &str) -> Result<(), i32> {
    let key_bytes = key.as_bytes();
    let key_len = key_bytes.len();
    let value_bytes = value.as_bytes();
    let value_len = value_bytes.len();

    if value_len > 918744 || key_len > 127 {
        return Err(-1);
    }

    let vl_len_size = if value_len <= 192 {
        1
    } else if value_len <= 12480 {
        2
    } else {
        3
    };

    let space_needed = 1 + key_len + 1 + 1 + vl_len_size + value_len;

    if buf.pos + space_needed > 1024 {
        return Err(-1);
    }

    buf.write_byte(buf.pos, key_len as u8);
    buf.pos += 1;

    let mut i = 0;
    while i < key_len {
        buf.write_byte(buf.pos + i, key_bytes[i]);
        i += 1;
    }
    buf.pos += key_len;

    buf.write_byte(buf.pos, (1 + vl_len_size + value_len) as u8);
    buf.pos += 1;

    buf.write_byte(buf.pos, STI_VL);
    buf.pos += 1;

    unsafe {
        let buffer_ptr = buf.data.as_mut_ptr() as *mut u8;
        let buffer_slice = core::slice::from_raw_parts_mut(buffer_ptr, 1024);
        let bytes_written = encode_vl_length(buffer_slice, buf.pos, value_len);
        buf.pos += bytes_written;
    }

    let mut i = 0;
    while i < value_len {
        buf.write_byte(buf.pos + i, value_bytes[i]);
        i += 1;
    }
    buf.pos += value_len;

    Ok(())
}
