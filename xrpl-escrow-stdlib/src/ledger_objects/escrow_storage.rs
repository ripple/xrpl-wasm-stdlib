use crate::ctx::escrow_finish::EscrowFinishContext;
use crate::ledger_objects::traits::CurrentEscrowFields;
use xrpl_common_stdlib::host::Result;

pub trait EscrowStorage: Sized {
    fn encode(&self, out: &mut [u8]) -> Result<usize>;
    fn decode(bytes: &[u8]) -> Result<Self>;
}

pub fn load_data<T: EscrowStorage>(ctx: &EscrowFinishContext) -> Result<Option<T>> {
    let bytes = match ctx.escrow().get_data() {
        Result::Ok(contract_data) => contract_data,
        Result::Err(e) => return Result::Err(e),
    };
    if bytes.len == 0 {
        return Result::Ok(None);
    }
    match T::decode(&bytes.data[..bytes.len]) {
        Result::Ok(data) => Result::Ok(Some(data)),
        Result::Err(e) => Result::Err(e),
    }
}

pub fn save_data<T: EscrowStorage>(ctx: &EscrowFinishContext, data: &T) -> Result<()> {
    let mut bytes = [0u8; 1024];
    let n = match data.encode(&mut bytes) {
        Result::Ok(n) => n,
        Result::Err(e) => return Result::Err(e),
    };
    ctx.update_data(&bytes[..n])
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::{always, eq};
    use xrpl_common_stdlib::host::Error;
    use xrpl_common_stdlib::host::error_codes::INTERNAL_ERROR;
    use xrpl_common_stdlib::host::host_bindings_trait::MockHostBindings;
    use xrpl_common_stdlib::host::setup_mock;
    use xrpl_common_stdlib::sfield;
    use xrpl_stdlib_test_utils::EscrowScenario;

    /// Length-prefixed test payload: 4-byte big-endian length header followed by the
    /// payload bytes. Self-describing so `decode` works whether it's handed an exact-size
    /// slice or a larger buffer padded with trailing zeros.
    #[derive(Debug, PartialEq)]
    struct TestPayload(u32);

    impl EscrowStorage for TestPayload {
        fn encode(&self, out: &mut [u8]) -> Result<usize> {
            out[..4].copy_from_slice(&self.0.to_be_bytes());
            Result::Ok(4)
        }

        fn decode(bytes: &[u8]) -> Result<Self> {
            if bytes.len() < 4 {
                return Result::Err(Error::InternalError);
            }
            let mut header = [0u8; 4];
            header.copy_from_slice(&bytes[..4]);
            Result::Ok(TestPayload(u32::from_be_bytes(header)))
        }
    }

    fn expect_get_data(mock: &mut MockHostBindings, returning: i32, payload: Option<Vec<u8>>) {
        mock.expect_get_current_ledger_obj_field()
            .with(eq(sfield::Data), always(), eq(4096))
            .times(1)
            .returning(move |_, out_buff_ptr, _| {
                if let Some(payload) = &payload {
                    unsafe {
                        std::ptr::copy_nonoverlapping(
                            payload.as_ptr(),
                            out_buff_ptr,
                            payload.len(),
                        );
                    }
                }
                returning
            });
    }

    #[test]
    fn load_data_returns_none_when_escrow_has_no_stored_data() {
        let mut mock = MockHostBindings::new();
        expect_get_data(&mut mock, 0, None);
        let _guard = setup_mock(mock);

        let ctx = EscrowFinishContext::default();
        let result: Result<Option<TestPayload>> = load_data(&ctx);

        assert!(matches!(result, Result::Ok(None)));
    }

    #[test]
    fn load_data_decodes_stored_bytes() {
        let payload = 42u32.to_be_bytes().to_vec();
        let mut mock = MockHostBindings::new();
        expect_get_data(&mut mock, payload.len() as i32, Some(payload));
        let _guard = setup_mock(mock);

        let ctx = EscrowFinishContext::default();
        let result: Result<Option<TestPayload>> = load_data(&ctx);

        assert!(matches!(result, Result::Ok(Some(TestPayload(42)))));
    }

    #[test]
    fn load_data_propagates_host_error() {
        let mut mock = MockHostBindings::new();
        expect_get_data(&mut mock, INTERNAL_ERROR, None);
        let _guard = setup_mock(mock);

        let ctx = EscrowFinishContext::default();
        let result: Result<Option<TestPayload>> = load_data(&ctx);

        assert_eq!(result.err().unwrap().code(), INTERNAL_ERROR);
    }

    #[test]
    fn load_data_propagates_decode_error() {
        let mut mock = MockHostBindings::new();
        // Fewer bytes than TestPayload's 4-byte header, so decode fails.
        expect_get_data(&mut mock, 2, Some(vec![0xAB, 0xCD]));
        let _guard = setup_mock(mock);

        let ctx = EscrowFinishContext::default();
        let result: Result<Option<TestPayload>> = load_data(&ctx);

        assert!(result.is_err());
    }

    #[test]
    fn save_data_writes_encoded_bytes_via_update_data() {
        let mut mock = MockHostBindings::new();
        mock.expect_update_data()
            .withf(|_data_ptr, data_len| *data_len == 4)
            .times(1)
            .returning(|data_ptr, data_len| {
                let bytes = unsafe { std::slice::from_raw_parts(data_ptr, data_len) };
                assert_eq!(bytes, &42u32.to_be_bytes());
                data_len as i32
            });
        let _guard = setup_mock(mock);

        let ctx = EscrowFinishContext::default();
        let result = save_data(&ctx, &TestPayload(42));

        assert!(result.is_ok());
    }

    #[test]
    fn save_data_propagates_host_error_without_swallowing_it() {
        let _guard = EscrowScenario::builder()
            .with_update_data_returns(Err(Error::InternalError))
            .install();

        let ctx = EscrowFinishContext::default();
        let result = save_data(&ctx, &TestPayload(42));

        assert_eq!(result.err().unwrap().code(), Error::InternalError.code());
    }
}
