#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_contract_stdlib::ContractCallContext;
use xrpl_macros::smart_contract;

#[smart_contract]
mod freelancer {
    use super::ContractCallContext;
    use xrpl_contract_stdlib::host::trace::trace;

    #[init]
    fn initialize(_ctx: ContractCallContext) -> i32 {
        let _ = trace("initialize");
        0
    }

    // Exported ABI name is pinned to "submit_work" independently of the Rust
    // identifier, so callers aren't broken by a Rust-side rename.
    #[call(name = "submit_work")]
    fn handle_submit_work(_ctx: ContractCallContext) -> i32 {
        let _ = trace("submit_work");
        0
    }

    #[user_delete]
    fn cleanup(_ctx: ContractCallContext) -> i32 {
        let _ = trace("cleanup");
        0
    }

    #[clawback]
    fn seize(_ctx: ContractCallContext) -> i32 {
        let _ = trace("seize");
        0
    }
}
