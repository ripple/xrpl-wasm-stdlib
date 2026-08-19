use futures_util::{SinkExt, StreamExt};
use serde_json::{json, Value};
use std::env;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::sleep;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};

type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

const NETWORK_URL: &str = "ws://127.0.0.1:6006";
const MASTER_SEED: &str = "snoPBrXtMeMyMHUVTgbuqAfg1SUTb";
const MASTER_ADDR: &str = "rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh";
const NUM_RUNS: usize = 5;
const COMPUTATION_ALLOWANCE: u64 = 1_000_000;
const XRP_FUND_DROPS: &str = "10000000000"; // 10,000 XRP

// ---- shell + git helpers ----

fn current_branch() -> String {
    Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string())
}

fn now_iso8601() -> String {
    // Minimal ISO-8601 timestamp using SystemTime; avoids pulling in chrono.
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    let secs = now.as_secs();
    format!("{secs}Z")
}

fn benchmark_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.benchmark")
}

fn e2e_tests_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../e2e-tests")
}

fn contract_names_from_args() -> Result<Vec<String>, Box<dyn Error>> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        return Ok(vec!["gas_benchmark".to_string()]);
    }

    if args[0] == "all" {
        let mut contracts = Vec::new();
        for entry in fs::read_dir(e2e_tests_dir())? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                let name = entry.file_name().to_string_lossy().to_string();
                let cargo_path = entry.path().join("Cargo.toml");
                if cargo_path.exists() {
                    contracts.push(name);
                }
            }
        }
        if contracts.is_empty() {
            return Err("No contracts found in e2e-tests".into());
        }
        return Ok(contracts);
    }

    Ok(args)
}

// ---- WebSocket JSON-RPC ----

// Send a rippled command and return the parsed response.
// rippled's WebSocket API puts `command` at the top level with params merged in.
async fn rpc(ws: &mut WsStream, command: &str, mut params: Value) -> Result<Value, Box<dyn Error>> {
    if !params.is_object() {
        params = json!({});
    }
    let obj = params.as_object_mut().unwrap();
    obj.insert("id".into(), json!(1));
    obj.insert("command".into(), json!(command));

    let text = serde_json::to_string(&params)?;
    ws.send(Message::Text(text)).await?;

    while let Some(msg) = ws.next().await {
        let msg = msg?;
        if let Message::Text(t) = msg {
            let v: Value = serde_json::from_str(&t)?;
            return Ok(v);
        }
        // Ignore pings/pongs/etc.
    }
    Err("WebSocket closed before response".into())
}

async fn ledger_accept(ws: &mut WsStream) -> Result<(), Box<dyn Error>> {
    rpc(ws, "ledger_accept", json!({})).await?;
    Ok(())
}

// Ask rippled to generate a fresh wallet. Returns (address, seed).
async fn wallet_propose(ws: &mut WsStream) -> Result<(String, String), Box<dyn Error>> {
    let resp = rpc(ws, "wallet_propose", json!({})).await?;
    let result = &resp["result"];
    let address = result["account_id"]
        .as_str()
        .ok_or("missing account_id in wallet_propose")?
        .to_string();
    let seed = result["master_seed"]
        .as_str()
        .ok_or("missing master_seed in wallet_propose")?
        .to_string();
    Ok((address, seed))
}

// Submit (rippled signs with the given secret). Returns full response.
async fn submit_signed(
    ws: &mut WsStream,
    tx_json: Value,
    seed: &str,
) -> Result<Value, Box<dyn Error>> {
    let params = json!({
        "tx_json": tx_json,
        "secret": seed,
    });
    rpc(ws, "submit", params).await
}

// Poll for validation, advancing the local ledger between polls.
async fn wait_for_validated(ws: &mut WsStream, tx_hash: &str) -> Result<Value, Box<dyn Error>> {
    for _ in 0..40 {
        ledger_accept(ws).await?;
        sleep(Duration::from_millis(100)).await;
        let resp = rpc(ws, "tx", json!({ "transaction": tx_hash })).await?;
        if resp["result"]["validated"].as_bool().unwrap_or(false) {
            return Ok(resp);
        }
    }
    Err(format!("Timeout waiting for validation of {tx_hash}").into())
}

// Sign+submit, then poll until validated.
async fn submit_and_wait(
    ws: &mut WsStream,
    tx_json: Value,
    seed: &str,
) -> Result<Value, Box<dyn Error>> {
    let submit_resp = submit_signed(ws, tx_json, seed).await?;
    let hash = submit_resp["result"]["tx_json"]["hash"]
        .as_str()
        .or_else(|| submit_resp["result"]["hash"].as_str())
        .ok_or("no hash in submit response")?
        .to_string();
    wait_for_validated(ws, &hash).await
}

async fn fund_wallet(ws: &mut WsStream, address: &str) -> Result<(), Box<dyn Error>> {
    let tx = json!({
        "TransactionType": "Payment",
        "Account": MASTER_ADDR,
        "Destination": address,
        "Amount": XRP_FUND_DROPS,
    });
    submit_and_wait(ws, tx, MASTER_SEED).await?;
    Ok(())
}

// ---- benchmark core ----

async fn deploy_escrow(
    ws: &mut WsStream,
    source_addr: &str,
    source_seed: &str,
    dest_addr: &str,
    wasm_hex: &str,
) -> Result<u64, Box<dyn Error>> {
    let ledger_info = rpc(ws, "ledger", json!({ "ledger_index": "validated" })).await?;
    let close_time = ledger_info["result"]["ledger"]["close_time"]
        .as_u64()
        .ok_or("no close_time")?;

    let tx = json!({
        "TransactionType": "EscrowCreate",
        "Account": source_addr,
        "Amount": "100000",
        "Destination": dest_addr,
        "CancelAfter": close_time + 2000,
        "FinishFunction": wasm_hex,
    });

    let result = submit_and_wait(ws, tx, source_seed).await?;
    let tx_result = result["result"]["meta"]["TransactionResult"]
        .as_str()
        .unwrap_or("");
    if tx_result != "tesSUCCESS" {
        return Err(format!("EscrowCreate failed: {tx_result}").into());
    }

    let sequence = result["result"]["tx_json"]["Sequence"]
        .as_u64()
        .or_else(|| result["result"]["Sequence"].as_u64())
        .ok_or("no sequence in EscrowCreate response")?;
    Ok(sequence)
}

async fn execute_escrow(
    ws: &mut WsStream,
    source_addr: &str,
    source_seed: &str,
    offer_sequence: u64,
) -> Result<u64, Box<dyn Error>> {
    let tx = json!({
        "TransactionType": "EscrowFinish",
        "Account": source_addr,
        "Owner": source_addr,
        "OfferSequence": offer_sequence,
        "ComputationAllowance": COMPUTATION_ALLOWANCE,
    });

    let result = submit_and_wait(ws, tx, source_seed).await?;
    let tx_result = result["result"]["meta"]["TransactionResult"]
        .as_str()
        .unwrap_or("");
    if tx_result != "tesSUCCESS" {
        return Err(format!("EscrowFinish failed: {tx_result}").into());
    }

    let gas_used = result["result"]["meta"]["GasUsed"]
        .as_u64()
        .unwrap_or(0);
    Ok(gas_used)
}

#[derive(Debug)]
struct BenchmarkResult {
    binary_size: u64,
    gas_readings: Vec<u64>,
    avg_gas: f64,
    std_dev: f64,
    min_gas: u64,
    max_gas: u64,
}

async fn measure_gas(
    ws: &mut WsStream,
    contract_name: &str,
) -> Result<BenchmarkResult, Box<dyn Error>> {
    println!("\n=== Measuring gas for {contract_name} ===");

    let wasm_path = e2e_tests_dir()
        .join("target/wasm32v1-none/release")
        .join(format!("{contract_name}.wasm"));

    let metadata = fs::metadata(&wasm_path)
        .map_err(|e| format!("WASM file not found: {} ({e})", wasm_path.display()))?;
    let binary_size = metadata.len();
    println!("Binary size: {binary_size} bytes");

    let wasm_bytes = fs::read(&wasm_path)?;
    let wasm_hex = hex::encode(&wasm_bytes);

    let (source_addr, source_seed) = wallet_propose(ws).await?;
    let (dest_addr, _dest_seed) = wallet_propose(ws).await?;
    fund_wallet(ws, &source_addr).await?;
    fund_wallet(ws, &dest_addr).await?;
    println!("Source wallet: {source_addr}");
    println!("Dest wallet:   {dest_addr}");

    let mut gas_readings: Vec<u64> = Vec::new();
    for i in 0..NUM_RUNS {
        println!("Run {}/{}...", i + 1, NUM_RUNS);
        println!("  Deploying escrow with contract...");
        let sequence = deploy_escrow(ws, &source_addr, &source_seed, &dest_addr, &wasm_hex).await?;
        println!("  Escrow created with sequence: {sequence}");

        let gas = execute_escrow(ws, &source_addr, &source_seed, sequence).await?;
        gas_readings.push(gas);
        println!("  Gas used: {gas}");
    }

    let sum: u64 = gas_readings.iter().sum();
    let count = gas_readings.len() as f64;
    let avg_gas = sum as f64 / count;
    let variance = gas_readings
        .iter()
        .map(|&g| (g as f64 - avg_gas).powi(2))
        .sum::<f64>()
        / count;
    let std_dev = variance.sqrt();
    let min_gas = *gas_readings.iter().min().unwrap_or(&0);
    let max_gas = *gas_readings.iter().max().unwrap_or(&0);

    Ok(BenchmarkResult {
        binary_size,
        gas_readings,
        avg_gas,
        std_dev,
        min_gas,
        max_gas,
    })
}

// ---- main ----

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let contract_names = contract_names_from_args()?;

    println!("Gas Benchmark Tool");
    println!("{}", "=".repeat(40));
    println!("Benchmarking: {}", contract_names.join(", "));

    let branch = current_branch();
    let timestamp = now_iso8601();

    fs::create_dir_all(benchmark_dir())?;

    let (mut ws, _) = connect_async(NETWORK_URL).await?;
    println!("Connected to {NETWORK_URL}");

    for contract_name in &contract_names {
        let results = measure_gas(&mut ws, contract_name).await?;
        let results_file = benchmark_dir().join(format!("{contract_name}_results.json"));

        // Rotate: existing "current" becomes "previous"
        let mut all_results: Value = if results_file.exists() {
            serde_json::from_str(&fs::read_to_string(&results_file)?)?
        } else {
            json!({})
        };
        let obj = all_results.as_object_mut().unwrap();
        obj.insert("timestamp".into(), json!(timestamp));
        obj.insert("branch".into(), json!(branch));

        if let Some(current) = obj.get("current").cloned() {
            obj.insert("previous".into(), current);
        }
        obj.insert(
            "current".into(),
            json!({
                "binarySize": results.binary_size,
                "gasReadings": results.gas_readings,
                "avgGas": results.avg_gas,
                "stdDev": results.std_dev,
                "minGas": results.min_gas,
                "maxGas": results.max_gas,
            }),
        );

        fs::write(&results_file, serde_json::to_string_pretty(&all_results)?)?;
        println!("\nResults saved to {}", results_file.display());

        println!("\n=== Summary ===");
        println!("Binary size: {} bytes", results.binary_size);
        println!("Average gas: {:.2}", results.avg_gas);
        println!("Std dev:     {:.2}", results.std_dev);
        println!("Min gas:     {}", results.min_gas);
        println!("Max gas:     {}", results.max_gas);
    }

    Ok(())
}
