//! Escrow-finish-specific transaction field accessor trait.

use xrpl_common_stdlib::core::current_tx::get_field;
use xrpl_common_stdlib::core::current_tx::traits::TransactionCommonFields;
use xrpl_common_stdlib::core::types::account_id::AccountID;
use xrpl_common_stdlib::core::types::blob::{ConditionBlob, FulfillmentBlob};
use xrpl_common_stdlib::host::error_codes::match_result_code_optional;
use xrpl_common_stdlib::host::{Result, get_tx_field};
use xrpl_common_stdlib::sfield;

/// Trait providing access to fields specific to EscrowFinish transactions.
///
/// This trait extends `TransactionCommonFields` with methods for retrieving fields that are
/// unique to EscrowFinish transactions. EscrowFinish transactions are used to complete
/// time-based or condition-based escrows that were previously created with EscrowCreate
/// transactions.
///
/// ## Implementation Requirements
///
/// Types implementing this trait should:
/// - Also implement `TransactionCommonFields` for access to common transaction fields
/// - Only be used in the context of processing EscrowFinish transactions
/// - Ensure proper error handling when accessing conditional fields
pub trait EscrowFinishFields: TransactionCommonFields {
    /// Retrieves the owner account from the current EscrowFinish transaction.
    ///
    /// This mandatory field identifies the XRPL account that originally created the escrow
    /// with an EscrowCreate transaction. The owner is the account that deposited the XRP
    /// into the escrow and specified the conditions for its release.
    ///
    /// # Returns
    ///
    /// Returns a `Result<AccountID>` where:
    /// * `Ok(AccountID)` - The 20-byte account identifier of the escrow owner
    /// * `Err(Error)` - If the field cannot be retrieved or has an unexpected size
    fn get_owner(&self) -> Result<AccountID> {
        get_field(sfield::Owner)
    }

    /// Retrieves the offer sequence from the current EscrowFinish transaction.
    ///
    /// This mandatory field specifies the sequence number of the original EscrowCreate
    /// transaction that created the escrow being finished. This creates a unique reference
    /// to the specific escrow object, as escrows are identified by the combination of
    /// the owner account and the sequence number of the creating transaction.
    ///
    /// # Returns
    ///
    /// Returns a `Result<u32>` where:
    /// * `Ok(u32)` - The sequence number of the EscrowCreate transaction
    /// * `Err(Error)` - If the field cannot be retrieved or has an unexpected size
    fn get_offer_sequence(&self) -> Result<u32> {
        get_field(sfield::OfferSequence)
    }

    /// Retrieves the cryptographic condition from the current EscrowFinish transaction.
    ///
    /// This optional field contains the cryptographic condition in full crypto-condition format.
    /// For PREIMAGE-SHA-256 conditions, this is 39 bytes:
    /// - 2 bytes: type tag (A025)
    /// - 2 bytes: fingerprint length tag (8020)
    /// - 32 bytes: SHA-256 hash (fingerprint)
    /// - 2 bytes: cost length tag (8101)
    /// - 1 byte: cost value (00)
    ///
    /// # Returns
    ///
    /// Returns a `Result<Option<Condition>>` where:
    /// * `Ok(Some(Condition))` - The full crypto-condition if the escrow is conditional
    /// * `Ok(None)` - If the escrow has no cryptographic condition (time-based only)
    /// * `Err(Error)` - If an error occurred during field retrieval
    fn get_condition(&self) -> Result<Option<ConditionBlob>> {
        let mut buffer = ConditionBlob::new();
        let result_code = unsafe {
            get_tx_field(
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

    /// Retrieves the cryptographic fulfillment from the current EscrowFinish transaction.
    ///
    /// This optional field contains the cryptographic fulfillment that satisfies the condition
    /// specified in the original EscrowCreate transaction. The fulfillment must cryptographically
    /// prove that the condition's requirements have been met. This field is only required
    /// when the escrow has an associated condition.
    ///
    /// # Returns
    ///
    /// Returns a `Result<Option<Fulfillment>>` where:
    /// * `Ok(Some(Fulfillment))` - The fulfillment data if provided
    /// * `Ok(None)` - If no fulfillment is provided (valid for unconditional escrows)
    /// * `Err(Error)` - If an error occurred during field retrieval
    ///
    /// # Fulfillment Validation
    ///
    /// The XRPL network automatically validates that:
    /// - The fulfillment satisfies the escrow's condition
    /// - The fulfillment is properly formatted according to RFC 3814
    /// - The cryptographic proof is mathematically valid
    ///
    /// # Size Limits
    ///
    /// Fulfillments are limited to 256 bytes in the current XRPL implementation.
    /// This limit ensures network performance while supporting the most practical
    /// cryptographic proof scenarios.
    fn get_fulfillment(&self) -> Result<Option<FulfillmentBlob>> {
        let mut buffer = FulfillmentBlob::new();
        let result_code = unsafe {
            get_tx_field(
                sfield::Fulfillment.into(),
                buffer.data.as_mut_ptr(),
                buffer.capacity(),
            )
        };
        match_result_code_optional(result_code, || {
            buffer.len = result_code as usize;
            (result_code > 0).then_some(buffer)
        })
    }
}

#[cfg(test)]
mod tests {
    use mockall::predicate::{always, eq};
    use xrpl_common_stdlib::host::host_bindings_trait::MockHostBindings;
    use xrpl_common_stdlib::sfield::SField;

    /// Helper to set up a mock expectation for `get_tx_field`.
    fn expect_tx_field<T: Send + std::fmt::Debug + PartialEq + 'static, const CODE: i32>(
        mock: &mut MockHostBindings,
        field: SField<T, CODE>,
        size: usize,
        times: usize,
    ) {
        mock.expect_get_tx_field()
            .with(eq(field), always(), eq(size))
            .times(times)
            .returning(move |_, _, _| size as i32);
    }

    mod escrow_finish_fields {

        mod optional_fields {
            use crate::current_tx::escrow_finish::EscrowFinish;
            use crate::current_tx::traits::EscrowFinishFields;
            use crate::current_tx::traits::tests::expect_tx_field;
            use xrpl_common_stdlib::core::types::blob::{
                CONDITION_BLOB_SIZE, FULFILLMENT_BLOB_SIZE,
            };
            use xrpl_common_stdlib::host::error_codes::{
                FIELD_NOT_FOUND, INTERNAL_ERROR, INVALID_FIELD,
            };
            use xrpl_common_stdlib::host::host_bindings_trait::MockHostBindings;
            use xrpl_common_stdlib::host::setup_mock;
            use xrpl_common_stdlib::sfield;

            use mockall::predicate::{always, eq};
            use xrpl_common_stdlib::sfield::{Condition, Fulfillment};

            #[test]
            fn test_optional_fields_return_some() {
                let mut mock = MockHostBindings::new();

                // get_condition
                expect_tx_field(&mut mock, Condition, CONDITION_BLOB_SIZE, 1);
                // get_fulfillment
                expect_tx_field(&mut mock, Fulfillment, FULFILLMENT_BLOB_SIZE, 1);

                let _guard = setup_mock(mock);

                let escrow = EscrowFinish;

                // All optional fields should return Ok(Some(...))
                let condition = escrow.get_condition().unwrap();
                assert!(condition.is_some());
                assert_eq!(condition.unwrap().len, CONDITION_BLOB_SIZE);

                let fulfillment = escrow.get_fulfillment().unwrap();
                assert!(fulfillment.is_some());
                assert_eq!(fulfillment.unwrap().len, FULFILLMENT_BLOB_SIZE);
            }

            #[test]
            fn test_optional_fields_return_none_when_zero_length() {
                let mut mock = MockHostBindings::new();

                // get_condition - returns None when result code is 0
                mock.expect_get_tx_field()
                    .with(eq(sfield::Condition), always(), eq(CONDITION_BLOB_SIZE))
                    .times(1)
                    .returning(|_, _, _| 0);
                // get_fulfillment - returns None when result code is 0
                mock.expect_get_tx_field()
                    .with(eq(sfield::Fulfillment), always(), eq(FULFILLMENT_BLOB_SIZE))
                    .times(1)
                    .returning(|_, _, _| 0);

                let _guard = setup_mock(mock);

                let escrow = EscrowFinish;

                // Variable-size optional fields return None when result code is 0 (not present)
                assert!(escrow.get_condition().unwrap().is_none());
                assert!(escrow.get_fulfillment().unwrap().is_none());
            }

            #[test]
            fn test_optional_fields_return_error_on_internal_error() {
                let mut mock = MockHostBindings::new();

                // get_condition
                mock.expect_get_tx_field()
                    .with(eq(sfield::Condition), always(), eq(CONDITION_BLOB_SIZE))
                    .times(1)
                    .returning(|_, _, _| INTERNAL_ERROR);
                // get_fulfillment
                mock.expect_get_tx_field()
                    .with(eq(sfield::Fulfillment), always(), eq(FULFILLMENT_BLOB_SIZE))
                    .times(1)
                    .returning(|_, _, _| INTERNAL_ERROR);

                let _guard = setup_mock(mock);

                let escrow = EscrowFinish;

                // Optional fields should also return Err on INTERNAL_ERROR
                let condition_result = escrow.get_condition();
                assert!(condition_result.is_err());
                assert_eq!(condition_result.err().unwrap().code(), INTERNAL_ERROR);

                let fulfillment_result = escrow.get_fulfillment();
                assert!(fulfillment_result.is_err());
                assert_eq!(fulfillment_result.err().unwrap().code(), INTERNAL_ERROR);
            }

            #[test]
            fn test_optional_fields_return_error_on_field_not_found() {
                let mut mock = MockHostBindings::new();

                // get_condition
                mock.expect_get_tx_field()
                    .with(eq(Condition), always(), eq(CONDITION_BLOB_SIZE))
                    .times(1)
                    .returning(|_, _, _| FIELD_NOT_FOUND);
                // get_fulfillment
                mock.expect_get_tx_field()
                    .with(eq(Fulfillment), always(), eq(FULFILLMENT_BLOB_SIZE))
                    .times(1)
                    .returning(|_, _, _| FIELD_NOT_FOUND);

                let _guard = setup_mock(mock);

                let escrow = EscrowFinish;

                // Optional fields return Err on FIELD_NOT_FOUND (not None)
                let condition_result = escrow.get_condition();
                assert!(condition_result.is_err());
                assert_eq!(condition_result.err().unwrap().code(), FIELD_NOT_FOUND);

                let fulfillment_result = escrow.get_fulfillment();
                assert!(fulfillment_result.is_err());
                assert_eq!(fulfillment_result.err().unwrap().code(), FIELD_NOT_FOUND);
            }

            #[test]
            fn test_optional_fields_return_error_on_invalid_field() {
                let mut mock = MockHostBindings::new();

                // get_condition
                mock.expect_get_tx_field()
                    .with(eq(sfield::Condition), always(), eq(CONDITION_BLOB_SIZE))
                    .times(1)
                    .returning(|_, _, _| INVALID_FIELD);
                // get_fulfillment
                mock.expect_get_tx_field()
                    .with(eq(sfield::Fulfillment), always(), eq(FULFILLMENT_BLOB_SIZE))
                    .times(1)
                    .returning(|_, _, _| INVALID_FIELD);

                let _guard = setup_mock(mock);

                let escrow = EscrowFinish;

                // Optional fields should also return Err on INVALID_FIELD
                let condition_result = escrow.get_condition();
                assert!(condition_result.is_err());
                assert_eq!(condition_result.err().unwrap().code(), INVALID_FIELD);

                let fulfillment_result = escrow.get_fulfillment();
                assert!(fulfillment_result.is_err());
                assert_eq!(fulfillment_result.err().unwrap().code(), INVALID_FIELD);
            }
        }

        mod mandatory_fields {
            use crate::current_tx::escrow_finish::EscrowFinish;
            use crate::current_tx::traits::EscrowFinishFields;
            use crate::current_tx::traits::tests::expect_tx_field;
            use mockall::predicate::{always, eq};
            use xrpl_common_stdlib::core::types::account_id::ACCOUNT_ID_SIZE;
            use xrpl_common_stdlib::host::error_codes::{
                FIELD_NOT_FOUND, INTERNAL_ERROR, INVALID_FIELD,
            };
            use xrpl_common_stdlib::host::host_bindings_trait::MockHostBindings;
            use xrpl_common_stdlib::host::setup_mock;
            use xrpl_common_stdlib::sfield;

            #[test]
            fn test_mandatory_fields_return_ok() {
                let mut mock = MockHostBindings::new();

                // get_owner
                expect_tx_field(&mut mock, sfield::Owner, ACCOUNT_ID_SIZE, 1);
                // get_offer_sequence
                expect_tx_field(&mut mock, sfield::OfferSequence, 4, 1);

                let _guard = setup_mock(mock);

                let escrow = EscrowFinish;

                // All mandatory fields should return Ok
                assert!(escrow.get_owner().is_ok());
                assert!(escrow.get_offer_sequence().is_ok());
            }

            // Zero length for a mandatory fixed-size field panics (byte mismatch). One test
            // per field, since `#[should_panic]` only catches the first panic.

            #[test]
            #[should_panic]
            fn test_get_owner_panics_when_zero_length() {
                let mut mock = MockHostBindings::new();
                mock.expect_get_tx_field()
                    .with(eq(sfield::Owner), always(), eq(ACCOUNT_ID_SIZE))
                    .returning(|_, _, _| 0);

                let _guard = setup_mock(mock);

                let _ = EscrowFinish.get_owner();
            }

            #[test]
            #[should_panic]
            fn test_get_offer_sequence_panics_when_zero_length() {
                let mut mock = MockHostBindings::new();
                mock.expect_get_tx_field()
                    .with(eq(sfield::OfferSequence), always(), eq(4))
                    .returning(|_, _, _| 0);

                let _guard = setup_mock(mock);

                let _ = EscrowFinish.get_offer_sequence();
            }

            #[test]
            fn test_mandatory_fields_return_error_on_field_not_found() {
                let mut mock = MockHostBindings::new();

                // get_owner
                mock.expect_get_tx_field()
                    .with(eq(sfield::Owner), always(), eq(ACCOUNT_ID_SIZE))
                    .times(1)
                    .returning(|_, _, _| FIELD_NOT_FOUND);
                // get_offer_sequence
                mock.expect_get_tx_field()
                    .with(eq(sfield::OfferSequence), always(), eq(4))
                    .times(1)
                    .returning(|_, _, _| FIELD_NOT_FOUND);

                let _guard = setup_mock(mock);

                let escrow = EscrowFinish;

                // All mandatory fields should return Err on FIELD_NOT_FOUND
                let owner_result = escrow.get_owner();
                assert!(owner_result.is_err());
                assert_eq!(owner_result.err().unwrap().code(), FIELD_NOT_FOUND);

                let offer_seq_result = escrow.get_offer_sequence();
                assert!(offer_seq_result.is_err());
                assert_eq!(offer_seq_result.err().unwrap().code(), FIELD_NOT_FOUND);
            }

            #[test]
            fn test_mandatory_fields_return_error_on_internal_error() {
                let mut mock = MockHostBindings::new();

                // get_owner
                mock.expect_get_tx_field()
                    .with(eq(sfield::Owner), always(), eq(ACCOUNT_ID_SIZE))
                    .times(1)
                    .returning(|_, _, _| INTERNAL_ERROR);
                // get_offer_sequence
                mock.expect_get_tx_field()
                    .with(eq(sfield::OfferSequence), always(), eq(4))
                    .times(1)
                    .returning(|_, _, _| INTERNAL_ERROR);

                let _guard = setup_mock(mock);

                let escrow = EscrowFinish;

                // All mandatory fields should return Err on INTERNAL_ERROR
                let owner_result = escrow.get_owner();
                assert!(owner_result.is_err());
                assert_eq!(owner_result.err().unwrap().code(), INTERNAL_ERROR);

                let offer_seq_result = escrow.get_offer_sequence();
                assert!(offer_seq_result.is_err());
                assert_eq!(offer_seq_result.err().unwrap().code(), INTERNAL_ERROR);
            }

            #[test]
            fn test_mandatory_fields_return_error_on_invalid_field() {
                let mut mock = MockHostBindings::new();

                // get_owner
                mock.expect_get_tx_field()
                    .with(eq(sfield::Owner), always(), eq(ACCOUNT_ID_SIZE))
                    .times(1)
                    .returning(|_, _, _| INVALID_FIELD);
                // get_offer_sequence
                mock.expect_get_tx_field()
                    .with(eq(sfield::OfferSequence), always(), eq(4))
                    .times(1)
                    .returning(|_, _, _| INVALID_FIELD);

                let _guard = setup_mock(mock);

                let escrow = EscrowFinish;

                // All mandatory fields should return Err on INVALID_FIELD
                let owner_result = escrow.get_owner();
                assert!(owner_result.is_err());
                assert_eq!(owner_result.err().unwrap().code(), INVALID_FIELD);

                let offer_seq_result = escrow.get_offer_sequence();
                assert!(offer_seq_result.is_err());
                assert_eq!(offer_seq_result.err().unwrap().code(), INVALID_FIELD);
            }
        }
    }
}
