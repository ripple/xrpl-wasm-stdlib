use xrpl_macros::r_address;

// XRPL classic addresses must start with 'r'. An address with the wrong
// prefix should be rejected at compile time.
fn main() {
    r_address!("xHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh");
}
