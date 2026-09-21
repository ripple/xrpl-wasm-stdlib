const xrpl = require("xrpl")

const client =
  process.argv.length > 4
    ? new xrpl.Client(process.argv[4])
    : new xrpl.Client("ws://127.0.0.1:6006")

async function submit(tx, wallet, debug = false) {
  const txResult = await client.submitAndWait(tx, { autofill: true, wallet })
  console.log(
    "SUBMITTED " + tx.TransactionType + "(" + txResult.result.hash + ")",
  )

  if (debug) console.log(txResult.result ?? txResult)
  else console.log("Result code: " + txResult.result?.meta?.TransactionResult)
  return txResult
}

/**
 * Deploy a smart-escrow by submitting an `EscrowCreate` transaction.
 *
 * `opts` follows the harness convention: any PascalCase XRPL transaction
 * field (`Amount`, `Data`, `FinishAfter`, `Condition`, `SourceTag`,
 * `DestinationTag`, ...) is spread verbatim onto the tx, so new fields
 * work without touching this helper.
 *
 * The only camelCase key is `cancelAfterOffset` — a harness convenience
 * that's added to the most-recent validated `close_time` to compute
 * `CancelAfter`. Pass `CancelAfter` directly to override it entirely.
 *
 * Defaults:
 *   - `Amount`:            `"100000"`
 *   - `cancelAfterOffset`: `2000` (seconds)
 */
async function deploy(sourceWallet, destWallet, finish, opts = {}) {
  await client.connect()
  console.log("connected")

  const close_time = (
    await client.request({
      command: "ledger",
      ledger_index: "validated",
    })
  ).result.ledger.close_time

  const {
    cancelAfterOffset = 2000,
    Amount = "100000",
    CancelAfter = close_time + cancelAfterOffset,
    ...fields
  } = opts

  const tx = {
    TransactionType: "EscrowCreate",
    Account: sourceWallet.address,
    Destination: destWallet.address,
    Bytecode: finish,
    Amount,
    CancelAfter,
    ...fields,
  }

  const response1 = await submit(tx, sourceWallet)

  if (response1.result.meta.TransactionResult !== "tesSUCCESS") process.exit(1)
  const sequence = response1.result.tx_json.Sequence

  // Extract escrow ledger entry ID from the created escrow node in metadata
  let escrowId = null
  if (response1.result.meta && response1.result.meta.AffectedNodes) {
    for (const node of response1.result.meta.AffectedNodes) {
      if (node.CreatedNode && node.CreatedNode.LedgerEntryType === "Escrow") {
        escrowId = node.CreatedNode.LedgerIndex
        break
      }
    }
  }

  await client.disconnect()

  return { sequence, escrowId }
}

module.exports = { deploy }
