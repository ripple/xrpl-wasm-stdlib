use xrpl_wasm_stdlib::core::type_codes::{
    STI_ACCOUNT, STI_AMOUNT, STI_UINT8, STI_UINT16, STI_UINT32, STI_UINT64, STI_UINT128,
    STI_UINT160, STI_UINT256,
};
use xrpl_wasm_stdlib::core::types::account_id::AccountID;
use xrpl_wasm_stdlib::host::{
    get_data_array_element_field, get_data_nested_array_element_field,
    get_data_nested_object_field, get_data_object_field, set_data_array_element_field,
    set_data_nested_array_element_field, set_data_nested_object_field, set_data_object_field,
};

// TODO: STI_VL, STI_CURRENCY, STI_ISSUE

// ============================================================================
// Key type trait - allows both &str and &[u8] to be used as keys
// ============================================================================

pub trait AsKeyBytes {
    fn as_key_bytes(&self) -> &[u8];
}

impl AsKeyBytes for str {
    #[inline]
    fn as_key_bytes(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl AsKeyBytes for [u8] {
    #[inline]
    fn as_key_bytes(&self) -> &[u8] {
        self
    }
}

impl AsKeyBytes for &str {
    #[inline]
    fn as_key_bytes(&self) -> &[u8] {
        str::as_bytes(self)
    }
}

impl AsKeyBytes for &[u8] {
    #[inline]
    fn as_key_bytes(&self) -> &[u8] {
        self
    }
}

// ============================================================================
// Generic Traits for Contract Data
// ============================================================================

/// Trait for types that can be read from contract data
/// Each implementation uses exact buffer size for maximum efficiency
/// All key parameters use &[u8] for maximum efficiency
pub trait FromDataBytes: Sized {
    fn get_data(account: &AccountID, key: &[u8]) -> Option<Self>;
    fn get_nested_data(account: &AccountID, nested: &[u8], key: &[u8]) -> Option<Self>;
    fn get_array_element(account: &AccountID, key: &[u8], index: usize) -> Option<Self>;
    fn get_nested_array_element(
        account: &AccountID,
        key: &[u8],
        index: usize,
        field_key: &[u8],
    ) -> Option<Self>;
}

/// Trait for types that can be written to contract data
/// Each implementation includes STI type code prefix
/// All key parameters use &[u8] for maximum efficiency
pub trait ToDataBytes {
    fn set_data(&self, account: &AccountID, key: &[u8]) -> Result<(), i32>;
    fn set_nested_data(&self, account: &AccountID, nested: &[u8], key: &[u8]) -> Result<(), i32>;
    fn set_array_element(&self, account: &AccountID, key: &[u8], index: usize) -> Result<(), i32>;
    fn set_nested_array_element(
        &self,
        account: &AccountID,
        key: &[u8],
        index: usize,
        field_key: &[u8],
    ) -> Result<(), i32>;
}

// ============================================================================
// Generic Wrapper Functions - Accept both &str and &[u8] as keys
// ============================================================================

#[inline]
pub fn get_data<T: FromDataBytes>(
    account: &AccountID,
    key: &(impl AsKeyBytes + ?Sized),
) -> Option<T> {
    T::get_data(account, key.as_key_bytes())
}

#[inline]
pub fn get_nested_data<T: FromDataBytes>(
    account: &AccountID,
    nested: &(impl AsKeyBytes + ?Sized),
    key: &(impl AsKeyBytes + ?Sized),
) -> Option<T> {
    T::get_nested_data(account, nested.as_key_bytes(), key.as_key_bytes())
}

#[inline]
pub fn get_array_element<T: FromDataBytes>(
    account: &AccountID,
    key: &(impl AsKeyBytes + ?Sized),
    index: usize,
) -> Option<T> {
    T::get_array_element(account, key.as_key_bytes(), index)
}

#[inline]
pub fn get_nested_array_element<T: FromDataBytes>(
    account: &AccountID,
    key: &(impl AsKeyBytes + ?Sized),
    index: usize,
    field_key: &(impl AsKeyBytes + ?Sized),
) -> Option<T> {
    T::get_nested_array_element(account, key.as_key_bytes(), index, field_key.as_key_bytes())
}

#[inline]
pub fn set_data<T: ToDataBytes>(
    account: &AccountID,
    key: &(impl AsKeyBytes + ?Sized),
    value: T,
) -> Result<(), i32> {
    value.set_data(account, key.as_key_bytes())
}

#[inline]
pub fn set_nested_data<T: ToDataBytes>(
    account: &AccountID,
    nested: &(impl AsKeyBytes + ?Sized),
    key: &(impl AsKeyBytes + ?Sized),
    value: T,
) -> Result<(), i32> {
    value.set_nested_data(account, nested.as_key_bytes(), key.as_key_bytes())
}

#[inline]
pub fn set_array_element<T: ToDataBytes>(
    account: &AccountID,
    key: &(impl AsKeyBytes + ?Sized),
    index: usize,
    value: T,
) -> Result<(), i32> {
    value.set_array_element(account, key.as_key_bytes(), index)
}

#[inline]
pub fn set_nested_array_element<T: ToDataBytes>(
    account: &AccountID,
    key: &(impl AsKeyBytes + ?Sized),
    index: usize,
    field_key: &(impl AsKeyBytes + ?Sized),
    value: T,
) -> Result<(), i32> {
    value.set_nested_array_element(account, key.as_key_bytes(), index, field_key.as_key_bytes())
}

// ============================================================================
// FromDataBytes Implementation for u8 (STI_UINT8)
// ============================================================================

impl FromDataBytes for u8 {
    #[inline]
    fn get_data(account: &AccountID, key: &[u8]) -> Option<Self> {
        let mut buffer = [0u8; 1];
        let output_len = unsafe {
            get_data_object_field(
                account.0.as_ptr(),
                account.0.len(),
                key.as_ptr(),
                key.len(),
                buffer.as_mut_ptr(),
                1,
            )
        };
        if output_len == 1 {
            Some(buffer[0])
        } else {
            None
        }
    }

    #[inline]
    fn get_nested_data(account: &AccountID, nested: &[u8], key: &[u8]) -> Option<Self> {
        let mut buffer = [0u8; 1];
        let output_len = unsafe {
            get_data_nested_object_field(
                account.0.as_ptr(),
                account.0.len(),
                nested.as_ptr(),
                nested.len(),
                key.as_ptr(),
                key.len(),
                buffer.as_mut_ptr(),
                1,
            )
        };
        if output_len == 1 {
            Some(buffer[0])
        } else {
            None
        }
    }

    #[inline]
    fn get_array_element(account: &AccountID, key: &[u8], index: usize) -> Option<Self> {
        let mut buffer = [0u8; 1];
        let output_len = unsafe {
            get_data_array_element_field(
                account.0.as_ptr(),
                account.0.len(),
                key.as_ptr(),
                key.len(),
                index as i32,
                buffer.as_mut_ptr(),
                1,
            )
        };
        if output_len == 1 {
            Some(buffer[0])
        } else {
            None
        }
    }

    #[inline]
    fn get_nested_array_element(
        account: &AccountID,
        key: &[u8],
        index: usize,
        field_key: &[u8],
    ) -> Option<Self> {
        let mut buffer = [0u8; 1];
        let output_len = unsafe {
            get_data_nested_array_element_field(
                account.0.as_ptr(),
                account.0.len(),
                key.as_ptr(),
                key.len(),
                index as i32,
                field_key.as_ptr(),
                field_key.len(),
                buffer.as_mut_ptr(),
                1,
            )
        };
        if output_len == 1 {
            Some(buffer[0])
        } else {
            None
        }
    }
}

// ============================================================================
// ToDataBytes Implementation for u8 (STI_UINT8)
// ============================================================================

impl ToDataBytes for u8 {
    #[inline]
    fn set_data(&self, account: &AccountID, key: &[u8]) -> Result<(), i32> {
        let buffer = [STI_UINT8, *self];
        unsafe {
            set_data_object_field(
                account.0.as_ptr(),
                account.0.len(),
                key.as_ptr(),
                key.len(),
                buffer.as_ptr(),
                2,
            );
        }
        Ok(())
    }

    #[inline]
    fn set_nested_data(&self, account: &AccountID, nested: &[u8], key: &[u8]) -> Result<(), i32> {
        let buffer = [STI_UINT8, *self];
        unsafe {
            set_data_nested_object_field(
                account.0.as_ptr(),
                account.0.len(),
                nested.as_ptr(),
                nested.len(),
                key.as_ptr(),
                key.len(),
                buffer.as_ptr(),
                2,
            );
        }
        Ok(())
    }

    #[inline]
    fn set_array_element(&self, account: &AccountID, key: &[u8], index: usize) -> Result<(), i32> {
        let buffer = [STI_UINT8, *self];
        unsafe {
            set_data_array_element_field(
                account.0.as_ptr(),
                account.0.len(),
                key.as_ptr(),
                key.len(),
                index as i32,
                buffer.as_ptr(),
                2,
            );
        }
        Ok(())
    }

    #[inline]
    fn set_nested_array_element(
        &self,
        account: &AccountID,
        key: &[u8],
        index: usize,
        field_key: &[u8],
    ) -> Result<(), i32> {
        let buffer = [STI_UINT8, *self];
        unsafe {
            set_data_nested_array_element_field(
                account.0.as_ptr(),
                account.0.len(),
                key.as_ptr(),
                key.len(),
                index as i32,
                field_key.as_ptr(),
                field_key.len(),
                buffer.as_ptr(),
                2,
            );
        }
        Ok(())
    }
}

// ============================================================================
// FromDataBytes Implementation for u16 (STI_UINT16)
// ============================================================================

impl FromDataBytes for u16 {
    #[inline]
    fn get_data(account: &AccountID, key: &[u8]) -> Option<Self> {
        let mut buffer = [0u8; 2];
        let output_len = unsafe {
            get_data_object_field(
                account.0.as_ptr(),
                account.0.len(),
                key.as_ptr(),
                key.len(),
                buffer.as_mut_ptr(),
                2,
            )
        };
        if output_len == 2 {
            Some(u16::from_be_bytes(buffer))
        } else {
            None
        }
    }

    #[inline]
    fn get_nested_data(account: &AccountID, nested: &[u8], key: &[u8]) -> Option<Self> {
        let mut buffer = [0u8; 2];
        let output_len = unsafe {
            get_data_nested_object_field(
                account.0.as_ptr(),
                account.0.len(),
                nested.as_ptr(),
                nested.len(),
                key.as_ptr(),
                key.len(),
                buffer.as_mut_ptr(),
                2,
            )
        };
        if output_len == 2 {
            Some(u16::from_be_bytes(buffer))
        } else {
            None
        }
    }

    #[inline]
    fn get_array_element(account: &AccountID, key: &[u8], index: usize) -> Option<Self> {
        let mut buffer = [0u8; 2];
        let output_len = unsafe {
            get_data_array_element_field(
                account.0.as_ptr(),
                account.0.len(),
                key.as_ptr(),
                key.len(),
                index as i32,
                buffer.as_mut_ptr(),
                2,
            )
        };
        if output_len == 2 {
            Some(u16::from_be_bytes(buffer))
        } else {
            None
        }
    }

    #[inline]
    fn get_nested_array_element(
        account: &AccountID,
        key: &[u8],
        index: usize,
        field_key: &[u8],
    ) -> Option<Self> {
        let mut buffer = [0u8; 2];
        let output_len = unsafe {
            get_data_nested_array_element_field(
                account.0.as_ptr(),
                account.0.len(),
                key.as_ptr(),
                key.len(),
                index as i32,
                field_key.as_ptr(),
                field_key.len(),
                buffer.as_mut_ptr(),
                2,
            )
        };
        if output_len == 2 {
            Some(u16::from_be_bytes(buffer))
        } else {
            None
        }
    }
}

// ============================================================================
// ToDataBytes Implementation for u16 (STI_UINT16)
// ============================================================================

impl ToDataBytes for u16 {
    #[inline]
    fn set_data(&self, account: &AccountID, key: &[u8]) -> Result<(), i32> {
        let be_bytes = self.to_be_bytes();
        let buffer = [STI_UINT16, be_bytes[0], be_bytes[1]];
        unsafe {
            set_data_object_field(
                account.0.as_ptr(),
                account.0.len(),
                key.as_ptr(),
                key.len(),
                buffer.as_ptr(),
                3,
            );
        }
        Ok(())
    }

    #[inline]
    fn set_nested_data(&self, account: &AccountID, nested: &[u8], key: &[u8]) -> Result<(), i32> {
        let be_bytes = self.to_be_bytes();
        let buffer = [STI_UINT16, be_bytes[0], be_bytes[1]];
        unsafe {
            set_data_nested_object_field(
                account.0.as_ptr(),
                account.0.len(),
                nested.as_ptr(),
                nested.len(),
                key.as_ptr(),
                key.len(),
                buffer.as_ptr(),
                3,
            );
        }
        Ok(())
    }

    #[inline]
    fn set_array_element(&self, account: &AccountID, key: &[u8], index: usize) -> Result<(), i32> {
        let be_bytes = self.to_be_bytes();
        let buffer = [STI_UINT16, be_bytes[0], be_bytes[1]];
        unsafe {
            set_data_array_element_field(
                account.0.as_ptr(),
                account.0.len(),
                key.as_ptr(),
                key.len(),
                index as i32,
                buffer.as_ptr(),
                3,
            );
        }
        Ok(())
    }

    #[inline]
    fn set_nested_array_element(
        &self,
        account: &AccountID,
        key: &[u8],
        index: usize,
        field_key: &[u8],
    ) -> Result<(), i32> {
        let be_bytes = self.to_be_bytes();
        let buffer = [STI_UINT16, be_bytes[0], be_bytes[1]];
        unsafe {
            set_data_nested_array_element_field(
                account.0.as_ptr(),
                account.0.len(),
                key.as_ptr(),
                key.len(),
                index as i32,
                field_key.as_ptr(),
                field_key.len(),
                buffer.as_ptr(),
                3,
            );
        }
        Ok(())
    }
}

// ============================================================================
// FromDataBytes Implementation for u32 (STI_UINT32)
// ============================================================================

impl FromDataBytes for u32 {
    #[inline]
    fn get_data(account: &AccountID, key: &[u8]) -> Option<Self> {
        let mut buffer = [0u8; 4];
        let output_len = unsafe {
            get_data_object_field(
                account.0.as_ptr(),
                account.0.len(),
                key.as_ptr(),
                key.len(),
                buffer.as_mut_ptr(),
                4,
            )
        };
        if output_len == 4 {
            Some(u32::from_be_bytes(buffer))
        } else {
            None
        }
    }

    #[inline]
    fn get_nested_data(account: &AccountID, nested: &[u8], key: &[u8]) -> Option<Self> {
        let mut buffer = [0u8; 4];
        let output_len = unsafe {
            get_data_nested_object_field(
                account.0.as_ptr(),
                account.0.len(),
                nested.as_ptr(),
                nested.len(),
                key.as_ptr(),
                key.len(),
                buffer.as_mut_ptr(),
                4,
            )
        };
        if output_len == 4 {
            Some(u32::from_be_bytes(buffer))
        } else {
            None
        }
    }

    #[inline]
    fn get_array_element(account: &AccountID, key: &[u8], index: usize) -> Option<Self> {
        let mut buffer = [0u8; 4];
        let output_len = unsafe {
            get_data_array_element_field(
                account.0.as_ptr(),
                account.0.len(),
                key.as_ptr(),
                key.len(),
                index as i32,
                buffer.as_mut_ptr(),
                4,
            )
        };
        if output_len == 4 {
            Some(u32::from_be_bytes(buffer))
        } else {
            None
        }
    }

    #[inline]
    fn get_nested_array_element(
        account: &AccountID,
        key: &[u8],
        index: usize,
        field_key: &[u8],
    ) -> Option<Self> {
        let mut buffer = [0u8; 4];
        let output_len = unsafe {
            get_data_nested_array_element_field(
                account.0.as_ptr(),
                account.0.len(),
                key.as_ptr(),
                key.len(),
                index as i32,
                field_key.as_ptr(),
                field_key.len(),
                buffer.as_mut_ptr(),
                4,
            )
        };
        if output_len == 4 {
            Some(u32::from_be_bytes(buffer))
        } else {
            None
        }
    }
}

// ============================================================================
// ToDataBytes Implementation for u32 (STI_UINT32)
// ============================================================================

impl ToDataBytes for u32 {
    #[inline]
    fn set_data(&self, account: &AccountID, key: &[u8]) -> Result<(), i32> {
        let be_bytes = self.to_be_bytes();
        let buffer = [
            STI_UINT32,
            be_bytes[0],
            be_bytes[1],
            be_bytes[2],
            be_bytes[3],
        ];
        unsafe {
            set_data_object_field(
                account.0.as_ptr(),
                account.0.len(),
                key.as_ptr(),
                key.len(),
                buffer.as_ptr(),
                5,
            );
        }
        Ok(())
    }

    #[inline]
    fn set_nested_data(&self, account: &AccountID, nested: &[u8], key: &[u8]) -> Result<(), i32> {
        let be_bytes = self.to_be_bytes();
        let buffer = [
            STI_UINT32,
            be_bytes[0],
            be_bytes[1],
            be_bytes[2],
            be_bytes[3],
        ];
        unsafe {
            set_data_nested_object_field(
                account.0.as_ptr(),
                account.0.len(),
                nested.as_ptr(),
                nested.len(),
                key.as_ptr(),
                key.len(),
                buffer.as_ptr(),
                5,
            );
        }
        Ok(())
    }

    #[inline]
    fn set_array_element(&self, account: &AccountID, key: &[u8], index: usize) -> Result<(), i32> {
        let be_bytes = self.to_be_bytes();
        let buffer = [
            STI_UINT32,
            be_bytes[0],
            be_bytes[1],
            be_bytes[2],
            be_bytes[3],
        ];
        unsafe {
            set_data_array_element_field(
                account.0.as_ptr(),
                account.0.len(),
                key.as_ptr(),
                key.len(),
                index as i32,
                buffer.as_ptr(),
                5,
            );
        }
        Ok(())
    }

    #[inline]
    fn set_nested_array_element(
        &self,
        account: &AccountID,
        key: &[u8],
        index: usize,
        field_key: &[u8],
    ) -> Result<(), i32> {
        let be_bytes = self.to_be_bytes();
        let buffer = [
            STI_UINT32,
            be_bytes[0],
            be_bytes[1],
            be_bytes[2],
            be_bytes[3],
        ];
        unsafe {
            set_data_nested_array_element_field(
                account.0.as_ptr(),
                account.0.len(),
                key.as_ptr(),
                key.len(),
                index as i32,
                field_key.as_ptr(),
                field_key.len(),
                buffer.as_ptr(),
                5,
            );
        }
        Ok(())
    }
}

// ============================================================================
// FromDataBytes Implementation for u64 (STI_UINT64)
// ============================================================================

impl FromDataBytes for u64 {
    #[inline]
    fn get_data(account: &AccountID, key: &[u8]) -> Option<Self> {
        let mut buffer = [0u8; 8];
        let output_len = unsafe {
            get_data_object_field(
                account.0.as_ptr(),
                account.0.len(),
                key.as_ptr(),
                key.len(),
                buffer.as_mut_ptr(),
                8,
            )
        };
        if output_len == 8 {
            Some(u64::from_be_bytes(buffer))
        } else {
            None
        }
    }

    #[inline]
    fn get_nested_data(account: &AccountID, nested: &[u8], key: &[u8]) -> Option<Self> {
        let mut buffer = [0u8; 8];
        let output_len = unsafe {
            get_data_nested_object_field(
                account.0.as_ptr(),
                account.0.len(),
                nested.as_ptr(),
                nested.len(),
                key.as_ptr(),
                key.len(),
                buffer.as_mut_ptr(),
                8,
            )
        };
        if output_len == 8 {
            Some(u64::from_be_bytes(buffer))
        } else {
            None
        }
    }

    #[inline]
    fn get_array_element(account: &AccountID, key: &[u8], index: usize) -> Option<Self> {
        let mut buffer = [0u8; 8];
        let output_len = unsafe {
            get_data_array_element_field(
                account.0.as_ptr(),
                account.0.len(),
                key.as_ptr(),
                key.len(),
                index as i32,
                buffer.as_mut_ptr(),
                8,
            )
        };
        if output_len == 8 {
            Some(u64::from_be_bytes(buffer))
        } else {
            None
        }
    }

    #[inline]
    fn get_nested_array_element(
        account: &AccountID,
        key: &[u8],
        index: usize,
        field_key: &[u8],
    ) -> Option<Self> {
        let mut buffer = [0u8; 8];
        let output_len = unsafe {
            get_data_nested_array_element_field(
                account.0.as_ptr(),
                account.0.len(),
                key.as_ptr(),
                key.len(),
                index as i32,
                field_key.as_ptr(),
                field_key.len(),
                buffer.as_mut_ptr(),
                8,
            )
        };
        if output_len == 8 {
            Some(u64::from_be_bytes(buffer))
        } else {
            None
        }
    }
}

// ============================================================================
// ToDataBytes Implementation for u64 (STI_UINT64)
// ============================================================================

impl ToDataBytes for u64 {
    #[inline]
    fn set_data(&self, account: &AccountID, key: &[u8]) -> Result<(), i32> {
        let mut buffer = [0u8; 9];
        buffer[0] = STI_UINT64;
        let be_bytes = self.to_be_bytes();
        let mut i = 0;
        while i < 8 {
            buffer[i + 1] = be_bytes[i];
            i += 1;
        }
        unsafe {
            set_data_object_field(
                account.0.as_ptr(),
                account.0.len(),
                key.as_ptr(),
                key.len(),
                buffer.as_ptr(),
                9,
            );
        }
        Ok(())
    }

    #[inline]
    fn set_nested_data(&self, account: &AccountID, nested: &[u8], key: &[u8]) -> Result<(), i32> {
        let mut buffer = [0u8; 9];
        buffer[0] = STI_UINT64;
        let be_bytes = self.to_be_bytes();
        let mut i = 0;
        while i < 8 {
            buffer[i + 1] = be_bytes[i];
            i += 1;
        }
        unsafe {
            set_data_nested_object_field(
                account.0.as_ptr(),
                account.0.len(),
                nested.as_ptr(),
                nested.len(),
                key.as_ptr(),
                key.len(),
                buffer.as_ptr(),
                9,
            );
        }
        Ok(())
    }

    #[inline]
    fn set_array_element(&self, account: &AccountID, key: &[u8], index: usize) -> Result<(), i32> {
        let mut buffer = [0u8; 9];
        buffer[0] = STI_UINT64;
        let be_bytes = self.to_be_bytes();
        let mut i = 0;
        while i < 8 {
            buffer[i + 1] = be_bytes[i];
            i += 1;
        }
        unsafe {
            set_data_array_element_field(
                account.0.as_ptr(),
                account.0.len(),
                key.as_ptr(),
                key.len(),
                index as i32,
                buffer.as_ptr(),
                9,
            );
        }
        Ok(())
    }

    #[inline]
    fn set_nested_array_element(
        &self,
        account: &AccountID,
        key: &[u8],
        index: usize,
        field_key: &[u8],
    ) -> Result<(), i32> {
        let mut buffer = [0u8; 9];
        buffer[0] = STI_UINT64;
        let be_bytes = self.to_be_bytes();
        let mut i = 0;
        while i < 8 {
            buffer[i + 1] = be_bytes[i];
            i += 1;
        }
        unsafe {
            set_data_nested_array_element_field(
                account.0.as_ptr(),
                account.0.len(),
                key.as_ptr(),
                key.len(),
                index as i32,
                field_key.as_ptr(),
                field_key.len(),
                buffer.as_ptr(),
                9,
            );
        }
        Ok(())
    }
}

// ============================================================================
// FromDataBytes Implementation for u128 (STI_UINT128)
// ============================================================================

impl FromDataBytes for u128 {
    #[inline]
    fn get_data(account: &AccountID, key: &[u8]) -> Option<Self> {
        let mut buffer = [0u8; 16];
        let output_len = unsafe {
            get_data_object_field(
                account.0.as_ptr(),
                account.0.len(),
                key.as_ptr(),
                key.len(),
                buffer.as_mut_ptr(),
                16,
            )
        };
        if output_len == 16 {
            Some(u128::from_be_bytes(buffer))
        } else {
            None
        }
    }

    #[inline]
    fn get_nested_data(account: &AccountID, nested: &[u8], key: &[u8]) -> Option<Self> {
        let mut buffer = [0u8; 16];
        let output_len = unsafe {
            get_data_nested_object_field(
                account.0.as_ptr(),
                account.0.len(),
                nested.as_ptr(),
                nested.len(),
                key.as_ptr(),
                key.len(),
                buffer.as_mut_ptr(),
                16,
            )
        };
        if output_len == 16 {
            Some(u128::from_be_bytes(buffer))
        } else {
            None
        }
    }

    #[inline]
    fn get_array_element(account: &AccountID, key: &[u8], index: usize) -> Option<Self> {
        let mut buffer = [0u8; 16];
        let output_len = unsafe {
            get_data_array_element_field(
                account.0.as_ptr(),
                account.0.len(),
                key.as_ptr(),
                key.len(),
                index as i32,
                buffer.as_mut_ptr(),
                16,
            )
        };
        if output_len == 16 {
            Some(u128::from_be_bytes(buffer))
        } else {
            None
        }
    }

    #[inline]
    fn get_nested_array_element(
        account: &AccountID,
        key: &[u8],
        index: usize,
        field_key: &[u8],
    ) -> Option<Self> {
        let mut buffer = [0u8; 16];
        let output_len = unsafe {
            get_data_nested_array_element_field(
                account.0.as_ptr(),
                account.0.len(),
                key.as_ptr(),
                key.len(),
                index as i32,
                field_key.as_ptr(),
                field_key.len(),
                buffer.as_mut_ptr(),
                16,
            )
        };
        if output_len == 16 {
            Some(u128::from_be_bytes(buffer))
        } else {
            None
        }
    }
}

// ============================================================================
// ToDataBytes Implementation for u128 (STI_UINT128)
// ============================================================================

impl ToDataBytes for u128 {
    #[inline]
    fn set_data(&self, account: &AccountID, key: &[u8]) -> Result<(), i32> {
        let mut buffer = [0u8; 17];
        buffer[0] = STI_UINT128;
        let be_bytes = self.to_be_bytes();
        let mut i = 0;
        while i < 16 {
            buffer[i + 1] = be_bytes[i];
            i += 1;
        }
        unsafe {
            set_data_object_field(
                account.0.as_ptr(),
                account.0.len(),
                key.as_ptr(),
                key.len(),
                buffer.as_ptr(),
                17,
            );
        }
        Ok(())
    }

    #[inline]
    fn set_nested_data(&self, account: &AccountID, nested: &[u8], key: &[u8]) -> Result<(), i32> {
        let mut buffer = [0u8; 17];
        buffer[0] = STI_UINT128;
        let be_bytes = self.to_be_bytes();
        let mut i = 0;
        while i < 16 {
            buffer[i + 1] = be_bytes[i];
            i += 1;
        }
        unsafe {
            set_data_nested_object_field(
                account.0.as_ptr(),
                account.0.len(),
                nested.as_ptr(),
                nested.len(),
                key.as_ptr(),
                key.len(),
                buffer.as_ptr(),
                17,
            );
        }
        Ok(())
    }

    #[inline]
    fn set_array_element(&self, account: &AccountID, key: &[u8], index: usize) -> Result<(), i32> {
        let mut buffer = [0u8; 17];
        buffer[0] = STI_UINT128;
        let be_bytes = self.to_be_bytes();
        let mut i = 0;
        while i < 16 {
            buffer[i + 1] = be_bytes[i];
            i += 1;
        }
        unsafe {
            set_data_array_element_field(
                account.0.as_ptr(),
                account.0.len(),
                key.as_ptr(),
                key.len(),
                index as i32,
                buffer.as_ptr(),
                17,
            );
        }
        Ok(())
    }

    #[inline]
    fn set_nested_array_element(
        &self,
        account: &AccountID,
        key: &[u8],
        index: usize,
        field_key: &[u8],
    ) -> Result<(), i32> {
        let mut buffer = [0u8; 17];
        buffer[0] = STI_UINT128;
        let be_bytes = self.to_be_bytes();
        let mut i = 0;
        while i < 16 {
            buffer[i + 1] = be_bytes[i];
            i += 1;
        }
        unsafe {
            set_data_nested_array_element_field(
                account.0.as_ptr(),
                account.0.len(),
                key.as_ptr(),
                key.len(),
                index as i32,
                field_key.as_ptr(),
                field_key.len(),
                buffer.as_ptr(),
                17,
            );
        }
        Ok(())
    }
}

// ============================================================================
// FromDataBytes Implementation for [u8; 20] (STI_UINT160)
// ============================================================================

impl FromDataBytes for [u8; 20] {
    #[inline]
    fn get_data(account: &AccountID, key: &[u8]) -> Option<Self> {
        let mut buffer = [0u8; 20];
        let output_len = unsafe {
            get_data_object_field(
                account.0.as_ptr(),
                account.0.len(),
                key.as_ptr(),
                key.len(),
                buffer.as_mut_ptr(),
                20,
            )
        };
        if output_len == 20 { Some(buffer) } else { None }
    }

    #[inline]
    fn get_nested_data(account: &AccountID, nested: &[u8], key: &[u8]) -> Option<Self> {
        let mut buffer = [0u8; 20];
        let output_len = unsafe {
            get_data_nested_object_field(
                account.0.as_ptr(),
                account.0.len(),
                nested.as_ptr(),
                nested.len(),
                key.as_ptr(),
                key.len(),
                buffer.as_mut_ptr(),
                20,
            )
        };
        if output_len == 20 { Some(buffer) } else { None }
    }

    #[inline]
    fn get_array_element(account: &AccountID, key: &[u8], index: usize) -> Option<Self> {
        let mut buffer = [0u8; 20];
        let output_len = unsafe {
            get_data_array_element_field(
                account.0.as_ptr(),
                account.0.len(),
                key.as_ptr(),
                key.len(),
                index as i32,
                buffer.as_mut_ptr(),
                20,
            )
        };
        if output_len == 20 { Some(buffer) } else { None }
    }

    #[inline]
    fn get_nested_array_element(
        account: &AccountID,
        key: &[u8],
        index: usize,
        field_key: &[u8],
    ) -> Option<Self> {
        let mut buffer = [0u8; 20];
        let output_len = unsafe {
            get_data_nested_array_element_field(
                account.0.as_ptr(),
                account.0.len(),
                key.as_ptr(),
                key.len(),
                index as i32,
                field_key.as_ptr(),
                field_key.len(),
                buffer.as_mut_ptr(),
                20,
            )
        };
        if output_len == 20 { Some(buffer) } else { None }
    }
}

// ============================================================================
// ToDataBytes Implementation for [u8; 20] (STI_UINT160)
// ============================================================================

impl ToDataBytes for [u8; 20] {
    #[inline]
    fn set_data(&self, account: &AccountID, key: &[u8]) -> Result<(), i32> {
        let mut buffer = [0u8; 21];
        buffer[0] = STI_UINT160;
        let mut i = 0;
        while i < 20 {
            buffer[i + 1] = self[i];
            i += 1;
        }
        unsafe {
            set_data_object_field(
                account.0.as_ptr(),
                account.0.len(),
                key.as_ptr(),
                key.len(),
                buffer.as_ptr(),
                21,
            );
        }
        Ok(())
    }

    #[inline]
    fn set_nested_data(&self, account: &AccountID, nested: &[u8], key: &[u8]) -> Result<(), i32> {
        let mut buffer = [0u8; 21];
        buffer[0] = STI_UINT160;
        let mut i = 0;
        while i < 20 {
            buffer[i + 1] = self[i];
            i += 1;
        }
        unsafe {
            set_data_nested_object_field(
                account.0.as_ptr(),
                account.0.len(),
                nested.as_ptr(),
                nested.len(),
                key.as_ptr(),
                key.len(),
                buffer.as_ptr(),
                21,
            );
        }
        Ok(())
    }

    #[inline]
    fn set_array_element(&self, account: &AccountID, key: &[u8], index: usize) -> Result<(), i32> {
        let mut buffer = [0u8; 21];
        buffer[0] = STI_UINT160;
        let mut i = 0;
        while i < 20 {
            buffer[i + 1] = self[i];
            i += 1;
        }
        unsafe {
            set_data_array_element_field(
                account.0.as_ptr(),
                account.0.len(),
                key.as_ptr(),
                key.len(),
                index as i32,
                buffer.as_ptr(),
                21,
            );
        }
        Ok(())
    }

    #[inline]
    fn set_nested_array_element(
        &self,
        account: &AccountID,
        key: &[u8],
        index: usize,
        field_key: &[u8],
    ) -> Result<(), i32> {
        let mut buffer = [0u8; 21];
        buffer[0] = STI_UINT160;
        let mut i = 0;
        while i < 20 {
            buffer[i + 1] = self[i];
            i += 1;
        }
        unsafe {
            set_data_nested_array_element_field(
                account.0.as_ptr(),
                account.0.len(),
                key.as_ptr(),
                key.len(),
                index as i32,
                field_key.as_ptr(),
                field_key.len(),
                buffer.as_ptr(),
                21,
            );
        }
        Ok(())
    }
}

// ============================================================================
// FromDataBytes Implementation for [u8; 32] (STI_UINT256)
// ============================================================================

impl FromDataBytes for [u8; 32] {
    #[inline]
    fn get_data(account: &AccountID, key: &[u8]) -> Option<Self> {
        let mut buffer = [0u8; 32];
        let output_len = unsafe {
            get_data_object_field(
                account.0.as_ptr(),
                account.0.len(),
                key.as_ptr(),
                key.len(),
                buffer.as_mut_ptr(),
                32,
            )
        };
        if output_len == 32 { Some(buffer) } else { None }
    }

    #[inline]
    fn get_nested_data(account: &AccountID, nested: &[u8], key: &[u8]) -> Option<Self> {
        let mut buffer = [0u8; 32];
        let output_len = unsafe {
            get_data_nested_object_field(
                account.0.as_ptr(),
                account.0.len(),
                nested.as_ptr(),
                nested.len(),
                key.as_ptr(),
                key.len(),
                buffer.as_mut_ptr(),
                32,
            )
        };
        if output_len == 32 { Some(buffer) } else { None }
    }

    #[inline]
    fn get_array_element(account: &AccountID, key: &[u8], index: usize) -> Option<Self> {
        let mut buffer = [0u8; 32];
        let output_len = unsafe {
            get_data_array_element_field(
                account.0.as_ptr(),
                account.0.len(),
                key.as_ptr(),
                key.len(),
                index as i32,
                buffer.as_mut_ptr(),
                32,
            )
        };
        if output_len == 32 { Some(buffer) } else { None }
    }

    #[inline]
    fn get_nested_array_element(
        account: &AccountID,
        key: &[u8],
        index: usize,
        field_key: &[u8],
    ) -> Option<Self> {
        let mut buffer = [0u8; 32];
        let output_len = unsafe {
            get_data_nested_array_element_field(
                account.0.as_ptr(),
                account.0.len(),
                key.as_ptr(),
                key.len(),
                index as i32,
                field_key.as_ptr(),
                field_key.len(),
                buffer.as_mut_ptr(),
                32,
            )
        };
        if output_len == 32 { Some(buffer) } else { None }
    }
}

// ============================================================================
// ToDataBytes Implementation for [u8; 32] (STI_UINT256)
// ============================================================================

impl ToDataBytes for [u8; 32] {
    #[inline]
    fn set_data(&self, account: &AccountID, key: &[u8]) -> Result<(), i32> {
        let mut buffer = [0u8; 33];
        buffer[0] = STI_UINT256;
        let mut i = 0;
        while i < 32 {
            buffer[i + 1] = self[i];
            i += 1;
        }
        unsafe {
            set_data_object_field(
                account.0.as_ptr(),
                account.0.len(),
                key.as_ptr(),
                key.len(),
                buffer.as_ptr(),
                33,
            );
        }
        Ok(())
    }

    #[inline]
    fn set_nested_data(&self, account: &AccountID, nested: &[u8], key: &[u8]) -> Result<(), i32> {
        let mut buffer = [0u8; 33];
        buffer[0] = STI_UINT256;
        let mut i = 0;
        while i < 32 {
            buffer[i + 1] = self[i];
            i += 1;
        }
        unsafe {
            set_data_nested_object_field(
                account.0.as_ptr(),
                account.0.len(),
                nested.as_ptr(),
                nested.len(),
                key.as_ptr(),
                key.len(),
                buffer.as_ptr(),
                33,
            );
        }
        Ok(())
    }

    #[inline]
    fn set_array_element(&self, account: &AccountID, key: &[u8], index: usize) -> Result<(), i32> {
        let mut buffer = [0u8; 33];
        buffer[0] = STI_UINT256;
        let mut i = 0;
        while i < 32 {
            buffer[i + 1] = self[i];
            i += 1;
        }
        unsafe {
            set_data_array_element_field(
                account.0.as_ptr(),
                account.0.len(),
                key.as_ptr(),
                key.len(),
                index as i32,
                buffer.as_ptr(),
                33,
            );
        }
        Ok(())
    }

    #[inline]
    fn set_nested_array_element(
        &self,
        account: &AccountID,
        key: &[u8],
        index: usize,
        field_key: &[u8],
    ) -> Result<(), i32> {
        let mut buffer = [0u8; 33];
        buffer[0] = STI_UINT256;
        let mut i = 0;
        while i < 32 {
            buffer[i + 1] = self[i];
            i += 1;
        }
        unsafe {
            set_data_nested_array_element_field(
                account.0.as_ptr(),
                account.0.len(),
                key.as_ptr(),
                key.len(),
                index as i32,
                field_key.as_ptr(),
                field_key.len(),
                buffer.as_ptr(),
                33,
            );
        }
        Ok(())
    }
}

// ============================================================================
// FromDataBytes Implementation for [u8; 8] (STI_AMOUNT)
// ============================================================================

impl FromDataBytes for [u8; 8] {
    #[inline]
    fn get_data(account: &AccountID, key: &[u8]) -> Option<Self> {
        let mut buffer = [0u8; 8];
        let output_len = unsafe {
            get_data_object_field(
                account.0.as_ptr(),
                account.0.len(),
                key.as_ptr(),
                key.len(),
                buffer.as_mut_ptr(),
                8,
            )
        };
        if output_len == 8 { Some(buffer) } else { None }
    }

    #[inline]
    fn get_nested_data(account: &AccountID, nested: &[u8], key: &[u8]) -> Option<Self> {
        let mut buffer = [0u8; 8];
        let output_len = unsafe {
            get_data_nested_object_field(
                account.0.as_ptr(),
                account.0.len(),
                nested.as_ptr(),
                nested.len(),
                key.as_ptr(),
                key.len(),
                buffer.as_mut_ptr(),
                8,
            )
        };
        if output_len == 8 { Some(buffer) } else { None }
    }

    #[inline]
    fn get_array_element(account: &AccountID, key: &[u8], index: usize) -> Option<Self> {
        let mut buffer = [0u8; 8];
        let output_len = unsafe {
            get_data_array_element_field(
                account.0.as_ptr(),
                account.0.len(),
                key.as_ptr(),
                key.len(),
                index as i32,
                buffer.as_mut_ptr(),
                8,
            )
        };
        if output_len == 8 { Some(buffer) } else { None }
    }

    #[inline]
    fn get_nested_array_element(
        account: &AccountID,
        key: &[u8],
        index: usize,
        field_key: &[u8],
    ) -> Option<Self> {
        let mut buffer = [0u8; 8];
        let output_len = unsafe {
            get_data_nested_array_element_field(
                account.0.as_ptr(),
                account.0.len(),
                key.as_ptr(),
                key.len(),
                index as i32,
                field_key.as_ptr(),
                field_key.len(),
                buffer.as_mut_ptr(),
                8,
            )
        };
        if output_len == 8 { Some(buffer) } else { None }
    }
}

// ============================================================================
// ToDataBytes Implementation for [u8; 8] (STI_AMOUNT)
// ============================================================================

impl ToDataBytes for [u8; 8] {
    #[inline]
    fn set_data(&self, account: &AccountID, key: &[u8]) -> Result<(), i32> {
        let mut buffer = [0u8; 9];
        buffer[0] = STI_AMOUNT;
        let mut i = 0;
        while i < 8 {
            buffer[i + 1] = self[i];
            i += 1;
        }
        unsafe {
            set_data_object_field(
                account.0.as_ptr(),
                account.0.len(),
                key.as_ptr(),
                key.len(),
                buffer.as_ptr(),
                9,
            );
        }
        Ok(())
    }

    #[inline]
    fn set_nested_data(&self, account: &AccountID, nested: &[u8], key: &[u8]) -> Result<(), i32> {
        let mut buffer = [0u8; 9];
        buffer[0] = STI_AMOUNT;
        let mut i = 0;
        while i < 8 {
            buffer[i + 1] = self[i];
            i += 1;
        }
        unsafe {
            set_data_nested_object_field(
                account.0.as_ptr(),
                account.0.len(),
                nested.as_ptr(),
                nested.len(),
                key.as_ptr(),
                key.len(),
                buffer.as_ptr(),
                9,
            );
        }
        Ok(())
    }

    #[inline]
    fn set_array_element(&self, account: &AccountID, key: &[u8], index: usize) -> Result<(), i32> {
        let mut buffer = [0u8; 9];
        buffer[0] = STI_AMOUNT;
        let mut i = 0;
        while i < 8 {
            buffer[i + 1] = self[i];
            i += 1;
        }
        unsafe {
            set_data_array_element_field(
                account.0.as_ptr(),
                account.0.len(),
                key.as_ptr(),
                key.len(),
                index as i32,
                buffer.as_ptr(),
                9,
            );
        }
        Ok(())
    }

    #[inline]
    fn set_nested_array_element(
        &self,
        account: &AccountID,
        key: &[u8],
        index: usize,
        field_key: &[u8],
    ) -> Result<(), i32> {
        let mut buffer = [0u8; 9];
        buffer[0] = STI_AMOUNT;
        let mut i = 0;
        while i < 8 {
            buffer[i + 1] = self[i];
            i += 1;
        }
        unsafe {
            set_data_nested_array_element_field(
                account.0.as_ptr(),
                account.0.len(),
                key.as_ptr(),
                key.len(),
                index as i32,
                field_key.as_ptr(),
                field_key.len(),
                buffer.as_ptr(),
                9,
            );
        }
        Ok(())
    }
}

// ============================================================================
// FromDataBytes Implementation for AccountID (STI_ACCOUNT)
// Note: AccountID getter returns 21 bytes with 0x14 prefix
// ============================================================================

impl FromDataBytes for AccountID {
    #[inline]
    fn get_data(account: &AccountID, key: &[u8]) -> Option<Self> {
        let mut buffer = [0u8; 21];
        let output_len = unsafe {
            get_data_object_field(
                account.0.as_ptr(),
                account.0.len(),
                key.as_ptr(),
                key.len(),
                buffer.as_mut_ptr(),
                21,
            )
        };
        if output_len == 21 {
            let mut acc = [0u8; 20];
            let mut i = 0;
            while i < 20 {
                acc[i] = buffer[i + 1]; // Skip 0x14 prefix
                i += 1;
            }
            Some(AccountID(acc))
        } else {
            None
        }
    }

    #[inline]
    fn get_nested_data(account: &AccountID, nested: &[u8], key: &[u8]) -> Option<Self> {
        let mut buffer = [0u8; 21];
        let output_len = unsafe {
            get_data_nested_object_field(
                account.0.as_ptr(),
                account.0.len(),
                nested.as_ptr(),
                nested.len(),
                key.as_ptr(),
                key.len(),
                buffer.as_mut_ptr(),
                21,
            )
        };
        if output_len == 21 {
            let mut acc = [0u8; 20];
            let mut i = 0;
            while i < 20 {
                acc[i] = buffer[i + 1]; // Skip 0x14 prefix
                i += 1;
            }
            Some(AccountID(acc))
        } else {
            None
        }
    }

    #[inline]
    fn get_array_element(account: &AccountID, key: &[u8], index: usize) -> Option<Self> {
        let mut buffer = [0u8; 21];
        let output_len = unsafe {
            get_data_array_element_field(
                account.0.as_ptr(),
                account.0.len(),
                key.as_ptr(),
                key.len(),
                index as i32,
                buffer.as_mut_ptr(),
                21,
            )
        };
        if output_len == 21 {
            let mut acc = [0u8; 20];
            let mut i = 0;
            while i < 20 {
                acc[i] = buffer[i + 1]; // Skip 0x14 prefix
                i += 1;
            }
            Some(AccountID(acc))
        } else {
            None
        }
    }

    #[inline]
    fn get_nested_array_element(
        account: &AccountID,
        key: &[u8],
        index: usize,
        field_key: &[u8],
    ) -> Option<Self> {
        let mut buffer = [0u8; 21];
        let output_len = unsafe {
            get_data_nested_array_element_field(
                account.0.as_ptr(),
                account.0.len(),
                key.as_ptr(),
                key.len(),
                index as i32,
                field_key.as_ptr(),
                field_key.len(),
                buffer.as_mut_ptr(),
                21,
            )
        };
        if output_len == 21 {
            let mut acc = [0u8; 20];
            let mut i = 0;
            while i < 20 {
                acc[i] = buffer[i + 1]; // Skip 0x14 prefix
                i += 1;
            }
            Some(AccountID(acc))
        } else {
            None
        }
    }
}

// ============================================================================
// ToDataBytes Implementation for AccountID (STI_ACCOUNT)
// Note: AccountID setter sends [STI_ACCOUNT, 0x14, ...20 bytes...]
// ============================================================================

impl ToDataBytes for AccountID {
    #[inline]
    fn set_data(&self, account: &AccountID, key: &[u8]) -> Result<(), i32> {
        let mut buffer = [0u8; 22];
        buffer[0] = STI_ACCOUNT;
        buffer[1] = 0x14; // Account length prefix
        let mut i = 0;
        while i < 20 {
            buffer[i + 2] = self.0[i];
            i += 1;
        }
        unsafe {
            set_data_object_field(
                account.0.as_ptr(),
                account.0.len(),
                key.as_ptr(),
                key.len(),
                buffer.as_ptr(),
                22,
            );
        }
        Ok(())
    }

    #[inline]
    fn set_nested_data(&self, account: &AccountID, nested: &[u8], key: &[u8]) -> Result<(), i32> {
        let mut buffer = [0u8; 22];
        buffer[0] = STI_ACCOUNT;
        buffer[1] = 0x14; // Account length prefix
        let mut i = 0;
        while i < 20 {
            buffer[i + 2] = self.0[i];
            i += 1;
        }
        unsafe {
            set_data_nested_object_field(
                account.0.as_ptr(),
                account.0.len(),
                nested.as_ptr(),
                nested.len(),
                key.as_ptr(),
                key.len(),
                buffer.as_ptr(),
                22,
            );
        }
        Ok(())
    }

    #[inline]
    fn set_array_element(&self, account: &AccountID, key: &[u8], index: usize) -> Result<(), i32> {
        let mut buffer = [0u8; 22];
        buffer[0] = STI_ACCOUNT;
        buffer[1] = 0x14; // Account length prefix
        let mut i = 0;
        while i < 20 {
            buffer[i + 2] = self.0[i];
            i += 1;
        }
        unsafe {
            set_data_array_element_field(
                account.0.as_ptr(),
                account.0.len(),
                key.as_ptr(),
                key.len(),
                index as i32,
                buffer.as_ptr(),
                22,
            );
        }
        Ok(())
    }

    #[inline]
    fn set_nested_array_element(
        &self,
        account: &AccountID,
        key: &[u8],
        index: usize,
        field_key: &[u8],
    ) -> Result<(), i32> {
        let mut buffer = [0u8; 22];
        buffer[0] = STI_ACCOUNT;
        buffer[1] = 0x14; // Account length prefix
        let mut i = 0;
        while i < 20 {
            buffer[i + 2] = self.0[i];
            i += 1;
        }
        unsafe {
            set_data_nested_array_element_field(
                account.0.as_ptr(),
                account.0.len(),
                key.as_ptr(),
                key.len(),
                index as i32,
                field_key.as_ptr(),
                field_key.len(),
                buffer.as_ptr(),
                22,
            );
        }
        Ok(())
    }
}
