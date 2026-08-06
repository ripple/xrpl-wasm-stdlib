use xrpl_macros::smart_escrow;

#[smart_escrow]
fn run(ctx: u32) -> Result<bool, ()> {
    Ok(ctx > 0)
}

fn main() {}
