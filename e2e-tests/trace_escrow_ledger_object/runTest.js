async function test(testContext) {
  const { client, finish, submit, sourceWallet, destWallet } = testContext

  // Create a custom escrow with more fields for comprehensive testing
  await client.connect()
  console.log("connected")

  // Create escrow with both Condition and FinishFunction
  // IMPORTANT: Condition must be in full crypto-condition format (39 bytes), not just the hash (32 bytes)
  // Format: A0258020<32-byte-hash>810100
  const condition =
    "A0258020121B69A8D20269CFA850F78931EFF3B1FCF3CCA1982A22D7FDB111734C65E5E3810103"
  const fulfillment = "A0058003736868"

  // close_time is in seconds since Ripple Epoch (Jan 1, 2000 00:00 UTC)
  const close_time = (
    await client.request({
      command: "ledger",
      ledger_index: "closed",
    })
  ).result.ledger.close_time
  const finishAfter = close_time + (process.env.DEVNET ? 3 : 0)

  // Create escrow with optional fields for better test coverage
  // All time values are in seconds since Ripple Epoch
  const escrowCreateTx = {
    TransactionType: "EscrowCreate",
    Account: sourceWallet.address,
    Amount: "1000000",
    Destination: destWallet.address,
    CancelAfter: close_time + 2000, // Can cancel after ~33 minutes
    FinishAfter: finishAfter, // Already passed, can finish immediately
    SourceTag: 11747,
    DestinationTag: 23480,
    Condition: condition,
    FinishFunction: finish,
  }

  const createResponse = await submit(escrowCreateTx, sourceWallet)

  if (createResponse.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error(
      "\nFailed to create escrow:",
      createResponse.result.meta.TransactionResult,
    )
    process.exit(1)
  }

  const offerSequence = createResponse.result.tx_json.Sequence

  // Now finish the escrow
  const tx = {
    TransactionType: "EscrowFinish",
    Account: sourceWallet.address,
    Owner: sourceWallet.address,
    OfferSequence: parseInt(offerSequence),
    Condition: condition,
    Fulfillment: fulfillment,
    ComputationAllowance: 1000000,
  }

  const response = await submit(tx, sourceWallet)

  await client.disconnect()

  if (response.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error(
      "\nFailed to finish escrow:",
      response.result.meta.TransactionResult,
    )
    console.error("Full response:", JSON.stringify(response, null, 2))
    process.exit(1)
  }

  console.log("✅  Successfully finished escrow with FinishFunction")
}

module.exports = { test }
