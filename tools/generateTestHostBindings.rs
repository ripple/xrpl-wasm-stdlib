use regex::Regex;
use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

// A method parsed from host_bindings_trait.rs.
struct TraitMethod {
    name: String,
    params: Vec<(String, String)>, // (param_name, param_type)
    return_type: String,
}

const FILE_TRAIT: &str = "host_bindings_trait.rs";
const FILE_WASM: &str = "host_bindings_wasm.rs";
const FILE_EMPTY: &str = "host_bindings_empty.rs";
const FILE_TEST: &str = "host_bindings_test.rs";

fn host_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../xrpl-wasm-stdlib/src/host")
}

fn read_host_file(filename: &str) -> Result<String, Box<dyn Error>> {
    Ok(fs::read_to_string(host_dir().join(filename))?)
}

fn write_host_file(filename: &str, content: &str) -> Result<(), Box<dyn Error>> {
    let path = host_dir().join(filename);
    fs::write(&path, content)?;
    println!("  Updated {}", path.display());
    Ok(())
}

// Parse a param list like "name: Type, other: Type2" into (name, type) pairs.
// Optionally strips a leading '_' from each name.
fn parse_params(params_str: &str, strip_underscore: bool) -> Vec<(String, String)> {
    params_str
        .split(',')
        .map(|p| p.trim())
        .filter(|p| !p.is_empty())
        .map(|p| {
            let (name_raw, type_raw) = p.split_once(':').unwrap_or((p, ""));
            let mut name = name_raw.trim().to_string();
            if strip_underscore && name.starts_with('_') {
                name = name[1..].to_string();
            }
            (name, type_raw.trim().to_string())
        })
        .collect()
}

fn parse_trait_methods(content: &str) -> Result<Vec<TraitMethod>, Box<dyn Error>> {
    let re = Regex::new(
        r"unsafe fn ([A-Za-z0-9_]+)\s*\(\s*&self\s*(?:,\s*([^)]*))?\)\s*->\s*([A-Za-z0-9]+)\s*;",
    )?;
    let mut methods = Vec::new();
    for caps in re.captures_iter(content) {
        let name = caps.get(1).unwrap().as_str().to_string();
        let params_str = caps.get(2).map(|m| m.as_str()).unwrap_or("");
        let return_type = caps.get(3).unwrap().as_str().to_string();
        let params = parse_params(params_str, false);
        methods.push(TraitMethod {
            name,
            params,
            return_type,
        });
    }
    Ok(methods)
}

// Categorize methods into named groups for the macro output.
fn categorize(name: &str) -> Option<&'static str> {
    if name.starts_with("get_ledger")
        || name.starts_with("get_parent")
        || name.starts_with("get_base")
        || name.starts_with("get_tx")
        || name.starts_with("get_current")
        || name.starts_with("amendment")
        || name.starts_with("cache")
    {
        Some("Host Function Category: ledger and transaction info")
    } else if name == "update_data" {
        Some("Host Function Category: update current ledger entry")
    } else if name.contains("keylet") || name == "compute_sha512_half" || name == "check_sig" {
        Some("Host Function Category: hash and keylet computation")
    } else if name.starts_with("get_nft") {
        Some("Host Function Category: NFT")
    } else if name.starts_with("float_") {
        Some("Host Function Category: FLOAT")
    } else if name.starts_with("trace") {
        Some("Host Function Category: TRACE")
    } else {
        None
    }
}

fn generate_export_macro_content(methods: &[TraitMethod], with_underscore_prefix: bool) -> String {
    // Order matters — mirrors the JS's category order
    let category_order = [
        "Host Function Category: ledger and transaction info",
        "Host Function Category: update current ledger entry",
        "Host Function Category: hash and keylet computation",
        "Host Function Category: NFT",
        "Host Function Category: FLOAT",
        "Host Function Category: TRACE",
    ];

    let mut lines: Vec<String> = Vec::new();
    let mut used: HashSet<String> = HashSet::new();

    let format_sig = |m: &TraitMethod| -> String {
        let params: Vec<String> = m
            .params
            .iter()
            .map(|(n, t)| {
                let n = if with_underscore_prefix {
                    format!("_{n}")
                } else {
                    n.clone()
                };
                format!("{n}: {t}")
            })
            .collect();
        format!("    fn {}({}) -> {};", m.name, params.join(", "), m.return_type)
    };

    for cat in &category_order {
        let matched: Vec<&TraitMethod> = methods
            .iter()
            .filter(|m| categorize(&m.name) == Some(*cat) && !used.contains(&m.name))
            .collect();
        if matched.is_empty() {
            continue;
        }
        lines.push(format!("    // {cat}"));
        for m in matched {
            used.insert(m.name.clone());
            lines.push(format_sig(m));
        }
        lines.push(String::new());
    }

    // Remaining methods that didn't fit any category
    let remaining: Vec<&TraitMethod> = methods.iter().filter(|m| !used.contains(&m.name)).collect();
    if !remaining.is_empty() {
        lines.push("    // Other functions".to_string());
        for m in remaining {
            lines.push(format_sig(m));
        }
    }

    lines.join("\n")
}

fn generate_extern_block_content(methods: &[TraitMethod]) -> String {
    let mut lines: Vec<String> = Vec::new();
    for m in methods {
        let params: Vec<String> = m.params.iter().map(|(n, t)| format!("{n}: {t}")).collect();
        if m.params.len() <= 2 {
            lines.push(format!(
                "        pub(super) fn {}({}) -> {};",
                m.name,
                params.join(", "),
                m.return_type
            ));
        } else {
            lines.push(format!("        pub(super) fn {}(", m.name));
            for (n, t) in &m.params {
                lines.push(format!("            {n}: {t},"));
            }
            lines.push(format!("        ) -> {};", m.return_type));
        }
    }
    lines.join("\n")
}

fn generate_impl_block_content(methods: &[TraitMethod]) -> String {
    let mut lines: Vec<String> = Vec::new();
    for m in methods {
        let params: Vec<String> = m.params.iter().map(|(n, t)| format!("{n}: {t}")).collect();
        let param_names: Vec<String> = m.params.iter().map(|(n, _)| n.clone()).collect();
        let self_params = if params.is_empty() {
            "&self".to_string()
        } else {
            format!("&self, {}", params.join(", "))
        };

        if m.params.len() <= 2 {
            lines.push(format!(
                "    unsafe fn {}({}) -> {} {{",
                m.name, self_params, m.return_type
            ));
            if param_names.is_empty() {
                lines.push(format!(
                    "        unsafe {{ host_defined_functions::{}() }}",
                    m.name
                ));
            } else {
                lines.push(format!(
                    "        unsafe {{ host_defined_functions::{}({}) }}",
                    m.name,
                    param_names.join(", ")
                ));
            }
            lines.push("    }".to_string());
        } else {
            lines.push(format!("    unsafe fn {}(", m.name));
            lines.push("        &self,".to_string());
            for (n, t) in &m.params {
                lines.push(format!("        {n}: {t},"));
            }
            lines.push(format!("    ) -> {} {{", m.return_type));
            lines.push("        unsafe {".to_string());
            lines.push(format!("            host_defined_functions::{}(", m.name));
            for (n, _) in &m.params {
                lines.push(format!("                {n},"));
            }
            lines.push("            )".to_string());
            lines.push("        }".to_string());
            lines.push("    }".to_string());
        }
        lines.push(String::new());
    }
    lines.join("\n")
}

// Locate the last `export_host_functions! { ... }` invocation (not the macro definition).
// Returns (match_start, match_end, prefix_capture).
fn find_export_macro_invocation(content: &str) -> Option<(usize, usize, String)> {
    let re = Regex::new(r"(?s)(export_host_functions!\s*\{)(.*?)(\n\})").ok()?;
    let mut target: Option<(usize, usize, String)> = None;
    for caps in re.captures_iter(content) {
        let inner = caps.get(2).unwrap().as_str();
        if !inner.contains("$name:ident") {
            let mat = caps.get(0).unwrap();
            let prefix = caps.get(1).unwrap().as_str().to_string();
            target = Some((mat.start(), mat.end(), prefix));
        }
    }
    target
}

fn update_export_macro(
    filename: &str,
    methods: &[TraitMethod],
    with_underscore_prefix: bool,
) -> Result<(), Box<dyn Error>> {
    let content = read_host_file(filename)?;
    let (start, end, prefix) = find_export_macro_invocation(&content)
        .ok_or_else(|| format!("No export_host_functions! invocation found in {filename}"))?;
    let new_body = generate_export_macro_content(methods, with_underscore_prefix);
    let new_content = format!(
        "{}{}\n{}\n}}{}",
        &content[..start],
        prefix,
        new_body,
        &content[end..]
    );
    write_host_file(filename, &new_content)?;
    Ok(())
}

fn update_wasm_bindings(methods: &[TraitMethod]) -> Result<(), Box<dyn Error>> {
    let mut content = read_host_file(FILE_WASM)?;

    // 1. Replace extern "C" block
    let extern_re = Regex::new(
        r#"(?s)(#\[link\(wasm_import_module = "host_lib"\)\]\s*unsafe extern "C" \{)(.*?)(\n    \})"#,
    )?;
    let caps = extern_re
        .captures(&content)
        .ok_or("Could not find extern block in host_bindings_wasm.rs")?;
    let mat = caps.get(0).unwrap();
    let prefix = caps.get(1).unwrap().as_str().to_string();
    let (start, end) = (mat.start(), mat.end());
    let new_body = generate_extern_block_content(methods);
    content = format!(
        "{}{}\n{}\n    }}{}",
        &content[..start],
        prefix,
        new_body,
        &content[end..]
    );

    // 2. Replace impl HostBindings for WasmHostBindings block
    let impl_re = Regex::new(
        r"(?s)(/// WASM implementation of HostBindings\.\nimpl HostBindings for WasmHostBindings \{)(.*?)(\n\})",
    )?;
    let caps = impl_re
        .captures(&content)
        .ok_or("Could not find impl HostBindings for WasmHostBindings in host_bindings_wasm.rs")?;
    let mat = caps.get(0).unwrap();
    let prefix = caps.get(1).unwrap().as_str().to_string();
    let (start, end) = (mat.start(), mat.end());
    let new_body = generate_impl_block_content(methods);
    content = format!(
        "{}{}\n{}}}{}",
        &content[..start],
        prefix,
        new_body,
        &content[end..]
    );

    // 3. Replace export_host_functions! macro invocation
    let (start, end, prefix) = find_export_macro_invocation(&content)
        .ok_or("No export_host_functions! invocation found in host_bindings_wasm.rs")?;
    let new_body = generate_export_macro_content(methods, false);
    content = format!(
        "{}{}\n{}\n}}{}",
        &content[..start],
        prefix,
        new_body,
        &content[end..]
    );

    write_host_file(FILE_WASM, &content)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Reading host bindings trait file (source of truth)...");
    let trait_content = read_host_file(FILE_TRAIT)?;

    let methods = parse_trait_methods(&trait_content)?;
    println!("  Found {} methods in {}", methods.len(), FILE_TRAIT);

    println!("\nUpdating derived files...");

    // Wasm gets all three sections regenerated
    update_wasm_bindings(&methods)?;

    // Empty gets the macro invocation regenerated with underscore-prefixed params
    update_export_macro(FILE_EMPTY, &methods, true)?;

    // Test gets the macro invocation regenerated without underscore prefix
    update_export_macro(FILE_TEST, &methods, false)?;

    println!(
        "\nSuccessfully updated {} function signatures in 3 files.",
        methods.len()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_no_arg_method() {
        let content = "    unsafe fn get_ledger_sqn(&self) -> i32;";
        let methods = parse_trait_methods(content).unwrap();
        assert_eq!(methods.len(), 1);
        assert_eq!(methods[0].name, "get_ledger_sqn");
        assert!(methods[0].params.is_empty());
        assert_eq!(methods[0].return_type, "i32");
    }

    #[test]
    fn parses_multi_arg_method() {
        let content =
            "    unsafe fn get_tx_field(&self, buf: *const u8, len: i32) -> i32;";
        let methods = parse_trait_methods(content).unwrap();
        assert_eq!(methods.len(), 1);
        assert_eq!(methods[0].name, "get_tx_field");
        assert_eq!(
            methods[0].params,
            vec![
                ("buf".to_string(), "*const u8".to_string()),
                ("len".to_string(), "i32".to_string()),
            ]
        );
        assert_eq!(methods[0].return_type, "i32");
    }

    #[test]
    fn categorizes_by_prefix() {
        assert_eq!(
            categorize("get_ledger_sqn"),
            Some("Host Function Category: ledger and transaction info")
        );
        assert_eq!(
            categorize("float_add"),
            Some("Host Function Category: FLOAT")
        );
        assert_eq!(categorize("trace"), Some("Host Function Category: TRACE"));
        assert_eq!(
            categorize("account_keylet"),
            Some("Host Function Category: hash and keylet computation")
        );
        assert_eq!(categorize("random_thing"), None);
    }

    #[test]
    fn parse_params_strips_underscore() {
        let ps = parse_params("_buf: *const u8, _len: i32", true);
        assert_eq!(ps[0].0, "buf");
        assert_eq!(ps[1].0, "len");
    }

    #[test]
    fn parse_params_keeps_underscore_when_flag_off() {
        let ps = parse_params("_buf: *const u8", false);
        assert_eq!(ps[0].0, "_buf");
    }
}
