pub mod account_id;
pub mod amount;
pub mod array;
pub mod blob;
pub mod constants;
pub mod contract_data;
pub mod currency;
pub mod issue;
pub mod mpt_id;
pub mod nft;
pub mod object;
pub mod opaque_float;
pub mod public_key;
pub mod transaction_type;
pub mod uint;

// TODO: Move these to the `types` crate.
// Relocated from the old top-level `src/types.rs`, which collided with this
// directory's module path once `core::types` was promoted to `crate::types`.
pub const XRPL_CONTRACT_DATA_SIZE: usize = 4096; //TODO size??
pub type ContractData = [u8; XRPL_CONTRACT_DATA_SIZE];
