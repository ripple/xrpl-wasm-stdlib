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

    #[call]
    fn submit_work(_ctx: ContractCallContext) -> i32 {
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
