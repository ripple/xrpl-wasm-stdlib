use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;
use std::{env, fs};

// A parsed host function, in the shape used for comparison.
struct HostFunction {
    name: String,
    return_type: String,
    params: Vec<String>,
}

fn read_file_from_github(repo: &str, filename: &str) -> Result<String, Box<dyn Error>> {
    let mut repo = repo.to_string();
    if !repo.contains("tree") {
        repo.push_str("/tree/HEAD");
    }
    let mut url = repo.replace("github.com", "raw.githubusercontent.com");
    url = url.replace("tree/", "");
    url.push_str(format!("/{}", filename).as_str());

    if !url.starts_with("http") {
        url = format!("https://{}", url);
    }

    let body = ureq::get(&url).call()?.into_string()?;
    Ok(body)
}

fn read_file(folder: &str, filename: &str) -> Result<String, Box<dyn Error>> {
    let path = PathBuf::from(folder).join(filename);
    Ok(fs::read_to_string(path)?)
}

fn are_lists_equal(a: &[String], b: &[String]) -> bool {
    a == b
}

// Translate a Rust type to its C++ equivalent.
// Panics if the Rust type isn't in the translation table (matches JS behavior).
fn translate(rust_type: &str, table: &HashMap<&str, &str>) -> String {
    match table.get(rust_type) {
        Some(cpp) => cpp.to_string(),
        None => {
            eprintln!("Unknown parameter type: {rust_type}");
            std::process::exit(1);
        }
    }
}

// Parse the params portion of a Rust function signature: "name1: Type1, name2: Type2".
// Returns just the type parts. Handles newlines, empty strings, and missing params.
fn parse_params(s: Option<&str>) -> Vec<String> {
    let Some(raw) = s else {
        return Vec::new();
    };
    let cleaned = raw.replace('\n', " ");
    cleaned
        .trim()
        .split(',')
        .map(|p| p.trim())
        .filter(|p| !p.is_empty())
        // Each param is "name: Type" — take the part after ":".
        .map(|p| p.split(':').nth(1).unwrap_or("").trim().to_string())
        .collect()
}

// Parse a Rust host-bindings file with the given regex. Group 1 = name, group 2 = params, group 3 = return type.
fn parse_rust_host_functions(
    content: &str,
    pattern: &str,
    table: &HashMap<&str, &str>,
) -> Result<Vec<HostFunction>, Box<dyn Error>> {
    let re = Regex::new(pattern)?;
    let mut funcs: Vec<HostFunction> = re
        .captures_iter(content)
        .map(|caps| {
            let name = caps.get(1).unwrap().as_str().to_string();
            let params_str = caps.get(2).map(|m| m.as_str());
            let return_raw = caps.get(3).unwrap().as_str();

            let params_rust = parse_params(params_str);
            let params = params_rust.iter().map(|p| translate(p, table)).collect();
            let return_type = translate(return_raw, table);

            HostFunction {
                name,
                return_type,
                params,
            }
        })
        .collect();
    funcs.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(funcs)
}

// Compare a list of Rust host functions against the C++ ones. Reports mismatches. Returns true iff all match.
fn check_hits(file_title: &str, rust_funcs: &[HostFunction], cpp_funcs: &[HostFunction]) -> bool {
    println!("\nComparing {file_title} with C++ host functions...");
    println!("   Found {} Rust functions", rust_funcs.len());
    println!("   Found {} C++ functions", cpp_funcs.len());

    let rust_names: Vec<String> = rust_funcs.iter().map(|f| f.name.clone()).collect();
    let cpp_names: Vec<String> = cpp_funcs.iter().map(|f| f.name.clone()).collect();

    if !are_lists_equal(&rust_names, &cpp_names) {
        eprintln!("\n{file_title}: Rust Host Functions and C++ Host Functions do not match!");

        let rust_missing: Vec<&str> = cpp_names
            .iter()
            .filter(|n| !rust_names.contains(n))
            .map(|s| s.as_str())
            .collect();
        let cpp_missing: Vec<&str> = rust_names
            .iter()
            .filter(|n| !cpp_names.contains(n))
            .map(|s| s.as_str())
            .collect();

        if !rust_missing.is_empty() {
            eprintln!(
                "   Missing Rust Host Functions in {file_title}: {}",
                rust_missing.join(", ")
            );
        }
        if !cpp_missing.is_empty() {
            eprintln!(
                "   Missing C++ Host Functions (extra in {file_title}): {}",
                cpp_missing.join(", ")
            );
        }
        return false;
    }

    let mut has_error = false;
    for (i, (r, c)) in rust_funcs.iter().zip(cpp_funcs.iter()).enumerate() {
        if r.name != c.name {
            eprintln!("Rust Host Function name mismatch in {file_title} at {i}: {} !== {}", r.name, c.name);
            has_error = true;
        } else if r.return_type != c.return_type {
            eprintln!(
                "Rust Host Function return type mismatch in {file_title} for {}: {} !== {}",
                r.name, r.return_type, c.return_type
            );
            has_error = true;
        } else if r.params.len() != c.params.len() {
            eprintln!(
                "Rust Host Function parameter count mismatch in {file_title} for {}: {} !== {} ({}) !== ({})",
                r.name,
                r.params.len(),
                c.params.len(),
                r.params.join(", "),
                c.params.join(", ")
            );
            has_error = true;
        } else {
            for (pi, (rp, cp)) in r.params.iter().zip(c.params.iter()).enumerate() {
                if rp != cp {
                    eprintln!(
                        "Rust Host Function parameter type mismatch in {file_title} for {}, parameter {pi}: {rp} !== {cp}",
                        r.name
                    );
                    has_error = true;
                }
            }
        }
    }

    !has_error
}

// Extract the body of an `export_host_functions! { ... }` macro invocation from a Rust source string.
// Returns None if not found.
fn extract_macro_body(content: &str) -> Option<String> {
    let re = Regex::new(r"(?s)export_host_functions!\s*\{(.*?)\n\}").ok()?;
    re.captures(content)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().to_string())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <path/to/rippled>", args[0]);
        std::process::exit(1);
    }
    let source = &args[1];
    println!("Reading from source: {source}");

    let reader: fn(&str, &str) -> Result<String, Box<dyn Error>> =
        if source.contains("github.com") {
            read_file_from_github
        } else {
            read_file
        };

    // ---- read the two rippled files ----
    let wasm_vm = reader(source, "src/libxrpl/tx/wasm/WasmVM.cpp")?;
    let host_wrapper = reader(source, "include/xrpl/tx/wasm/HostFuncWrapper.h")?;
    println!("WasmVM.cpp: {} bytes", wasm_vm.len());
    println!("HostFuncWrapper.h: {} bytes", host_wrapper.len());

    // ---- parse the imports from WasmVM.cpp ----
    let import_re = Regex::new(
        r#"(?m)^ *WASM_IMPORT_FUNC2? *\(\*?i, *([A-Za-z0-9]+), *("([A-Za-z0-9_]+)",)? *&?hfs, *[0-9']+\);$"#,
    )?;
    // Each import = (cpp_name, wasm_name). wasm_name defaults to cpp_name if no alias is present.
    let mut imports: Vec<(String, String)> = import_re
        .captures_iter(&wasm_vm)
        .map(|caps| {
            let cpp_name = caps.get(1).unwrap().as_str().to_string();
            let wasm_name = caps
                .get(3)
                .map(|m| m.as_str().to_string())
                .unwrap_or_else(|| cpp_name.clone());
            (cpp_name, wasm_name)
        })
        .collect();
    imports.sort_by(|a, b| a.0.cmp(&b.0));
    println!("WasmVM.cpp: matched {} import functions", imports.len());

    // ---- parse the wrappers from HostFuncWrapper.h ----
    let wrapper_re = Regex::new(
        r#"(?m)^ *using ([A-Za-z0-9]+)_proto =[ \n]*([A-Za-z0-9_]+)\(([A-Za-z0-9_\* \n,]*)\);$"#,
    )?;
    let mut wrappers: Vec<(String, String, Vec<String>)> = wrapper_re
        .captures_iter(&host_wrapper)
        .map(|caps| {
            let name = caps.get(1).unwrap().as_str().to_string();
            let return_type = caps.get(2).unwrap().as_str().to_string();
            let params_str = caps.get(3).unwrap().as_str();
            let params: Vec<String> = if params_str.trim().is_empty() {
                Vec::new()
            } else {
                params_str.split(',').map(|s| s.trim().to_string()).collect()
            };
            (name, return_type, params)
        })
        .collect();
    wrappers.sort_by(|a, b| a.0.cmp(&b.0));
    println!("HostFuncWrapper.h: matched {} wrapper functions", wrappers.len());

    // ---- cross-check imports vs wrappers ----
    let import_names: Vec<String> = imports.iter().map(|(n, _)| n.clone()).collect();
    let wrapper_names: Vec<String> = wrappers.iter().map(|(n, _, _)| n.clone()).collect();

    if !are_lists_equal(&import_names, &wrapper_names) {
        eprintln!("Imports and C++ Host Functions do not match!");

        let imports_missing: Vec<&str> = wrapper_names
            .iter()
            .filter(|n| !import_names.contains(n))
            .map(|s| s.as_str())
            .collect();
        let hf_missing: Vec<&str> = import_names
            .iter()
            .filter(|n| !wrapper_names.contains(n))
            .map(|s| s.as_str())
            .collect();

        if !imports_missing.is_empty() {
            eprintln!("Missing Imports: {}", imports_missing.join(", "));
        }
        if !hf_missing.is_empty() {
            eprintln!("Missing C++ Host Functions: {}", hf_missing.join(", "));
        }
        std::process::exit(1);
    }
    for i in 0..imports.len() {
        if imports[i].0 != wrappers[i].0 {
            eprintln!(
                "Imports and Host Functions do not match at index {i}: {} !== {}",
                imports[i].0, wrappers[i].0
            );
            std::process::exit(1);
        }
    }

    // ---- build the combined C++ host function list ----
    // Name comes from imports (aliased, WASM-facing); return + params from wrappers.
    let mut cpp_host_functions: Vec<HostFunction> = imports
        .iter()
        .enumerate()
        .map(|(i, (_cpp_name, wasm_name))| HostFunction {
            name: wasm_name.clone(),
            return_type: wrappers[i].1.clone(),
            params: wrappers[i].2.clone(),
        })
        .collect();
    cpp_host_functions.sort_by(|a, b| a.name.cmp(&b.name));

    // ---- type translation table (Rust -> C++) ----
    let table: HashMap<&str, &str> = HashMap::from([
        ("i32", "int32_t"),
        ("u32", "uint32_t"),
        ("usize", "int32_t"),
        ("i64", "int64_t"),
        ("*const u8", "uint8_t const*"),
        ("*mut u8", "uint8_t*"),
    ]);

    // ---- locate the local xrpl-wasm-stdlib source ----
    let stdlib_host_dir =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../xrpl-wasm-stdlib/src/host");

    // ---- check host_bindings_trait.rs ----
    {
        let path = stdlib_host_dir.join("host_bindings_trait.rs");
        let content = fs::read_to_string(&path)?;
        let pattern =
            r"unsafe fn ([A-Za-z0-9_]+)\(\s*&self(?:,\s*([^)]*))?\s*\)\s*->\s*([A-Za-z0-9]+);";
        let rust_funcs = parse_rust_host_functions(&content, pattern, &table)?;
        println!(
            "\nhost_bindings_trait.rs: Regex matched {} functions",
            rust_funcs.len()
        );
        if !check_hits("host_bindings_trait.rs", &rust_funcs, &cpp_host_functions) {
            std::process::exit(1);
        }
    }

    // ---- check host_bindings_wasm.rs ----
    {
        let path = stdlib_host_dir.join("host_bindings_wasm.rs");
        let content = fs::read_to_string(&path)?;
        let pattern =
            r"pub\(super\) fn ([A-Za-z0-9_]+)\(\s*([^)]*)\s*\)\s*->\s*([A-Za-z0-9]+);";
        let rust_funcs = parse_rust_host_functions(&content, pattern, &table)?;
        println!(
            "\nhost_bindings_wasm.rs: Regex matched {} functions",
            rust_funcs.len()
        );
        if !check_hits("host_bindings_wasm.rs", &rust_funcs, &cpp_host_functions) {
            std::process::exit(1);
        }
    }

    // ---- check host_bindings_test.rs (functions inside export_host_functions! { ... }) ----
    {
        let path = stdlib_host_dir.join("host_bindings_test.rs");
        let content = fs::read_to_string(&path)?;
        let macro_body = extract_macro_body(&content).ok_or_else(|| {
            "Could not find export_host_functions! macro invocation in host_bindings_test.rs"
                .to_string()
        })?;
        let pattern = r"fn ([A-Za-z0-9_]+)\(\s*([^)]*)\s*\)\s*->\s*([A-Za-z0-9]+);?";
        let rust_funcs = parse_rust_host_functions(&macro_body, pattern, &table)?;
        println!(
            "\nhost_bindings_test.rs: Regex matched {} functions",
            rust_funcs.len()
        );
        if !check_hits("host_bindings_test.rs", &rust_funcs, &cpp_host_functions) {
            std::process::exit(1);
        }
    }

    // ---- check host_bindings_empty.rs (functions inside export_host_functions! { ... }) ----
    {
        let path = stdlib_host_dir.join("host_bindings_empty.rs");
        let content = fs::read_to_string(&path)?;
        let macro_body = extract_macro_body(&content).ok_or_else(|| {
            "Could not find export_host_functions! macro invocation in host_bindings_empty.rs"
                .to_string()
        })?;
        let pattern = r"fn ([A-Za-z0-9_]+)\(\s*([^)]*)\s*\)\s*->\s*([A-Za-z0-9]+);?";
        let rust_funcs = parse_rust_host_functions(&macro_body, pattern, &table)?;
        println!(
            "\nhost_bindings_empty.rs: Regex matched {} functions",
            rust_funcs.len()
        );
        if !check_hits("host_bindings_empty.rs", &rust_funcs, &cpp_host_functions) {
            std::process::exit(1);
        }
    }

    println!("\nAll host functions match between Rust and C++ implementations.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lists_equal() {
        let a = vec!["a".to_string(), "b".to_string()];
        let b = vec!["a".to_string(), "b".to_string()];
        let c = vec!["a".to_string(), "c".to_string()];
        assert!(are_lists_equal(&a, &b));
        assert!(!are_lists_equal(&a, &c));
        assert!(!are_lists_equal(&a[..], &a[..1]));
    }

    #[test]
    fn translate_known_types() {
        let table = HashMap::from([("i32", "int32_t"), ("*const u8", "uint8_t const*")]);
        assert_eq!(translate("i32", &table), "int32_t");
        assert_eq!(translate("*const u8", &table), "uint8_t const*");
    }

    #[test]
    fn parse_params_none() {
        assert_eq!(parse_params(None), Vec::<String>::new());
    }

    #[test]
    fn parse_params_two_args() {
        let s = "buf: *const u8, len: i32";
        assert_eq!(
            parse_params(Some(s)),
            vec!["*const u8".to_string(), "i32".to_string()]
        );
    }

    #[test]
    fn parse_params_multiline() {
        let s = "buf: *const u8,\n    len: i32";
        assert_eq!(
            parse_params(Some(s)),
            vec!["*const u8".to_string(), "i32".to_string()]
        );
    }
}
