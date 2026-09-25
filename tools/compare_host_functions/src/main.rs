use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::path::PathBuf;
use std::process::exit;
use std::{env, fs};

type Reader = fn(&str, &str) -> Result<String, Box<dyn Error>>;

struct HostFunction {
    name: String,
    return_type: String,
    parameters: Vec<String>,
}

fn parse_rust_params(raw: &str) -> Vec<String> {
    raw.replace('\n', " ")
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .filter_map(|s| s.split_once(':').map(|(_, t)| t.trim().to_string()))
        .collect()
}

fn parse_rust_bindings(content: &str, regex: &Regex) -> Vec<HostFunction> {
    let mut funcs: Vec<HostFunction> = regex
        .captures_iter(content)
        .map(|caps| {
            let name = caps.get(1).unwrap().as_str().to_string();
            let params_str = caps.get(2).map(|m| m.as_str()).unwrap_or("");
            let return_type = caps.get(3).unwrap().as_str().to_string();
            HostFunction {
                name,
                return_type,
                parameters: parse_rust_params(params_str),
            }
        })
        .collect();
    funcs.sort_by(|a, b| a.name.cmp(&b.name));
    funcs
}

fn translate_types(
    funcs: &mut [HostFunction],
    translation: &HashMap<&str, &str>,
) -> Result<(), Box<dyn Error>> {
    for f in funcs.iter_mut() {
        f.return_type = translation
            .get(f.return_type.as_str())
            .ok_or_else(|| format!("Unknown return type `{}` on `{}`", f.return_type, f.name))?
            .to_string();
        for p in f.parameters.iter_mut() {
            *p = translation
                .get(p.as_str())
                .ok_or_else(|| format!("Unknown parameter type `{p}` on `{}`", f.name))?
                .to_string();
        }
    }
    Ok(())
}

fn extract_macro_body(content: &str) -> Result<String, Box<dyn Error>> {
    let re = Regex::new(r"(?s)export_host_functions!\s*\{(.*?)\n\}")?;
    let caps = re
        .captures(content)
        .ok_or("Could not find export_host_functions! macro invocation")?;
    Ok(caps.get(1).unwrap().as_str().to_string())
}

fn check_hits(file_title: &str, rust: &[HostFunction], cpp: &[HostFunction]) -> bool {
    let rust_names: HashSet<&str> = rust.iter().map(|f| f.name.as_str()).collect();
    let cpp_names: HashSet<&str> = cpp.iter().map(|f| f.name.as_str()).collect();

    if rust_names != cpp_names {
        eprintln!("{file_title}: Rust and C++ host functions do not match!");
        let missing_rust: Vec<&str> = cpp_names.difference(&rust_names).copied().collect();
        let missing_cpp: Vec<&str> = rust_names.difference(&cpp_names).copied().collect();
        if !missing_rust.is_empty() {
            eprintln!("  Missing from Rust: {}", missing_rust.join(", "));
        }
        if !missing_cpp.is_empty() {
            eprintln!("  Missing from C++: {}", missing_cpp.join(", "));
        }
        return true;
    }

    let mut has_error = false;
    for (r, c) in rust.iter().zip(cpp.iter()) {
        if r.return_type != c.return_type {
            eprintln!(
                "{file_title}: return type mismatch for {}: Rust `{}` != C++ `{}`",
                r.name, r.return_type, c.return_type
            );
            has_error = true;
        } else if r.parameters.len() != c.parameters.len() {
            eprintln!(
                "{file_title}: parameter count mismatch for {}: Rust {} != C++ {}",
                r.name,
                r.parameters.len(),
                c.parameters.len()
            );
            has_error = true;
        } else {
            for (i, (rp, cp)) in r.parameters.iter().zip(c.parameters.iter()).enumerate() {
                if rp != cp {
                    eprintln!(
                        "{file_title}: parameter {i} mismatch for {}: Rust `{}` != C++ `{}`",
                        r.name, rp, cp
                    );
                    has_error = true;
                }
            }
        }
    }
    has_error
}
fn read_from_github(repo: &str, filename: &str) -> Result<String, Box<dyn Error>> {
    let mut repo = repo.to_string();
    if !repo.contains("tree") {
        repo.push_str("/tree/HEAD");
    }

    let mut url = repo.replace("github.com", "raw.githubusercontent.com");
    url = url.replace("tree/", "");

    url.push('/');
    url.push_str(filename);

    if !url.starts_with("http") {
        url = format!("https://{url}");
    }

    let body = ureq::get(&url).call()?.into_string()?;
    Ok(body)
}

fn read_from_local(folder: &str, filename: &str) -> Result<String, Box<dyn Error>> {
    let path = PathBuf::from(folder).join(filename);
    Ok(fs::read_to_string(path)?)
}
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <path/to/rippled | GitHub URL>", args[0]);
        exit(1);
    }

    let source = &args[1];
    let reader: Reader = if source.contains("github") {
        read_from_github
    } else {
        read_from_local
    };

    let wasm_import_file = reader(source, "src/libxrpl/tx/wasm/WasmVM.cpp")?;
    let host_wrapper_file = reader(source, "include/xrpl/tx/wasm/HostFuncWrapper.h")?;

    // parse imports from WasmVM.cpp
    let import_re = Regex::new(
        r#"(?m)^ *WASM_IMPORT_FUNC2? *\(\*?i, *([A-Za-z0-9]+), *("([A-Za-z0-9_]+)",)? *&?hfs, *[0-9']+\);$"#,
    )?;

    let mut imports: Vec<(String, String)> = import_re
        .captures_iter(&wasm_import_file)
        .map(|caps| {
            let name = caps.get(1).unwrap().as_str().to_string();
            let alias = caps
                .get(3)
                .map(|m| m.as_str().to_string())
                .unwrap_or_else(|| name.clone());
            (name, alias)
        })
        .collect();
    imports.sort_by(|a, b| a.0.cmp(&b.0));

    println!("WasmVM.cpp: matched {} import functions", imports.len());

    // parse wrappers from HostFuncWrapper.h
    let wrapper_re = Regex::new(
        r#"(?m)^ *using ([A-Za-z0-9]+)_proto =[ \n]*([A-Za-z0-9_]+)\(([A-Za-z0-9_\* \n,]*)\);$"#,
    )?;

    let mut wrappers: Vec<(String, String, Vec<String>)> = wrapper_re
        .captures_iter(&host_wrapper_file)
        .map(|caps| {
            let name = caps.get(1).unwrap().as_str().to_string();
            let return_type = caps.get(2).unwrap().as_str().to_string();
            let params_str = caps.get(3).unwrap().as_str();

            let params: Vec<String> = if params_str.trim().is_empty() {
                Vec::new()
            } else {
                params_str
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect()
            };

            (name, return_type, params)
        })
        .collect();
    wrappers.sort_by(|a, b| a.0.cmp(&b.0));

    println!(
        "HostFuncWrapper.h: matched {} wrapper functions",
        wrappers.len()
    );

    // cross-check import names vs wrapper names
    let import_names: HashSet<&str> = imports.iter().map(|(n, _)| n.as_str()).collect();
    let wrapper_names: HashSet<&str> = wrappers.iter().map(|(n, _, _)| n.as_str()).collect();

    if import_names != wrapper_names {
        eprintln!("Imports and C++ Host Functions do not match!");

        let missing_imports: Vec<&str> = wrapper_names.difference(&import_names).copied().collect();
        let missing_wrappers: Vec<&str> =
            import_names.difference(&wrapper_names).copied().collect();

        if !missing_imports.is_empty() {
            eprintln!("Missing Imports: {}", missing_imports.join(", "));
        }
        if !missing_wrappers.is_empty() {
            eprintln!(
                "Missing C++ Host Functions: {}",
                missing_wrappers.join(", ")
            );
        }
        exit(1);
    }

    let mut cpp_host_functions: Vec<HostFunction> = imports
        .iter()
        .zip(wrappers.iter())
        .map(
            |((_cpp_name, wasm_name), (_, return_type, params))| HostFunction {
                name: wasm_name.clone(),
                return_type: return_type.clone(),
                parameters: params.clone(),
            },
        )
        .collect();

    cpp_host_functions.sort_by(|a, b| a.name.cmp(&b.name));
    println!("Merged {} C++ host functions", cpp_host_functions.len());

    // Rust type → C++ type translation
    let translation: HashMap<&str, &str> = HashMap::from([
        ("i32", "int32_t"),
        ("u32", "uint32_t"),
        ("usize", "int32_t"),
        ("i64", "int64_t"),
        ("*const u8", "uint8_t const*"),
        ("*mut u8", "uint8_t*"),
    ]);

    let host_dir =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../xrpl-common-stdlib/src/host");

    let trait_re = Regex::new(
        r"unsafe fn ([A-Za-z0-9_]+)\(\s*&self(?:,\s*([^)]*))?\s*\)\s*->\s*([A-Za-z0-9]+);",
    )?;
    let wasm_re =
        Regex::new(r"pub\(super\) fn ([A-Za-z0-9_]+)\(\s*([^)]*)\s*\)\s*->\s*([A-Za-z0-9]+);")?;
    let macro_fn_re = Regex::new(r"fn ([A-Za-z0-9_]+)\(\s*([^)]*)\s*\)\s*->\s*([A-Za-z0-9]+);?")?;

    let mut any_error = false;

    // host_bindings_trait.rs
    let content = fs::read_to_string(host_dir.join("host_bindings_trait.rs"))?;
    let mut funcs = parse_rust_bindings(&content, &trait_re);
    println!("host_bindings_trait.rs: matched {} functions", funcs.len());
    translate_types(&mut funcs, &translation)?;
    any_error |= check_hits("host_bindings_trait.rs", &funcs, &cpp_host_functions);

    // host_bindings_wasm.rs
    let content = fs::read_to_string(host_dir.join("host_bindings_wasm.rs"))?;
    let mut funcs = parse_rust_bindings(&content, &wasm_re);
    println!("host_bindings_wasm.rs: matched {} functions", funcs.len());
    translate_types(&mut funcs, &translation)?;
    any_error |= check_hits("host_bindings_wasm.rs", &funcs, &cpp_host_functions);

    // host_bindings_test.rs (functions live inside an export_host_functions! macro)
    let content = fs::read_to_string(host_dir.join("host_bindings_test.rs"))?;
    let macro_body = extract_macro_body(&content)?;
    let mut funcs = parse_rust_bindings(&macro_body, &macro_fn_re);
    println!("host_bindings_test.rs: matched {} functions", funcs.len());
    translate_types(&mut funcs, &translation)?;
    any_error |= check_hits("host_bindings_test.rs", &funcs, &cpp_host_functions);

    // host_bindings_empty.rs (same macro-body extraction as test)
    let content = fs::read_to_string(host_dir.join("host_bindings_empty.rs"))?;
    let macro_body = extract_macro_body(&content)?;
    let mut funcs = parse_rust_bindings(&macro_body, &macro_fn_re);
    println!("host_bindings_empty.rs: matched {} functions", funcs.len());
    translate_types(&mut funcs, &translation)?;
    any_error |= check_hits("host_bindings_empty.rs", &funcs, &cpp_host_functions);

    if any_error {
        exit(1);
    }

    println!("All host function definitions match ✓");
    Ok(())
}
