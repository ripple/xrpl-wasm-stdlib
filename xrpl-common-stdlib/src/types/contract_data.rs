//! `ContractData` — the value type of an escrow's `Data` field.
//!
//! An escrow's contract data is a plain fixed-max byte buffer, structurally identical to a
//! [`StandardBlob`] (`Blob<DEFAULT_BLOB_SIZE>`). It is therefore defined as a transparent alias
//! rather than a distinct struct: existing `ContractData` references stay valid, and because the
//! generic ledger-object getters decode any `Blob<N>`, the `Data` field can be read through the
//! generated `EscrowFields::data()` getter like any other field. The escrow-only *write*
//! (`set_data`) still lives in `xrpl-escrow-stdlib`.

use crate::types::blob::{DEFAULT_BLOB_SIZE, StandardBlob};

/// Maximum size, in bytes, of an escrow's contract data. Tied to [`DEFAULT_BLOB_SIZE`] so it can
/// never drift from the [`StandardBlob`] buffer that backs [`ContractData`].
pub const XRPL_CONTRACT_DATA_SIZE: usize = DEFAULT_BLOB_SIZE;

/// A fixed-max byte buffer holding an escrow's `Data` field. Alias for [`StandardBlob`].
pub type ContractData = StandardBlob;
