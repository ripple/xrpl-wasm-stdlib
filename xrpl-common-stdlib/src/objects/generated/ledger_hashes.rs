// GENERATED -- do not hand-edit. Run scripts/generate-ledger-objects.sh to regenerate.

/// Placeholder buffer size for fields whose XRPL wire type has no genuine Rust
/// mapping yet (VECTOR256, XCHAIN_BRIDGE, NUMBER, INT32, ...). Such getters return
/// raw, unparsed bytes; see the summary at the top of `generated/mod.rs`.
const RAW_UNMAPPED_FIELD_SIZE: usize = 512;

use crate::host::Result;
use crate::host::error_codes::match_result_code;
use crate::host::home_le_field;
use crate::host::le_field;
use crate::objects::traits::CurrentLedgerObjectCommonFields;
use crate::objects::traits::LedgerObjectCommonFields;
use crate::objects::{current_ledger_object, ledger_object};
use crate::sfield;

/// Trait providing access to fields specific to LedgerHashes objects in any ledger.
pub trait LedgerHashesFields: LedgerObjectCommonFields {
    /// DEPRECATED Do not use. (The "recent hashes" object on Mainnet has the value `2` in this
    /// field as a result of an old software bug. That value gets carried forward as the "recent
    /// hashes" object is updated. New "previous history" objects do not have this field, nor do
    /// "recent hashes" objects in parallel networks started with more recent versions of `xrpld`.)
    fn first_ledger_sequence(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::FirstLedgerSequence)
    }

    /// The Ledger Index of the last entry in this object's `Hashes` array.
    fn last_ledger_sequence(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::LastLedgerSequence)
    }

    /// An array of up to 256 ledger hashes. The contents depend on which sub-type of `LedgerHashes`
    /// object this is.
    /// Raw bytes; VECTOR256 is not yet typed in Rust.
    fn hashes(&self) -> Result<[u8; RAW_UNMAPPED_FIELD_SIZE]> {
        let mut buffer = [0u8; RAW_UNMAPPED_FIELD_SIZE];
        let result_code = unsafe {
            le_field(
                self.get_slot_num(),
                sfield::Hashes.into(),
                buffer.as_mut_ptr(),
                buffer.len(),
            )
        };
        match_result_code(result_code, || buffer)
    }
}

/// Trait providing access to fields specific to the current LedgerHashes object.
pub trait CurrentLedgerHashesFields: CurrentLedgerObjectCommonFields {
    /// DEPRECATED Do not use. (The "recent hashes" object on Mainnet has the value `2` in this
    /// field as a result of an old software bug. That value gets carried forward as the "recent
    /// hashes" object is updated. New "previous history" objects do not have this field, nor do
    /// "recent hashes" objects in parallel networks started with more recent versions of `xrpld`.)
    fn first_ledger_sequence(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::FirstLedgerSequence)
    }

    /// The Ledger Index of the last entry in this object's `Hashes` array.
    fn last_ledger_sequence(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::LastLedgerSequence)
    }

    /// An array of up to 256 ledger hashes. The contents depend on which sub-type of `LedgerHashes`
    /// object this is.
    /// Raw bytes; VECTOR256 is not yet typed in Rust.
    fn hashes(&self) -> Result<[u8; RAW_UNMAPPED_FIELD_SIZE]> {
        let mut buffer = [0u8; RAW_UNMAPPED_FIELD_SIZE];
        let result_code =
            unsafe { home_le_field(sfield::Hashes.into(), buffer.as_mut_ptr(), buffer.len()) };
        match_result_code(result_code, || buffer)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct LedgerHashes {
    pub(crate) slot_num: i32,
}

impl LedgerHashes {
    /// Binds this handle to a host-managed slot holding a LedgerHashes ledger object.
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }
}

impl LedgerObjectCommonFields for LedgerHashes {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl LedgerHashesFields for LedgerHashes {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::host::host_bindings_trait::MockHostBindings;
    use crate::host::setup_mock;
    use crate::objects::test_utils::*;

    #[test]
    fn read_all_fields() {
        let mut mock = MockHostBindings::new();
        mock_all_fields_present(&mut mock);
        let _guard = setup_mock(mock);

        let obj = LedgerHashes::new(0);

        assert!(obj.hashes().is_ok());
        assert!(obj.first_ledger_sequence().is_ok());
        assert!(obj.last_ledger_sequence().is_ok());
    }

    #[test]
    fn optional_fields_none() {
        let mut mock = MockHostBindings::new();
        mock_all_fields_not_found(&mut mock);
        let _guard = setup_mock(mock);

        let obj = LedgerHashes::new(0);

        assert!(obj.first_ledger_sequence().unwrap().is_none());
        assert!(obj.last_ledger_sequence().unwrap().is_none());
    }
}
