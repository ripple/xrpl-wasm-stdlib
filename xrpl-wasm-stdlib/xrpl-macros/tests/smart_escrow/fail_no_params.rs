use xrpl_macros::smart_escrow;

#[smart_escrow]
fn run() -> Result<bool, ()> {
    Ok(true)
}

fn main() {}
