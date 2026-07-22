//! Escrow-specific ledger-object field accessor traits.

use xrpl_common_stdlib::host::error_codes::{match_result_code, match_result_code_optional};
use xrpl_common_stdlib::host::{Error, get_current_ledger_obj_field, update_data};
use xrpl_common_stdlib::host::{Result, Result::Err, Result::Ok};
use xrpl_common_stdlib::objects::current_ledger_object;
use xrpl_common_stdlib::objects::traits::CurrentLedgerObjectCommonFields;
use xrpl_common_stdlib::sfield;
use xrpl_common_stdlib::types::account_id::AccountID;
use xrpl_common_stdlib::types::amount::Amount;
use xrpl_common_stdlib::types::blob::{ConditionBlob, WasmBlob};
use xrpl_common_stdlib::types::contract_data::{ContractData, XRPL_CONTRACT_DATA_SIZE};
use xrpl_common_stdlib::types::uint::Hash256;

/// Trait providing access to fields specific to Escrow objects in the current ledger.
///
/// This trait extends `CurrentLedgerObjectCommonFields` and provides methods to access
/// fields that are specific to Escrow objects in the current ledger being processed.
pub trait CurrentEscrowFields: CurrentLedgerObjectCommonFields {
    /// The address of the owner (sender) of this escrow. This is the account that provided the XRP
    /// and gets it back if the escrow is canceled.
    fn get_account(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Account)
    }

    /// The amount currently held in the escrow (could be XRP, IOU, or MPT).
    fn get_amount(&self) -> Result<Amount> {
        current_ledger_object::get_field(sfield::Amount)
    }

    /// The escrow can be canceled if and only if this field is present and the time it specifies
    /// has passed. Specifically, this is specified as seconds since the Ripple Epoch and it
    /// "has passed" if it's earlier than the close time of the previous validated ledger.
    fn get_cancel_after(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::CancelAfter)
    }

    /// A PREIMAGE-SHA-256 crypto-condition in full crypto-condition format. If present, the EscrowFinish
    /// transaction must contain a fulfillment that satisfies this condition.
    fn get_condition(&self) -> Result<Option<ConditionBlob>> {
        let mut buffer = ConditionBlob::new();
        let result_code = unsafe {
            get_current_ledger_obj_field(
                sfield::Condition.into(),
                buffer.data.as_mut_ptr(),
                buffer.capacity(),
            )
        };
        match_result_code_optional(result_code, || {
            buffer.len = result_code as usize;
            (result_code > 0).then_some(buffer)
        })
    }

    /// The destination address where the XRP is paid if the escrow is successful.
    fn get_destination(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Destination)
    }

    /// A hint indicating which page of the destination's owner directory links to this object, in
    /// case the directory consists of multiple pages. Omitted on escrows created before enabling the fix1523 amendment.
    fn get_destination_node(&self) -> Result<Option<u64>> {
        current_ledger_object::get_field_optional(sfield::DestinationNode)
    }

    /// An arbitrary tag to further specify the destination for this escrow, such as a hosted
    /// recipient at the destination address.
    fn get_destination_tag(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::DestinationTag)
    }

    /// The time, in seconds since the Ripple Epoch, after which this escrow can be finished. Any
    /// EscrowFinish transaction before this time fails. (Specifically, this is compared with the
    /// close time of the previous validated ledger.)
    fn get_finish_after(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::FinishAfter)
    }

    /// A hint indicating which page of the sender's owner directory links to this entry, in case
    /// the directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::OwnerNode)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::PreviousTxnLgrSeq)
    }

    /// An arbitrary tag to further specify the source for this escrow, such as a hosted recipient
    /// at the owner's address.
    fn get_source_tag(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::SourceTag)
    }

    /// The WASM code that is executing.
    fn get_finish_function(&self) -> Result<Option<WasmBlob>> {
        current_ledger_object::get_field_optional(sfield::FinishFunction)
    }

    /// Retrieves the contract `data` from the current escrow object.
    ///
    /// This function fetches the `data` field from the current ledger object and returns it as a
    /// ContractData structure. The data is read into a fixed-size buffer of XRPL_CONTRACT_DATA_SIZE.
    ///
    /// # Returns
    ///
    /// Returns a `Result<ContractData>` where:
    /// * `Ok(ContractData)` - Contains the retrieved data and its actual length
    /// * `Err(Error)` - If the retrieval operation failed
    fn get_data(&self) -> Result<ContractData> {
        let mut data: [u8; XRPL_CONTRACT_DATA_SIZE] = [0; XRPL_CONTRACT_DATA_SIZE];

        let result_code = unsafe {
            get_current_ledger_obj_field(sfield::Data.into(), data.as_mut_ptr(), data.len())
        };

        match result_code {
            code if code >= 0 => Ok(ContractData {
                data,
                len: code as usize,
            }),
            code => Err(Error::from_code(code)),
        }
    }

    /// Updates the contract data in the current escrow object.
    ///
    /// # Arguments
    ///
    /// * `data` - The contract data to update
    ///
    /// # Returns
    ///
    /// Returns a `Result<()>` where:
    /// * `Ok(())` - The data was successfully updated
    /// * `Err(Error)` - If the update operation failed
    fn update_current_escrow_data(data: ContractData) -> Result<()> {
        // TODO: Make sure rippled always deletes any existing data bytes in rippled, and sets the new
        // length to be `data.len` (e.g., if the developer writes 2 bytes, then that's the new
        // length and any old bytes are lost).
        let result_code = unsafe { update_data(data.data.as_ptr(), data.len) };
        match_result_code(result_code, || ())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::{always, eq};
    use xrpl_common_stdlib::host::error_codes::{FIELD_NOT_FOUND, INTERNAL_ERROR, INVALID_FIELD};
    use xrpl_common_stdlib::host::host_bindings_trait::MockHostBindings;
    use xrpl_common_stdlib::objects::LedgerObjectFieldGetter;
    use xrpl_common_stdlib::sfield::SField;

    fn expect_current_field<
        T: LedgerObjectFieldGetter + Send + std::fmt::Debug + PartialEq + 'static,
        const CODE: i32,
    >(
        mock: &mut MockHostBindings,
        _field: SField<T, CODE>,
        size: usize,
        times: usize,
    ) {
        mock.expect_get_current_ledger_obj_field()
            .with(eq(CODE), always(), eq(size))
            .times(times)
            .returning(move |_, _, _| size as i32);
    }

    mod current_escrow_fields {
        use super::*;
        use crate::ledger_objects::current_escrow::CurrentEscrow;
        use xrpl_common_stdlib::host::setup_mock;
        use xrpl_common_stdlib::types::blob::CONDITION_BLOB_SIZE;
        use xrpl_common_stdlib::types::blob::WASM_BLOB_SIZE;

        #[test]
        fn test_mandatory_fields_return_ok() {
            let mut mock = MockHostBindings::new();

            // get_account
            expect_current_field(&mut mock, sfield::Account, 20, 1);
            // get_amount
            expect_current_field(&mut mock, sfield::Amount, 48, 1);
            // get_destination
            expect_current_field(&mut mock, sfield::Destination, 20, 1);
            // get_owner_node
            expect_current_field(&mut mock, sfield::OwnerNode, 8, 1);
            // get_previous_txn_id
            expect_current_field(&mut mock, sfield::PreviousTxnID, 32, 1);
            // get_previous_txn_lgr_seq
            expect_current_field(&mut mock, sfield::PreviousTxnLgrSeq, 4, 1);
            // get_data (mandatory for escrow)
            expect_current_field(&mut mock, sfield::Data, 4096, 1);

            let _guard = setup_mock(mock);

            let escrow = CurrentEscrow;

            // All mandatory fields should return Ok
            assert!(escrow.get_account().is_ok());
            assert!(escrow.get_amount().is_ok());
            assert!(escrow.get_destination().is_ok());
            assert!(escrow.get_owner_node().is_ok());
            assert!(escrow.get_previous_txn_id().is_ok());
            assert!(escrow.get_previous_txn_lgr_seq().is_ok());
            assert!(escrow.get_data().is_ok());
        }

        #[test]
        fn test_optional_fields_return_some() {
            let mut mock = MockHostBindings::new();

            // get_cancel_after
            expect_current_field(&mut mock, sfield::CancelAfter, 4, 1);
            // get_condition
            expect_current_field(&mut mock, sfield::Condition, CONDITION_BLOB_SIZE, 1);
            // get_destination_node
            expect_current_field(&mut mock, sfield::DestinationNode, 8, 1);
            // get_destination_tag
            expect_current_field(&mut mock, sfield::DestinationTag, 4, 1);
            // get_finish_after
            expect_current_field(&mut mock, sfield::FinishAfter, 4, 1);
            // get_source_tag
            expect_current_field(&mut mock, sfield::SourceTag, 4, 1);
            // get_finish_function
            expect_current_field(&mut mock, sfield::FinishFunction, WASM_BLOB_SIZE, 1);

            let _guard = setup_mock(mock);

            let escrow = CurrentEscrow;

            // All optional fields should return Ok(Some(...))
            assert!(escrow.get_cancel_after().unwrap().is_some());
            assert!(escrow.get_condition().unwrap().is_some());
            assert!(escrow.get_destination_node().unwrap().is_some());
            assert!(escrow.get_destination_tag().unwrap().is_some());
            assert!(escrow.get_finish_after().unwrap().is_some());
            assert!(escrow.get_source_tag().unwrap().is_some());
            assert!(escrow.get_finish_function().unwrap().is_some());
        }

        #[test]
        fn test_optional_fields_return_none_when_field_not_found() {
            let mut mock = MockHostBindings::new();

            // get_cancel_after
            mock.expect_get_current_ledger_obj_field()
                .with(eq(sfield::CancelAfter), always(), eq(4))
                .times(1)
                .returning(|_, _, _| FIELD_NOT_FOUND);
            // get_condition - returns 0 for None
            mock.expect_get_current_ledger_obj_field()
                .with(eq(sfield::Condition), always(), eq(CONDITION_BLOB_SIZE))
                .times(1)
                .returning(|_, _, _| 0);
            // get_destination_node
            mock.expect_get_current_ledger_obj_field()
                .with(eq(sfield::DestinationNode), always(), eq(8))
                .times(1)
                .returning(|_, _, _| FIELD_NOT_FOUND);
            // get_destination_tag
            mock.expect_get_current_ledger_obj_field()
                .with(eq(sfield::DestinationTag), always(), eq(4))
                .times(1)
                .returning(|_, _, _| FIELD_NOT_FOUND);
            // get_finish_after
            mock.expect_get_current_ledger_obj_field()
                .with(eq(sfield::FinishAfter), always(), eq(4))
                .times(1)
                .returning(|_, _, _| FIELD_NOT_FOUND);
            // get_source_tag
            mock.expect_get_current_ledger_obj_field()
                .with(eq(sfield::SourceTag), always(), eq(4))
                .times(1)
                .returning(|_, _, _| FIELD_NOT_FOUND);
            // get_finish_function - variable size field, returns 0 for empty (Some with len=0)
            mock.expect_get_current_ledger_obj_field()
                .with(eq(sfield::FinishFunction), always(), eq(WASM_BLOB_SIZE))
                .times(1)
                .returning(|_, _, _| 0);

            let _guard = setup_mock(mock);

            let escrow = CurrentEscrow;

            // Fixed-size optional fields should return Ok(None) when FIELD_NOT_FOUND
            assert!(escrow.get_cancel_after().unwrap().is_none());
            assert!(escrow.get_condition().unwrap().is_none());
            assert!(escrow.get_destination_node().unwrap().is_none());
            assert!(escrow.get_destination_tag().unwrap().is_none());
            assert!(escrow.get_finish_after().unwrap().is_none());
            assert!(escrow.get_source_tag().unwrap().is_none());

            // Variable-size optional fields return Some with len=0 when not found
            let finish_function = escrow.get_finish_function().unwrap();
            assert!(finish_function.is_some());
            assert_eq!(finish_function.unwrap().len, 0);
        }

        #[test]
        fn test_mandatory_fields_return_error_on_internal_error() {
            let mut mock = MockHostBindings::new();

            // get_account with INTERNAL_ERROR
            mock.expect_get_current_ledger_obj_field()
                .with(eq(sfield::Account), always(), eq(20))
                .times(1)
                .returning(|_, _, _| INTERNAL_ERROR);

            let _guard = setup_mock(mock);

            let escrow = CurrentEscrow;
            let result = escrow.get_account();

            assert!(result.is_err());
            assert_eq!(result.err().unwrap().code(), INTERNAL_ERROR);
        }

        #[test]
        fn test_get_data_returns_error_on_internal_error() {
            let mut mock = MockHostBindings::new();

            mock.expect_get_current_ledger_obj_field()
                .with(eq(sfield::Data), always(), eq(4096))
                .times(1)
                .returning(|_, _, _| INTERNAL_ERROR);

            let _guard = setup_mock(mock);

            let escrow = CurrentEscrow;
            let result = escrow.get_data();

            assert!(result.is_err());
            assert_eq!(result.err().unwrap().code(), INTERNAL_ERROR);
        }

        #[test]
        fn test_mandatory_fields_return_error_on_invalid_field() {
            let mut mock = MockHostBindings::new();

            // get_account with INVALID_FIELD
            mock.expect_get_current_ledger_obj_field()
                .with(eq(sfield::Account), always(), eq(20))
                .times(1)
                .returning(|_, _, _| INVALID_FIELD);

            let _guard = setup_mock(mock);

            let escrow = CurrentEscrow;
            let result = escrow.get_account();

            assert!(result.is_err());
            assert_eq!(result.err().unwrap().code(), INVALID_FIELD);
        }
    }
}
