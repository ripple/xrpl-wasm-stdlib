//! Signature validation for entry-point macros.

use super::parse::EntryFn;

pub(crate) struct ValidationRules<'a> {
    pub expected_ctx_type: &'a str,
    /// Name of an additional accepted return type that wraps an `i32` (e.g.
    /// `FinishResult`). `None` means only a bare `i32` return is accepted.
    pub wrapped_return_type: Option<&'a str>,
}

pub(crate) enum ReturnKind {
    /// A type other than `i32` that converts into one via `i32::from`.
    Wrapped,
    I32,
}

pub(crate) fn validate(entry: &EntryFn, rules: &ValidationRules<'_>) -> syn::Result<ReturnKind> {
    check_params(entry, rules)?;
    classify_return_type(entry, rules)
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

fn classify_return_type(entry: &EntryFn, rules: &ValidationRules<'_>) -> syn::Result<ReturnKind> {
    match &entry.func.sig.output {
        syn::ReturnType::Default => Err(syn::Error::new_spanned(
            &entry.func.sig.ident,
            format!(
                "entry-point function must return {}",
                expected_return_desc(rules)
            ),
        )),
        syn::ReturnType::Type(_, ty) => classify_return(ty, rules),
    }
}

fn classify_return(ty: &syn::Type, rules: &ValidationRules<'_>) -> syn::Result<ReturnKind> {
    let type_path = match ty {
        syn::Type::Path(p) => p,
        _ => return Err(return_type_error(ty, rules)),
    };

    let last = match type_path.path.segments.last() {
        Some(s) => s,
        None => return Err(return_type_error(ty, rules)),
    };

    if last.ident == "i32" {
        return Ok(ReturnKind::I32);
    } else if rules.wrapped_return_type.is_some_and(|w| last.ident == w) {
        return Ok(ReturnKind::Wrapped);
    }

    Err(return_type_error(ty, rules))
}

fn expected_return_desc(rules: &ValidationRules<'_>) -> String {
    match rules.wrapped_return_type {
        Some(wrapped) => format!("`{wrapped}` or `i32`"),
        None => "`i32`".to_string(),
    }
}

fn return_type_error(ty: &syn::Type, rules: &ValidationRules<'_>) -> syn::Error {
    syn::Error::new_spanned(
        ty,
        format!("return type must be {}", expected_return_desc(rules)),
    )
}
