use xrpl_macros::smart_contract;

struct ContractCallContext;

#[smart_contract]
mod freelancer {
    use super::ContractCallContext;

    #[call(name = "not a valid ident")]
    fn run(_ctx: ContractCallContext) -> i32 {
        0
    }
}

fn main() {}
