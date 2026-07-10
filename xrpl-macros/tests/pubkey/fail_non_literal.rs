use xrpl_macros::pubkey;

// The macro only accepts string literals, never runtime expressions.
fn convert(hex: &str) {
    pubkey!(hex);
}

fn main() {
    convert("02C7387FFC25C156CA7F8A6D760C8D01EF642CEE9CE4680C33FFB3FF39AFECFE70");
}
