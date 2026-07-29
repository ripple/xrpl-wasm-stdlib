use xrpl_macros::smart_contract;

struct ContractCallContext;

#[smart_contract]
mod freelancer {
    use super::ContractCallContext;

    #[call(name = "")]
    fn run(_ctx: ContractCallContext) -> i32 {
        0
    }
}

fn main() {}
