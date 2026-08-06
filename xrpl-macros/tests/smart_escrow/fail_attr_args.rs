use xrpl_macros::smart_escrow;

struct EscrowFinishContext;

#[smart_escrow(foo)]
fn run(_ctx: EscrowFinishContext) -> Result<bool, ()> {
    Ok(true)
}

fn main() {}
