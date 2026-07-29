use xrpl_macros::{call, init, smart_contract};

struct ContractCallContext;

#[smart_contract]
mod freelancer {
    use super::ContractCallContext;

    #[init]
    #[call]
    fn run(_ctx: ContractCallContext) -> i32 {
        0
    }
}

fn main() {}
