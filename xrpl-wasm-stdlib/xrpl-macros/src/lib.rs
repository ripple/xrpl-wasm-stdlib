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

#[proc_macro]
pub fn pubkey(input: TokenStream) -> TokenStream {
    typed_const::pubkey(input)
}

#[proc_macro]
pub fn currency(input: TokenStream) -> TokenStream {
    typed_const::currency(input)
}

#[proc_macro]
pub fn blob(input: TokenStream) -> TokenStream {
    typed_const::blob(input)
}
