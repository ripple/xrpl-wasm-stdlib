//! Signature validation for entry-point macros.

use super::parse::EntryFn;

pub(crate) struct ValidationRules<'a> {
    pub expected_ctx_type: &'a str,
}

pub(crate) enum ReturnKind {
    FinishResult,
    I32,
}

pub(crate) fn validate(entry: &EntryFn, rules: &ValidationRules<'_>) -> syn::Result<ReturnKind> {
    check_params(entry, rules)?;
    classify_return_type(entry)
}

fn check_params(entry: &EntryFn, rules: &ValidationRules<'_>) -> syn::Result<()> {
    let inputs = &entry.func.sig.inputs;

    if inputs.is_empty() {
        return Err(syn::Error::new_spanned(
            &entry.func.sig.ident,
            format!(
                "entry-point function must take exactly one parameter of type `{}`",
                rules.expected_ctx_type
            ),
        ));
    }

    if inputs.len() > 1 {
        return Err(syn::Error::new_spanned(
            inputs.iter().nth(1).unwrap(),
            "entry-point function must take exactly one parameter",
        ));
    }

    let param = inputs.first().unwrap();
    let ty = match param {
        syn::FnArg::Typed(pat_type) => &*pat_type.ty,
        syn::FnArg::Receiver(r) => {
            return Err(syn::Error::new_spanned(
                r,
                "entry-point functions cannot take `self`",
            ));
        }
    };

    let matches = match ty {
        syn::Type::Path(type_path) => type_path
            .path
            .segments
            .last()
            .is_some_and(|seg| seg.ident == rules.expected_ctx_type),
        _ => false,
    };

    if !matches {
        return Err(syn::Error::new_spanned(
            ty,
            format!("parameter type must be `{}`", rules.expected_ctx_type),
        ));
    }

    Ok(())
}

fn classify_return_type(entry: &EntryFn) -> syn::Result<ReturnKind> {
    match &entry.func.sig.output {
        syn::ReturnType::Default => Err(syn::Error::new_spanned(
            &entry.func.sig.ident,
            "entry-point function must return `FinishResult` or `i32`",
        )),
        syn::ReturnType::Type(_, ty) => classify_return(ty),
    }
}

fn classify_return(ty: &syn::Type) -> syn::Result<ReturnKind> {
    let type_path = match ty {
        syn::Type::Path(p) => p,
        _ => return Err(return_type_error(ty)),
    };

    let last = match type_path.path.segments.last() {
        Some(s) => s,
        None => return Err(return_type_error(ty)),
    };

    if last.ident == "i32" {
        return Ok(ReturnKind::I32);
    } else if last.ident == "FinishResult" {
        return Ok(ReturnKind::FinishResult);
    }

    Err(return_type_error(ty))
}

fn return_type_error(ty: &syn::Type) -> syn::Error {
    syn::Error::new_spanned(ty, "return type must be `FinishResult` or `i32`")
}
