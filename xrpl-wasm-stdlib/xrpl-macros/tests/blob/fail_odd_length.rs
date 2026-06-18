use xrpl_macros::blob;

// Hex must come in whole bytes — odd character counts must be rejected.
fn main() {
    blob!("ABC");
}
