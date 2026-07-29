use xrpl_macros::{call, smart_contract};

#[smart_contract]
mod freelancer {
    #[call]
    fn run(ctx: u32) -> i32 {
        ctx as i32
    }
}

fn main() {}
