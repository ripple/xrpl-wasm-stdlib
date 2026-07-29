use xrpl_macros::{init, smart_contract};

struct ContractCallContext;

#[smart_contract]
mod freelancer {
    use super::ContractCallContext;

    #[init]
    fn setup(_ctx: ContractCallContext) -> i32 {
        0
    }

    #[init]
    fn setup_again(_ctx: ContractCallContext) -> i32 {
        0
    }
}

fn main() {}
