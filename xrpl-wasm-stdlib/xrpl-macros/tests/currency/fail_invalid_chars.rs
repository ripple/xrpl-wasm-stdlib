use xrpl_macros::currency;

// Standard 3-char codes must be alphanumeric; punctuation is rejected.
fn main() {
    currency!("U$D");
}
