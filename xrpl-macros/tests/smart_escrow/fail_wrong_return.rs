use xrpl_macros::smart_escrow;

struct EscrowFinishContext;

#[smart_escrow]
fn run(_ctx: EscrowFinishContext) -> String {
    String::new()
}

fn main() {}
