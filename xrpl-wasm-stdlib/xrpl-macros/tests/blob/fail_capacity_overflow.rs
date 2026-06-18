use xrpl_macros::blob;

// The hex payload must not exceed the declared blob capacity.
fn main() {
    blob!("DEADBEEFCAFEBABE", 4);
}
