#![doc = include_str!("../../README.md")]
#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

// Re-export the r_address macro for convenient access
pub use xrpl_address_macro::r_address;

pub mod core;
pub mod host;
pub mod sfield;
pub mod types;

/// Complete Developer Guide
///
/// This comprehensive guide covers everything you need to develop smart escrows using
/// the XRPL WebAssembly Standard Library, from getting started to advanced development.
///
/// All internal links work properly within this single documentation page.
#[cfg(doc)]
#[doc = include_str!("../../docs/comprehensive-guide.md")]
pub mod guide {}

/// This function is called on panic but only in the WASM architecture. In non-WASM (e.g., in the
/// Host Simulator) the standard lib is available, which includes a panic handler.
#[cfg(target_arch = "wasm32")]
#[panic_handler]
fn panic(_info: &::core::panic::PanicInfo) -> ! {
    // This instruction will halt execution of the WASM module.
    // It's the WASM equivalent of a trap or an unrecoverable error.
    ::core::arch::wasm32::unreachable();
}

#[inline(always)]
fn hex_char_to_nibble(c: u8) -> Option<u8> {
    // WASM-optimized hex decoding with branch conditions for better performance
    #[cfg(target_arch = "wasm32")]
    {
        // Use branchless computation optimized for WASM
        if c >= b'0' && c <= b'9' {
            Some(c - b'0')
        } else if c >= b'a' && c <= b'f' {
            Some(c - b'a' + 10)
        } else if c >= b'A' && c <= b'F' {
            Some(c - b'A' + 10)
        } else {
            None
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        // Use pattern matching for non-WASM targets; this is more idiomatic and may have different compiler
        // optimization characteristics but is functionally equivalent to the WASM branch.
        match c {
            b'0'..=b'9' => Some(c - b'0'),
            b'a'..=b'f' => Some(c - b'a' + 10),
            b'A'..=b'F' => Some(c - b'A' + 10),
            _ => None,
        }
    }
}

/// Decode a 64-hex-character string into a 32-byte array.
///
/// The input must be exactly 64 hexadecimal ASCII bytes (lower- or upper-case).
/// Returns `None` if any character is not a valid hex digit.
///
/// Example:
/// ```
/// # use xrpl_wasm_stdlib::decode_hex_32;
/// let hex = *b"0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
/// let bytes = decode_hex_32(&hex).unwrap();
/// assert_eq!(bytes.len(), 32);
/// ```
#[inline(always)]
pub fn decode_hex_32(hex: &[u8; 64]) -> Option<[u8; 32]> {
    let mut out = [0u8; 32];

    // Unrolled loop for better WASM performance - eliminates loop counter overhead
    macro_rules! decode_byte {
        ($i:expr) => {{
            let high = hex_char_to_nibble(hex[$i * 2])?;
            let low = hex_char_to_nibble(hex[$i * 2 + 1])?;
            out[$i] = (high << 4) | low;
        }};
    }

    decode_byte!(0);
    decode_byte!(1);
    decode_byte!(2);
    decode_byte!(3);
    decode_byte!(4);
    decode_byte!(5);
    decode_byte!(6);
    decode_byte!(7);
    decode_byte!(8);
    decode_byte!(9);
    decode_byte!(10);
    decode_byte!(11);
    decode_byte!(12);
    decode_byte!(13);
    decode_byte!(14);
    decode_byte!(15);
    decode_byte!(16);
    decode_byte!(17);
    decode_byte!(18);
    decode_byte!(19);
    decode_byte!(20);
    decode_byte!(21);
    decode_byte!(22);
    decode_byte!(23);
    decode_byte!(24);
    decode_byte!(25);
    decode_byte!(26);
    decode_byte!(27);
    decode_byte!(28);
    decode_byte!(29);
    decode_byte!(30);
    decode_byte!(31);

    Some(out)
}

/// Decode a 40-hex-character string into a 20-byte array.
///
/// The input must be exactly 40 hexadecimal ASCII bytes.
/// Returns `None` if any character is not a valid hex digit.
///
/// Example:
/// ```
/// # use xrpl_wasm_stdlib::decode_hex_20;
/// let hex = *b"00112233445566778899aabbccddeeff00112233";
/// let bytes = decode_hex_20(&hex).unwrap();
/// assert_eq!(bytes.len(), 20);
/// ```
#[inline(always)]
pub fn decode_hex_20(hex: &[u8; 40]) -> Option<[u8; 20]> {
    let mut out = [0u8; 20];

    // Unrolled loop for better WASM performance - eliminates loop counter overhead
    macro_rules! decode_byte {
        ($i:expr) => {{
            let high = hex_char_to_nibble(hex[$i * 2])?;
            let low = hex_char_to_nibble(hex[$i * 2 + 1])?;
            out[$i] = (high << 4) | low;
        }};
    }

    decode_byte!(0);
    decode_byte!(1);
    decode_byte!(2);
    decode_byte!(3);
    decode_byte!(4);
    decode_byte!(5);
    decode_byte!(6);
    decode_byte!(7);
    decode_byte!(8);
    decode_byte!(9);
    decode_byte!(10);
    decode_byte!(11);
    decode_byte!(12);
    decode_byte!(13);
    decode_byte!(14);
    decode_byte!(15);
    decode_byte!(16);
    decode_byte!(17);
    decode_byte!(18);
    decode_byte!(19);

    Some(out)
}
