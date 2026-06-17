use proc_macro::TokenStream;
mod r_address;

#[proc_macro]
pub fn r_address(input: TokenStream) -> TokenStream {
    r_address::expand(input)
}
