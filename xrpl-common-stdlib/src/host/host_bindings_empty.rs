// Stub host functions for plain native builds (`cargo build`, doc tests). Every host function
// exists, does nothing, and returns its last argument as an `i32`. For most functions that
// last argument is an output buffer length, so callers see "the whole buffer was written".
// This lets code compile and run on the host without setting up a mock.
//
// Everything here is expanded from the generated `for_each_host_function!` list
// (host_bindings_list.rs), so it needs no changes when rippled adds a host function.

/// Converts a stub's last argument into the `i32` it returns.
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

/// Converts that `i32` into the function's declared return type: `i32`, or `()` for `trace`.
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
    // `@last a, b, c` drops parameters from the front until only the last one is left.
    (@last) => { 0i32 };
    (@last $last:ident) => { StubArg::to_i32($last) };
    (@last $head:ident, $($rest:ident),+) => { stub_host_functions!(@last $($rest),+) };
}
for_each_host_function!(stub_host_functions);
