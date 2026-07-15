use core::mem::MaybeUninit;
use xrpl_wasm_stdlib::core::type_codes::{
    STI_ACCOUNT, STI_AMOUNT, STI_CURRENCY, STI_UINT8, STI_UINT16, STI_UINT32, STI_UINT64,
    STI_UINT128, STI_UINT160, STI_UINT192, STI_UINT256, STI_VL,
};
use xrpl_wasm_stdlib::host::emit_event;

// Minimal event buffer that just tracks position
pub struct EventBuffer {
    data: MaybeUninit<[u8; 1024]>,
    pos: usize,
    // start_pos: usize,  // Track where current event data starts
    vl_size: usize, // Track current VL encoding size (1, 2, or 3 bytes)
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
            pos: 1, // Reserve first byte for total size
            // start_pos: 0,
            vl_size: 1, // Start with 1 byte for VL encoding
        };
        // Initialize the total size to 0
        buf.write_byte(0, 0);
        buf
    }

    // Helper to write a byte at position
    #[inline(always)]
    fn write_byte(&mut self, index: usize, value: u8) {
        unsafe {
            (*self.data.as_mut_ptr())[index] = value;
        }
    }

    // Update the total size at the beginning of the buffer
    #[inline(always)]
    fn update_total_size(&mut self) {
        // Calculate content size (everything after the VL encoding)
        let content_size = self.pos - self.vl_size;

        // Determine how many bytes we need for VL encoding
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

            // If VL encoding size changed, we need to shift data
            if vl_size_needed != self.vl_size {
                let shift = vl_size_needed - self.vl_size;

                // Move data to make room for larger VL encoding
                if shift > 0 {
                    // Moving forward - work backwards to avoid overwriting
                    let mut i = self.pos;
                    while i > self.vl_size {
                        i -= 1;
                        buffer_slice[i + shift] = buffer_slice[i];
                    }
                    self.pos += shift;
                }

                self.vl_size = vl_size_needed;
            }

            // Now write the VL encoding for the content size
            let final_content_size = self.pos - self.vl_size;
            encode_vl_length(buffer_slice, 0, final_content_size);
        }
    }

    // Changed to take self by reference and return the buffer pointer directly
    #[inline]
    pub fn emit(&mut self, event_type: &str) -> Result<(), i32> {
        // Update total size before emitting
        self.update_total_size();

        unsafe {
            // Pass the buffer directly without any copying
            let ptr = self.data.as_ptr() as *const u8;
            emit_event(event_type.as_ptr(), event_type.len(), ptr, self.pos);
        }
        Ok(())
    }

    // Alternative: Get the buffer pointer and length for manual emission
    #[inline]
    pub fn get_buffer(&mut self) -> (*const u8, usize) {
        self.update_total_size();
        (self.data.as_ptr() as *const u8, self.pos)
    }
}

// Alternative emit function that takes buffer by reference
#[inline(never)]
pub fn emit_event_direct(buf: &mut EventBuffer, event_type: &str) -> Result<(), i32> {
    buf.update_total_size();
    unsafe {
        let ptr = buf.data.as_ptr() as *const u8;
        emit_event(event_type.as_ptr(), event_type.len(), ptr, buf.pos);
    }
    Ok(())
}

// Free functions - only import what you use!

#[inline]
pub fn event_add_u8(buf: &mut EventBuffer, key: &str, value: u8) -> Result<(), i32> {
    let key_bytes = key.as_bytes();
    let key_len = key_bytes.len();

    // Format per field: [Key.Size] [key.HEX] [Value.Size] [Value.SType] [Value.HEX]
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

    // Write value size (type + data = 2 bytes for u8)
    buf.write_byte(buf.pos, 2);
    buf.pos += 1;

    // Write type
    buf.write_byte(buf.pos, STI_UINT8);
    buf.pos += 1;

    // Write value
    buf.write_byte(buf.pos, value);
    buf.pos += 1;

    Ok(())
}

#[inline]
pub fn event_add_u16(buf: &mut EventBuffer, key: &str, value: u16) -> Result<(), i32> {
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

    // Write value size (type + data = 3 bytes for u16)
    buf.write_byte(buf.pos, 3);
    buf.pos += 1;

    // Write type
    buf.write_byte(buf.pos, STI_UINT16);
    buf.pos += 1;

    // Write value (big-endian)
    buf.write_byte(buf.pos, (value >> 8) as u8);
    buf.write_byte(buf.pos + 1, value as u8);
    buf.pos += 2;

    Ok(())
}

#[inline]
pub fn event_add_u32(buf: &mut EventBuffer, key: &str, value: u32) -> Result<(), i32> {
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

    // Write value size (type + data = 5 bytes for u32)
    buf.write_byte(buf.pos, 5);
    buf.pos += 1;

    // Write type
    buf.write_byte(buf.pos, STI_UINT32);
    buf.pos += 1;

    // Write value (big-endian)
    buf.write_byte(buf.pos, (value >> 24) as u8);
    buf.write_byte(buf.pos + 1, (value >> 16) as u8);
    buf.write_byte(buf.pos + 2, (value >> 8) as u8);
    buf.write_byte(buf.pos + 3, value as u8);
    buf.pos += 4;

    Ok(())
}

#[inline]
pub fn event_add_u64(buf: &mut EventBuffer, key: &str, value: u64) -> Result<(), i32> {
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

    // Write value size (type + data = 9 bytes for u64)
    buf.write_byte(buf.pos, 9);
    buf.pos += 1;

    // Write type
    buf.write_byte(buf.pos, STI_UINT64);
    buf.pos += 1;

    // Write value (big-endian)
    buf.write_byte(buf.pos, (value >> 56) as u8);
    buf.write_byte(buf.pos + 1, (value >> 48) as u8);
    buf.write_byte(buf.pos + 2, (value >> 40) as u8);
    buf.write_byte(buf.pos + 3, (value >> 32) as u8);
    buf.write_byte(buf.pos + 4, (value >> 24) as u8);
    buf.write_byte(buf.pos + 5, (value >> 16) as u8);
    buf.write_byte(buf.pos + 6, (value >> 8) as u8);
    buf.write_byte(buf.pos + 7, value as u8);
    buf.pos += 8;

    Ok(())
}

#[inline]
pub fn event_add_u128(buf: &mut EventBuffer, key: &str, value: &[u8; 16]) -> Result<(), i32> {
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

    // Write value size (type + data = 17 bytes for u128)
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

#[inline]
pub fn event_add_u160(buf: &mut EventBuffer, key: &str, value: &[u8; 20]) -> Result<(), i32> {
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

    // Write value size (type + data = 21 bytes for u160)
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

#[inline]
pub fn event_add_u192(buf: &mut EventBuffer, key: &str, value: &[u8; 24]) -> Result<(), i32> {
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

    // Write value size (type + data = 25 bytes for u192)
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

#[inline]
pub fn event_add_u256(buf: &mut EventBuffer, key: &str, value: &[u8; 32]) -> Result<(), i32> {
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

    // Write value size (type + data = 33 bytes for u256)
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

#[inline]
pub fn event_add_amount(buf: &mut EventBuffer, key: &str, value: &[u8; 8]) -> Result<(), i32> {
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

    // Write value size (type + data = 9 bytes for amount)
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

#[inline]
pub fn event_add_account(buf: &mut EventBuffer, key: &str, value: &[u8; 20]) -> Result<(), i32> {
    let key_bytes = key.as_bytes();
    let key_len = key_bytes.len();

    // Space needed = key_size(1) + key_len + value_size(1) + type(1) + length_prefix(1) + value(20)
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

    // Write value size (type + length_prefix + data = 22 bytes total)
    // The size field represents the total bytes of the value portion which includes:
    // 1 byte for type (STI_ACCOUNT) + 1 byte for length prefix + 20 bytes for account data = 22 bytes
    buf.write_byte(buf.pos, 22);
    buf.pos += 1;

    // Write type
    buf.write_byte(buf.pos, STI_ACCOUNT);
    buf.pos += 1;

    // Write account length prefix
    buf.write_byte(buf.pos, 0x14); // 20 in hex
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

#[inline]
pub fn event_add_currency(buf: &mut EventBuffer, key: &str, value: &[u8; 20]) -> Result<(), i32> {
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

    // Write value size (type + data = 21 bytes for currency)
    buf.write_byte(buf.pos, 21);
    buf.pos += 1;

    // Write type
    buf.write_byte(buf.pos, STI_CURRENCY);
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
        0 // Error case
    }
}

#[inline(always)]
pub fn event_add_str(buf: &mut EventBuffer, key: &str, value: &str) -> Result<(), i32> {
    let key_bytes = key.as_bytes();
    let key_len = key_bytes.len();
    let value_bytes = value.as_bytes();
    let value_len = value_bytes.len();

    // We'll accept strings up to 918744 bytes (max VL encoding supports)
    // but for practical purposes, limiting to something reasonable
    if value_len > 918744 || key_len > 127 {
        return Err(-1);
    }

    // Calculate how many bytes we need for the VL length encoding
    let vl_len_size = if value_len <= 192 {
        1
    } else if value_len <= 12480 {
        2
    } else {
        3
    };

    // Space needed = key_size(1) + key_len + value_size(1) + type(1) + vl_len_size + value_len
    let space_needed = 1 + key_len + 1 + 1 + vl_len_size + value_len;

    // Check if we have space
    if buf.pos + space_needed > 1024 {
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

    // Write value size (type + vl_len_size + data)
    buf.write_byte(buf.pos, (1 + vl_len_size + value_len) as u8);
    buf.pos += 1;

    // Write type
    buf.write_byte(buf.pos, STI_VL);
    buf.pos += 1;

    // Write VL-encoded string length
    unsafe {
        let buffer_ptr = buf.data.as_mut_ptr() as *mut u8;
        let buffer_slice = core::slice::from_raw_parts_mut(buffer_ptr, 1024);
        let bytes_written = encode_vl_length(buffer_slice, buf.pos, value_len);
        buf.pos += bytes_written;
    }

    // Write string value
    let mut i = 0;
    while i < value_len {
        buf.write_byte(buf.pos + i, value_bytes[i]);
        i += 1;
    }
    buf.pos += value_len;

    Ok(())
}
