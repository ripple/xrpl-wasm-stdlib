extern crate std;
use std::vec::Vec;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    FnArg, Ident, ItemFn, Pat, Token, Type, parse::Parse, parse::ParseStream, parse_macro_input,
    punctuated::Punctuated,
};

// Parse: #[wasm_export(exit = my_exit, instance(max_limit: u32, amount: Amount))]
struct WasmExportArgs {
    exit: Option<Ident>,
    instance_params: Vec<(Ident, Type)>,
}

impl Parse for WasmExportArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut exit = None;
        let mut instance_params = Vec::new();

        while !input.is_empty() {
            let key: Ident = input.parse()?;

            if key == "exit" {
                input.parse::<Token![=]>()?;
                let value: Ident = input.parse()?;
                exit = Some(value);
            } else if key == "instance" {
                // Parse: instance(param1: Type1, param2: Type2)
                let content;
                syn::parenthesized!(content in input);

                let params: Punctuated<syn::FnArg, Token![,]> =
                    content.parse_terminated(syn::FnArg::parse, Token![,])?;

                for param in params {
                    if let syn::FnArg::Typed(pat_type) = param
                        && let Pat::Ident(pat_ident) = *pat_type.pat
                    {
                        instance_params.push((pat_ident.ident, *pat_type.ty));
                    }
                }
            } else {
                return Err(syn::Error::new(key.span(), "expected 'exit' or 'instance'"));
            }

            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(WasmExportArgs {
            exit,
            instance_params,
        })
    }
}

#[proc_macro_attribute]
pub fn wasm_export(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as WasmExportArgs);
    let input_fn = parse_macro_input!(item as ItemFn);

    let fn_name = &input_fn.sig.ident;
    let internal_fn_name = syn::Ident::new(&format!("{}_internal", fn_name), fn_name.span());

    // Extract instance parameters
    let mut instance_extractions = Vec::new();
    let mut instance_param_names = Vec::new();

    for (idx, (param_name, param_type)) in args.instance_params.iter().enumerate() {
        let idx_i32 = idx as i32;

        if let Some(ref exit_fn) = args.exit {
            instance_extractions.push(quote! {
                let #param_name = match xrpl_contract_stdlib::params::instance::get_instance_param::<#param_type>(#idx_i32) {
                    core::result::Result::Ok(val) => val,
                    core::result::Result::Err(err) => {
                        return #exit_fn(
                            "Failed to get instance parameter",
                            err as i32
                        );
                    }
                };
            });
        } else {
            instance_extractions.push(quote! {
                let #param_name = xrpl_contract_stdlib::params::instance::get_instance_param::<#param_type>(#idx_i32)
                    .expect("Failed to get instance parameter");
            });
        }

        instance_param_names.push(param_name);
    }

    // Extract function parameters
    let mut function_extractions = Vec::new();
    let mut function_param_names = Vec::new();

    for (idx, arg) in input_fn.sig.inputs.iter().enumerate() {
        if let FnArg::Typed(pat_type) = arg {
            let param_name = if let Pat::Ident(pat_ident) = &*pat_type.pat {
                &pat_ident.ident
            } else {
                panic!("Unsupported parameter pattern");
            };

            let param_type = &pat_type.ty;
            let idx_i32 = idx as i32;

            if let Some(ref exit_fn) = args.exit {
                function_extractions.push(quote! {
                    let #param_name = match xrpl_contract_stdlib::params::function::get_function_param::<#param_type>(#idx_i32) {
                        core::result::Result::Ok(val) => val,
                        core::result::Result::Err(err) => {
                            return #exit_fn(
                                "Failed to get function parameter",
                                err as i32
                            );
                        }
                    };
                });
            } else {
                function_extractions.push(quote! {
                    let #param_name = xrpl_contract_stdlib::params::function::safe_get_function_param::<#param_type>(#idx_i32);
                });
            }

            function_param_names.push(param_name);
        }
    }

    let fn_body = &input_fn.block;
    let fn_inputs = &input_fn.sig.inputs;
    let fn_output = &input_fn.sig.output;

    // Build internal function signature: instance params + function params
    let mut internal_params = Vec::new();
    for (param_name, param_type) in &args.instance_params {
        internal_params.push(quote! { #param_name: #param_type });
    }
    for arg in fn_inputs.iter() {
        internal_params.push(quote! { #arg });
    }

    // Combine all parameter names for the call
    let mut all_param_names = instance_param_names.clone();
    all_param_names.extend(function_param_names);

    let expanded = quote! {
        fn #internal_fn_name(#(#internal_params),*) #fn_output {
            #fn_body
        }

        #[unsafe(no_mangle)]
        pub extern "C" fn #fn_name() -> i32 {
            // Extract instance parameters (class members)
            #(#instance_extractions)*

            // Extract function parameters
            #(#function_extractions)*

            // Call internal function with all parameters
            #internal_fn_name(#(#all_param_names),*)
        }
    };

    TokenStream::from(expanded)
}
