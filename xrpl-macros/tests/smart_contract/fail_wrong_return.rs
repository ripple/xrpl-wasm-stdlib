use xrpl_macros::{call, smart_contract};

struct ContractCallContext;

#[smart_contract]
mod freelancer {
    use super::ContractCallContext;

    #[call]
    fn run(_ctx: ContractCallContext) -> String {
        String::new()
    }
}

fn main() {}
