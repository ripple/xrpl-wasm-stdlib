use proc_macro::TokenStream;
mod r_address;

#[proc_macro]
pub fn r_address(input: TokenStream) -> TokenStream {
    match r_address::expand(input.into()) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
