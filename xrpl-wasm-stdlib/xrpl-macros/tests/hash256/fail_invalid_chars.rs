use xrpl_macros::hash256;

// Only [0-9a-fA-F] are valid hex characters. 'G' must be rejected.
fn main() {
    hash256!("GGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGG");
}
