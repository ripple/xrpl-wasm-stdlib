use proc_macro::TokenStream;

// TODO: Smart Contract entry-point validation and codegen are pending.
// Pass through the annotated item unchanged so the crate compiles during the
// rearchitecture phase.
pub(crate) fn expand(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
