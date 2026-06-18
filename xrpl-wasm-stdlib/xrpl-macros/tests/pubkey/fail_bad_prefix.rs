use xrpl_macros::pubkey;

// Only 02/03 (secp256k1) and ED (Ed25519) prefixes are valid; 04 must be rejected.
fn main() {
    pubkey!("04C7387FFC25C156CA7F8A6D760C8D01EF642CEE9CE4680C33FFB3FF39AFECFE70");
}
