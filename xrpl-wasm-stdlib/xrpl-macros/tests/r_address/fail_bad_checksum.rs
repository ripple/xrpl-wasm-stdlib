use xrpl_macros::r_address;

// Last character flipped from the canonical address, so the base58
// checksum no longer matches the payload.
fn main() {
    r_address!("rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTa");
}
