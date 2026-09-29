//! A three-function `host_functions!` block covering every declared type the lowering
//! handles: a `&mut [u8]` output, a `&str` + `TraceDataType` + `&[u8]` trace, and a
//! `&[u8]` + `u32` + `&mut [u8]` keylet.

pub(crate) const SNIPPET: &str = r#"
use xrpl_host_functions_macros::host_functions;

pub type HostResult<T> = Result<T, HostError>;

host_functions! {
    /// The sequence number of the ledger being built, as 4 little-endian bytes.
    #[gas = 60]
    #[wasm_name = "ldgr_index"]
    fn get_ledger_sqn(&self, out: &mut [u8]) -> HostResult<usize>;

    /// Writes `msg` to the trace log, see [`HostFunctions::get_ledger_sqn`] and [`HASH_LEN`].
    #[gas = 30]
    #[wasm_name = "trace"]
    fn trace(&self, msg: &str, data_type: TraceDataType, data: &[u8]) -> HostResult<()>;

    /// The 32-byte keylet of a `Check`.
    #[gas = 350]
    #[wasm_name = "check_id"]
    fn check_keylet(&self, account: &[u8], seq: u32, out: &mut [u8]) -> HostResult<usize>;
}
"#;
