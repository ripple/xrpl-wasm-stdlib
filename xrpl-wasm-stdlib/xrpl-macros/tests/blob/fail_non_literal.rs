use xrpl_macros::blob;

// The macro only accepts string literals, never runtime expressions.
fn convert(hex: &str) {
    blob!(hex);
}

fn main() {
    convert("DEADBEEF");
}
