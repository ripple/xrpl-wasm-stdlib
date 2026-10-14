use regex::Regex;
use std::collections::BTreeMap;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

const FILE_TRAIT: &str = "host_bindings_trait.rs";
const FILE_WASM: &str = "host_bindings_wasm.rs";
const FILE_EMPTY: &str = "host_bindings_empty.rs";
const FILE_TEST: &str = "host_bindings_test.rs";

struct Param {
    name: String,
    ty: String,
}

struct TraitMethod {
    name: String,
    params: Vec<Param>,
    return_ty: String,
}

fn parse_trait_methods(content: &str) -> Vec<TraitMethod> {
    let mut methods = Vec::new();
    let re = Regex::new(
        r"unsafe fn ([A-Za-z0-9_]+)\s*\(\s*&self\s*(?:,\s*([^)]*))?\)\s*->\s*([A-Za-z0-9]+)\s*;",
    )
    .unwrap();

    for cap in re.captures_iter(content) {
        let name = cap.get(1).unwrap().as_str();
        let params_str = cap.get(2).map_or("", |m| m.as_str());
        let return_ty = cap.get(3).unwrap().as_str();

        let params: Vec<Param> = params_str
            .split(',')
            .map(|p| p.trim())
            .filter(|p| !p.is_empty())
            .map(|p| {
                let (name, ty) = p.split_once(':').unwrap();
                Param {
                    name: name.trim().to_string(),
                    ty: ty.trim().to_string(),
                }
            })
            .collect();

        methods.push(TraitMethod {
            name: name.to_string(),
            params,
            return_ty: return_ty.to_string(),
        });
    }
    methods
}

// ---------- Method 3: categorization ----------

const CATEGORY_NAMES: &[&str] = &[
    "Host Function Category: ledger and transaction info",
    "Host Function Category: update current ledger entry",
    "Host Function Category: hash and ledger entry ID computation",
    "Host Function Category: NFT",
    "Host Function Category: FLOAT",
    "Host Function Category: TRACE",
];

fn category_index(name: &str) -> Option<usize> {
    if name == "ldgr_index"
        || name.starts_with("parent_ldgr")
        || name == "base_fee"
        || name.starts_with("amendment")
        || name.starts_with("cache")
        || name.starts_with("tx_")
        || name.starts_with("home_le_")
        || name.starts_with("le_")
    {
        Some(0)
    } else if name == "set_data" {
        Some(1)
    } else if name.contains("id") || name == "sha512_half" || name == "check_sig" {
        Some(2)
    } else if name.starts_with("nft_") {
        Some(3)
    } else if name.starts_with("float_") {
        Some(4)
    } else if name.starts_with("trace") {
        Some(5)
    } else {
        None
    }
}

// ---------- Method 4: generators ----------

fn format_macro_fn_line(method: &TraitMethod, with_underscore_prefix: bool) -> String {
    let params = method
        .params
        .iter()
        .map(|p| {
            let name = if with_underscore_prefix {
                format!("_{}", p.name)
            } else {
                p.name.clone()
            };
            format!("{name}: {}", p.ty)
        })
        .collect::<Vec<_>>()
        .join(", ");
    format!("    fn {}({params}) -> {};", method.name, method.return_ty)
}

fn generate_export_macro_content(methods: &[TraitMethod], with_underscore_prefix: bool) -> String {
    // Group methods by category, preserving source order within each category.
    let mut grouped: BTreeMap<usize, Vec<&TraitMethod>> = BTreeMap::new();
    let mut uncategorized: Vec<&TraitMethod> = Vec::new();

    for m in methods {
        match category_index(&m.name) {
            Some(idx) => grouped.entry(idx).or_default().push(m),
            None => uncategorized.push(m),
        }
    }

    let mut lines: Vec<String> = Vec::new();
    for (idx, cat_methods) in &grouped {
        lines.push(format!("    // {}", CATEGORY_NAMES[*idx]));
        for method in cat_methods {
            lines.push(format_macro_fn_line(method, with_underscore_prefix));
        }
        lines.push(String::new());
    }

    if !uncategorized.is_empty() {
        lines.push("    // Other functions".to_string());
        for method in &uncategorized {
            lines.push(format_macro_fn_line(method, with_underscore_prefix));
        }
    }

    lines.join("\n")
}

fn generate_extern_block_content(methods: &[TraitMethod]) -> String {
    let mut lines: Vec<String> = Vec::new();
    for method in methods {
        if method.params.len() <= 2 {
            let params_inline = method
                .params
                .iter()
                .map(|p| format!("{}: {}", p.name, p.ty))
                .collect::<Vec<_>>()
                .join(", ");
            lines.push(format!(
                "        pub(super) fn {}({params_inline}) -> {};",
                method.name, method.return_ty
            ));
        } else {
            lines.push(format!("        pub(super) fn {}(", method.name));
            for p in &method.params {
                lines.push(format!("            {}: {},", p.name, p.ty));
            }
            lines.push(format!("        ) -> {};", method.return_ty));
        }
    }
    lines.join("\n")
}

fn generate_impl_block_content(methods: &[TraitMethod]) -> String {
    let mut lines: Vec<String> = Vec::new();
    for method in methods {
        let params_inline = method
            .params
            .iter()
            .map(|p| format!("{}: {}", p.name, p.ty))
            .collect::<Vec<_>>()
            .join(", ");
        let param_names = method
            .params
            .iter()
            .map(|p| p.name.as_str())
            .collect::<Vec<_>>()
            .join(", ");

        let self_params = if params_inline.is_empty() {
            "&self".to_string()
        } else {
            format!("&self, {params_inline}")
        };

        if method.params.len() <= 2 {
            lines.push(format!(
                "    unsafe fn {}({self_params}) -> {} {{",
                method.name, method.return_ty
            ));
            if param_names.is_empty() {
                lines.push(format!(
                    "        unsafe {{ host_defined_functions::{}() }}",
                    method.name
                ));
            } else {
                lines.push(format!(
                    "        unsafe {{ host_defined_functions::{}({param_names}) }}",
                    method.name
                ));
            }
            lines.push("    }".to_string());
        } else {
            lines.push(format!("    unsafe fn {}(", method.name));
            lines.push("        &self,".to_string());
            for p in &method.params {
                lines.push(format!("        {}: {},", p.name, p.ty));
            }
            lines.push(format!("    ) -> {} {{", method.return_ty));
            lines.push("        unsafe {".to_string());
            lines.push(format!(
                "            host_defined_functions::{}(",
                method.name
            ));
            for p in &method.params {
                lines.push(format!("                {},", p.name));
            }
            lines.push("            )".to_string());
            lines.push("        }".to_string());
            lines.push("    }".to_string());
        }
        lines.push(String::new());
    }
    lines.join("\n")
}

// ---------- Method 5: file splicing ----------

/// Replace the body of the last `export_host_functions! { ... }` invocation
/// (ignoring the macro definition, which contains `$name:ident`) with `new_body`.
fn splice_macro_invocation(content: &str, new_body: &str) -> Result<String, Box<dyn Error>> {
    let re = Regex::new(r"(?s)(export_host_functions!\s*\{)(.*?)(\n\})").unwrap();

    let mut target_start = 0usize;
    let mut target_end = 0usize;
    let mut target_prefix = String::new();
    let mut found = false;

    for cap in re.captures_iter(content) {
        let body = cap.get(2).unwrap().as_str();
        if body.contains("$name:ident") {
            continue;
        }
        let full = cap.get(0).unwrap();
        target_start = full.start();
        target_end = full.end();
        target_prefix = cap.get(1).unwrap().as_str().to_string();
        found = true;
    }

    if !found {
        return Err("no export_host_functions! invocation found".into());
    }

    let before = &content[..target_start];
    let after = &content[target_end..];
    Ok(format!("{before}{target_prefix}\n{new_body}\n}}{after}"))
}

fn splice_wasm_extern_block(content: &str, new_body: &str) -> Result<String, Box<dyn Error>> {
    let re = Regex::new(
        r#"(?s)(#\[link\(wasm_import_module = "host_lib"\)\]\s*unsafe extern "C" \{)(.*?)(\n    \})"#,
    )
    .unwrap();
    let cap = re
        .captures(content)
        .ok_or("could not find extern block in host_bindings_wasm.rs")?;
    let full = cap.get(0).unwrap();
    let prefix = cap.get(1).unwrap().as_str();
    let before = &content[..full.start()];
    let after = &content[full.end()..];
    Ok(format!("{before}{prefix}\n{new_body}\n    }}{after}"))
}

fn splice_wasm_impl_block(content: &str, new_body: &str) -> Result<String, Box<dyn Error>> {
    let re = Regex::new(
        r"(?s)(/// WASM implementation of HostBindings\.\nimpl HostBindings for WasmHostBindings \{)(.*?)(\n\})",
    )
    .unwrap();
    let cap = re
        .captures(content)
        .ok_or("could not find impl HostBindings for WasmHostBindings in host_bindings_wasm.rs")?;
    let full = cap.get(0).unwrap();
    let prefix = cap.get(1).unwrap().as_str();
    let before = &content[..full.start()];
    let after = &content[full.end()..];
    // Impl body already ends with "\n" (trailing blank line per method), so no extra "\n" before `}`.
    Ok(format!("{before}{prefix}\n{new_body}}}{after}"))
}

// ---------- Method 6: apply generators to the three files ----------

fn update_wasm_bindings(methods: &[TraitMethod]) -> Result<(), Box<dyn Error>> {
    let path = host_dir().join(FILE_WASM);
    let mut content = fs::read_to_string(&path)?;

    content = splice_wasm_extern_block(&content, &generate_extern_block_content(methods))?;
    content = splice_wasm_impl_block(&content, &generate_impl_block_content(methods))?;
    content = splice_macro_invocation(&content, &generate_export_macro_content(methods, false))?;

    fs::write(&path, &content)?;
    println!("  Updated {}", path.display());
    Ok(())
}

fn update_export_macro(
    filename: &str,
    methods: &[TraitMethod],
    with_underscore_prefix: bool,
) -> Result<(), Box<dyn Error>> {
    let path = host_dir().join(filename);
    let content = fs::read_to_string(&path)?;
    let new_body = generate_export_macro_content(methods, with_underscore_prefix);
    let new_content = splice_macro_invocation(&content, &new_body)?;
    fs::write(&path, &new_content)?;
    println!("  Updated {}", path.display());
    Ok(())
}

// ---------- entry point ----------

fn host_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../xrpl-common-stdlib/src/host")
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Reading host bindings trait file (source of truth)...");
    let trait_content = fs::read_to_string(host_dir().join(FILE_TRAIT))?;

    let methods = parse_trait_methods(&trait_content);
    println!("  Found {} methods in {FILE_TRAIT}", methods.len());

    println!("\nUpdating derived files...");
    update_wasm_bindings(&methods)?;
    update_export_macro(FILE_EMPTY, &methods, true)?;
    update_export_macro(FILE_TEST, &methods, false)?;

    println!(
        "\n✅ Successfully updated {} function signatures in 3 files.",
        methods.len()
    );
    Ok(())
}
