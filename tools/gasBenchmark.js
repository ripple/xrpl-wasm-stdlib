#!/usr/bin/env node

const xrpl = require("xrpl")
const fs = require("fs")
const path = require("path")
const { execSync } = require("child_process")

// Get current git branch name
function getCurrentBranch() {
  try {
    return execSync("git rev-parse --abbrev-ref HEAD", {
      encoding: "utf8",
    }).trim()
  } catch {
    return "unknown"
  }
}

const BENCHMARK_DIR = path.join(__dirname, "../.benchmark")
const E2E_TESTS_DIR = path.join(__dirname, "../e2e-tests")
const NETWORK_URL = "ws://127.0.0.1:6006"
const COMPUTATION_ALLOWANCE = 1000000
const NUM_RUNS = 5

// Get contract names from command line arguments
function getContractNames() {
  const args = process.argv.slice(2)

  if (args.length === 0) {
    return ["gas_benchmark"]
  }

  if (args[0] === "all") {
    // Find all Cargo.toml files in e2e-tests subdirectories
    const entries = fs.readdirSync(E2E_TESTS_DIR, { withFileTypes: true })
    const contracts = entries
      .filter((entry) => entry.isDirectory())
      .map((entry) => entry.name)
      .filter((name) => {
        const cargoPath = path.join(E2E_TESTS_DIR, name, "Cargo.toml")
        return fs.existsSync(cargoPath)
      })

    if (contracts.length === 0) {
      throw new Error("No contracts found in e2e-tests")
    }

    return contracts
  }

  return args
}

const CONTRACT_NAMES = getContractNames()

const client = new xrpl.Client(NETWORK_URL)

async function submit(tx, wallet) {
  const result = await client.submitAndWait(tx, { autofill: true, wallet })
  return result
}

async function fundWallet(wallet = undefined) {
  const master = xrpl.Wallet.fromSeed("snoPBrXtMeMyMHUVTgbuqAfg1SUTb", {
    algorithm: xrpl.ECDSA.secp256k1,
  })

  const walletToFund = wallet || xrpl.Wallet.generate()
  await submit(
    {
      TransactionType: "Payment",
      Account: "rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh",
      Amount: xrpl.xrpToDrops(10000),
      Destination: walletToFund.address,
    },
    master,
  )
  return walletToFund
}

function getWasmHex(filePath) {
  if (!fs.existsSync(filePath)) {
    throw new Error(`WASM file not found: ${filePath}`)
  }
  const data = fs.readFileSync(filePath)
  return data.toString("hex")
}

function getBinarySize(filePath) {
  if (!fs.existsSync(filePath)) {
    throw new Error(`WASM file not found: ${filePath}`)
  }
  return fs.statSync(filePath).size
}

async function deployEscrow(sourceWallet, destWallet, wasmHex) {
  // Get current ledger close time for CancelAfter
  const ledgerInfo = await client.request({
    command: "ledger",
    ledger_index: "validated",
  })
  const closeTime = ledgerInfo.result.ledger.close_time

  const tx = {
    TransactionType: "EscrowCreate",
    Account: sourceWallet.address,
    Amount: "100000",
    Destination: destWallet.address,
    CancelAfter: closeTime + 2000,
    FinishFunction: wasmHex,
  }

  const result = await submit(tx, sourceWallet)
  if (result.result?.meta?.TransactionResult !== "tesSUCCESS") {
    throw new Error(
      `Failed to create escrow: ${result.result?.meta?.TransactionResult}`,
    )
  }

  // Return the sequence number of the EscrowCreate transaction
  return result.result.tx_json.Sequence
}

async function executeEscrow(sourceWallet, destWallet, offerSequence) {
  const tx = {
    TransactionType: "EscrowFinish",
    Account: sourceWallet.address,
    Owner: sourceWallet.address,
    OfferSequence: parseInt(offerSequence),
    ComputationAllowance: COMPUTATION_ALLOWANCE,
  }

  const result = await submit(tx, sourceWallet)
  if (result.result?.meta?.TransactionResult !== "tesSUCCESS") {
    throw new Error(
      `Failed to finish escrow: ${result.result?.meta?.TransactionResult}`,
    )
  }

  const gasUsed = result.result?.meta?.GasUsed || 0
  return gasUsed
}

async function measureGas(contractName) {
  console.log(`\n=== Measuring gas for ${contractName} ===`)

  const wasmPath = path.join(
    E2E_TESTS_DIR,
    `target/wasm32v1-none/release/${contractName}.wasm`,
  )

  // Get binary size
  const binarySize = getBinarySize(wasmPath)
  console.log(`Binary size: ${binarySize} bytes`)

  // Get WASM hex
  const wasmHex = getWasmHex(wasmPath)

  // Connect to network
  await client.connect()
  console.log("Connected to network")

  // Setup ledger acceptance interval for local testing
  let interval
  if (client.url.includes("localhost") || client.url.includes("127.0.0.1")) {
    interval = setInterval(() => {
      if (client.isConnected()) {
        client.request({ command: "ledger_accept" })
      }
    }, 1000)
  }

  try {
    // Fund wallets
    const sourceWallet = await fundWallet()
    const destWallet = await fundWallet()
    console.log(`Source wallet: ${sourceWallet.address}`)
    console.log(`Dest wallet: ${destWallet.address}`)

    // Execute escrow multiple times and measure gas
    const gasReadings = []
    for (let i = 0; i < NUM_RUNS; i++) {
      // Deploy escrow with contract
      console.log(`Run ${i + 1}/${NUM_RUNS}...`)
      console.log("  Deploying escrow with contract...")
      let { sequence } = await deployEscrow(sourceWallet, destWallet, wasmHex)
      console.log(`  Escrow created with sequence: ${offerSequence}`)

      // Execute escrow and measure gas
      const gas = await executeEscrow(sourceWallet, destWallet, sequence)
      gasReadings.push(gas)
      console.log(`  Gas used: ${gas}`)
    }

    // Calculate statistics
    const avgGas = gasReadings.reduce((a, b) => a + b, 0) / gasReadings.length
    const stdDev = Math.sqrt(
      gasReadings.reduce((sum, val) => sum + Math.pow(val - avgGas, 2), 0) /
        gasReadings.length,
    )

    return {
      binarySize,
      gasReadings,
      avgGas,
      stdDev,
      minGas: Math.min(...gasReadings),
      maxGas: Math.max(...gasReadings),
    }
  } finally {
    if (interval) clearInterval(interval)
    await client.disconnect()
  }
}

async function main() {
  console.log(`Gas Benchmark Tool`)
  console.log("=".repeat(40))
  console.log(`Benchmarking: ${CONTRACT_NAMES.join(", ")}`)
  console.log("")

  try {
    const branch = getCurrentBranch()
    const timestamp = new Date().toISOString()

    // Ensure benchmark directory exists
    if (!fs.existsSync(BENCHMARK_DIR)) {
      fs.mkdirSync(BENCHMARK_DIR, { recursive: true })
    }

    // Measure gas for each contract
    for (const contractName of CONTRACT_NAMES) {
      const results = await measureGas(contractName)
      const resultsFile = path.join(
        BENCHMARK_DIR,
        `${contractName}_results.json`,
      )

      // Load existing results if they exist
      let allResults = {
        timestamp,
        branch,
      }

      if (fs.existsSync(resultsFile)) {
        const existing = JSON.parse(fs.readFileSync(resultsFile, "utf8"))
        allResults = existing
        allResults.timestamp = timestamp
        allResults.branch = branch
      }

      // Save results - move current to previous, then update current
      if (allResults.current) {
        allResults.previous = allResults.current
      }
      allResults.current = results

      fs.writeFileSync(resultsFile, JSON.stringify(allResults, null, 2))
      console.log(`\nResults saved to ${resultsFile}`)

      // Print summary
      console.log("\n=== Summary ===")
      console.log(`Binary size: ${results.binarySize} bytes`)
      console.log(`Average gas: ${results.avgGas.toFixed(2)}`)
      console.log(`Std dev: ${results.stdDev.toFixed(2)}`)
      console.log(`Min gas: ${results.minGas}`)
      console.log(`Max gas: ${results.maxGas}`)
    }
  } catch (error) {
    console.error("Error:", error.message)
    process.exit(1)
  }
}

main().catch(console.error)
