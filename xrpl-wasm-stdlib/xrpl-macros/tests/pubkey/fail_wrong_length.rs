use xrpl_macros::pubkey;

// Pubkey requires exactly 66 hex characters (33 bytes). Fewer chars must be rejected.
fn main() {
    pubkey!("02C7387FFC");
}
