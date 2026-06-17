use proc_macro::TokenStream;
mod r_address;
mod typed_const;

#[proc_macro]
pub fn r_address(input: TokenStream) -> TokenStream {
    r_address::expand(input)
}

#[proc_macro]
pub fn hash256(input: TokenStream) -> TokenStream {
    typed_const::hash256(input)
}
