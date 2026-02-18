use crate::core::params::types::{FuncParamBytes, ParamError};
use crate::core::type_codes::{
    STI_ACCOUNT, STI_AMOUNT, STI_NUMBER, STI_UINT8, STI_UINT16, STI_UINT32, STI_UINT64,
    STI_UINT128, STI_UINT160, STI_UINT192, STI_UINT256,
};
use crate::core::types::account_id::AccountID;
use crate::core::types::amount::Amount;
use crate::core::types::currency::Currency;
use crate::core::types::number::Number;
use crate::core::types::uint::Hash160;
use crate::core::types::uint::Hash192;
use crate::core::types::uint::Hash256;
use crate::host::function_param;
use crate::host::trace::{DataRepr, trace_data, trace_num};

// ============================================================================
// Generic Functions
// ============================================================================

// MISSING: Issue, VL

/// Get parameter of any type implementing FuncParamBytes
pub fn get_function_param<T: FuncParamBytes>(index: i32) -> Result<T, ParamError> {
    let mut buf = [0u8; 48]; // Use maximum size buffer
    let byte_size = T::byte_size();

    let result = unsafe { function_param(index, T::type_code(), buf.as_mut_ptr(), byte_size) };

    if result > 0 {
        T::from_param_bytes(&buf[0..result as usize])
    } else {
        Err(ParamError::NotFound)
    }
}

pub fn safe_get_function_param<T: FuncParamBytes>(index: i32) -> T {
    match get_function_param::<T>(index) {
        Ok(v) => v,
        Err(e) => {
            let msg = T::error_message();
            let _ = trace_data("Message:", msg, DataRepr::AsHex);
            let _ = trace_num("Error Code:", e as i64);
            // exit(-1, msg.as_ptr(), msg.len());
            T::default_value()
        }
    }
}

// ============================================================================
// FuncParamBytes Implementations for Primitive Types
// ============================================================================

impl FuncParamBytes for u8 {
    fn type_code() -> i32 {
        STI_UINT8.into()
    }
    fn byte_size() -> usize {
        1
    }

    fn from_param_bytes(bytes: &[u8]) -> Result<Self, ParamError> {
        if !bytes.is_empty() {
            Ok(bytes[0])
        } else {
            Err(ParamError::InvalidData)
        }
    }

    fn default_value() -> Self {
        0
    }
    fn error_message() -> &'static [u8] {
        b"Required UINT8 parameter not found"
    }
}

impl FuncParamBytes for u16 {
    fn type_code() -> i32 {
        STI_UINT16.into()
    }
    fn byte_size() -> usize {
        2
    }

    fn from_param_bytes(bytes: &[u8]) -> Result<Self, ParamError> {
        if bytes.len() >= 2 {
            Ok(u16::from_le_bytes([bytes[0], bytes[1]]))
        } else {
            Err(ParamError::InvalidData)
        }
    }

    fn default_value() -> Self {
        0
    }
    fn error_message() -> &'static [u8] {
        b"Required UINT16 parameter not found"
    }
}

impl FuncParamBytes for u32 {
    fn type_code() -> i32 {
        STI_UINT32.into()
    }
    fn byte_size() -> usize {
        4
    }

    fn from_param_bytes(bytes: &[u8]) -> Result<Self, ParamError> {
        if bytes.len() >= 4 {
            let mut buf = [0u8; 4];
            buf.copy_from_slice(&bytes[0..4]);
            Ok(u32::from_le_bytes(buf))
        } else {
            Err(ParamError::InvalidData)
        }
    }

    fn default_value() -> Self {
        0
    }
    fn error_message() -> &'static [u8] {
        b"Required UINT32 parameter not found"
    }
}

impl FuncParamBytes for u64 {
    fn type_code() -> i32 {
        STI_UINT64.into()
    }
    fn byte_size() -> usize {
        8
    }

    fn from_param_bytes(bytes: &[u8]) -> Result<Self, ParamError> {
        if bytes.len() >= 8 {
            let mut buf = [0u8; 8];
            buf.copy_from_slice(&bytes[0..8]);
            Ok(u64::from_le_bytes(buf))
        } else {
            Err(ParamError::InvalidData)
        }
    }

    fn default_value() -> Self {
        0
    }
    fn error_message() -> &'static [u8] {
        b"Required UINT64 parameter not found"
    }
}

impl FuncParamBytes for u128 {
    fn type_code() -> i32 {
        STI_UINT128.into()
    }
    fn byte_size() -> usize {
        16
    }

    fn from_param_bytes(bytes: &[u8]) -> Result<Self, ParamError> {
        if bytes.len() >= 16 {
            let mut buf = [0u8; 16];
            buf.copy_from_slice(&bytes[0..16]);
            Ok(u128::from_le_bytes(buf))
        } else {
            Err(ParamError::InvalidData)
        }
    }

    fn default_value() -> Self {
        0
    }
    fn error_message() -> &'static [u8] {
        b"Required UINT128 parameter not found"
    }
}

impl FuncParamBytes for Hash160 {
    fn type_code() -> i32 {
        STI_UINT160.into()
    }
    fn byte_size() -> usize {
        20
    }

    fn from_param_bytes(bytes: &[u8]) -> Result<Self, ParamError> {
        if bytes.len() >= 20 {
            let mut buf = [0u8; 20];
            buf.copy_from_slice(&bytes[0..20]);
            Ok(Hash160::from(buf))
        } else {
            Err(ParamError::InvalidData)
        }
    }

    fn default_value() -> Self {
        Hash160::from([0u8; 20])
    }
    fn error_message() -> &'static [u8] {
        b"Required UINT160 parameter not found"
    }
}

impl FuncParamBytes for Hash192 {
    fn type_code() -> i32 {
        STI_UINT192.into()
    }
    fn byte_size() -> usize {
        24
    }

    fn from_param_bytes(bytes: &[u8]) -> Result<Self, ParamError> {
        if bytes.len() >= 20 {
            let mut buf = [0u8; 24];
            buf.copy_from_slice(&bytes[0..24]);
            Ok(Hash192::from(buf))
        } else {
            Err(ParamError::InvalidData)
        }
    }

    fn default_value() -> Self {
        Hash192::from([0u8; 24])
    }
    fn error_message() -> &'static [u8] {
        b"Required UINT192 parameter not found"
    }
}

impl FuncParamBytes for Hash256 {
    fn type_code() -> i32 {
        STI_UINT256.into()
    }
    fn byte_size() -> usize {
        32
    }

    fn from_param_bytes(bytes: &[u8]) -> Result<Self, ParamError> {
        if bytes.len() >= 32 {
            let mut buf = [0u8; 32];
            buf.copy_from_slice(&bytes[0..32]);
            Ok(Hash256::from(buf))
        } else {
            Err(ParamError::InvalidData)
        }
    }

    fn default_value() -> Self {
        Hash256::from([0u8; 32])
    }
    fn error_message() -> &'static [u8] {
        b"Required UINT256 parameter not found"
    }
}

impl FuncParamBytes for AccountID {
    fn type_code() -> i32 {
        STI_ACCOUNT.into()
    }
    fn byte_size() -> usize {
        20
    }

    fn from_param_bytes(bytes: &[u8]) -> Result<Self, ParamError> {
        if bytes.len() >= 20 {
            let mut buf = [0u8; 20];
            buf.copy_from_slice(&bytes[0..20]);
            Ok(AccountID::from(buf))
        } else {
            Err(ParamError::InvalidData)
        }
    }

    fn default_value() -> Self {
        AccountID::from([0u8; 20])
    }
    fn error_message() -> &'static [u8] {
        b"Required Account parameter not found"
    }
}

impl FuncParamBytes for Currency {
    fn type_code() -> i32 {
        STI_ACCOUNT.into()
    }
    fn byte_size() -> usize {
        20
    }

    fn from_param_bytes(bytes: &[u8]) -> Result<Self, ParamError> {
        if bytes.len() >= 20 {
            let mut buf = [0u8; 20];
            buf.copy_from_slice(&bytes[0..20]);
            Ok(Currency::from(buf))
        } else {
            Err(ParamError::InvalidData)
        }
    }

    fn default_value() -> Self {
        Currency::from([0u8; 20])
    }
    fn error_message() -> &'static [u8] {
        b"Required Currency parameter not found"
    }
}

impl FuncParamBytes for Amount {
    fn type_code() -> i32 {
        STI_AMOUNT.into()
    }
    fn byte_size() -> usize {
        48
    }

    fn from_param_bytes(bytes: &[u8]) -> Result<Self, ParamError> {
        match Amount::from_bytes(bytes) {
            crate::host::Result::Ok(amount) => Ok(amount),
            crate::host::Result::Err(_) => Err(ParamError::InvalidData),
        }
    }

    fn default_value() -> Self {
        Amount::XRP { num_drops: 0 }
    }
    fn error_message() -> &'static [u8] {
        b"Required Amount parameter not found"
    }
}

impl FuncParamBytes for Number {
    fn type_code() -> i32 {
        STI_NUMBER.into()
    }
    fn byte_size() -> usize {
        12
    }

    fn from_param_bytes(bytes: &[u8]) -> Result<Self, ParamError> {
        if bytes.len() >= 12 {
            let mut buf = [0u8; 12];
            buf.copy_from_slice(&bytes[0..12]);
            Ok(Number::from(&buf))
        } else {
            Err(ParamError::InvalidData)
        }
    }

    fn default_value() -> Self {
        Number::from(&[0u8; 12])
    }
    fn error_message() -> &'static [u8] {
        b"Required Number parameter not found"
    }
}
