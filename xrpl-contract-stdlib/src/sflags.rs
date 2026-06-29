#![allow(non_upper_case_globals)]

//! Contract-specific transaction flags. Also re-exports the shared flags from
//! `xrpl_wasm_stdlib::sflags` so contract code can use both via a single
//! `use crate::sflags;` import.

pub use xrpl_wasm_stdlib::sflags::*;

// Contract flags:
pub const tfImmutable: u32 = 0x00010000;
pub const tfCodeImmutable: u32 = 0x00020000;
pub const tfABIImmutable: u32 = 0x00040000;
pub const tfUndeletable: u32 = 0x00080000;
pub const tfContractMask: u32 =
    !(tfUniversal | tfImmutable | tfCodeImmutable | tfABIImmutable | tfUndeletable);

// Contract parameter flags:
pub const tfSendAmount: u32 = 0x00010000;
pub const tfSendNFToken: u32 = 0x00020000;
pub const tfAuthorizeToken: u32 = 0x00040000;
pub const tfContractParameterMask: u32 = !(tfSendAmount | tfSendNFToken | tfAuthorizeToken);
