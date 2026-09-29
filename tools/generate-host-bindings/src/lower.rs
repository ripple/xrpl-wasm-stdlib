//! Lowers a declaration to its wasm wire signature. rippled's own macro documents the rules
//! (`crates/xrpl-host-functions/src/lib.rs`, "Two rules hold over every declaration"):
//! `i32`/`i64` pass through; `&[u8]`, `&str`, `&mut [u8]` **and `u32`** are `(ptr, len)`
//! regions; `TraceDataType` is an `i32` code; `HostResult<()>` has no result, everything else
//! is one `i32`.

use std::collections::HashSet;

use crate::parse::{Declared, HostFunction, Return};

/// One wasm-level parameter of a host import, in our Rust spelling (`usize` for lengths,
/// which is `i32` on wasm32).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WireParam {
    pub name: String,
    pub ty: &'static str,
}

/// The wire parameters of `f`, in order. A region `x` becomes `x_ptr`, `x_len`.
pub fn wire_params(f: &HostFunction) -> Result<Vec<WireParam>, String> {
    let mut out = Vec::new();
    for p in &f.params {
        match p.declared {
            Declared::I32 | Declared::TraceDataType => out.push(WireParam {
                name: p.name.clone(),
                ty: "i32",
            }),
            Declared::I64 => out.push(WireParam {
                name: p.name.clone(),
                ty: "i64",
            }),
            Declared::Bytes | Declared::Str | Declared::U32 => {
                out.push(WireParam {
                    name: format!("{}_ptr", p.name),
                    ty: "*const u8",
                });
                out.push(WireParam {
                    name: format!("{}_len", p.name),
                    ty: "usize",
                });
            }
            Declared::MutBytes => {
                out.push(WireParam {
                    name: format!("{}_ptr", p.name),
                    ty: "*mut u8",
                });
                out.push(WireParam {
                    name: format!("{}_len", p.name),
                    ty: "usize",
                });
            }
        }
    }
    let mut seen = HashSet::new();
    for w in &out {
        if !seen.insert(w.name.as_str()) {
            return Err(format!(
                "fn {}: wire parameter name `{}` is produced twice; rename the declared parameter",
                f.rust_name, w.name
            ));
        }
    }
    Ok(out)
}

/// The wire return type: `()` for `HostResult<()>`, otherwise `i32`.
pub fn wire_return(f: &HostFunction) -> &'static str {
    match f.returns {
        Return::Unit => "()",
        Return::Len | Return::Value => "i32",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::{Declared, Param, parse_host_functions};
    use crate::test_fixtures::SNIPPET;

    fn sig(f: &HostFunction) -> String {
        let params: Vec<String> = wire_params(f)
            .unwrap()
            .iter()
            .map(|p| format!("{}: {}", p.name, p.ty))
            .collect();
        format!(
            "fn {}({}) -> {};",
            f.wasm_name,
            params.join(", "),
            wire_return(f)
        )
    }

    #[test]
    fn a_mut_slice_becomes_a_mut_pointer_and_a_usize() {
        let fns = parse_host_functions(SNIPPET).unwrap();
        assert_eq!(
            sig(&fns[0]),
            "fn ldgr_index(out_ptr: *mut u8, out_len: usize) -> i32;"
        );
    }

    #[test]
    fn str_and_trace_data_type_and_bytes_keep_declaration_order() {
        let fns = parse_host_functions(SNIPPET).unwrap();
        assert_eq!(
            sig(&fns[1]),
            "fn trace(msg_ptr: *const u8, msg_len: usize, data_type: i32, data_ptr: *const u8, data_len: usize) -> ();"
        );
    }

    #[test]
    fn u32_is_a_const_region_not_a_scalar() {
        let fns = parse_host_functions(SNIPPET).unwrap();
        assert_eq!(
            sig(&fns[2]),
            "fn check_id(account_ptr: *const u8, account_len: usize, seq_ptr: *const u8, seq_len: usize, out_ptr: *mut u8, out_len: usize) -> i32;"
        );
    }

    #[test]
    fn scalars_pass_through() {
        let f = HostFunction {
            rust_name: "f".into(),
            wasm_name: "f".into(),
            gas: 1,
            docs: vec![],
            params: vec![
                Param {
                    name: "x".into(),
                    declared: Declared::I64,
                },
                Param {
                    name: "mode".into(),
                    declared: Declared::I32,
                },
            ],
            returns: Return::Value,
        };
        assert_eq!(sig(&f), "fn f(x: i64, mode: i32) -> i32;");
    }

    #[test]
    fn rejects_wire_name_collisions() {
        let f = HostFunction {
            rust_name: "f".into(),
            wasm_name: "f".into(),
            gas: 1,
            docs: vec![],
            params: vec![
                Param {
                    name: "x".into(),
                    declared: Declared::Bytes,
                },
                Param {
                    name: "x_ptr".into(),
                    declared: Declared::I32,
                },
            ],
            returns: Return::Value,
        };
        let err = wire_params(&f).unwrap_err();
        assert!(err.contains("fn f"), "{err}");
        assert!(err.contains("x_ptr"), "{err}");
    }
}
