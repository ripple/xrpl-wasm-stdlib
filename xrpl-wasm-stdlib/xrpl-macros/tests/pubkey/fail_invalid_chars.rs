use xrpl_macros::pubkey;

// Only [0-9a-fA-F] are valid hex characters. 'Z' must be rejected.
fn main() {
    pubkey!("02C7387FFC25C156CA7F8A6D760C8D01EF642CEE9CE4680C33FFB3FF39AFECFEZZ");
}
