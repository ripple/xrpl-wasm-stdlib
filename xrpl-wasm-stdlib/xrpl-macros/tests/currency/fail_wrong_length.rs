use xrpl_macros::currency;

// Currency codes must be either 3 ASCII chars (standard) or 40 hex chars (non-standard).
fn main() {
    currency!("USDT");
}
