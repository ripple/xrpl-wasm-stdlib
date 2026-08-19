use std::{env, fs};
use std::path::PathBuf;
use std::process::Command;
use regex::Regex;

fn main(){
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!("Usage: {} <project_name> <fixture_name> <rippled_path>", args[0]);
        std::process::exit(1);
    }

    let project_name = &args[1];
    let fixture_name = &args[2];
    let rippled_path = PathBuf::from(&args[3]);
    let project_path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../rippled-tests/").join(project_name);

    let status = Command::new("cargo")
        .arg("build")
        .arg("--target")
        .arg("wasm32v1-none")
        .arg("--release")
        .current_dir(&project_path)
        .status()
        .expect("failed to run cargo build");

    if !status.success(){
        eprintln!("cargo build failed");
        std::process::exit(1);
    }

    let wasm_path = project_path
        .join("target/wasm32v1-none/release")
        .join(format!("{project_name}.wasm"));

    let status = Command::new("wasm-opt")
        .arg(&wasm_path)
        .arg("-Oz")
        .arg("-o")
        .arg(&wasm_path)
        .status()
        .expect("failed to run wasm-opt");

    if !status.success(){
        eprintln!("wasm-opt failed");
        std::process::exit(1);
    }

    let wasm_bytes = fs::read(&wasm_path).expect("failed to read WASM file");
    let wasm_hex = hex::encode(&wasm_bytes);

    let dst_path = rippled_path.join("src/test/app/wasm_fixtures/fixtures.cpp");
    let dst_content = fs::read_to_string(&dst_path).expect("failed to read fixtures.cpp");

    let pattern = format!(
        r#"extern std::string const {fixture_name} =[ \n]+"[^;]*;"#
    );

    let re = Regex::new(&pattern).expect("invalid regex");

    let replacement = format!(
        r#"extern std::string const {fixture_name} = "{wasm_hex}";"#
    );
    let updated = re.replace_all(&dst_content, replacement.as_str());

    fs::write(&dst_path, updated.as_bytes()).expect("failed to write fixtures.cpp");
    println!("Updated fixture: {fixture_name}");




}