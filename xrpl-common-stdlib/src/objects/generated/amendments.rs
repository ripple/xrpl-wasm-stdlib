// GENERATED -- do not hand-edit. Run scripts/generate-ledger-objects.sh to regenerate.

/// Placeholder buffer size for fields whose XRPL wire type has no genuine Rust
/// mapping yet (VECTOR256, XCHAIN_BRIDGE, PATHSET, ...). Such getters return
/// raw, unparsed bytes; see the summary at the top of `generated/mod.rs`.
const RAW_UNMAPPED_FIELD_SIZE: usize = 512;

use crate::host::Result;
use crate::host::error_codes::match_result_code_optional;
use crate::host::home_le_field;
use crate::host::le_field;
use crate::objects::traits::CurrentLedgerObjectCommonFields;
use crate::objects::traits::LedgerObjectCommonFields;
use crate::objects::{current_ledger_object, ledger_object};
use crate::sfield;
use crate::types::uint::Hash256;

/// Trait providing access to fields specific to Amendments objects in any ledger.
pub trait AmendmentsFields: LedgerObjectCommonFields {
    /// Array of 256-bit amendment IDs for all currently enabled amendments. If omitted, there are
    /// no enabled amendments.
    /// Raw bytes; VECTOR256 is not yet typed in Rust.
    fn amendments(&self) -> Result<Option<[u8; RAW_UNMAPPED_FIELD_SIZE]>> {
        let mut buffer = [0u8; RAW_UNMAPPED_FIELD_SIZE];
        let result_code = unsafe {
            le_field(
                self.get_slot_num(),
                sfield::Amendments.into(),
                buffer.as_mut_ptr(),
                buffer.len(),
            )
        };
        match_result_code_optional(result_code, || (result_code > 0).then_some(buffer))
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn previous_txn_id(&self) -> Result<Option<Hash256>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn previous_txn_lgr_seq(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }
}

/// Trait providing access to fields specific to the current Amendments object.
pub trait CurrentAmendmentsFields: CurrentLedgerObjectCommonFields {
    /// Array of 256-bit amendment IDs for all currently enabled amendments. If omitted, there are
    /// no enabled amendments.
    /// Raw bytes; VECTOR256 is not yet typed in Rust.
    fn amendments(&self) -> Result<Option<[u8; RAW_UNMAPPED_FIELD_SIZE]>> {
        let mut buffer = [0u8; RAW_UNMAPPED_FIELD_SIZE];
        let result_code =
            unsafe { home_le_field(sfield::Amendments.into(), buffer.as_mut_ptr(), buffer.len()) };
        match_result_code_optional(result_code, || (result_code > 0).then_some(buffer))
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn previous_txn_id(&self) -> Result<Option<Hash256>> {
        current_ledger_object::get_field_optional(sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn previous_txn_lgr_seq(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::PreviousTxnLgrSeq)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Amendments {
    pub(crate) slot_num: i32,
}

impl Amendments {
    /// Binds this handle to a host-managed slot holding an Amendments ledger object.
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }
}

impl LedgerObjectCommonFields for Amendments {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl AmendmentsFields for Amendments {}

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

        let obj = Amendments::new(0);

        assert!(obj.amendments().is_ok());
        assert!(obj.previous_txn_id().is_ok());
        assert!(obj.previous_txn_lgr_seq().is_ok());
    }

    #[test]
    fn optional_fields_none() {
        let mut mock = MockHostBindings::new();
        mock_all_fields_not_found(&mut mock);
        let _guard = setup_mock(mock);

        let obj = Amendments::new(0);

        assert!(obj.previous_txn_id().unwrap().is_none());
        assert!(obj.previous_txn_lgr_seq().unwrap().is_none());
    }
}
