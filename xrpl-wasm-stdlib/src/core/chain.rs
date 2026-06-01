use crate::host::{
    Error, Result, amendment_enabled as host_amendment_enabled,
    error_codes::match_result_code_with_expected_bytes, get_base_fee, get_ledger_sqn,
    get_parent_ledger_hash, get_parent_ledger_time,
};

pub fn ledger_sqn() -> Result<u32> {
    let mut uint_bytes = [0u8; 4];
    let rescode = unsafe { get_ledger_sqn(uint_bytes.as_mut_ptr(), 4) };
    match_result_code_with_expected_bytes(rescode, 4, || u32::from_be_bytes(uint_bytes))
}

pub fn parent_ledger_time() -> Result<u64> {
    let mut uint_bytes = [0u8; 8];
    let rescode = unsafe { get_parent_ledger_time(uint_bytes.as_mut_ptr(), 8) };
    match_result_code_with_expected_bytes(rescode, 8, || u64::from_be_bytes(uint_bytes))
}

pub fn parent_ledger_hash() -> Result<[u8; 32]> {
    let mut bytes = [0u8; 32];
    let rescode = unsafe { get_parent_ledger_hash(bytes.as_mut_ptr(), 32) };
    match_result_code_with_expected_bytes(rescode, 32, || bytes)
}

pub fn base_fee() -> Result<u64> {
    let mut uint_bytes = [0u8; 8];
    let rescode = unsafe { get_base_fee(uint_bytes.as_mut_ptr(), 8) };
    match_result_code_with_expected_bytes(rescode, 8, || u64::from_be_bytes(uint_bytes))
}

pub fn amendment_enabled(hash: &[u8; 32]) -> Result<bool> {
    let rescode = unsafe { host_amendment_enabled(hash.as_ptr(), 32) };
    match rescode {
        0 => Result::Ok(false),
        1 => Result::Ok(true),
        _ => Result::Err(Error::from_code(rescode)),
    }
}
