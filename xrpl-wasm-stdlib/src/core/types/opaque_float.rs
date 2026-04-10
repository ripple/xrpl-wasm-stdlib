/// Opaque 12-byte representation of an XRPL floating-point number.
///
/// This struct encapsulates the XRPL's `STNumber` serialization format used for
/// float arithmetic in WASM host functions. The format is the standard XRP Ledger
/// `STNumber` serialization (12 bytes).
///
/// # Important
///
/// This type is intentionally opaque - arithmetic operations MUST be performed through
/// host functions (float_add, float_multiply, etc.) which use rippled's Number class
/// to ensure exact compatibility with XRPL consensus rules.
///
/// ## Derived Traits
///
/// - `Copy`: Efficient for this 12-byte struct, enabling implicit copying
/// - `PartialEq, Eq`: Enable comparisons (bitwise comparison only)
/// - `Debug, Clone`: Standard traits for development and consistency
///
/// **Note**: `PartialEq` and `Eq` perform bitwise comparison only. For semantic
/// comparison of amounts (e.g., handling different representations of zero),
/// use host functions.
///
/// # Example
///
/// ```no_run
/// # use xrpl_wasm_stdlib::core::types::opaque_float::OpaqueFloat;
/// // Create from host function
/// let mut float_bytes = [0u8; 12];
/// // float_from_int(100, float_bytes.as_mut_ptr(), 12, 0);
/// let amount = OpaqueFloat(float_bytes);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct OpaqueFloat(pub [u8; 12]);

impl OpaqueFloat {}

impl From<[u8; 12]> for OpaqueFloat {
    fn from(value: [u8; 12]) -> Self {
        OpaqueFloat(value)
    }
}

/// The size of an OpaqueFloat in bytes (STNumber serialization format).
pub const OPAQUE_FLOAT_SIZE: usize = 12;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_bytes() {
        let bytes = [0u8; 12];
        let float = OpaqueFloat::from(bytes);
        assert_eq!(float.0, [0u8; 12]);
    }

    #[test]
    fn test_copy() {
        let a = OpaqueFloat([1u8; 12]);
        let b = a; // Copy
        assert_eq!(a, b);
    }
}
