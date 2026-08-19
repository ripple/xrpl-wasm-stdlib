use serde::Deserialize;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Measurement {
    binary_size: u64,
    avg_gas: f64,
    std_dev: f64,
    gas_readings: Vec<u64>,
}

#[derive(Deserialize, Debug)]
struct Results {
    current: Measurement,
    previous: Option<Measurement>,
    timestamp: String,
    branch: Option<String>,
}

fn get_result_files(benchmark_dir: &Path, args: &[String]) -> Vec<PathBuf> {
    if !args.is_empty() {
        return args
            .iter()
            .map(|contract| benchmark_dir.join(format!("{contract}_results.json")))
            .collect();
    }
    if !benchmark_dir.exists() {
        return Vec::new();
    }
    fs::read_dir(benchmark_dir)
        .unwrap()
        .filter_map(|res| res.ok())
        .filter(|entry| {
            entry
                .file_name()
                .to_string_lossy()
                .ends_with("_results.json")
        })
        .map(|entry| entry.path())
        .collect()
}

fn format_number(num: f64) -> String {
    let formatted = format!("{num:.2}");
    let (int_part, dec_part) = formatted.split_once('.').unwrap();
    let with_commas = add_thousands_separators(int_part);
    format!("{with_commas}.{dec_part}")
}

fn format_int(n: u64) -> String {
    add_thousands_separators(&n.to_string())
}

fn add_thousands_separators(s: &str) -> String {
    let mut result = String::new();
    let len = s.len();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (len - i) % 3 == 0 && c.is_ascii_digit() {
            result.push(',');
        }
        result.push(c);
    }
    result
}

fn append_measurement(report: &mut String, m: &Measurement) {
    report.push_str(&format!(
        "- Binary Size: {} bytes\n",
        format_int(m.binary_size)
    ));
    report.push_str(&format!("- Average Gas: {}\n", format_number(m.avg_gas)));
    report.push_str(&format!("- Std Dev: {}\n", format_number(m.std_dev)));

    let readings: Vec<String> = m.gas_readings.iter().map(|r| r.to_string()).collect();
    report.push_str(&format!("- Gas Readings: {}\n\n", readings.join(", ")));
}

fn generate_contract_details(contract_name: &str, results: &Results) -> String {
    let mut report = format!("### {contract_name}\n\n");

    if let Some(prev) = &results.previous {
        report.push_str("#### Detailed Results\n\n");
        report.push_str("**Previous Measurement:**\n");
        append_measurement(&mut report, prev);
    }

    report.push_str("**Current Measurement:**\n");
    append_measurement(&mut report, &results.current);

    report
}

fn generate_summary_row(contract_name: &str, results: &Results) -> String {
    let current = &results.current;
    format!(
        "| {} | {} | {} | {} |\n",
        contract_name,
        format_int(current.binary_size),
        format_number(current.avg_gas),
        format_number(current.std_dev),
    )
}

fn main() {
    let benchmark_dir: PathBuf =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.benchmark");

    let args: Vec<String> = env::args().skip(1).collect();
    let result_files = get_result_files(&benchmark_dir, &args);

    if result_files.is_empty() {
        eprintln!("No results files found in {}", benchmark_dir.display());
        eprintln!("Run the gas benchmark first to generate results");
        std::process::exit(1);
    }

    let mut unified_report = String::from("# Gas Benchmark Report\n\n");

    let first_contents = fs::read_to_string(&result_files[0]).unwrap();
    let first_results: Results = serde_json::from_str(&first_contents).unwrap();

    unified_report.push_str(&format!("Generated: {}\n\n", first_results.timestamp));
    let branch = first_results.branch.as_deref().unwrap_or("unknown");
    unified_report.push_str(&format!("Branch: {}\n\n", branch));

    unified_report.push_str("## Summary\n\n");
    unified_report.push_str("| Contract | Binary Size | Avg Gas | Std Dev |\n");
    unified_report.push_str("|----------|-------------|---------|----------|\n");

    let mut all_results: Vec<(String, Results)> = Vec::new();
    for results_file in &result_files {
        if !results_file.exists() {
            eprintln!("Results file not found: {}", results_file.display());
            continue;
        }

        let contents = fs::read_to_string(results_file).unwrap();
        let results: Results = serde_json::from_str(&contents).unwrap();

        let contract_name = results_file
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap()
            .trim_end_matches("_results")
            .to_string();

        unified_report.push_str(&generate_summary_row(&contract_name, &results));
        all_results.push((contract_name, results));
    }

    unified_report.push_str("\n## Details\n\n");
    for (name, results) in &all_results {
        unified_report.push_str(&generate_contract_details(name, results));
    }

    unified_report.push_str("## Notes\n\n");
    unified_report.push_str("- Gas measurements are taken from multiple runs per contract\n");
    unified_report.push_str("- Standard deviation indicates variance in gas usage across runs\n");
    unified_report.push_str("- Binary size is deterministic and should be identical across runs\n");
    unified_report.push_str("- Negative gas changes indicate improvements (less gas consumed)\n");

    let report_file = benchmark_dir.join("GAS_BENCHMARK_REPORT.md");
    fs::write(&report_file, &unified_report).unwrap();
    println!("Report generated: {}", report_file.display());
    println!("\n{unified_report}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thousands_separators_basic() {
        assert_eq!(add_thousands_separators("1234567"), "1,234,567");
        assert_eq!(add_thousands_separators("1000"), "1,000");
        assert_eq!(add_thousands_separators("42"), "42");
        assert_eq!(add_thousands_separators(""), "");
    }

    #[test]
    fn thousands_separators_negative() {
        // Leading '-' should not receive a comma right after it
        assert_eq!(add_thousands_separators("-1234"), "-1,234");
        assert_eq!(add_thousands_separators("-1000000"), "-1,000,000");
    }

    #[test]
    fn format_number_two_decimals() {
        assert_eq!(format_number(1234.5), "1,234.50");
        assert_eq!(format_number(0.0), "0.00");
        assert_eq!(format_number(1_000_000.999), "1,000,001.00"); // rounds up
    }

    #[test]
    fn format_int_no_decimals() {
        assert_eq!(format_int(0), "0");
        assert_eq!(format_int(1_234_567), "1,234,567");
        assert_eq!(format_int(999), "999");
    }

    #[test]
    fn append_measurement_shape() {
        let m = Measurement {
            binary_size: 12345,
            avg_gas: 100.5,
            std_dev: 2.5,
            gas_readings: vec![100, 101, 100],
        };
        let mut buf = String::new();
        append_measurement(&mut buf, &m);
        assert!(buf.contains("Binary Size: 12,345 bytes"));
        assert!(buf.contains("Average Gas: 100.50"));
        assert!(buf.contains("Std Dev: 2.50"));
        assert!(buf.contains("Gas Readings: 100, 101, 100"));
    }
}
