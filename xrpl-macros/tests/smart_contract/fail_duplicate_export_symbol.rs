use xrpl_macros::smart_contract;

struct ContractCallContext;

#[smart_contract]
mod freelancer {
    use super::ContractCallContext;

    #[call(name = "submit_work")]
    fn a(_ctx: ContractCallContext) -> i32 {
        0
    }

    #[call(name = "submit_work")]
    fn b(_ctx: ContractCallContext) -> i32 {
        0
    }
}

fn main() {}
