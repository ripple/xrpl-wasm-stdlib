use xrpl_macros::blob;

// Only [0-9a-fA-F] are valid hex characters. 'Z' must be rejected.
fn main() {
    blob!("ZZZZ");
}
