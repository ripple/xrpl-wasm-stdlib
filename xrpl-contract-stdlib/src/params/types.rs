//! Parameter Types Module
//! Core types for parameter handling

// ============================================================================
// Error Type
// ============================================================================

/// Error type for parameter retrieval
#[derive(Debug)]
pub enum ParamError {
    NotFound = -1,
    InvalidType = -2,
    BufferTooSmall = -3,
    InvalidData = -4,
}

/// Trait for types that can be deserialized from function parameter bytes
pub trait FuncParamBytes: Sized {
    /// The type code for this parameter type
    fn type_code() -> i32;

    /// The expected byte size for this parameter type
    fn byte_size() -> usize;

    /// Deserialize from bytes
    fn from_param_bytes(bytes: &[u8]) -> Result<Self, ParamError>;

    /// Default value to return on error (for require_ variants)
    fn default_value() -> Self;

    /// Error message when parameter is not found
    fn error_message() -> &'static [u8];
}

/// Trait for types that can be deserialized from instance parameter bytes
pub trait InstParamBytes: Sized {
    /// The type code for this parameter type
    fn type_code() -> i32;

    /// The expected byte size for this parameter type
    fn byte_size() -> usize;

    /// Deserialize from bytes
    fn from_param_bytes(bytes: &[u8]) -> Result<Self, ParamError>;

    /// Default value to return on error (for require_ variants)
    fn default_value() -> Self;

    /// Error message when parameter is not found
    fn error_message() -> &'static [u8];
}
