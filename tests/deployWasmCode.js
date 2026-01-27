const xrpl = require("xrpl")
const fs = require("fs")
const path = require("path")

const client =
  process.argv.length > 4
    ? new xrpl.Client(process.argv[4])
    : new xrpl.Client("ws://127.0.0.1:6006")

async function submit(tx, wallet, debug = false) {
  const txResult = await client.submitAndWait(tx, { autofill: true, wallet })
  console.log("SUBMITTED " + tx.TransactionType)

  if (debug) console.log(txResult.result ?? txResult)
  else console.log("Result code: " + txResult.result?.meta?.TransactionResult)
  return txResult
}

async function deploy(sourceWallet, destWallet, finish, data = null) {
  await client.connect()
  console.log("connected")

  const close_time = (
    await client.request({
      command: "ledger",
      ledger_index: "validated",
    })
  ).result.ledger.close_time

  const response1 = await submit(
    {
      TransactionType: "EscrowCreate",
      Account: sourceWallet.address,
      Amount: "100000",
      Destination: destWallet.address,
      CancelAfter: close_time + 2000,
      FinishFunction: finish,
      Data: data,
    },
    sourceWallet,
  )

  if (response1.result.meta.TransactionResult !== "tesSUCCESS") process.exit(1)
  const sequence = response1.result.tx_json.Sequence

  // Extract escrow keylet from the created escrow node in metadata
  let escrowKeylet = null
  if (response1.result.meta && response1.result.meta.AffectedNodes) {
    for (const node of response1.result.meta.AffectedNodes) {
      if (node.CreatedNode && node.CreatedNode.LedgerEntryType === "Escrow") {
        escrowKeylet = node.CreatedNode.LedgerIndex
        break
      }
    }
  }

  await client.disconnect()

  return { sequence, escrowKeylet }
}

module.exports = { deploy }
