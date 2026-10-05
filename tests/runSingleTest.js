const xrpl = require("xrpl")
const fs = require("fs")
const path = require("path")

const client =
  process.argv.length > 4
    ? new xrpl.Client(process.argv[4])
    : new xrpl.Client("ws://127.0.0.1:6006")

async function submit(tx, wallet, debug = false) {
  const result = await client.submitAndWait(tx, { autofill: true, wallet })
  console.log(
    "SUBMITTED " + tx.TransactionType + "(" + result.result.hash + ")",
  )
  if (debug) console.log(result.result ?? result)
  else console.log("Result code: " + result.result?.meta?.TransactionResult)
  return result
}

async function fundWallet(wallet = undefined) {
  if (!(client.url.includes("localhost") || client.url.includes("127.0.0.1"))) {
    // This faucet ignores `destination` and returns a new funded account with
    // its seed, so client.fundWallet() (which expects `account.classicAddress`
    // for the wallet it sent) can't be used.
    if (wallet) throw new Error("Devnet faucet cannot fund an existing wallet")
    const response = await fetch(
      "https://wasm-devnet-faucet.dev.ripplex.io/accounts",
      { method: "POST" },
    )
    if (!response.ok) {
      throw new Error(
        `Faucet request failed: ${response.status} ${await response.text()}`,
      )
    }
    const walletToFund = xrpl.Wallet.fromSeed(
      (await response.json()).account.secret,
    )
    for (let i = 0; i < 30; i++) {
      try {
        if (Number(await client.getXrpBalance(walletToFund.address)) > 0)
          return walletToFund
      } catch {
        // Account not created yet.
      }
      await new Promise((r) => setTimeout(r, 1000))
    }
    throw new Error(`Faucet did not fund ${walletToFund.address} within 30s`)
  }
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

function getBytecodeFromFile(filePath) {
  if (!filePath) {
    console.error("Please provide a file path as a CLI argument.")
    process.exit(1)
  }

  const absolutePath = path.resolve(filePath)
  try {
    const data = fs.readFileSync(absolutePath)
    return data.toString("hex")
  } catch (err) {
    console.error(`Error reading file at ${absolutePath}:`, err.message)
    process.exit(1)
  }
}

async function main() {
  try {
    await client.connect()
    console.log("connected")

    let interval
    if (client.url.includes("localhost") || client.url.includes("127.0.0.1")) {
      interval = setInterval(() => {
        if (client.isConnected()) client.request({ command: "ledger_accept" })
      }, 1000)
    }

    // Generate fresh wallets for this test
    const sourceWallet = await fundWallet()
    const destWallet = await fundWallet()
    console.log(`Source wallet: ${sourceWallet.address}`)

    const args = process.argv.slice(2)
    if (args.length === 0) {
      throw new Error(
        "Please provide a directory path as a command line argument.",
      )
    }
    const targetDir = args[0]
    const wasmSource = args[1]
    const finish = getBytecodeFromFile(wasmSource)

    const { deploy } = require("./deployWasmCode.js")
    const harness = require("./harness.js")

    console.log(`Running test in directory: ${targetDir}`)
    const runTestPath = path.resolve(targetDir, "runTest.js")
    const { test } = require(runTestPath)

    // Dynamically import the test function from the target directory

    const testContext = {
      client,
      submit,
      sourceWallet,
      destWallet,
      fundWallet,
      deploy,
      finish,
      // Shared harness helpers (see tests/harness.js).
      // finishEscrow, expectResult, expectEscrowConsumed, expectEscrowSurvived,
      // getLedgerCloseTime, getLedgerCloseTimeIso, loadWasmHex.
      ...harness,
    }

    let failed = false
    try {
      await test(testContext)
    } catch (error) {
      console.error("Error:", error.message)
      console.log(error)
      failed = true
    } finally {
      if (interval) clearInterval(interval)
      await client.disconnect()
      if (failed) process.exit(1)
    }
  } catch (error) {
    console.error("Error:", error.message)
    process.exit(1)
  }
}

if (require.main === module) {
  main().catch(console.error)
}
