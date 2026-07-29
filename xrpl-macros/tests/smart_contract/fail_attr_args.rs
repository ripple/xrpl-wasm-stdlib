use xrpl_macros::{call, smart_contract};

struct ContractCallContext;

#[smart_contract(foo)]
mod freelancer {
    use super::ContractCallContext;

    #[call]
    fn run(_ctx: ContractCallContext) -> i32 {
        0
    }
}

fn main() {}
