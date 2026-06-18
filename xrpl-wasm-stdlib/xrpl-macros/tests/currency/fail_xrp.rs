use xrpl_macros::currency;

// "XRP" is reserved for the native currency and must be rejected (any case).
fn main() {
    currency!("XRP");
}
