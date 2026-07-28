//! The slot-based `Escrow` handle plus its escrow-domain `Data` accessor.
//!
//! `Escrow` and the generic `EscrowFields` trait are generated in
//! `xrpl_common_stdlib::objects` (re-exported here for a stable path). The `Data` field
//! carries escrow-specific, host-mutable `ContractData` semantics the generator doesn't
//! model, so its accessor lives here as a hand-written extension trait.

pub use xrpl_common_stdlib::objects::Escrow;

use xrpl_common_stdlib::host::{Error, Result, Result::Err, Result::Ok, get_ledger_obj_field};
use xrpl_common_stdlib::objects::traits::EscrowFields;
use xrpl_common_stdlib::sfield;
use xrpl_common_stdlib::types::contract_data::{ContractData, XRPL_CONTRACT_DATA_SIZE};

/// Access to the escrow-specific `Data` (`ContractData`) field on any escrow ledger object.
pub trait EscrowContractData: EscrowFields {
    /// Retrieves the contract data stored on this escrow object.
    ///
    /// The data is read into a fixed-size buffer of `XRPL_CONTRACT_DATA_SIZE`.
    ///
    /// # Returns
    ///
    /// * `Ok(ContractData)` - the retrieved data and its actual length
    /// * `Err(Error)` - if the retrieval operation failed
    fn get_data(&self) -> Result<ContractData> {
        let mut data: [u8; XRPL_CONTRACT_DATA_SIZE] = [0; XRPL_CONTRACT_DATA_SIZE];

        let result_code = unsafe {
            get_ledger_obj_field(
                self.get_slot_num(),
                sfield::Data.into(),
                data.as_mut_ptr(),
                data.len(),
            )
        };

        match result_code {
            code if code >= 0 => Ok(ContractData {
                data,
                len: code as usize,
            }),
            code => Err(Error::from_code(code)),
        }
    }
}

impl EscrowContractData for Escrow {}

#[cfg(test)]
mod tests {
    use super::*;
    use xrpl_common_stdlib::objects::traits::LedgerObjectCommonFields;

    #[test]
    fn test_new() {
        let escrow = Escrow::new(42);
        assert_eq!(escrow.get_slot_num(), 42);
    }
}
