use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;
use std::{env, fs};

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

fn parse_stypes(content: &str) -> Result<HashMap<String, i32>, Box<dyn Error>> {
    let re1 = Regex::new(r"(?m)^ *STYPE\(STI_([^ ]*?)[ \n]*,[ \n]*([0-9-]+)[ \n]*\)[ \n]*\\?$")?;
    let mut map: HashMap<String, i32> = HashMap::new();
    for caps in re1.captures_iter(content) {
        let name = caps.get(1).unwrap().as_str().to_string();
        let value = caps.get(2).unwrap().as_str().parse::<i32>()?;
        map.insert(name, value);
    }

    if map.is_empty() {
        let re2 = Regex::new(r"(?m)^ *STI_([^ ]*?)[ \n]*=[ \n]*([0-9-]+)[ \n]*,?$")?;
        for caps in re2.captures_iter(content) {
            let name = caps.get(1).unwrap().as_str().to_string();
            let id: i32 = caps.get(2).unwrap().as_str().parse()?;
            map.insert(name, id);
        }
    }
    Ok(map)
}

// Parse TYPED_SFIELD macros. Returns (field_name, xrpl_type, code) per match.
fn parse_sfields(content: &str) -> Result<Vec<(String, String, i32)>, Box<dyn Error>> {
    let re = Regex::new(
        r"(?m)^ *[A-Z]*TYPED_SFIELD *\( *sf([^,\n]*),[ \n]*([^, \n]+)[ \n]*,[ \n]*([0-9]+)",
    )?;
    let mut fields = Vec::new();
    for caps in re.captures_iter(content) {
        let name = caps.get(1).unwrap().as_str().trim().to_string();
        let xrpl_type = caps.get(2).unwrap().as_str().trim().to_string();
        let code: i32 = caps.get(3).unwrap().as_str().parse()?;
        fields.push((name, xrpl_type, code));
    }
    Ok(fields)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 && args.len() != 3 {
        eprintln!("Usage: {} <path/to/rippled> [output_path]", args[0]);
        std::process::exit(1);
    }
    let source = &args[1];
    let output_path: Option<&String> = args.get(2);

    let reader: fn(&str, &str) -> Result<String, Box<dyn Error>> =
        if source.contains("github.com") {
            read_file_from_github
        } else {
            read_file
        };

    let sfield_h = reader(source, "include/xrpl/protocol/SField.h")?;
    let sfield_macro = reader(source, "include/xrpl/protocol/detail/sfields.macro")?;

    let stype_map = parse_stypes(&sfield_h)?;
    println!("STYPE map: {} entries", stype_map.len());

    // XRPL type → Rust type
    let type_map: HashMap<&str, &str> = HashMap::from([
        ("UINT8", "u8"),
        ("UINT16", "u16"),
        ("UINT32", "u32"),
        ("UINT64", "u64"),
        ("UINT128", "Hash128"),
        ("UINT160", "Hash160"),
        ("UINT192", "Hash192"),
        ("UINT256", "Hash256"),
        ("AMOUNT", "Amount"),
        ("ACCOUNT", "AccountID"),
        ("VL", "StandardBlob"),
        ("CURRENCY", "Currency"),
        ("ISSUE", "Issue"),
        ("ARRAY", "Array"),
        ("OBJECT", "Object"),
    ]);

    // Per-field-name overrides
    let custom_field_types: HashMap<&str, &str> = HashMap::from([
        ("TransactionType", "TransactionType"),
        ("Condition", "ConditionBlob"),
        ("Fulfillment", "FulfillmentBlob"),
        ("FinishFunction", "WasmBlob"),
        ("PublicKey", "PublicKeyBlob"),
        ("Domain", "UriBlob"),
        ("MessageKey", "PublicKeyBlob"),
        ("SigningPubKey", "PublicKeyBlob"),
        ("TxnSignature", "SignatureBlob"),
        ("URI", "UriBlob"),
    ]);

    // Build the output buffer
    let mut output = String::new();
    output.push_str("pub const Invalid: SField<u8, -1> = SField::new();\n");
    output.push_str("pub const Generic: SField<u8, 0> = SField::new();\n");
    output.push_str("pub const hash: SField<u8, -1> = SField::new();\n");
    output.push_str("pub const index: SField<u8, 0> = SField::new();\n");
    output.push('\n');
    output.push_str("// Placeholder SField constants for array and object types\n");
    output.push_str("// These types don't have FieldGetter implementations but are represented as SField<u8, CODE>\n");

    // Parse and sort SFields
    let mut sfields = parse_sfields(&sfield_macro)?;
    println!("Parsed {} SFields", sfields.len());

    // Sort key: stype_id * 65536 + code, ascending. Widen to i64 so large STYPE IDs can't wrap.
    sfields.sort_by_key(|(_name, xrpl_type, code)| {
        let stype_id = stype_map.get(xrpl_type.as_str()).copied().unwrap_or(0) as i64;
        stype_id * 65536 + *code as i64
    });

    // Emit a constant per field
    for (field_name, xrpl_type, code) in &sfields {
        let stype_id = stype_map.get(xrpl_type.as_str()).copied().unwrap_or(0) as i64;
        let field_code = stype_id * 65536 + *code as i64;

        // Custom name override wins over generic type map
        let rust_type = custom_field_types
            .get(field_name.as_str())
            .or_else(|| type_map.get(xrpl_type.as_str()))
            .copied();

        match rust_type {
            Some(rt) => {
                output.push_str(&format!(
                    "pub const {field_name}: SField<{rt}, {field_code}> = SField::new();\n"
                ));

                if custom_field_types.contains_key(field_name.as_str()) {
                    println!("  {field_name}: {rt} (custom mapping from {xrpl_type})");
                }
            }
            None => {
                eprintln!("Warning: No Rust type mapping for XRPL type: {xrpl_type}");
                output.push_str(&format!(
                    "pub const {field_name}: SField<u8, {field_code}> = SField::new();\n"
                ));
            }
        }
    }

    // Decide where to write output
    let default_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../xrpl-wasm-stdlib/src/sfield.rs");
    let output_file: PathBuf = match output_path {
        Some(p) => PathBuf::from(p),
        None => default_path,
    };

    // Preserve everything in the existing file above "pub const Invalid:" (type defs, impl blocks)
    let existing = fs::read_to_string(&output_file).unwrap_or_default();
    let final_output = match existing.find("pub const Invalid:") {
        Some(idx) => {
            let mut combined = existing[..idx].to_string();
            combined.push_str(&output);
            combined
        }
        None => output,
    };

    fs::write(&output_file, &final_output)?;
    println!("File written successfully to {}", output_file.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stypes_macro_form() {
        let content = "\
            STYPE(STI_UINT16, 1)  \\\n\
            STYPE(STI_UINT32, 2)  \\\n\
            STYPE(STI_AMOUNT, 6)  \\\n";
        let map = parse_stypes(content).unwrap();
        assert_eq!(map.len(), 3);
        assert_eq!(map["UINT16"], 1);
        assert_eq!(map["UINT32"], 2);
        assert_eq!(map["AMOUNT"], 6);
    }

    #[test]
    fn stypes_enum_form_fallback() {
        let content = "\
            enum SerializedTypeID {\n\
                STI_UINT16 = 1,\n\
                STI_UINT32 = 2,\n\
                STI_AMOUNT = 6,\n\
            };\n";
        let map = parse_stypes(content).unwrap();
        assert_eq!(map.len(), 3);
        assert_eq!(map["UINT16"], 1);
        assert_eq!(map["AMOUNT"], 6);
    }

    #[test]
    fn sfields_basic() {
        let content = "\
            TYPED_SFIELD(sfAccount, ACCOUNT, 1)\n\
            TYPED_SFIELD(sfAmount, AMOUNT, 1)\n\
            TYPED_SFIELD(sfSequence, UINT32, 4)\n";
        let fields = parse_sfields(content).unwrap();
        assert_eq!(fields.len(), 3);
        assert_eq!(fields[0], ("Account".to_string(), "ACCOUNT".to_string(), 1));
        assert_eq!(fields[2], ("Sequence".to_string(), "UINT32".to_string(), 4));
    }
}
