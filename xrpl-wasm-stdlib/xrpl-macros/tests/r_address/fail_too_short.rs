use xrpl_macros::r_address;

// Far shorter than a valid r-address; decode cannot produce 1 + 20 + 4 bytes.
fn main() {
    r_address!("rTooShort");
}
