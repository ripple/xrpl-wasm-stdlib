use xrpl_macros::r_address;

// The macro only accepts string literals, never runtime expressions.
fn convert(addr: &str) {
    r_address!(addr);
}

fn main() {
    convert("rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh");
}
