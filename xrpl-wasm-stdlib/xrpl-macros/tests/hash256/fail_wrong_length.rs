use xrpl_macros::hash256;

// Hash256 requires exactly 64 hex characters (32 bytes). Fewer chars must be rejected.
fn main() {
    hash256!("abc");
}
