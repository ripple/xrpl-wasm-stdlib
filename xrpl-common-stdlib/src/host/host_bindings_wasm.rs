// The real FFI implementation of `HostBindings`, for wasm32 targets. Expands from the
// generated `for_each_host_function!` list (host_bindings_list.rs), so adding a host
// function upstream changes nothing here.
use crate::host::host_bindings_trait::HostBindings;

/// Hides the raw imports so every caller goes through the wrappers below, which is what lets
/// `host_bindings_test.rs` / `host_bindings_empty.rs` swap in for non-wasm targets.
mod host_defined_functions {
    macro_rules! declare_host_imports {
        ($( fn $name:ident($($param:ident: $param_ty:ty),*) -> $ret:ty; )*) => {
            #[link(wasm_import_module = "host_lib")]
            unsafe extern "C" {
                $(
                    #[allow(clippy::unused_unit)]
                    pub(super) fn $name($($param: $param_ty),*) -> $ret;
                )*
            }
        };
    }
    for_each_host_function!(declare_host_imports);
}

/// Implementation of host bindings for WASM targets.
pub struct WasmHostBindings;

macro_rules! impl_wasm_host_bindings {
    ($( fn $name:ident($($param:ident: $param_ty:ty),*) -> $ret:ty; )*) => {
        /// WASM implementation of HostBindings.
        impl HostBindings for WasmHostBindings {
            $(
                #[allow(clippy::too_many_arguments, clippy::unused_unit)]
                unsafe fn $name(&self, $($param: $param_ty),*) -> $ret {
                    unsafe { host_defined_functions::$name($($param),*) }
                }
            )*
        }

        // Free-function wrappers — `host::ldgr_index(...)` etc. — the API the rest of the
        // crate calls, so the implementation can be swapped per target.
        $(
            #[allow(clippy::too_many_arguments, clippy::missing_safety_doc, clippy::unused_unit)]
            pub unsafe fn $name($($param: $param_ty),*) -> $ret {
                unsafe { host_defined_functions::$name($($param),*) }
            }
        )*
    };
}
for_each_host_function!(impl_wasm_host_bindings);
