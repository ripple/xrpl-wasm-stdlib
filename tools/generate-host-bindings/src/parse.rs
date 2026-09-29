//! Parses the `host_functions! { ... }` declaration block out of rippled's
//! `crates/xrpl-host-functions/src/lib.rs`.

use syn::parse::{Parse, ParseStream};
use syn::{
    Expr, FnArg, GenericArgument, Item, Lit, Pat, PathArguments, ReturnType, TraitItemFn, Type,
};

/// A parameter type exactly as rippled declares it. Wire lowering lives in `lower.rs`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Declared {
    I32,
    I64,
    /// Travels as a `(ptr, len)` region of four little-endian bytes, not as a scalar.
    U32,
    /// `&[u8]`
    Bytes,
    /// `&str`
    Str,
    /// `&mut [u8]`
    MutBytes,
    /// An `i32` code on the wire.
    TraceDataType,
}

/// The `HostResult<T>` payload, which decides the wasm result and the doc text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Return {
    /// `HostResult<usize>`: the length written to an output region.
    Len,
    /// `HostResult<i32>` or `HostResult<FloatOrdering>`: the answer itself.
    Value,
    /// `HostResult<()>`: the import has no wasm result.
    Unit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Param {
    pub name: String,
    pub declared: Declared,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostFunction {
    /// The declaration's Rust name (`get_ledger_sqn`); only used to rewrite doc links.
    pub rust_name: String,
    /// The `#[wasm_name]` import name (`ldgr_index`): our trait method name.
    pub wasm_name: String,
    pub gas: u64,
    /// Doc lines exactly as written after `///`, leading space included.
    pub docs: Vec<String>,
    pub params: Vec<Param>,
    pub returns: Return,
}

/// Parses the whole `lib.rs` source and returns its declarations in order.
pub fn parse_host_functions(source: &str) -> Result<Vec<HostFunction>, String> {
    let file = syn::parse_file(source).map_err(|e| format!("source is not valid Rust: {e}"))?;
    let tokens = file
        .items
        .iter()
        .find_map(|item| match item {
            Item::Macro(m) if m.mac.path.is_ident("host_functions") => Some(m.mac.tokens.clone()),
            _ => None,
        })
        .ok_or_else(|| "no `host_functions! { ... }` block found".to_string())?;
    let Block(fns) = syn::parse2(tokens).map_err(|e| format!("inside host_functions!: {e}"))?;
    fns.iter().map(convert).collect()
}

struct Block(Vec<TraitItemFn>);

impl Parse for Block {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut fns = Vec::new();
        while !input.is_empty() {
            fns.push(input.parse()?);
        }
        Ok(Block(fns))
    }
}

fn convert(f: &TraitItemFn) -> Result<HostFunction, String> {
    let rust_name = f.sig.ident.to_string();
    let ctx = {
        let name = rust_name.clone();
        move |msg: String| format!("fn {name}: {msg}")
    };

    let mut wasm_name = None;
    let mut gas = None;
    let mut docs = Vec::new();
    for attr in &f.attrs {
        let Ok(nv) = attr.meta.require_name_value() else {
            continue;
        };
        let Expr::Lit(lit) = &nv.value else { continue };
        let path = attr.path();
        if path.is_ident("doc")
            && let Lit::Str(s) = &lit.lit
        {
            docs.push(s.value());
        } else if path.is_ident("wasm_name")
            && let Lit::Str(s) = &lit.lit
        {
            wasm_name = Some(s.value());
        } else if path.is_ident("gas")
            && let Lit::Int(i) = &lit.lit
        {
            gas = Some(i.base10_parse::<u64>().map_err(|e| ctx(e.to_string()))?);
        }
    }
    let wasm_name = wasm_name.ok_or_else(|| ctx("missing `#[wasm_name = \"...\"]`".into()))?;
    let gas = gas.ok_or_else(|| ctx("missing `#[gas = N]`".into()))?;

    let mut params = Vec::new();
    for arg in &f.sig.inputs {
        let FnArg::Typed(pt) = arg else { continue }; // skips `&self`
        let Pat::Ident(pi) = &*pt.pat else {
            return Err(ctx("parameter pattern must be a plain identifier".into()));
        };
        params.push(Param {
            name: pi.ident.to_string(),
            declared: classify(&pt.ty).map_err(&ctx)?,
        });
    }
    let returns = classify_return(&f.sig.output).map_err(&ctx)?;

    Ok(HostFunction {
        rust_name,
        wasm_name,
        gas,
        docs,
        params,
        returns,
    })
}

fn classify(ty: &Type) -> Result<Declared, String> {
    match ty {
        Type::Reference(r) => match (&*r.elem, r.mutability.is_some()) {
            (Type::Slice(s), false) if is_ident(&s.elem, "u8") => Ok(Declared::Bytes),
            (Type::Slice(s), true) if is_ident(&s.elem, "u8") => Ok(Declared::MutBytes),
            (elem, false) if is_ident(elem, "str") => Ok(Declared::Str),
            _ => Err(format!("unsupported parameter type `{}`", render(ty))),
        },
        _ if is_ident(ty, "i32") => Ok(Declared::I32),
        _ if is_ident(ty, "i64") => Ok(Declared::I64),
        _ if is_ident(ty, "u32") => Ok(Declared::U32),
        _ if is_ident(ty, "TraceDataType") => Ok(Declared::TraceDataType),
        _ => Err(format!("unsupported parameter type `{}`", render(ty))),
    }
}

fn classify_return(output: &ReturnType) -> Result<Return, String> {
    let ReturnType::Type(_, ty) = output else {
        return Err("missing `-> HostResult<..>`".into());
    };
    let Type::Path(p) = &**ty else {
        return Err(format!(
            "return type must be `HostResult<..>`, got `{}`",
            render(ty)
        ));
    };
    let seg = p
        .path
        .segments
        .last()
        .ok_or_else(|| "empty return type path".to_string())?;
    if seg.ident != "HostResult" {
        return Err(format!(
            "return type must be `HostResult<..>`, got `{}`",
            render(ty)
        ));
    }
    let PathArguments::AngleBracketed(args) = &seg.arguments else {
        return Err("`HostResult` needs a type argument".into());
    };
    let Some(GenericArgument::Type(inner)) = args.args.first() else {
        return Err("`HostResult` needs a type argument".into());
    };
    match inner {
        Type::Tuple(t) if t.elems.is_empty() => Ok(Return::Unit),
        _ if is_ident(inner, "usize") => Ok(Return::Len),
        _ if is_ident(inner, "i32") || is_ident(inner, "FloatOrdering") => Ok(Return::Value),
        _ => Err(format!(
            "unsupported `HostResult` payload `{}`",
            render(inner)
        )),
    }
}

fn is_ident(ty: &Type, name: &str) -> bool {
    matches!(ty, Type::Path(p) if p.qself.is_none() && p.path.is_ident(name))
}

fn render(ty: &Type) -> String {
    quote::ToTokens::to_token_stream(ty).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_fixtures::SNIPPET;

    #[test]
    fn parses_every_declaration_in_order() {
        let fns = parse_host_functions(SNIPPET).unwrap();
        let names: Vec<&str> = fns.iter().map(|f| f.wasm_name.as_str()).collect();
        assert_eq!(names, ["ldgr_index", "trace", "check_id"]);
        assert_eq!(fns[0].rust_name, "get_ledger_sqn");
        assert_eq!(fns[0].gas, 60);
    }

    #[test]
    fn keeps_doc_lines_verbatim_including_the_leading_space() {
        let fns = parse_host_functions(SNIPPET).unwrap();
        assert_eq!(
            fns[0].docs,
            vec![" The sequence number of the ledger being built, as 4 little-endian bytes."]
        );
    }

    #[test]
    fn classifies_every_declared_parameter_type() {
        let fns = parse_host_functions(SNIPPET).unwrap();
        let kinds = |i: usize| -> Vec<(String, Declared)> {
            fns[i]
                .params
                .iter()
                .map(|p| (p.name.clone(), p.declared))
                .collect()
        };
        assert_eq!(kinds(0), vec![("out".into(), Declared::MutBytes)]);
        assert_eq!(
            kinds(1),
            vec![
                ("msg".into(), Declared::Str),
                ("data_type".into(), Declared::TraceDataType),
                ("data".into(), Declared::Bytes),
            ]
        );
        assert_eq!(
            kinds(2),
            vec![
                ("account".into(), Declared::Bytes),
                ("seq".into(), Declared::U32),
                ("out".into(), Declared::MutBytes),
            ]
        );
    }

    #[test]
    fn classifies_return_types() {
        let fns = parse_host_functions(SNIPPET).unwrap();
        assert_eq!(fns[0].returns, Return::Len);
        assert_eq!(fns[1].returns, Return::Unit);
        assert_eq!(fns[2].returns, Return::Len);
    }

    #[test]
    fn i32_and_float_ordering_payloads_are_values() {
        let src = r#"host_functions! {
            #[gas = 1] #[wasm_name = "a"] fn a(&self, x: i32) -> HostResult<i32>;
            #[gas = 1] #[wasm_name = "b"] fn b(&self, x: &[u8], y: i64) -> HostResult<FloatOrdering>;
        }"#;
        let fns = parse_host_functions(src).unwrap();
        assert_eq!(fns[0].returns, Return::Value);
        assert_eq!(fns[0].params[0].declared, Declared::I32);
        assert_eq!(fns[1].returns, Return::Value);
        assert_eq!(fns[1].params[1].declared, Declared::I64);
    }

    #[test]
    fn rejects_unsupported_parameter_type() {
        let src = r#"host_functions! {
            #[gas = 1] #[wasm_name = "a"] fn a(&self, n: u64) -> HostResult<i32>;
        }"#;
        let err = parse_host_functions(src).unwrap_err();
        assert!(err.contains("fn a"), "{err}");
        assert!(err.contains("u64"), "{err}");
    }

    #[test]
    fn rejects_unsupported_return_payload() {
        let src = r#"host_functions! {
            #[gas = 1] #[wasm_name = "a"] fn a(&self, x: i32) -> HostResult<u64>;
        }"#;
        let err = parse_host_functions(src).unwrap_err();
        assert!(err.contains("fn a"), "{err}");
        assert!(err.contains("u64"), "{err}");
    }

    #[test]
    fn rejects_missing_wasm_name() {
        let src = r#"host_functions! {
            #[gas = 1] fn a(&self, x: i32) -> HostResult<i32>;
        }"#;
        let err = parse_host_functions(src).unwrap_err();
        assert!(err.contains("wasm_name"), "{err}");
    }

    #[test]
    fn rejects_source_without_a_block() {
        let err = parse_host_functions("fn main() {}").unwrap_err();
        assert!(err.contains("host_functions!"), "{err}");
    }
}
