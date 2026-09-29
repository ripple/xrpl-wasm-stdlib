//! `generate-host-bindings <path/to/lib.rs> <output-dir> --source-label <text>`
//!
//! Driven by `scripts/generate-host-bindings.sh`, which fetches rippled's file and decides
//! the output directory; this binary only parses, renders, writes and formats.

use std::path::PathBuf;
use std::process::{Command, exit};
use std::{env, fs};

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {e}");
        exit(1);
    }
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().skip(1).collect();
    let (lib_rs, out_dir, label) = match args.as_slice() {
        [lib_rs, out_dir, flag, label] if flag == "--source-label" => {
            (PathBuf::from(lib_rs), PathBuf::from(out_dir), label.clone())
        }
        _ => {
            return Err(
                "usage: generate-host-bindings <lib.rs> <output-dir> --source-label <text>".into(),
            );
        }
    };

    let source =
        fs::read_to_string(&lib_rs).map_err(|e| format!("reading {}: {e}", lib_rs.display()))?;
    let generated = generate_host_bindings::generate(&source, &label)?;

    let trait_path = out_dir.join("host_bindings_trait.rs");
    let list_path = out_dir.join("host_bindings_list.rs");
    fs::write(&trait_path, generated.trait_rs)
        .map_err(|e| format!("writing {}: {e}", trait_path.display()))?;
    fs::write(&list_path, generated.list_rs)
        .map_err(|e| format!("writing {}: {e}", list_path.display()))?;

    // Format the files we just wrote so they match what `cargo fmt` would produce. The wrapper
    // script's `--check` mode writes them to a temp dir and diffs them against the committed
    // files, so this is safe: the repo has no rustfmt.toml, and formatting therefore does not
    // depend on which directory the files are in.
    let status = Command::new("rustfmt")
        .args(["--edition", "2024"])
        .arg(&trait_path)
        .arg(&list_path)
        .status()
        .map_err(|e| format!("running rustfmt: {e}"))?;
    if !status.success() {
        return Err(format!("rustfmt exited with {status}"));
    }

    eprintln!("wrote {} and {}", trait_path.display(), list_path.display());
    Ok(())
}
