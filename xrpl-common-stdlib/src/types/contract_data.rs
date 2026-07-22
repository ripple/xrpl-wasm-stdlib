pub const XRPL_CONTRACT_DATA_SIZE: usize = 4096; //TODO size??

/// A 4096-byte buffer for contract data on the XRP Ledger.
///
/// This type holds arbitrary contract data with a tracked length field
/// to indicate the actual amount of data stored (which may be less than
/// the full buffer capacity).
///
/// ## Derived Traits
///
/// - `PartialEq, Eq`: Enable comparisons
/// - `Debug, Clone`: Standard traits for development and consistency
///
/// Note: `Copy` is intentionally not derived due to the struct's size (4096+ bytes).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContractData {
    pub data: [u8; XRPL_CONTRACT_DATA_SIZE],

    /// The actual length of this contract data, if less than data.len()
    pub len: usize,
}
