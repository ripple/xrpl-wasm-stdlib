// Stub implementations for plain native builds (`cargo build`, doc tests): every host
// function exists and returns its last argument as an `i32` — a buffer length, for most —
// so code that only needs to compile and run trivially on the host works without a mock.
// Expands from the generated `for_each_host_function!` list (host_bindings_list.rs).

/// What a stub reports: its last argument, as the wire `i32`.
trait StubArg {
    fn to_i32(self) -> i32;
}
impl StubArg for usize {
    fn to_i32(self) -> i32 {
        self as i32
    }
}
impl StubArg for i32 {
    fn to_i32(self) -> i32 {
        self
    }
}
impl StubArg for i64 {
    fn to_i32(self) -> i32 {
        self as i32
    }
}
impl StubArg for *const u8 {
    fn to_i32(self) -> i32 {
        0
    }
}
impl StubArg for *mut u8 {
    fn to_i32(self) -> i32 {
        0
    }
}

/// Lifts that `i32` into the declared return type: `i32`, or `()` for `trace`.
trait StubReturn {
    fn from_i32(value: i32) -> Self;
}
impl StubReturn for i32 {
    fn from_i32(value: i32) -> Self {
        value
    }
}
impl StubReturn for () {
    fn from_i32(_: i32) -> Self {}
}

macro_rules! stub_host_functions {
    ($( fn $name:ident($($param:ident: $param_ty:ty),*) -> $ret:ty; )*) => {
        $(
            #[allow(
                unused_variables,
                clippy::too_many_arguments,
                clippy::missing_safety_doc,
                clippy::unused_unit
            )]
            pub unsafe fn $name($($param: $param_ty),*) -> $ret {
                <$ret as StubReturn>::from_i32(stub_host_functions!(@last $($param),*))
            }
        )*
    };
    // Tail recursion to the last parameter name.
    (@last) => { 0i32 };
    (@last $last:ident) => { StubArg::to_i32($last) };
    (@last $head:ident, $($rest:ident),+) => { stub_host_functions!(@last $($rest),+) };
}
for_each_host_function!(stub_host_functions);
