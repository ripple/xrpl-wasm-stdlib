use xrpl_macros::hash256;

// The macro only accepts string literals, never runtime expressions.
fn convert(hash: &str) {
    hash256!(hash);
}

fn main() {
    convert("abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890");
}
