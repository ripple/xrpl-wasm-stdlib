use xrpl_macros::currency;

// The macro only accepts string literals, never runtime expressions.
fn convert(code: &str) {
    currency!(code);
}

fn main() {
    convert("USD");
}
