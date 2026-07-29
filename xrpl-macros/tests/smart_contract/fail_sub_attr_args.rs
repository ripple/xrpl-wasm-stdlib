use xrpl_macros::{call, smart_contract};

struct ContractCallContext;

#[smart_contract]
mod freelancer {
    use super::ContractCallContext;

    #[call(name = "submit_work")]
    fn run(_ctx: ContractCallContext) -> i32 {
        0
    }
}

fn main() {}
