use crate::host::{
    Error, Result, amendment_enabled as host_amendment_enabled, base_fee as host_base_fee,
    error_codes::match_result_code_with_expected_bytes, ldgr_index, parent_ldgr_hash,
    parent_ldgr_time,
};

pub fn ledger_sqn() -> Result<u32> {
    let mut uint_bytes = [0u8; 4];
    let rescode = unsafe { ldgr_index(uint_bytes.as_mut_ptr(), 4) };
    match_result_code_with_expected_bytes(rescode, 4, || u32::from_le_bytes(uint_bytes))
}

pub fn parent_ledger_time() -> Result<u32> {
    let mut uint_bytes = [0u8; 4];
    let rescode = unsafe { parent_ldgr_time(uint_bytes.as_mut_ptr(), 4) };
    match_result_code_with_expected_bytes(rescode, 4, || u32::from_le_bytes(uint_bytes))
}

pub fn parent_ledger_hash() -> Result<[u8; 32]> {
    let mut bytes = [0u8; 32];
    let rescode = unsafe { parent_ldgr_hash(bytes.as_mut_ptr(), 32) };
    match_result_code_with_expected_bytes(rescode, 32, || bytes)
}

pub fn base_fee() -> Result<u32> {
    let mut uint_bytes = [0u8; 4];
    let rescode = unsafe { host_base_fee(uint_bytes.as_mut_ptr(), 4) };
    match_result_code_with_expected_bytes(rescode, 4, || u32::from_le_bytes(uint_bytes))
}

pub fn amendment_enabled(hash: &[u8; 32]) -> Result<bool> {
    let rescode = unsafe { host_amendment_enabled(hash.as_ptr(), 32) };
    match rescode {
        0 => Result::Ok(false),
        1 => Result::Ok(true),
        _ => Result::Err(Error::from_code(rescode)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::host::error_codes::INTERNAL_ERROR;
    use crate::host::host_bindings_trait::MockHostBindings;
    use crate::host::setup_mock;

    fn write_u32(ptr: *mut u8, value: u32) {
        let bytes = value.to_le_bytes();
        unsafe {
            for (i, b) in bytes.iter().enumerate() {
                *ptr.add(i) = *b;
            }
        }
    }

    fn write_hash(ptr: *mut u8, fill: u8) {
        unsafe {
            for i in 0..32 {
                *ptr.add(i) = fill;
            }
        }
    }

    // ---- ledger_sqn ----

    #[test]
    fn test_ledger_sqn_success() {
        let mut mock = MockHostBindings::new();
        mock.expect_ldgr_index().times(1).returning(|ptr, _| {
            write_u32(ptr, 42);
            4
        });
        let _guard = setup_mock(mock);

        let result = ledger_sqn();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_ledger_sqn_error() {
        let mut mock = MockHostBindings::new();
        mock.expect_ldgr_index()
            .times(1)
            .returning(|_, _| INTERNAL_ERROR);
        let _guard = setup_mock(mock);

        assert!(ledger_sqn().is_err());
    }

    // ---- parent_ledger_time ----

    #[test]
    fn test_parent_ledger_time_success() {
        let mut mock = MockHostBindings::new();
        mock.expect_parent_ldgr_time().times(1).returning(|ptr, _| {
            write_u32(ptr, 1_000_000);
            4
        });
        let _guard = setup_mock(mock);

        let result = parent_ledger_time();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1_000_000);
    }

    #[test]
    fn test_parent_ledger_time_error() {
        let mut mock = MockHostBindings::new();
        mock.expect_parent_ldgr_time()
            .times(1)
            .returning(|_, _| INTERNAL_ERROR);
        let _guard = setup_mock(mock);

        assert!(parent_ledger_time().is_err());
    }

    // ---- parent_ledger_hash ----

    #[test]
    fn test_parent_ledger_hash_success() {
        let mut mock = MockHostBindings::new();
        mock.expect_parent_ldgr_hash().times(1).returning(|ptr, _| {
            write_hash(ptr, 0xAB);
            32
        });
        let _guard = setup_mock(mock);

        let result = parent_ledger_hash();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), [0xAB; 32]);
    }

    #[test]
    fn test_parent_ledger_hash_error() {
        let mut mock = MockHostBindings::new();
        mock.expect_parent_ldgr_hash()
            .times(1)
            .returning(|_, _| INTERNAL_ERROR);
        let _guard = setup_mock(mock);

        assert!(parent_ledger_hash().is_err());
    }

    // ---- base_fee ----

    #[test]
    fn test_base_fee_success() {
        let mut mock = MockHostBindings::new();
        mock.expect_base_fee().times(1).returning(|ptr, _| {
            write_u32(ptr, 12);
            4
        });
        let _guard = setup_mock(mock);

        let result = base_fee();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 12);
    }

    #[test]
    fn test_base_fee_error() {
        let mut mock = MockHostBindings::new();
        mock.expect_base_fee()
            .times(1)
            .returning(|_, _| INTERNAL_ERROR);
        let _guard = setup_mock(mock);

        assert!(base_fee().is_err());
    }

    // ---- amendment_enabled ----

    #[test]
    fn test_amendment_enabled_true() {
        let mut mock = MockHostBindings::new();
        mock.expect_amendment_enabled().times(1).returning(|_, _| 1);
        let _guard = setup_mock(mock);

        assert!(amendment_enabled(&[0u8; 32]).unwrap());
    }

    #[test]
    fn test_amendment_enabled_false() {
        let mut mock = MockHostBindings::new();
        mock.expect_amendment_enabled().times(1).returning(|_, _| 0);
        let _guard = setup_mock(mock);

        assert!(!amendment_enabled(&[0u8; 32]).unwrap());
    }

    #[test]
    fn test_amendment_enabled_error() {
        let mut mock = MockHostBindings::new();
        mock.expect_amendment_enabled()
            .times(1)
            .returning(|_, _| INTERNAL_ERROR);
        let _guard = setup_mock(mock);

        assert!(amendment_enabled(&[0u8; 32]).is_err());
    }
}
